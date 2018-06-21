import pytest
import inspect
import attr
import os
import socket
import asyncpg
import contextlib
from unittest.mock import patch
import trio
import trio_asyncio
import hypothesis
from async_generator import asynccontextmanager
from nacl.public import PrivateKey
from nacl.signing import SigningKey
import nacl

from parsec.signals import Namespace as SignalNamespace
from parsec.core import Core, CoreConfig
from parsec.core.schemas import loads_manifest, dumps_manifest
from parsec.core.fs.data import new_access, new_local_user_manifest, local_to_remote_manifest
from parsec.core.encryption_manager import encrypt_with_secret_key
from parsec.core.devices_manager import Device
from parsec.backend import BackendApp, BackendConfig
from parsec.backend.drivers import postgresql as pg_driver
from parsec.backend.exceptions import AlreadyExistsError as UserAlreadyExistsError
from parsec.handshake import ClientHandshake, AnonymousClientHandshake

from tests.common import freeze_time, FreezeTestOnBrokenStreamCookedSocket, InMemoryLocalDB
from tests.open_tcp_stream_mock_wrapper import OpenTCPStreamMockWrapper


def pytest_addoption(parser):
    parser.addoption("--hypothesis-max-examples", default=100, type=int)
    parser.addoption("--hypothesis-derandomize", action="store_true")
    parser.addoption("--postgresql", action="store_true", help="Run tests making use of PostgreSQL")
    parser.addoption(
        "--only-postgresql", action="store_true", help="Only run tests making use of PostgreSQL"
    )
    parser.addoption("--runslow", action="store_true", help="Don't skip slow tests")
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
    if "slow" in item.keywords and not item.config.getoption("--runslow"):
        pytest.skip("need --runslow option to run")


@pytest.fixture(autouse=True, scope="session")
def mock_crypto():
    from nacl.secret import SecretBox

    # Crypto is CPU hungry
    if pytest.config.getoption("--realcrypto"):
        yield
    else:

        def unsecure_but_fast_sbf(password, salt):
            key = password[: SecretBox.KEY_SIZE] + b"\x00" * (SecretBox.KEY_SIZE - len(password))
            return SecretBox(key)

        with patch("parsec.core.devices_manager._secret_box_factory", new=unsecure_but_fast_sbf):
            yield


from parsec.core.devices_manager import _secret_box_factory as vanilla_sbf


@pytest.fixture
def realcrypto():
    with patch("parsec.core.devices_manager._secret_box_factory", new=vanilla_sbf):
        yield


# Use current unix user's credential, don't forget to do
# `psql -c 'CREATE DATABASE parsec_test;'` prior to run tests
DEFAULT_POSTGRESQL_TEST_URL = "postgresql:///parsec_test"

# Use current unix user's credential, don't forget to do
# `psql -c 'CREATE DATABASE triopg_test;'` prior to run tests
TRIOPG_POSTGRESQL_TEST_URL = "postgresql:///triopg_test"


def get_postgresql_url():
    return os.environ.get("PARSEC_POSTGRESQL_TEST_URL", DEFAULT_POSTGRESQL_TEST_URL)


@pytest.fixture
def postgresql_url(request):
    if not pytest.config.getoption("--postgresql"):
        pytest.skip("`--postgresql` option not provided")
    return get_postgresql_url()


@pytest.fixture
async def asyncio_loop():
    async with trio_asyncio.open_loop() as loop:
        yield loop


@pytest.fixture
def always_logs():
    """
    By default, pytest-logbook only print last test's logs in case of error.
    With this fixture all logs are outputed as soon as they are created.
    """
    from logbook import StreamHandler
    import sys

    sh = StreamHandler(sys.stdout)
    with sh.applicationbound():
        yield


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
def signal_ns_factory():
    return SignalNamespace


@pytest.fixture
def signal_ns(signal_ns_factory):
    return signal_ns_factory()


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
def server_factory(nursery, tcp_stream_spy):
    count = 0

    def _server_factory(entry_point, url=None, nursery=nursery):
        nonlocal count
        count += 1

        if not url:
            url = f"tcp://server-{count}.placeholder.com:9999"

        def connection_factory(*args, **kwargs):
            right, left = trio.testing.memory_stream_pair()
            nursery.start_soon(entry_point, left)
            return right

        tcp_stream_spy.push_hook(url, connection_factory)
        return AppServer(entry_point, url, connection_factory)

    return _server_factory


def contextify(factory, teardown=lambda x: None):
    @asynccontextmanager
    async def _contextified(*args, **kwargs):
        async with trio.open_nursery() as nursery:
            if inspect.iscoroutinefunction(factory):
                res = await factory(*args, **kwargs, nursery=nursery)
            else:
                res = factory(*args, **kwargs, nursery=nursery)
            yield res
            if inspect.iscoroutinefunction(teardown):
                await teardown(res)
            else:
                teardown(res)
            nursery.cancel_scope.cancel()

    return _contextified


@pytest.fixture
def server_factory_cm(server_factory):
    return contextify(server_factory)


@pytest.fixture
def device_factory():
    users = {}
    devices = {}
    count = 0

    def _device_factory(user_id=None, device_name=None):
        nonlocal count
        count += 1

        if not user_id:
            user_id = f"user-{count}"
        if not device_name:
            device_name = f"device-{count}"

        device_id = f"{user_id}@{device_name}"
        assert device_id not in devices

        try:
            user_privkey, user_manifest_access, user_manifest_v1 = users[user_id]
        except KeyError:
            user_privkey = PrivateKey.generate().encode()
            with freeze_time("2000-01-01"):
                user_manifest_v1 = new_local_user_manifest(device_id)
            user_manifest_v1["base_version"] = 1
            user_manifest_v1["is_placeholder"] = False
            user_manifest_v1["need_sync"] = False
            user_manifest_access = new_access()
            users[user_id] = (user_privkey, user_manifest_access, user_manifest_v1)

        device_signkey = SigningKey.generate().encode()
        local_symkey = nacl.utils.random(nacl.secret.SecretBox.KEY_SIZE)
        device = Device(
            f"{user_id}@{device_name}",
            user_privkey,
            device_signkey,
            local_symkey,
            user_manifest_access,
            InMemoryLocalDB(),
        )
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


