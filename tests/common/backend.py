# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import json
import socket
import ssl
import sys
import tempfile
from contextlib import asynccontextmanager, contextmanager
from dataclasses import dataclass
from functools import partial
from inspect import iscoroutine
from typing import AsyncContextManager, Callable, Optional, Union

import pytest
import trio
import trustme
from hypercorn.config import Config as HyperConfig
from hypercorn.trio.run import worker_serve
from hypercorn.trio.tcp_server import TCPServer
from hypercorn.trio.worker_context import WorkerContext
from quart.typing import TestClientProtocol
from quart_trio import QuartTrio

from parsec.backend import backend_app_factory
from parsec.backend.app import BackendApp
from parsec.backend.asgi import app_factory
from parsec.backend.config import BackendConfig, MockedBlockStoreConfig, MockedEmailConfig
from parsec.core.types import BackendAddr, LocalDevice
from tests.common.binder import OrganizationFullData
from tests.common.freeze_time import freeze_time
from tests.common.trio_clock import real_clock_timeout


@pytest.fixture(scope="session")
def unused_tcp_port():
    """Find an unused localhost TCP port from 1024-65535 and return it."""
    sock = socket.socket()
    sock.bind(("127.0.0.1", 0))
    port = sock.getsockname()[1]
    # On macOS connecting to a bind-to-no-listening socket hangs, so we have to
    # close the socket and risk port recycling (which hopefully is not that common)
    if sys.platform == "darwin":
        sock.close()
    yield port


def correct_addr(
    to_correct: Union[BackendAddr, LocalDevice, OrganizationFullData], port: int
) -> BackendAddr:
    """
    Helper to fix a backend address so that it will reach the current server.
    This not needed when using `running_backend` (given in this case the
    alice/bob/coolorg etc. fixtures are created with the correct port), but
    must be used when the test has to manually start server (e.g. in
    the hypothesis tests)
    """
    if isinstance(to_correct, LocalDevice):
        return LocalDevice(
            organization_addr=correct_addr(to_correct.organization_addr, port),
            device_id=to_correct.device_id,
            device_label=to_correct.device_label,
            human_handle=to_correct.human_handle,
            signing_key=to_correct.signing_key,
            private_key=to_correct.private_key,
            profile=to_correct.profile,
            user_manifest_id=to_correct.user_manifest_id,
            user_manifest_key=to_correct.user_manifest_key,
            local_symkey=to_correct.local_symkey,
        )
    elif isinstance(to_correct, OrganizationFullData):
        return OrganizationFullData(
            bootstrap_addr=correct_addr(to_correct.bootstrap_addr, port),
            addr=correct_addr(to_correct.addr, port),
            root_signing_key=to_correct.root_signing_key,
        )
    else:
        # Consider it's a regular addr
        *_, to_keep = to_correct.to_url().removeprefix("parsec://").split("/", 1)
        url = f"parsec://127.0.0.1:{port}/{to_keep}"
        return to_correct.__class__.from_url(url)


@asynccontextmanager
async def server_factory(handle_client: Callable):
    async with trio.open_service_nursery() as nursery:

        listeners = await nursery.start(
            partial(
                trio.serve_tcp,
                handle_client,
                port=0,
                host="127.0.0.1",
                handler_nursery=nursery,
            )
        )
        yield listeners[0].socket.getsockname()[1]

        nursery.cancel_scope.cancel()


BackendFactory = Callable[..., AsyncContextManager[BackendApp]]


