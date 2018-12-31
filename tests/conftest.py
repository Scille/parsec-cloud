import pytest
import attr
import os
import socket
import asyncpg
import contextlib
from unittest.mock import patch
import trio
import trio_asyncio
from async_generator import asynccontextmanager
import hypothesis
from pathlib import Path

from parsec.types import DeviceID, BackendOrganizationAddr
from parsec.crypto import SigningKey, export_root_verify_key, encrypt_with_secret_key
from parsec.trustchain import certify_user, certify_device
from parsec.core import CoreConfig
from parsec.core.types import LocalDevice
from parsec.core.devices_manager import generate_new_device
from parsec.core.logged_core import logged_core_factory
from parsec.core.fs.utils import new_local_user_manifest, local_to_remote_manifest, copy_manifest
from parsec.core.schemas import dumps_manifest
from parsec.core.mountpoint import FUSE_AVAILABLE
from parsec.backend import BackendApp, config_factory as backend_config_factory
from parsec.backend.user import User as BackendUser, new_user_factory as new_backend_user_factory
from parsec.backend.user import UserAlreadyExistsError
from parsec.api.protocole import ClientHandshake, AnonymousClientHandshake
from parsec.api.transport import Transport

# TODO: needed ?
pytest.register_assert_rewrite("tests.event_bus_spy")

from tests.common import freeze_time, FreezeTestOnTransportError
from tests.open_tcp_stream_mock_wrapper import OpenTCPStreamMockWrapper
from tests.event_bus_spy import SpiedEventBus

from tests.oracles import oracle_fs_factory, oracle_fs_with_sync_factory  # noqa: Republishing


def pytest_addoption(parser):
    parser.addoption("--hypothesis-max-examples", default=100, type=int)
    parser.addoption("--hypothesis-derandomize", action="store_true")
    parser.addoption(
        "--postgresql",
        action="store_true",
        help="Use PostgreSQL backend instead of default memory mock",
    )
    parser.addoption("--runslow", action="store_true", help="Don't skip slow tests")
    parser.addoption("--runfuse", action="store_true", help="Don't skip fuse tests")
    parser.addoption(
        "--realcrypto", action="store_true", help="Don't mock crypto operation to save time"
    )


@pytest.fixture(scope="session")
def hypothesis_settings(request):
    return hypothesis.settings(
        max_examples=pytest.config.getoption("--hypothesis-max-examples"),
        derandomize=pytest.config.getoption("--hypothesis-derandomize"),
    )


def pytest_runtest_setup(item):
    # Mock and non-UTC timezones are a really bad mix, so keep things simple
    os.environ.setdefault("TZ", "UTC")
    if item.get_closest_marker("slow") and not item.config.getoption("--runslow"):
        pytest.skip("need --runslow option to run")
    if item.get_closest_marker("fuse"):
        if not item.config.getoption("--runfuse"):
            pytest.skip("need --runfuse option to run")
        elif not FUSE_AVAILABLE:
            pytest.skip("fuse is not available")


@pytest.fixture(autouse=True, scope="session", name="unmock_crypto")
def mock_crypto():
    # Crypto is CPU hungry
    if pytest.config.getoption("--realcrypto"):

        def unmock():
            pass

        yield unmock

    else:

        def unsecure_but_fast_argon2i_kdf(size, password, salt, *args, **kwargs):
            data = password + salt
            return data[:size] + b"\x00" * (size - len(data))

        from parsec.crypto import argon2i

        vanilla_kdf = argon2i.kdf

        def unmock():
            return patch("parsec.crypto.argon2i.kdf", new=vanilla_kdf)

        with patch("parsec.crypto.argon2i.kdf", new=unsecure_but_fast_argon2i_kdf):
            yield unmock


@pytest.fixture
def realcrypto(unmock_crypto):
    with unmock_crypto():
        yield


def _patch_url_if_xdist(url):
    xdist_worker = os.environ.get("PYTEST_XDIST_WORKER")
    if xdist_worker:
        return f"{url}_{xdist_worker}"
    else:
        return url


# Use current unix user's credential, don't forget to do
# `psql -c 'CREATE DATABASE parsec_test;'` prior to run tests
DEFAULT_POSTGRESQL_TEST_URL = "postgresql:///parsec_test"


def _get_postgresql_url():
    return _patch_url_if_xdist(
        os.environ.get("PARSEC_POSTGRESQL_TEST_URL", DEFAULT_POSTGRESQL_TEST_URL)
    )


@pytest.fixture(scope="session")
def postgresql_url():
    if not pytest.config.getoption("--postgresql"):
        pytest.skip("`--postgresql` option not provided")
    return _get_postgresql_url()


