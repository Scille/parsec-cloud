# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
import sys
import attr
import json
import ssl
import trustme
import trio
from contextlib import contextmanager, asynccontextmanager
import tempfile

from parsec.core.types import BackendAddr
from parsec.backend import backend_app_factory
from parsec.backend.config import BackendConfig, MockedEmailConfig, MockedBlockStoreConfig

from tests.common.freeze_time import freeze_time
from tests.common.helpers import addr_with_device_subdomain
from tests.common.timeout import real_clock_timeout
from tests.common.tcp import offline


@attr.s
class AppServer:
    entry_point = attr.ib()
    addr = attr.ib()
    connection_factory = attr.ib()

    def offline(self):
        return offline(self.addr)


@pytest.fixture
def server_factory(tcp_stream_spy):
    count = 0

    @asynccontextmanager
    async def _server_factory(entry_point, addr=None):
        nonlocal count
        count += 1

        if not addr:
            addr = BackendAddr(hostname=f"server-{count}.localhost", port=9999, use_ssl=False)

        async def _serve_client(stream):
            if addr.use_ssl:
                ssl_context = ssl.create_default_context()
                stream = trio.SSLStream(
                    stream, ssl_context, server_hostname=addr.hostname, server_side=True
                )
            await entry_point(stream)

        async with trio.open_service_nursery() as nursery:

            def connection_factory(*args, **kwargs):
                client_stream, server_stream = trio.testing.memory_stream_pair()
                nursery.start_soon(_serve_client, server_stream)
                return client_stream

            tcp_stream_spy.push_hook(addr.hostname, addr.port, connection_factory)
            try:
                yield AppServer(entry_point, addr, connection_factory)
                nursery.cancel_scope.cancel()

            finally:
                # It's important to remove the hook just after having cancelled
                # the nursery. Otherwise another coroutine trying to connect would
                # end up with a `RuntimeError('Nursery is closed to new arrivals',)`
                # given `connection_factory` make use of the now-closed nursery.
                tcp_stream_spy.pop_hook(addr.hostname, addr.port)

    return _server_factory


@pytest.fixture()
def backend_addr(tcp_stream_spy, fixtures_customization, monkeypatch):
    # Depending on tcp_stream_spy fixture prevent from doing real connection
    # attempt (which can be long to resolve) when backend is not running
    use_ssl = fixtures_customization.get("backend_over_ssl", False)
    addr = BackendAddr(hostname="example.com", port=9999, use_ssl=use_ssl)
    if use_ssl:
        # TODO: Trustme & Windows doesn't seem to play well
        # (that and Python < 3.7 & Windows bug https://bugs.python.org/issue35941)
        if sys.platform == "win32":
            pytest.skip("Windows and Trustme are not friends :'(")

        # Create a ssl certificate and overload default ssl context generation
        ca = trustme.CA()
        cert = ca.issue_cert("*.example.com", "example.com")
        vanilla_create_default_context = ssl.create_default_context

        def patched_create_default_context(*args, **kwargs):
            ctx = vanilla_create_default_context(*args, **kwargs)
            ca.configure_trust(ctx)
            cert.configure_cert(ctx)  # TODO: only server should load this part ?
            return ctx

        monkeypatch.setattr("ssl.create_default_context", patched_create_default_context)

    return addr


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
    initial_user_manifest_state,
    blockstore,
    backend_store,
    fixtures_customization,
):
    # Given the postgresql driver uses trio-asyncio, any coroutine dealing with
    # the backend should inherit from the one with the asyncio loop context manager.
    # This mean the nursery fixture cannot use the backend object otherwise we
    # can end up in a dead lock if the asyncio loop is torndown before the
    # nursery fixture is done with calling the backend's postgresql stuff.

    @asynccontextmanager
    async def _backend_factory(populated=True, config={}, event_bus=None):
        ssl_context = fixtures_customization.get("backend_over_ssl", False)
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
                "debug": False,
                "db_url": backend_store,
                "blockstore_config": blockstore,
                "email_config": None,
                "backend_addr": None,
                "forward_proto_enforce_https": None,
                "ssl_context": ssl_context if ssl_context else False,
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

            yield backend

    return _backend_factory


@pytest.fixture
async def backend(backend_factory, fixtures_customization, backend_addr):
    populated = not fixtures_customization.get("backend_not_populated", False)
    config = {}
    tmpdir = tempfile.mkdtemp(prefix="tmp-email-folder-")
    config["email_config"] = MockedEmailConfig(sender="Parsec <no-reply@parsec.com>", tmpdir=tmpdir)
    config["backend_addr"] = backend_addr
    if fixtures_customization.get("backend_spontaneous_organization_boostrap", False):
        config["organization_spontaneous_bootstrap"] = True
    if fixtures_customization.get("backend_has_webhook", False):
        # Invalid port, hence we should crash if by mistake we try to reach this url
        config["organization_bootstrap_webhook_url"] = "http://example.com:888888/webhook"
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


@pytest.fixture
def running_backend_ready(request):
    # Useful to synchronize other fixtures that need to connect to
    # the backend if it is available
    event = trio.Event()
    # Nothing to wait if current test doesn't use `running_backend` fixture
    if "running_backend" not in request.fixturenames:
        event.set()

    return event


@pytest.fixture
async def running_backend(server_factory, backend_addr, backend, running_backend_ready):

    async with server_factory(backend.handle_client, backend_addr) as server:
        server.backend = backend

        def _offline_for(device_id):
            return offline(addr_with_device_subdomain(server.addr, device_id))

        server.offline_for = _offline_for

        running_backend_ready.set()
        yield server