@pytest.fixture
def backend_factory(
    asyncio_loop,
    event_bus_factory,
    backend_data_binder_factory,
    coolorg,
    expiredorg,
    otherorg,
    alice,
    alice2,
    expiredorgalice,
    otheralice,
    adam,
    bob,
    blockstore,
    backend_store,
    fixtures_customization,
) -> BackendFactory:
    # Given the postgresql driver uses trio-asyncio, any coroutine dealing with
    # the backend should inherit from the one with the asyncio loop context manager.
    # This mean the nursery fixture cannot use the backend object otherwise we
    # can end up in a dead lock if the asyncio loop is torndown before the
    # nursery fixture is done with calling the backend's postgresql stuff.

    @asynccontextmanager
    async def _backend_factory(
        populated=True, config={}, event_bus=None
    ) -> AsyncContextManager[BackendApp]:
        nonlocal backend_store, blockstore
        if fixtures_customization.get("backend_force_mocked"):
            backend_store = "MOCKED"
            assert fixtures_customization.get("blockstore_mode", "NO_RAID") == "NO_RAID"
            blockstore = MockedBlockStoreConfig()

        config = BackendConfig(
            **{
                "administration_token": "s3cr3t",
                "db_min_connections": 1,
                "db_max_connections": 5,
                "debug": True,
                "db_url": backend_store,
                "sse_keepalive": 30,
                "blockstore_config": blockstore,
                "email_config": None,
                "backend_addr": None,
                "forward_proto_enforce_https": None,
                "organization_bootstrap_webhook_url": None,
                "organization_spontaneous_bootstrap": False,
                **config,
            }
        )

        if not event_bus:
            event_bus = event_bus_factory()
        async with backend_app_factory(config, event_bus=event_bus) as backend:
            if populated:
                with freeze_time("2000-01-01"):
                    binder = backend_data_binder_factory(backend)
                    await binder.bind_organization(
                        coolorg,
                        alice,
                        initial_user_manifest=fixtures_customization.get(
                            "alice_initial_remote_user_manifest", "v1"
                        ),
                    )
                    await binder.bind_organization(expiredorg, expiredorgalice)
                    await backend.organization.update(expiredorg.organization_id, is_expired=True)
                    await binder.bind_organization(otherorg, otheralice)
                    await binder.bind_device(alice2, certifier=alice)
                    await binder.bind_device(
                        adam,
                        certifier=alice2,
                        initial_user_manifest=fixtures_customization.get(
                            "adam_initial_remote_user_manifest", "v1"
                        ),
                    )
                    await binder.bind_device(
                        bob,
                        certifier=adam,
                        initial_user_manifest=fixtures_customization.get(
                            "bob_initial_remote_user_manifest", "v1"
                        ),
                    )
                    if fixtures_customization.get("adam_is_revoked", False):
                        await binder.bind_revocation(adam.user_id, certifier=alice)

            yield backend

    return _backend_factory


@pytest.fixture
async def backend(unused_tcp_port, backend_factory, fixtures_customization, backend_addr):
    populated = not fixtures_customization.get("backend_not_populated", False)
    config = {}
    tmpdir = tempfile.mkdtemp(prefix="tmp-email-folder-")
    config["email_config"] = MockedEmailConfig(sender="Parsec <no-reply@parsec.com>", tmpdir=tmpdir)
    config["backend_addr"] = backend_addr
    if fixtures_customization.get("backend_spontaneous_organization_bootstrap", False):
        config["organization_spontaneous_bootstrap"] = True
    if fixtures_customization.get("backend_has_webhook", False):
        # Invalid port, hence we should crash if by mistake we try to reach this url
        config["organization_bootstrap_webhook_url"] = f"http://127.0.0.1:{unused_tcp_port}/webhook"
    forward_proto_enforce_https = fixtures_customization.get("backend_forward_proto_enforce_https")
    if forward_proto_enforce_https:
        config["forward_proto_enforce_https"] = forward_proto_enforce_https
    async with backend_factory(populated=populated, config=config) as backend:
        yield backend


@pytest.fixture
def backend_data_binder(backend, backend_data_binder_factory):
    return backend_data_binder_factory(backend)


class LetterBox:
    def __init__(self):
        self._send_email, self._recv_email = trio.open_memory_channel(10)
        self.emails = []

    async def get_next_with_timeout(self):
        async with real_clock_timeout():
            return await self.get_next()

    async def get_next(self):
        return await self._recv_email.receive()

    def _push(self, to_addr, message):
        email = (to_addr, message)
        self._send_email.send_nowait(email)
        self.emails.append(email)


@pytest.fixture
def email_letterbox(monkeypatch):
    letterbox = LetterBox()

    async def _mocked_send_email(email_config, to_addr, message):
        letterbox._push(to_addr, message)

    monkeypatch.setattr("parsec.backend.invite.send_email", _mocked_send_email)
    return letterbox


