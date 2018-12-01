import pytest
import attr
import os
import socket
import asyncpg
import contextlib
from unittest.mock import patch
import trio
import trio_asyncio
from typing import Tuple
from async_generator import asynccontextmanager
import hypothesis

from parsec.types import DeviceID, UserID
from parsec.crypto import (
    PrivateKey,
    PublicKey,
    SigningKey,
    VerifyKey,
    generate_secret_key,
    encode_urlsafe_root_verify_key,
)
from parsec.trustchain import certify_user, certify_device
from parsec.core import Core, CoreConfig
from parsec.core.local_db import LocalDBMissingEntry
from parsec.core.schemas import loads_manifest, dumps_manifest
from parsec.core.fs.utils import new_access, new_local_user_manifest, local_to_remote_manifest
from parsec.core.encryption_manager import encrypt_with_secret_key
from parsec.core.devices_manager import Device
from parsec.core.mountpoint import FUSE_AVAILABLE
from parsec.backend import BackendApp, config_factory as backend_config_factory
from parsec.backend.user import (
    User as BackendUser,
    Device as BackendDevice,
    DevicesMapping as BackendDevicesMapping,
)
from parsec.backend.exceptions import AlreadyExistsError as UserAlreadyExistsError
from parsec.handshake import ClientHandshake, AnonymousClientHandshake

# TODO: needed ?
pytest.register_assert_rewrite("tests.event_bus_spy")

from tests.common import freeze_time, FreezeTestOnBrokenStreamCookedSocket, InMemoryLocalDB
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


@pytest.fixture
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


@pytest.fixture
def postgresql_url(request):
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


@pytest.fixture
def unused_tcp_port():
    """Find an unused localhost TCP port from 1024-65535 and return it."""
    with contextlib.closing(socket.socket()) as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


@pytest.fixture
def unused_tcp_addr(unused_tcp_port):
    return "tcp://127.0.0.1:%s" % unused_tcp_port


@pytest.fixture
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


@pytest.fixture
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
            url = f"tcp://server-{count}.localhost:9999"

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
def root_key_certifier():
    class RootKeyCertifier:
        id = DeviceID("root@root")
        signing_key = SigningKey.generate()

        @property
        def verify_key(self):
            return self.signing_key.verify_key

        def certify_user(self, user_id, public_key):
            return certify_user(self.id, self.signing_key, user_id, public_key)

        def certify_device(self, device_id, verify_key):
            return certify_device(self.id, self.signing_key, device_id, verify_key)

        def device_factory(self, device_id: str, verify_key: VerifyKey = None) -> BackendDevice:
            device_id = DeviceID(device_id)
            verify_key = verify_key or VerifyKey.generate()
            return BackendDevice(
                device_id=device_id,
                certified_device=self.certify_device(device_id, verify_key),
                device_certifier=None,
            )

        def user_factory(
            self, user_id: str, public_key: PublicKey, devices: Tuple[str, VerifyKey] = ()
        ) -> BackendUser:
            user_id = UserID(user_id)
            cooked_devices = []
            for device_name, device_verify_key in devices:
                cooked_devices.append(
                    self.device_factory(f"{user_id}@{device_name}", device_verify_key)
                )
            return BackendUser(
                user_id,
                certified_user=self.certify_user(user_id, public_key),
                user_certifier=None,
                devices=BackendDevicesMapping(*cooked_devices),
            )

    return RootKeyCertifier()