@pytest.fixture
async def asyncio_loop():
    # When a ^C happens, trio send a Cancelled exception to each running
    # coroutine. We must protect this one to avoid deadlock if it is cancelled
    # before another coroutine that uses trio-asyncio.
    with trio.open_cancel_scope(shield=True):
        async with trio_asyncio.open_loop() as loop:
            yield loop


@pytest.fixture(scope="session")
def unused_tcp_port():
    """Find an unused localhost TCP port from 1024-65535 and return it."""
    with contextlib.closing(socket.socket()) as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


@pytest.fixture(scope="session")
def unused_tcp_addr(unused_tcp_port):
    return "ws://127.0.0.1:%s/foo" % unused_tcp_port


@pytest.fixture(scope="session")
def event_bus_factory():
    return SpiedEventBus


@pytest.fixture
def event_bus(event_bus_factory):
    return event_bus_factory()


@pytest.fixture
def tcp_stream_spy():
    open_tcp_stream_mock_wrapper = OpenTCPStreamMockWrapper()
    with patch("trio.open_tcp_stream", new=open_tcp_stream_mock_wrapper):
        yield open_tcp_stream_mock_wrapper


@pytest.fixture(scope="session")
def monitor():
    from tests.monitor import Monitor

    return Monitor()


@attr.s
class AppServer:
    entry_point = attr.ib()
    addr = attr.ib()
    connection_factory = attr.ib()


@pytest.fixture
def server_factory(tcp_stream_spy):
    count = 0

    @asynccontextmanager
    async def _server_factory(entry_point, url=None):
        nonlocal count
        count += 1

        if not url:
            url = f"ws://server-{count}.localhost:9999"

        try:
            async with trio.open_nursery() as nursery:

                def connection_factory(*args, **kwargs):
                    right, left = trio.testing.memory_stream_pair()
                    nursery.start_soon(entry_point, left)
                    return right

                tcp_stream_spy.push_hook(url, connection_factory)

                yield AppServer(entry_point, url, connection_factory)

                nursery.cancel_scope.cancel()

        finally:
            tcp_stream_spy.pop_hook(url)

    return _server_factory


@pytest.fixture(scope="session")
def alice(backend_addr, root_verify_key):
    return generate_new_device(DeviceID("alice@dev1"), backend_addr, root_verify_key)


@pytest.fixture(scope="session")
def alice2(backend_addr, root_verify_key, alice):
    alice2 = generate_new_device(DeviceID("alice@dev2"), backend_addr, root_verify_key)
    alice2 = alice2.evolve(
        private_key=alice.private_key, user_manifest_access=alice.user_manifest_access
    )
    return alice2


@pytest.fixture(scope="session")
def bob(backend_addr, root_verify_key):
    return generate_new_device(DeviceID("bob@dev1"), backend_addr, root_verify_key)


@pytest.fixture(scope="session")
def mallory(backend_addr, root_verify_key):
    return generate_new_device(DeviceID("mallory@dev1"), backend_addr, root_verify_key)


@pytest.fixture
def default_devices(alice, alice2, bob):
    return (alice, alice2, bob)


@pytest.fixture
def bootstrap_postgresql(url):
    # In theory we should use TrioPG here to do db init, but:
    # - Duck typing and similar api makes `_init_db` compatible with both
    # - AsyncPG should be slightly faster than TrioPG
    # - Most important: a trio loop is potentially already started inside this
    #   thread (i.e. if the test is mark as trio). Hence we would have to spawn
    #   another thread just to run the new trio loop.

    import asyncio
    import asyncpg
    from parsec.backend.drivers.postgresql.handler import _init_db

    async def _bootstrap():
        conn = await asyncpg.connect(url)
        await _init_db(conn, force=True)
        await conn.close()

    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    loop.run_until_complete(_bootstrap())


@pytest.fixture()
def backend_store(request):
    if pytest.config.getoption("--postgresql"):
        if request.node.get_closest_marker("slow"):
            import warnings

            warnings.warn(
                "TODO: trio-asyncio loop currently incompatible with hypothesis tests :'("
            )
            return "MOCKED"

        # TODO: would be better to create a new postgresql cluster for each test
        url = _get_postgresql_url()
        try:
            bootstrap_postgresql(url)
        except asyncpg.exceptions.InvalidCatalogNameError as exc:
            raise RuntimeError(
                "Is `parsec_test` a valid database in PostgreSQL ?\n"
                "Running `psql -c 'CREATE DATABASE parsec_test;'` may fix this"
            ) from exc
        return url

    elif request.node.get_closest_marker("postgresql"):
        pytest.skip("`Test is postgresql-only")

    else:
        return "MOCKED"