@pytest.fixture(params=["mocked", "postgresql"])
async def backend_store(request, asyncio_loop):
    if request.param == "postgresql":
        if not pytest.config.getoption("--postgresql"):
            pytest.skip("`--postgresql` option not provided")
        url = get_postgresql_url()
        try:
            await pg_driver.handler.init_db(url, True)
        except asyncpg.exceptions.InvalidCatalogNameError as exc:
            raise RuntimeError(
                "Is `parsec_test` a valid database in PostgreSQL ?\n"
                "Running `psql -c 'CREATE DATABASE parsec_test;'` may fix this"
            ) from exc
        return url

    else:
        if pytest.config.getoption("--only-postgresql"):
            pytest.skip("`--only-postgresql` option provided")
        return "mocked://"


@pytest.fixture
def backend_factory(nursery, signal_ns_factory, backend_store, default_devices):
    async def _backend_factory(devices=default_devices, config={}, signal_ns=None, nursery=nursery):
        config = BackendConfig(**{"blockstore_postgresql": True, "dburl": backend_store, **config})
        if not signal_ns:
            signal_ns = signal_ns_factory()
        backend = BackendApp(config, signal_ns=signal_ns)

        # Need to initialize backend with users/devices, and for each user
        # store it user manifest
        with freeze_time("2000-01-01"):
            for device in devices:
                try:
                    await backend.user.create(
                        author="<backend-fixture>",
                        user_id=device.user_id,
                        broadcast_key=device.user_pubkey.encode(),
                        devices=[(device.device_name, device.device_verifykey.encode())],
                    )

                    access = device.user_manifest_access
                    local_user_manifest = loads_manifest(device.local_db.get(access))
                    remote_user_manifest = local_to_remote_manifest(local_user_manifest)
                    ciphered = encrypt_with_secret_key(
                        device, access["key"], dumps_manifest(remote_user_manifest)
                    )

                    await backend.vlob.create(access["id"], access["rts"], access["wts"], ciphered)

                except UserAlreadyExistsError:
                    await backend.user.create_device(
                        user_id=device.user_id,
                        device_name=device.device_name,
                        verify_key=device.device_verifykey.encode(),
                    )

        await backend.init(nursery)
        return backend

    return _backend_factory


@pytest.fixture
def backend_factory_cm(backend_factory):
    async def teardown(backend):
        await backend.teardown()

    return contextify(backend_factory, teardown)


@pytest.fixture
async def backend(backend_factory):
    return await backend_factory()


@pytest.fixture
def backend_addr(tcp_stream_spy):
    return "tcp://placeholder.com:9999"


@pytest.fixture
def running_backend(server_factory, backend_addr, backend):
    server = server_factory(backend.handle_client, backend_addr)
    server.backend = backend
    return server


@pytest.fixture
def backend_sock_factory(server_factory, nursery):
    async def _backend_sock_factory(backend, auth_as, nursery=nursery):
        server = server_factory(backend.handle_client, nursery=nursery)
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
        return sock

    return _backend_sock_factory


@pytest.fixture
async def anonymous_backend_sock(backend_sock_factory, backend):
    return await backend_sock_factory(backend, "anonymous")


@pytest.fixture
async def alice_backend_sock(backend_sock_factory, backend, alice):
    return await backend_sock_factory(backend, alice)


@pytest.fixture
async def bob_backend_sock(backend_sock_factory, backend, bob):
    return await backend_sock_factory(backend, bob)


@pytest.fixture
def core_factory(tmpdir, nursery, signal_ns_factory, backend_addr, default_devices):
    count = 0

    async def _core_factory(devices=default_devices, config={}, signal_ns=None, nursery=nursery):
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
        if not signal_ns:
            signal_ns = signal_ns_factory()
        core = Core(config, signal_ns=signal_ns)
        await core.init(nursery)

        for device in devices:
            core.local_devices_manager.register_new_device(
                device.id,
                device.user_privkey.encode(),
                device.device_signkey.encode(),
                device.user_manifest_access,
                "<secret>",
            )

        return core

    return _core_factory


@pytest.fixture
def core_factory_cm(core_factory):
    async def teardown(core):
        await core.teardown()

    return contextify(core_factory, teardown)


@pytest.fixture
async def core(core_factory):
    return await core_factory()


@pytest.fixture
def core_sock_factory(server_factory, nursery):
    def _core_sock_factory(core, nursery=nursery):
        server = server_factory(core.handle_client, nursery=nursery)
        sockstream = server.connection_factory()
        return FreezeTestOnBrokenStreamCookedSocket(sockstream)

    return _core_sock_factory


@pytest.fixture
def core_sock_factory_cm(core_sock_factory):
    return contextify(core_sock_factory)


@pytest.fixture
async def alice_core(core, alice):
    assert not core.auth_device, "Core already logged"
    await core.login(alice)
    return core


@pytest.fixture
async def alice_core_sock(core_sock_factory, alice_core):
    return core_sock_factory(alice_core)


@pytest.fixture
async def core2(core_factory):
    return await core_factory()


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
    return core_sock_factory(alice2_core2)


@pytest.fixture
async def bob_core2_sock(core_sock_factory, bob_core2):
    return core_sock_factory(bob_core2)
