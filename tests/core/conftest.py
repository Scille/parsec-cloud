# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from pathlib import Path
from typing import AsyncIterator, Callable, Optional, Type
import pytest
from contextlib import asynccontextmanager, _AsyncGeneratorContextManager

from parsec.core.backend_connection import (
    backend_authenticated_cmds_factory,
    apiv1_backend_anonymous_cmds_factory,
)
from parsec.core.remote_devices_manager import RemoteDevicesManager
from parsec.core.fs import UserFS
from parsec.core.logged_core import get_prevent_sync_pattern
from parsec.core.types import LocalDevice
from parsec.event_bus import EventBus
from tests.common.event_bus_spy import SpiedEventBus


@pytest.fixture
def data_base_dir(tmp_path: Path) -> Path:
    return tmp_path / "local_data"


@pytest.fixture
def remote_devices_manager_factory():
    @asynccontextmanager
    async def _remote_devices_manager_factory(device):
        async with backend_authenticated_cmds_factory(
            device.organization_addr, device.device_id, device.signing_key
        ) as cmds:
            yield RemoteDevicesManager(cmds, device.root_verify_key, device.time_provider)

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
async def alice_backend_cmds(running_backend, alice):
    async with backend_authenticated_cmds_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def alice2_backend_cmds(running_backend, alice2):
    async with backend_authenticated_cmds_factory(
        alice2.organization_addr, alice2.device_id, alice2.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def bob_backend_cmds(running_backend, bob):
    async with backend_authenticated_cmds_factory(
        bob.organization_addr, bob.device_id, bob.signing_key
    ) as cmds:
        yield cmds


@pytest.fixture
async def apiv1_anonymous_backend_cmds(running_backend, coolorg):
    async with apiv1_backend_anonymous_cmds_factory(coolorg.addr) as cmds:
        yield cmds


@pytest.fixture
def user_fs_factory(data_base_dir: Path, event_bus_factory: Type[SpiedEventBus]):
    @asynccontextmanager
    async def _user_fs_factory(
        device: LocalDevice, event_bus: EventBus = None, data_base_dir: Path = data_base_dir
    ) -> AsyncIterator[UserFS]:
        event_bus = event_bus or event_bus_factory()

        async with backend_authenticated_cmds_factory(
            device.organization_addr, device.device_id, device.signing_key
        ) as cmds:
            rdm = RemoteDevicesManager(cmds, device.root_verify_key, device.time_provider)
            async with UserFS.run(
                data_base_dir, device, cmds, rdm, event_bus, get_prevent_sync_pattern()
            ) as user_fs:

                yield user_fs

    return _user_fs_factory


UserFsFactory = Callable[
    [LocalDevice, Optional[EventBus], Optional[Path]], _AsyncGeneratorContextManager[UserFS]
]


@pytest.fixture
async def alice_user_fs(
    data_base_dir, fixtures_customization, initialize_local_user_manifest, user_fs_factory, alice
) -> AsyncIterator[UserFS]:
    initial_user_manifest = fixtures_customization.get("alice_initial_local_user_manifest", "v1")
    await initialize_local_user_manifest(
        data_base_dir, alice, initial_user_manifest=initial_user_manifest
    )
    async with user_fs_factory(alice) as user_fs:
        user_fs: UserFS = user_fs
        yield user_fs


@pytest.fixture
async def alice2_user_fs(
    data_base_dir, fixtures_customization, initialize_local_user_manifest, user_fs_factory, alice2
):
    initial_user_manifest = fixtures_customization.get("alice2_initial_local_user_manifest", "v1")
    await initialize_local_user_manifest(
        data_base_dir, alice2, initial_user_manifest=initial_user_manifest
    )
    async with user_fs_factory(alice2) as user_fs:
        yield user_fs


@pytest.fixture
async def bob_user_fs(
    data_base_dir, fixtures_customization, initialize_local_user_manifest, user_fs_factory, bob
):
    initial_user_manifest = fixtures_customization.get("bob_initial_local_user_manifest", "v1")
    await initialize_local_user_manifest(
        data_base_dir, bob, initial_user_manifest=initial_user_manifest
    )
    async with user_fs_factory(bob) as user_fs:
        yield user_fs
