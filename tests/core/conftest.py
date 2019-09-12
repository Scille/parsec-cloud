# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pathlib import Path
from async_generator import asynccontextmanager

from parsec.core.backend_connection import backend_cmds_pool_factory, backend_anonymous_cmds_factory
from parsec.core.remote_devices_manager import RemoteDevicesManager
from parsec.core.fs import UserFS, FSLocalMissError
from parsec.core.fs.realm_storage import RealmStorage

from tests.common import freeze_time


@pytest.fixture
def local_storage_path(tmpdir):
    def _local_storage_path(device):
        return Path(tmpdir) / "local_storage" / device.slug

    return _local_storage_path


@pytest.fixture
def initialize_userfs_storage(initial_user_manifest_state, persistent_mockup):
    async def _initialize_userfs_storage(device, storage):
        try:
            await storage.get_manifest(device.user_manifest_id)
        except FSLocalMissError:
            with freeze_time("2000-01-01"):
                user_manifest = initial_user_manifest_state.get_user_manifest_v1_for_device(device)
            await storage.set_manifest(device.user_manifest_id, user_manifest)

    return _initialize_userfs_storage


@pytest.fixture()
async def alice_local_storage(local_storage_path, initialize_userfs_storage, alice):
    path = local_storage_path(alice)
    async with RealmStorage.factory(alice, path) as storage:
        await initialize_userfs_storage(alice, storage)
        yield storage


@pytest.fixture()
async def alice2_local_storage(local_storage_path, initialize_userfs_storage, alice2):
    path = local_storage_path(alice2)
    async with RealmStorage.factory(alice2, path) as storage:
        await initialize_userfs_storage(alice2, storage)
        yield storage


@pytest.fixture()
async def bob_local_storage(local_storage_path, initialize_userfs_storage, bob):
    path = local_storage_path(bob)
    async with RealmStorage.factory(bob, path) as storage:
        await initialize_userfs_storage(bob, storage)
        yield storage


@pytest.fixture
def remote_devices_manager_factory():
    @asynccontextmanager
    async def _remote_devices_manager_factory(device):
        async with backend_cmds_pool_factory(
            device.organization_addr, device.device_id, device.signing_key
        ) as cmds:
            yield RemoteDevicesManager(cmds, device.root_verify_key)

    return _remote_devices_manager_factory


@pytest.fixture
async def alice_remote_devices_manager(remote_devices_manager_factory, alice):
    async with remote_devices_manager_factory(alice) as rdm:
        yield rdm


@pytest.fixture
async def alice2_remote_devices_manager(remote_devices_manager_factory, alice2):
    async with remote_devices_manager_factory(alice2) as rdm:
        yield rdm


@pytest.fixture
async def bob_remote_devices_manager(remote_devices_manager_factory, bob):
    async with remote_devices_manager_factory(bob) as rdm:
        yield rdm


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
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def alice2_backend_cmds(running_backend, alice2):
    async with backend_cmds_pool_factory(
        alice2.organization_addr, alice2.device_id, alice2.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def bob_backend_cmds(running_backend, bob):
    async with backend_cmds_pool_factory(
        bob.organization_addr, bob.device_id, bob.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def anonymous_backend_cmds(running_backend, coolorg):
    async with backend_anonymous_cmds_factory(coolorg.addr) as cmds:
        yield cmds


@pytest.fixture
def user_fs_factory(
    local_storage_path, event_bus_factory, persistent_mockup, initialize_userfs_storage
):
    @asynccontextmanager
    async def _user_fs_factory(device, event_bus=None, initialize_user_storage=True):
        event_bus = event_bus or event_bus_factory()
        async with backend_cmds_pool_factory(
            device.organization_addr, device.device_id, device.signing_key
        ) as cmds:
            path = local_storage_path(device)
            rdm = RemoteDevicesManager(cmds, device.root_verify_key)
            async with UserFS(device, path, cmds, rdm, event_bus) as user_fs:
                if initialize_user_storage:
                    await initialize_userfs_storage(device, user_fs.storage)
                yield user_fs

    return _user_fs_factory


@pytest.fixture
async def alice_user_fs(user_fs_factory, alice):
    async with user_fs_factory(alice) as user_fs:
        yield user_fs


@pytest.fixture
async def alice2_user_fs(user_fs_factory, alice2):
    async with user_fs_factory(alice2) as user_fs:
        yield user_fs


@pytest.fixture
async def bob_user_fs(user_fs_factory, bob):
    async with user_fs_factory(bob) as user_fs:
        yield user_fs