@pytest.fixture()
def device_factory():
    users = {}
    devices = {}
    count = 0

    def _device_factory(user_id=None, device_name=None, user_manifest_in_v0=False, local_db=None):
        nonlocal count
        count += 1

        if not user_id:
            user_id = f"user_{count}"
        if not device_name:
            device_name = f"device_{count}"

        device_id = DeviceID(f"{user_id}@{device_name}")
        assert device_id not in devices

        try:
            user_privkey, user_manifest_access, user_manifest_v1 = users[user_id]
        except KeyError:
            user_privkey = PrivateKey.generate().encode()
            user_manifest_access = new_access()
            user_manifest_v1 = None

        if not user_manifest_v1 and not user_manifest_in_v0:
            with freeze_time("2000-01-01"):
                user_manifest_v1 = new_local_user_manifest(device_id)
            user_manifest_v1["base_version"] = 1
            user_manifest_v1["is_placeholder"] = False
            user_manifest_v1["need_sync"] = False

        users[user_id] = (user_privkey, user_manifest_access, user_manifest_v1)

        device_signkey = SigningKey.generate().encode()
        local_symkey = generate_secret_key()
        if not local_db:
            local_db = InMemoryLocalDB()
        device = Device(
            device_id, user_privkey, device_signkey, local_symkey, user_manifest_access, local_db
        )
        if not user_manifest_in_v0:
            device.local_db.set(user_manifest_access, dumps_manifest(user_manifest_v1))

        devices[device_id] = device
        return device

    return _device_factory


@pytest.fixture
def alice(device_factory):
    return device_factory("alice", "dev1")


@pytest.fixture
def alice2(device_factory):
    return device_factory("alice", "dev2")


@pytest.fixture
def bob(device_factory):
    return device_factory("bob", "dev1")


@pytest.fixture
def mallory(device_factory):
    return device_factory("mallory", "dev1")


@pytest.fixture
def default_devices(alice, alice2, bob):
    return (alice, alice2, bob)


@pytest.fixture
def bootstrap_postgresql(url):
    # In theory we should use TrioPG here to do db init, but:
    # - Duck typing and similar api makes `_ensure_tables_in_place` compatible
    #   with both
    # - AsyncPG should be slightly faster than TrioPG
    # - Most important: a trio loop is potentially already started inside this
    #   thread (i.e. if the test is mark as trio). Hence we would have to spawn
    #   another thread just to run the new trio loop.

    import asyncio
    import asyncpg
    from parsec.backend.drivers.postgresql.handler import _ensure_tables_in_place

    async def _bootstrap():
        conn = await asyncpg.connect(url)
        await _ensure_tables_in_place(conn, force=True)
        await conn.close()

    asyncio.get_event_loop().run_until_complete(_bootstrap())


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