@pytest.fixture
def blockstore(request, backend_store):
    blockstore_config = {}
    # TODO: allow to test against swift ?
    if backend_store.startswith("postgresql://"):
        blockstore_type = "POSTGRESQL"
    else:
        blockstore_type = "MOCKED"

    # More or less a hack to be able to to configure this fixture from
    # the test function by adding tags to it
    if request.node.get_closest_marker("raid1_blockstore"):
        blockstore_config["RAID1_0_TYPE"] = blockstore_type
        blockstore_config["RAID1_1_TYPE"] = "MOCKED"
        blockstore_type = "RAID1"
    if request.node.get_closest_marker("raid0_blockstore"):
        blockstore_config["RAID0_0_TYPE"] = blockstore_type
        blockstore_config["RAID0_1_TYPE"] = "MOCKED"
        blockstore_type = "RAID0"

    return blockstore_type, blockstore_config


@pytest.fixture
async def nursery():
    # A word about the nursery fixture:
    # The whole point of trio is to be able to build a graph of coroutines to
    # simplify teardown. Using a single top level nursery kind of mitigate this
    # given unrelated coroutines will end up there and be closed all together.
    # Worst, among those coroutine it could exists a relationship that will be lost
    # in a more or less subtle way (typically using a factory fixture that use the
    # default nursery behind the scene).
    # Bonus points occur if using trio-asyncio that creates yet another hidden
    # layer of relationship that could end up in cryptic dead lock hardened enough
    # to survive ^C.
    # Finally if your still no convinced, factory fixtures not depending on async
    # fixtures (like nursery is) can be used inside the Hypothesis tests.
    # I know you love Hypothesis. Checkmate. You won't use this fixture ;-)
    raise RuntimeError("Bad kitty ! Bad !!!")


class InitialState:
    def __init__(self):
        self._location_in_v0 = set()
        self._v1 = {}

    def set_v0_in_backend(self, user_id):
        assert user_id not in self._location_in_v0
        self._location_in_v0.add(user_id)

    def set_v0_in_device(self, device_id):
        assert device_id not in self._location_in_v0
        self._location_in_v0.add(device_id)

    def get_initial_for_device(self, device):
        if device.device_id in self._location_in_v0:
            return None

        in_device, _ = self._generate_or_retrieve_v1(device)
        return copy_manifest(in_device)

    def get_initial_for_backend(self, device):
        if device.device_id in self._location_in_v0:
            return None

        _, in_backend = self._generate_or_retrieve_v1(device)
        return copy_manifest(in_backend)

    def _generate_or_retrieve_v1(self, device):
        try:
            return self._v1[device.user_id]

        except KeyError:
            user_manifest = new_local_user_manifest(device.device_id)
            user_manifest["base_version"] = 1
            user_manifest["is_placeholder"] = False
            user_manifest["need_sync"] = False

            remote_user_manifest = local_to_remote_manifest(user_manifest)

            self._v1[device.user_id] = (user_manifest, remote_user_manifest)
            return (user_manifest, remote_user_manifest)


@pytest.fixture
def initial_user_manifest_state():
    # User manifest is stored in backend vlob and in devices's local db.
    # Hence this fixture allow us to centralize the first version of this user
    # manifest.
    # In most tests we want to be in a state were backend and devices all
    # store the same user manifest (named the "v1" here).
    # But sometime we want a completly fresh start ("v1" doesn't exist,
    # hence devices and backend are empty) or only a single device to begin
    # with no knowledge of the "v1".
    return InitialState()


