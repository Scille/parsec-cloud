# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from async_generator import asynccontextmanager

from parsec.core.types import local_manifest_serializer
from parsec.core.backend_connection import backend_cmds_factory
from parsec.core.encryption_manager import EncryptionManager
from parsec.core.fs import FS

from tests.common import freeze_time, InMemoryLocalDB


@pytest.fixture
def local_db_factory(initial_user_manifest_state):
    local_dbs = {}

    def _local_db_factory(device, user_manifest_in_v0=False, force=True):
        device_id = device.device_id
        assert force or (device_id not in local_dbs)

        local_db = InMemoryLocalDB()
        local_dbs[device_id] = local_db
        if not user_manifest_in_v0:
            user_manifest = initial_user_manifest_state.get_user_manifest_v1_for_device(device)
            local_db.set_dirty_manifest(
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
            device.organization_addr, device.device_id, device.signing_key
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
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def alice2_backend_cmds(running_backend, alice2):
    async with backend_cmds_factory(
        alice2.organization_addr, alice2.device_id, alice2.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def bob_backend_cmds(running_backend, bob):
    async with backend_cmds_factory(bob.organization_addr, bob.device_id, bob.signing_key) as cmds:
        yield cmds
