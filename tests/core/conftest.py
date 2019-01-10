import pytest
import trio
from async_generator import asynccontextmanager

from parsec.types import DeviceID
from parsec.core.types import local_manifest_serializer
from parsec.core.devices_manager import generate_new_device
from parsec.core.backend_connection import backend_cmds_factory
from parsec.core.encryption_manager import EncryptionManager
from parsec.core.fs import FS

from tests.common import freeze_time, InMemoryLocalDB


@pytest.fixture
def device_factory(backend_addr):
    users = {}
    devices = {}
    count = 0

    def _device_factory(user_id=None, device_name=None):
        nonlocal count
        count += 1

        if not user_id:
            user_id = f"user_{count}"
        if not device_name:
            device_name = f"device_{count}"

        device_id = DeviceID(f"{user_id}@{device_name}")
        assert device_id not in devices

        device = generate_new_device(device_id, backend_addr)
        try:
            private_key, user_manifest_access = users[user_id]
        except KeyError:
            users[user_id] = (device.private_key, device.user_manifest_access)
        else:
            device = device.evolve(
                private_key=private_key, user_manifest_access=user_manifest_access
            )
        return device

    return _device_factory


@pytest.fixture
def local_db_factory(initial_user_manifest_state):
    local_dbs = {}

    def _local_db_factory(device, user_manifest_in_v0=False, force=True):
        device_id = device.device_id
        assert force or (device_id not in local_dbs)

        local_db = InMemoryLocalDB()
        local_dbs[device_id] = local_db
        if user_manifest_in_v0:
            initial_user_manifest_state.set_v0_in_device(device_id)
        else:
            user_manifest = initial_user_manifest_state.get_initial_for_device(device)
            local_db.set(
                device.user_manifest_access, local_manifest_serializer.dumps(user_manifest)
            )

        return local_db

    return _local_db_factory


@pytest.fixture()
def alice_local_db(local_db_factory, alice):
    with freeze_time("2000-01-01"):
        return local_db_factory(alice)


@pytest.fixture()
def alice2_local_db(local_db_factory, alice2):
    with freeze_time("2000-01-01"):
        return local_db_factory(alice2)


@pytest.fixture()
def bob_local_db(local_db_factory, bob):
    with freeze_time("2000-01-01"):
        return local_db_factory(bob)


@pytest.fixture
def encryption_manager_factory():
    @asynccontextmanager
    async def _encryption_manager_factory(device, local_db=None):
        local_db = local_db or InMemoryLocalDB()
        async with backend_cmds_factory(
            device.backend_addr, device.device_id, device.signing_key
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
def fs_factory(encryption_manager_factory, local_db_factory, event_bus_factory):
    @asynccontextmanager
    async def _fs_factory(device, local_db=None, event_bus=None):
        if not event_bus:
            event_bus = event_bus_factory()
        local_db = local_db or local_db_factory(device)

        async with encryption_manager_factory(device, local_db) as em:
            fs = FS(device, local_db, em.backend_cmds, em, event_bus)
            yield fs

    return _fs_factory


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
    async with backend_cmds_factory(
        running_backend.addr, alice.device_id, alice.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def alice2_backend_cmds(running_backend, alice2):
    async with backend_cmds_factory(
        running_backend.addr, alice2.device_id, alice2.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def bob_backend_cmds(running_backend, bob):
    async with backend_cmds_factory(running_backend.addr, bob.device_id, bob.signing_key) as cmds:
        yield cmds