@pytest.fixture
def backend_factory(
    asyncio_loop,
    event_bus_factory,
    initial_user_manifest_state,
    blockstore,
    backend_store,
    default_devices,
    root_signing_key,
):
    # Given the postgresql driver uses trio-asyncio, any coroutine dealing with
    # the backend should inherit from the one with the asyncio loop context manager.
    # This mean the nursery fixture cannot use the backend object otherwise we
    # can end up in a dead lock if the asyncio loop is torndown before the
    # nursery fixture is done with calling the backend's postgresql stuff.

    blockstore_type, blockstore_config = blockstore

    def _local_to_backend_device(device: LocalDevice) -> BackendUser:
        certified_user = certify_user(None, root_signing_key, device.user_id, device.public_key)
        certified_device = certify_device(
            None, root_signing_key, device.device_id, device.verify_key
        )
        return new_backend_user_factory(device.device_id, None, certified_user, certified_device)

    @asynccontextmanager
    async def _backend_factory(devices=default_devices, config={}, event_bus=None):
        async with trio.open_nursery() as nursery:
            config = backend_config_factory(
                db_url=backend_store,
                blockstore_type=blockstore_type,
                environ={**blockstore_config, **config},
            )
            if not event_bus:
                event_bus = event_bus_factory()
            backend = BackendApp(config, event_bus=event_bus)
            # TODO: backend connection to postgresql will timeout if we use a trio
            # mock clock with autothreshold. We should detect this and do something here...
            await backend.init(nursery)

            # Need to initialize backend with users/devices
            with freeze_time("2000-01-01"):
                for device in devices:
                    backend_user = _local_to_backend_device(device)
                    try:
                        await backend.user.create_user(backend_user)

                        access = device.user_manifest_access
                        user_manifest = initial_user_manifest_state.get_initial_for_backend(device)
                        if user_manifest:
                            ciphered = encrypt_with_secret_key(
                                device.device_id,
                                device.signing_key,
                                access["key"],
                                dumps_manifest(user_manifest),
                            )
                            await backend.vlob.create(
                                access["id"],
                                access["rts"],
                                access["wts"],
                                ciphered,
                                device.device_id,
                            )

                    except UserAlreadyExistsError:
                        await backend.user.create_device(backend_user.devices[device.device_name])

            try:
                yield backend

            finally:
                await backend.teardown()
                # Don't do `nursery.cancel_scope.cancel()` given `backend.teardown()`
                # should have stopped all our coroutines (i.e. if the nursery is
                # hanging, something on top of us should be fixed...)

    return _backend_factory


@pytest.fixture
async def backend(backend_factory):
    async with backend_factory() as backend:
        yield backend


# TODO: rename to organization_addr ?
@pytest.fixture(scope="session")
def backend_addr(unused_tcp_addr, root_verify_key):
    rvk = export_root_verify_key(root_verify_key)
    return BackendOrganizationAddr(f"{unused_tcp_addr}?rvk={rvk}")


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
        running_backend_ready.set()
        yield server


# TODO: rename to backend_transport_factory
@pytest.fixture
def backend_sock_factory(server_factory):
    @asynccontextmanager
    async def _backend_sock_factory(backend, auth_as):
        async with server_factory(backend.handle_client) as server:
            stream = server.connection_factory()
            transport = await Transport.init_for_client(stream, server.addr)
            transport = FreezeTestOnTransportError(transport)

            if auth_as:
                # Handshake
                if auth_as == "anonymous":
                    ch = AnonymousClientHandshake()
                else:
                    # TODO: change auth_as type
                    ch = ClientHandshake(auth_as.device_id, auth_as.signing_key)
                challenge_req = await transport.recv()
                answer_req = ch.process_challenge_req(challenge_req)
                await transport.send(answer_req)
                result_req = await transport.recv()
                ch.process_result_req(result_req)

            yield transport

    return _backend_sock_factory


@pytest.fixture
async def anonymous_backend_sock(backend_sock_factory, backend):
    async with backend_sock_factory(backend, "anonymous") as sock:
        yield sock


@pytest.fixture
async def alice_backend_sock(backend_sock_factory, backend, alice):
    async with backend_sock_factory(backend, alice) as sock:
        yield sock


@pytest.fixture
async def alice2_backend_sock(backend_sock_factory, backend, alice2):
    async with backend_sock_factory(backend, alice2) as sock:
        yield sock


@pytest.fixture
async def bob_backend_sock(backend_sock_factory, backend, bob):
    async with backend_sock_factory(backend, bob) as sock:
        yield sock


@pytest.fixture(scope="session")
def root_signing_key():
    return SigningKey.generate()


@pytest.fixture(scope="session")
def root_verify_key(root_signing_key):
    return root_signing_key.verify_key


@pytest.fixture
def core_config(tmpdir):
    tmpdir = Path(tmpdir)
    return CoreConfig(
        config_dir=tmpdir / "config",
        cache_dir=tmpdir / "cache",
        data_dir=tmpdir / "data",
        mountpoint_base_dir=tmpdir / "mnt",
    )


@pytest.fixture
async def alice_core(running_backend_ready, event_bus_factory, core_config, alice):
    await running_backend_ready.wait()
    async with logged_core_factory(core_config, alice, event_bus_factory()) as alice_core:
        yield alice_core


@pytest.fixture
async def alice2_core(running_backend_ready, event_bus_factory, core_config, alice2):
    await running_backend_ready.wait()
    async with logged_core_factory(core_config, alice2, event_bus_factory()) as alice2_core:
        yield alice2_core


@pytest.fixture
async def bob_core(running_backend_ready, event_bus_factory, core_config, bob):
    await running_backend_ready.wait()
    async with logged_core_factory(core_config, bob, event_bus_factory()) as bob_core:
        yield bob_core