@pytest.fixture
def backend_factory(
    asyncio_loop, event_bus_factory, blockstore, backend_store, default_devices, root_key_certifier
):
    # Given the postgresql driver uses trio-asyncio, any coroutine dealing with
    # the backend should inherit from the one with the asyncio loop context manager.
    # This mean the nursery fixture cannot use the backend object otherwise we
    # can end up in a dead lock if the asyncio loop is torndown before the
    # nursery fixture is done with calling the backend's postgresql stuff.

    blockstore_type, blockstore_config = blockstore
    default_urlsafe_root_verify_key = encode_urlsafe_root_verify_key(root_key_certifier.verify_key)

    @asynccontextmanager
    async def _backend_factory(devices=default_devices, config={}, event_bus=None):
        async with trio.open_nursery() as nursery:
            config = backend_config_factory(
                db_url=backend_store,
                blockstore_type=blockstore_type,
                environ={
                    "ROOT_VERIFY_KEY": default_urlsafe_root_verify_key,
                    **blockstore_config,
                    **config,
                },
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
                    backend_user = root_key_certifier.user_factory(
                        device.user_id,
                        device.user_pubkey,
                        [(device.device_name, device.device_verifykey)],
                    )
                    try:
                        await backend.user.create(backend_user)

                        access = device.user_manifest_access
                        try:
                            local_user_manifest = loads_manifest(device.local_db.get(access))
                        except LocalDBMissingEntry:
                            continue
                        else:
                            if local_user_manifest["base_version"] == 0:
                                continue

                        remote_user_manifest = local_to_remote_manifest(local_user_manifest)
                        ciphered = encrypt_with_secret_key(
                            device.id,
                            device.device_signkey,
                            access["key"],
                            dumps_manifest(remote_user_manifest),
                        )
                        await backend.vlob.create(
                            access["id"], access["rts"], access["wts"], ciphered
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


@pytest.fixture(scope="session")
def backend_addr(root_signing_key):
    default_urlsafe_root_verify_key = encode_urlsafe_root_verify_key(root_signing_key.verify_key)
    return f"tcp://parsec-backend.localhost:9999?root-verify-key={default_urlsafe_root_verify_key}"


@pytest.fixture
async def running_backend(server_factory, backend_addr, backend):
    async with server_factory(backend.handle_client, backend_addr) as server:
        server.backend = backend
        yield server


@pytest.fixture
def backend_sock_factory(server_factory):
    @asynccontextmanager
    async def _backend_sock_factory(backend, auth_as):
        async with server_factory(backend.handle_client) as server:
            sockstream = server.connection_factory()
            sock = FreezeTestOnBrokenStreamCookedSocket(sockstream)
            if auth_as:
                # Handshake
                if auth_as == "anonymous":
                    ch = AnonymousClientHandshake()
                else:
                    ch = ClientHandshake(auth_as.id, auth_as.device_signkey)
                challenge_req = await sock.recv()
                answer_req = ch.process_challenge_req(challenge_req)
                await sock.send(answer_req)
                result_req = await sock.recv()
                ch.process_result_req(result_req)
            yield sock

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
async def bob_backend_sock(backend_sock_factory, backend, bob):
    async with backend_sock_factory(backend, bob) as sock:
        yield sock


@pytest.fixture
def core_factory(tmpdir, event_bus_factory, backend_addr, default_devices):
    count = 0

    @asynccontextmanager
    async def _core_factory(devices=default_devices, config={}, event_bus=None):
        nonlocal count
        count += 1

        core_dir = tmpdir / f"core-{count}"
        config = CoreConfig(
            **{
                "backend_addr": backend_addr,
                "local_storage_dir": (core_dir / "local_storage").strpath,
                "base_settings_path": (core_dir / "settings").strpath,
                **config,
            }
        )
        if not event_bus:
            event_bus = event_bus_factory()
        core = Core(config, event_bus=event_bus)
        async with trio.open_nursery() as nursery:
            await core.init(nursery)

            for device in devices:
                core.local_devices_manager.register_new_device(
                    device.id,
                    device.user_privkey.encode(),
                    device.device_signkey.encode(),
                    device.user_manifest_access,
                    "<secret>",
                )

            try:
                yield core
            finally:
                await core.teardown()
                # Don't do `nursery.cancel_scope.cancel()` given `core.teardown()`
                # should have stopped all our coroutines (i.e. if the nursery is
                # hanging, something on top of us should be fixed...)

    return _core_factory


@pytest.fixture
async def core(core_factory):
    async with core_factory() as core:
        yield core


@pytest.fixture
def core_sock_factory(server_factory):
    @asynccontextmanager
    async def _core_sock_factory(core):
        async with server_factory(core.handle_client) as server:
            sockstream = server.connection_factory()
            yield FreezeTestOnBrokenStreamCookedSocket(sockstream)

    return _core_sock_factory


@pytest.fixture
async def alice_core(core, alice):
    assert not core.auth_device, "Core already logged"
    await core.login(alice)
    return core


@pytest.fixture
async def alice_core_sock(core_sock_factory, alice_core):
    async with core_sock_factory(alice_core) as sock:
        yield sock


@pytest.fixture
async def core2(core_factory):
    async with core_factory() as core2:
        yield core2


@pytest.fixture
async def alice2_core2(core2, alice2):
    assert not core2.auth_device, "Core already logged"
    await core2.login(alice2)
    return core2


@pytest.fixture
async def bob_core2(core2, bob):
    assert not core2.auth_device, "Core already logged"
    await core2.login(bob)
    return core2


@pytest.fixture
async def alice2_core2_sock(core_sock_factory, alice2_core2):
    async with core_sock_factory(alice2_core2) as sock:
        yield sock


@pytest.fixture
async def bob_core2_sock(core_sock_factory, bob_core2):
    async with core_sock_factory(bob_core2) as sock:
        yield sock