@pytest.fixture
def webhook_spy(monkeypatch):
    events = []

    class MockedRep:
        @property
        def status(self):
            return 200

    @contextmanager
    def _mock_urlopen(req, **kwargs):
        # Webhook are alway POST with utf-8 JSON body
        assert req.method == "POST"
        assert req.headers == {"Content-type": "application/json; charset=utf-8"}
        cooked_data = json.loads(req.data.decode("utf-8"))
        events.append((req.full_url, cooked_data))
        yield MockedRep()

    monkeypatch.setattr("parsec.backend.webhooks.urlopen", _mock_urlopen)
    return events


# `running_backend` is a really useful fixture, but comes with it own issue:
# - the TCP port on which the backend is running is only known once we have started the server
# - we need the TCP port to determine the backend address
# - we need the backend address to create the alice/bob/coolorg etc. fixtures
# - those fixtures are needed to populate the backend
# - the running_backend fixture should only resolve once the backend is populated and running
#
# So to break this dependency loop, we introduce `running_backend_sockets`
# fixture which starts a server (hence getting it TCP port) but allow us to plug the
# actual backend server code later on.
#
# On top of that, we have the `running_backend_ready` that allows other fixtures (typically
# the core related ones) to be able to ensure the server is up and running (the fixtures
# cannot directly depend on the `running_backend` fixture for this given it should be
# up to the test to decide whether or not the backend should be running)
#
# In case `running_backend` is among the test's fixtures:
# <test>
# ├─ running_backend
# |  ├─ backend
# |  |  └─ backend_factory
# |  |     └─ alice/bob/coolorg etc.
# |  |        └─ backend_addr  <- wait port to be known
# |  |           └─ running_backend_port_known
# |  ├─ running_backend_sockets  <- signal port is known
# |  |  └─ running_backend_port_known
# |  └─ running_backend_ready
# └─ core  <- wait for running_backend before it own init
#    └─ running_backend_ready
#
# In case `running_backend` is not among the test's fixtures:
# <test>
# ├─ core
# |  └─ running_backend_ready <- nothing to wait for !
# └─ alice/bob/coolorg etc.
#    └─ backend_addr
#       └─ running_backend_port_known
#          └─ unused_tcp_port


@pytest.fixture
def running_backend_ready(request):
    # Useful to synchronize other fixtures that need to connect to
    # the backend if it is available
    event = trio.Event()

    # Called by `running_backend` fixture once port is known and ASGI app resolved
    def _set_running_backend_ready() -> None:
        assert not event.is_set()
        event.set()

    async def _wait_running_backend_ready() -> None:
        await event.wait()

    # Nothing to wait if current test doesn't use `running_backend` fixture
    if "running_backend" not in request.fixturenames:
        _set_running_backend_ready()

    # Only accessed by `running_backend` fixture
    _wait_running_backend_ready._set_running_backend_ready = _set_running_backend_ready

    return _wait_running_backend_ready


@pytest.fixture
def running_backend_port_known(request, unused_tcp_port):
    # Useful to synchronize other fixtures that need to connect to
    # the backend if it is available
    # This is also needed to create backend addr with a valid port
    event = trio.Event()
    backend_port = None

    # Called by `running_backend`` fixture once port is known
    def _set_running_backend_port_known(port: int) -> None:
        nonlocal backend_port
        assert not event.is_set()
        event.set()
        backend_port = port

    async def _wait_running_backend_port_known() -> int:
        nonlocal backend_port
        await event.wait()
        assert backend_port is not None
        return backend_port

    # Nothing to wait if current test doesn't use `running_backend` fixture
    if "running_backend" not in request.fixturenames:
        _set_running_backend_port_known(unused_tcp_port)

    # Only accessed by `running_backend` fixture
    _wait_running_backend_port_known._set_running_backend_port_known = (
        _set_running_backend_port_known
    )

    return _wait_running_backend_port_known


