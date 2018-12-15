import pytest
import trio
from async_generator import asynccontextmanager

from parsec.types import DeviceID
from parsec.core.schemas import dumps_manifest
from parsec.core.devices_manager import generate_new_device
from parsec.core.backend_connection import backend_cmds_pool_factory
from parsec.core.encryption_manager import EncryptionManager
from parsec.core.fs import FS
from parsec.core.fs.utils import new_local_user_manifest

from tests.common import freeze_time, InMemoryLocalDB


@pytest.fixture
def encryption_manager_factory(backend_addr):
    @asynccontextmanager
    async def _encryption_manager_factory(device, local_db=None, backend_addr=backend_addr):
        local_db = local_db or InMemoryLocalDB()
        async with backend_cmds_pool_factory(
            backend_addr, device.device_id, device.signing_key
        ) as cmds:
            em = EncryptionManager(device, local_db, cmds)
            async with trio.open_nursery() as nursery:
                await em.init(nursery)
                try:
                    yield em
                finally:
                    await em.teardown()

    return _encryption_manager_factory


@pytest.fixture
async def encryption_manager(running_backend, encryption_manager_factory, alice):
    async with encryption_manager_factory(alice) as em:
        yield em


@pytest.fixture
def fs_factory(encryption_manager_factory, event_bus_factory, backend_addr):
    @asynccontextmanager
    async def _fs_factory(device, local_db, backend_addr=backend_addr, event_bus=None):
        if not event_bus:
            event_bus = event_bus_factory()

        async with encryption_manager_factory(device, local_db, backend_addr=backend_addr) as em:
            fs = FS(device, local_db, em.backend_cmds, em, event_bus)
            yield fs

    return _fs_factory


@pytest.fixture()
def alice_local_db():
    return InMemoryLocalDB()


@pytest.fixture()
def alice2_local_db():
    return InMemoryLocalDB()


@pytest.fixture()
def bob_local_db():
    return InMemoryLocalDB()


@pytest.fixture
async def alice_fs(fs_factory, alice, alice_local_db):
    async with fs_factory(alice, alice_local_db) as fs:
        yield fs


@pytest.fixture
async def alice2_fs(fs_factory, alice2, alice2_local_db):
    async with fs_factory(alice2, alice2_local_db) as fs:
        yield fs


@pytest.fixture
async def bob_fs(fs_factory, bob, bob_local_db):
    async with fs_factory(bob, bob_local_db) as fs:
        yield fs


@pytest.fixture
def backend_addr_factory(running_backend, tcp_stream_spy):
    # Creating new addr for backend make it easy be selective on what to
    # turn offline
    counter = 0

    def _backend_addr_factory():
        nonlocal counter
        addr = f"tcp://backend-addr-{counter}.localhost:9999"
        tcp_stream_spy.push_hook(addr, running_backend.connection_factory)
        counter += 1
        return addr

    return _backend_addr_factory


@pytest.fixture
async def alice_backend_cmds(running_backend, alice):
    async with backend_cmds_pool_factory(
        running_backend.addr, alice.device_id, alice.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def alice2_backend_cmds(running_backend, alice2):
    async with backend_cmds_pool_factory(
        running_backend.addr, alice2.device_id, alice2.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def bob_backend_cmds(running_backend, bob):
    async with backend_cmds_pool_factory(
        running_backend.addr, bob.device_id, bob.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
def device_factory(backend_addr, root_verify_key):
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

        device = generate_new_device(device_id, backend_addr, root_verify_key)
        try:
            private_key, user_manifest_access, user_manifest_v1 = users[user_id]
        except KeyError:
            user_manifest_v1 = None
            private_key = device.private_key
            user_manifest_access = device.user_manifest_access
        else:
            device = device.evolve(
                private_key=private_key, user_manifest_access=user_manifest_access
            )

        if not user_manifest_v1 and not user_manifest_in_v0:
            with freeze_time("2000-01-01"):
                user_manifest_v1 = new_local_user_manifest(device_id)
            user_manifest_v1["base_version"] = 1
            user_manifest_v1["is_placeholder"] = False
            user_manifest_v1["need_sync"] = False

        users[user_id] = (private_key, user_manifest_access, user_manifest_v1)

        if not local_db:
            local_db = InMemoryLocalDB()
        if not user_manifest_in_v0:
            local_db.set(user_manifest_access, dumps_manifest(user_manifest_v1))

        devices[device_id] = device
        return device, local_db

    return _device_factory