@pytest.fixture
async def async_fixture_backend_addr(running_backend_port_known, fixtures_customization):
    port = await running_backend_port_known()
    use_ssl = fixtures_customization.get("backend_over_ssl", False)
    return BackendAddr(hostname="127.0.0.1", port=port, use_ssl=use_ssl)


@pytest.fixture
def backend_addr(async_fixture_backend_addr, unused_tcp_port):
    # Given server port is not known until `running_backend_sockets`
    # is ready, `backend_addr` should be an asynchronous fixture.
    # However `backend_addr` is a really common fixture and we want to be able
    # to use it event in the non-trio tests (for instance in hypothesis tests).
    # So we cheat by pretending `backend_addr` is a sync fixture and fallback to
    # a default addr value if we are in a non-trio test.
    if iscoroutine(async_fixture_backend_addr):
        # We are in a non-trio test, just close the coroutine and provide
        # an addr which is guaranteed to cause connection error
        async_fixture_backend_addr.close()
        # `use_ssl=False` is useful if this address is later modified by `correct_addr`
        return BackendAddr(hostname="127.0.0.1", port=unused_tcp_port, use_ssl=False)
    else:
        return async_fixture_backend_addr


# Generating CA & Certificate are costly, so cache them once created
_ca_and_cert = None


def get_ca_and_cert():
    global _ca_and_cert
    if _ca_and_cert is None:
        _ca = trustme.CA()
        _cert = _ca.issue_cert("127.0.0.1")
        _ca_and_cert = (_ca, _cert)
    return _ca_and_cert


class AsgiOfflineMiddleware:
    """
    Wrap an ASGI App to be able to simulate as if it were offline
    """

    def __init__(self, asgi_app):
        self.asgi_app = asgi_app
        self._offline_ports = set()
        self._offline_watchdogs_parking = trio.lowlevel.ParkingLot()

    async def __call__(self, scope, receive, send):
        # Special case for lifespan given it corresponds to server init and not
        # to an incoming client connection
        if scope["type"] == "lifespan":
            return await self.asgi_app(scope, receive, send)

        port = scope["server"][1]

        if port in self._offline_ports:
            return

        else:

            async def _offline_watchdog(cancel_scope):
                while True:
                    await self._offline_watchdogs_parking.park()
                    if port in self._offline_ports:
                        break
                cancel_scope.cancel()

            async with trio.open_nursery() as nursery:
                nursery.start_soon(_offline_watchdog, nursery.cancel_scope)
                await self.asgi_app(scope, receive, send)
                nursery.cancel_scope.cancel()

    @contextmanager
    def offline(self, port: int):
        assert port not in self._offline_ports
        self._offline_ports.add(port)
        self._offline_watchdogs_parking.unpark_all()
        try:
            yield
        finally:
            self._offline_ports.remove(port)
            # No need to unpark given our port has just been re-authorized !

    def __getattr__(self, val):
        return getattr(self.asgi_app, val)


@pytest.fixture
def hyper_config(monkeypatch, fixtures_customization):
    # Create a ssl certificate and overload default ssl context generation
    if fixtures_customization.get("backend_over_ssl", False):
        ca, cert = get_ca_and_cert()
        vanilla_create_default_context = ssl.create_default_context

        def patched_create_default_context(*args, **kwargs):
            ctx = vanilla_create_default_context(*args, **kwargs)
            ca.configure_trust(ctx)
            cert.configure_cert(ctx)  # TODO: only server should load this part ?
            return ctx

        monkeypatch.setattr("ssl.create_default_context", patched_create_default_context)

        use_ssl = True
    else:
        use_ssl = False

    # Create the empty ASGI app and serve it on TCP
    hyper_config = HyperConfig.from_mapping(
        {
            "bind": [f"127.0.0.1:0"],
            # "accesslog": logging.getLogger("hypercorn.access"),
            # "errorlog": logging.getLogger("hypercorn.error"),
            # "certfile": str(ssl_certfile) if ssl_certfile else None,
            # "keyfile": str(ssl_keyfile) if ssl_certfile else None,
        }
    )
    # Sanity checks
    assert hyper_config.ssl_enabled is use_ssl
    return hyper_config


@pytest.fixture
async def running_backend_sockets(hyper_config, running_backend_port_known):
    try:
        sockets = hyper_config.create_sockets()
        # Sanity checks
        if hyper_config.ssl_enabled:
            assert len(sockets.secure_sockets) == 1
            assert len(sockets.insecure_sockets) == 0
            sock = sockets.secure_sockets[0]
        else:
            assert len(sockets.secure_sockets) == 0
            assert len(sockets.insecure_sockets) == 1
            sock = sockets.insecure_sockets[0]

        sock.listen(hyper_config.backlog)
        port = sock.getsockname()[1]

    except Exception:
        # The fixture is broken, we must resolve `running_backend_port_known`
        # before leaving otherwise pytest-trio will hang forever given it waits
        # for all fixtures to finish intialization before re-raising exceptions
        running_backend_port_known._set_running_backend_port_known(9999)
        raise

    else:
        running_backend_port_known._set_running_backend_port_known(port)

    return sockets


@pytest.fixture
def backend_asgi_app(backend):
    # This fixture doesn't seem much, but it helps making sure we have only one
    # instance of the ASGI server
    return AsgiOfflineMiddleware(app_factory(backend))


def asgi_app_handle_client_factory(asgi_app: QuartTrio) -> Callable:
    asgi_config = HyperConfig()
    asgi_worker_context = WorkerContext()

    async def _handle_client(stream: trio.abc.Stream) -> None:
        return await TCPServer(
            app=asgi_app, config=asgi_config, context=asgi_worker_context, stream=stream
        )

    return _handle_client


@pytest.fixture
def backend_asgi_app_handle_client(backend_asgi_app):
    return asgi_app_handle_client_factory(backend_asgi_app)


@pytest.fixture
def asgi_server_factory(hyper_config):
    @asynccontextmanager
    async def asgi_server_factory(asgi_app):
        async with trio.open_nursery() as nursery:
            yield await nursery.start(partial(worker_serve, app=asgi_app, config=hyper_config))
            nursery.cancel_scope.cancel()

    return asgi_server_factory


@dataclass
class RunningBackend:
    asgi_app: QuartTrio
    backend: BackendApp
    addr: BackendAddr

    def offline(self, port: Optional[int] = None):
        return self.asgi_app.offline(port or self.addr.port)

    def correct_addr(self, addr: BackendAddr | LocalDevice) -> BackendAddr:
        return correct_addr(addr, self.addr.port)

    async def connection_factory(self) -> trio.abc.Stream:
        return await trio.open_tcp_stream(self.addr.hostname, self.addr.port)

    def test_client(self, use_cookies: bool = True) -> TestClientProtocol:
        """Creates and returns a test client"""
        return self.asgi_app.test_client(use_cookies)


RunningBackendFactory = Callable[..., AsyncContextManager[RunningBackend]]


@pytest.fixture
def running_backend_factory(asyncio_loop, hyper_config) -> RunningBackendFactory:
    # `asyncio_loop` is already declared by `backend_factory` (since it's only the
    # backend that need an asyncio loop for postgresql stuff), however this is not
    # enough here given we create the server *before* `backend_factory` is required

    @asynccontextmanager
    async def _running_backend_factory(
        backend, sockets=None
    ) -> AsyncContextManager[RunningBackend]:
        asgi_app = AsgiOfflineMiddleware(app_factory(backend))
        async with trio.open_nursery() as nursery:
            binds = await nursery.start(
                partial(worker_serve, app=asgi_app, config=hyper_config, sockets=sockets)
            )
            port = int(binds[0].rsplit(":", 1)[1])

            yield RunningBackend(
                asgi_app=asgi_app,
                backend=backend,
                addr=BackendAddr(hostname="127.0.0.1", port=port, use_ssl=hyper_config.ssl_enabled),
            )

            nursery.cancel_scope.cancel()

    return _running_backend_factory


@pytest.fixture
async def running_backend(
    running_backend_sockets, running_backend_ready, running_backend_factory, backend
):
    async with running_backend_factory(backend, sockets=running_backend_sockets) as rb:
        running_backend_ready._set_running_backend_ready()
        yield rb
