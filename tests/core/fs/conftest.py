# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager
from pathlib import Path
from typing import Any, AsyncIterator, Callable

import pytest
import trio
from hypothesis_trio.stateful import TrioAsyncioRuleBasedStateMachine, run_state_machine_as_test

from parsec._parsec import (
    DateTime,
    EntryID,
    EntryName,
    LocalDevice,
    LocalWorkspaceManifest,
    Regex,
    WorkspaceEntry,
)
from parsec.core.backend_connection.authenticated import backend_authenticated_cmds_factory
from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.fs.storage import WorkspaceStorage
from parsec.core.fs.workspacefs.entry_transactions import EntryTransactions
from parsec.core.fs.workspacefs.file_transactions import FileTransactions
from parsec.core.fs.workspacefs.sync_transactions import SyncTransactions
from tests.common import call_with_control
from tests.core.conftest import UserFsFactory


@pytest.fixture
def transactions_factory(event_bus, remote_devices_manager_factory, core_config):
    @asynccontextmanager
    async def _transactions_factory(
        device: LocalDevice, local_storage: WorkspaceStorage, cls=SyncTransactions
    ) -> AsyncIterator[SyncTransactions]:
        def _get_workspace_entry():
            return workspace_entry

        async def _get_previous_workspace_entry():
            # The tests shouldn't need this yet
            assert False

        workspace_entry = WorkspaceEntry.new(EntryName("test"), device.timestamp())
        workspace_manifest = LocalWorkspaceManifest.new_placeholder(
            device.device_id, id=workspace_entry.id, timestamp=DateTime(2000, 1, 1)
        )
        async with local_storage.lock_entry_id(workspace_entry.id):
            await local_storage.set_manifest(workspace_entry.id, workspace_manifest)

        async with backend_authenticated_cmds_factory(
            device.organization_addr, device.device_id, device.signing_key
        ) as cmds:

            remote_devices_manager = remote_devices_manager_factory(device)
            remote_loader = RemoteLoader(
                device,
                workspace_entry.id,
                _get_workspace_entry,
                _get_previous_workspace_entry,
                cmds,
                remote_devices_manager,
                local_storage,
            )

            yield cls(
                workspace_entry.id,
                _get_workspace_entry,
                device,
                local_storage,
                remote_loader,
                event_bus,
                core_config,
            )

    return _transactions_factory


@pytest.fixture
def file_transactions_factory(event_bus, remote_devices_manager_factory, transactions_factory):
    @asynccontextmanager
    async def _file_transactions_factory(
        device: LocalDevice, local_storage: WorkspaceStorage
    ) -> AsyncIterator[FileTransactions]:
        async with transactions_factory(
            device, local_storage=local_storage, cls=FileTransactions
        ) as file_transactions:
            yield file_transactions

    return _file_transactions_factory


@pytest.fixture
async def alice_transaction_local_storage(
    alice: LocalDevice, tmp_path: Path
) -> AsyncIterator[WorkspaceStorage]:
    async with WorkspaceStorage.run(
        tmp_path / "alice_transaction_local_storage", alice, EntryID.new()
    ) as storage:
        yield storage


@pytest.fixture
async def alice_file_transactions(
    file_transactions_factory, alice: LocalDevice, alice_transaction_local_storage: WorkspaceStorage
) -> AsyncIterator[FileTransactions]:
    async with file_transactions_factory(
        alice, local_storage=alice_transaction_local_storage
    ) as file_transactions:
        yield file_transactions


@pytest.fixture
def entry_transactions_factory(transactions_factory):
    @asynccontextmanager
    async def _entry_transactions_factory(device, local_storage):
        async with transactions_factory(
            device, local_storage=local_storage, cls=EntryTransactions
        ) as transactions:
            yield transactions

    return _entry_transactions_factory


@pytest.fixture
async def alice_entry_transactions(
    entry_transactions_factory, alice, alice_transaction_local_storage
):
    async with entry_transactions_factory(
        alice, local_storage=alice_transaction_local_storage
    ) as entry_transactions:
        yield entry_transactions


@pytest.fixture
async def alice_sync_transactions(transactions_factory, alice, alice_transaction_local_storage):
    async with transactions_factory(
        alice, local_storage=alice_transaction_local_storage
    ) as transactions:
        yield transactions


@pytest.fixture(params=[True, False], ids=["compatibility_between_rust_and_python", "vanilla"])
def user_fs_offline_state_machine(
    monkeypatch: pytest.MonkeyPatch,
    user_fs_factory: UserFsFactory,
    clear_database_dir: Callable[[bool], None],
    reset_testbed,
    hypothesis_settings,
    fixtures_customization: dict[str, Any],
    request: pytest.FixtureRequest,
):
    class UserFSOfflineStateMachine(TrioAsyncioRuleBasedStateMachine):
        def __init__(self):
            super().__init__()

        async def start_user_fs(self, device: LocalDevice):
            async def _user_fs_controlled_cb(started_cb):
                async with user_fs_factory(device=device) as user_fs:
                    await started_cb(user_fs=user_fs)

            nursery: trio.Nursery = self.get_root_nursery()
            self.user_fs_controller = await nursery.start(call_with_control, _user_fs_controlled_cb)
            return self.user_fs_controller

        async def stop_user_fs(self):
            try:
                await self.user_fs_controller.stop()
            except AttributeError:
                pass

        async def restart_user_fs(self, device: LocalDevice):
            await self.stop_user_fs()
            await self.start_user_fs(device)

        async def reset_user_fs(self, device: LocalDevice):
            await self.stop_user_fs()
            del self.user_fs_controller.user_fs
            del self.user_fs_controller
            clear_database_dir(True)
            await self.start_user_fs(device)

        async def reset_all(self):
            await self.stop_user_fs()
            await reset_testbed(keep_logs=True)

        @property
        def user_fs(self):
            return self.user_fs_controller.user_fs

        @classmethod
        def run_as_test(cls):
            run_state_machine_as_test(cls, settings=hypothesis_settings)

    param: bool = request.param

    patched_wk_storage = bool(fixtures_customization.get("alternate_workspace_storage", False))
    if not patched_wk_storage and param:
        pytest.skip()

    if param:
        from parsec.core.fs.storage import UserStorage as RSUserStorage
        from parsec.core.fs.storage import WorkspaceStorage as RSWorkspaceStorage
        from parsec.core.fs.storage import user_storage_non_speculative_init as rs_user_storage_init
        from parsec.core.fs.storage import workspace_storage_non_speculative_init as rs_wk_init
        from parsec.core.fs.storage.workspace_storage import (
            DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
            FAILSAFE_PATTERN_FILTER,
        )
        from tests.core.fs.old_storage.user_storage import UserStorage as PyUserStorage
        from tests.core.fs.old_storage.user_storage import (
            user_storage_non_speculative_init as py_user_storage_init,
        )
        from tests.core.fs.old_storage.workspace_storage import (
            WorkspaceStorage as PyWorkspaceStorage,
        )
        from tests.core.fs.old_storage.workspace_storage import (
            workspace_storage_non_speculative_init as py_wk_init,
        )

        use_rust_storage_impl = True
        original_restart_user_fs = UserFSOfflineStateMachine.restart_user_fs

        async def toggle_restart_user_fs(self: UserFSOfflineStateMachine, device: LocalDevice):
            nonlocal use_rust_storage_impl

            use_rust_storage_impl = not use_rust_storage_impl
            await original_restart_user_fs(self, device=device)

        monkeypatch.setattr(UserFSOfflineStateMachine, "restart_user_fs", toggle_restart_user_fs)

        async def reset_choice_init(self: object, *args):
            nonlocal use_rust_storage_impl

            use_rust_storage_impl = True

        monkeypatch.setattr(UserFSOfflineStateMachine, "init", reset_choice_init, raising=False)

        original_wk_storage_run = RSWorkspaceStorage.run

        @asynccontextmanager
        async def switching_wk_storage_run(
            data_base_dir: Path,
            device: LocalDevice,
            workspace_id: EntryID,
            prevent_sync_pattern: Regex = FAILSAFE_PATTERN_FILTER,
            cache_size: int = DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
        ) -> AsyncIterator[RSWorkspaceStorage | PyWorkspaceStorage]:
            nonlocal use_rust_storage_impl

            if use_rust_storage_impl:
                print("[WorkspaceStorage::run] Using rust impl")
                async with original_wk_storage_run(
                    data_base_dir=data_base_dir,
                    device=device,
                    workspace_id=workspace_id,
                    prevent_sync_pattern=prevent_sync_pattern,
                    cache_size=cache_size,
                ) as workspace:
                    yield workspace
            else:
                print("[WorkspaceStorage::run] Using python impl")
                async with PyWorkspaceStorage.run(
                    data_base_dir=data_base_dir,
                    device=device,
                    workspace_id=workspace_id,
                    prevent_sync_pattern=prevent_sync_pattern,
                    cache_size=cache_size,
                ) as workspace:
                    yield workspace

        monkeypatch.setattr("parsec.core.fs.storage.WorkspaceStorage.run", switching_wk_storage_run)

        original_user_storage_run = RSUserStorage.run

        @asynccontextmanager
        async def switching_user_storage_run(
            data_base_dir: Path, device: LocalDevice
        ) -> AsyncIterator[RSUserStorage, PyUserStorage]:
            nonlocal use_rust_storage_impl

            if use_rust_storage_impl:
                print("[UserStorage::run] Using rust impl")
                async with original_user_storage_run(
                    data_base_dir=data_base_dir, device=device
                ) as user_storage:
                    yield user_storage
            else:
                print("[UserStorage::run] Using python impl")
                async with PyUserStorage.run(
                    data_base_dir=data_base_dir, device=device
                ) as user_storage:
                    yield user_storage

        monkeypatch.setattr("parsec.core.fs.storage.UserStorage.run", switching_user_storage_run)

        original_wk_init = rs_wk_init

        async def switching_wk_init(
            data_base_dir: Path,
            device: LocalDevice,
            workspace_id: EntryID,
        ) -> None:
            nonlocal use_rust_storage_impl

            if use_rust_storage_impl:
                print("[workspace_storage_non_speculative_init] Using rust impl")
                await original_wk_init(
                    data_base_dir=data_base_dir, device=device, workspace_id=workspace_id
                )
            else:
                print("[workspace_storage_non_speculative_init] Using python impl")
                await py_wk_init(
                    data_base_dir=data_base_dir, device=device, workspace_id=workspace_id
                )

        for path in [
            "parsec.core.fs.storage",
            "parsec.core.fs.storage.workspace_storage",
            "parsec.core.fs.userfs.userfs",
        ]:
            monkeypatch.setattr(path + ".workspace_storage_non_speculative_init", switching_wk_init)

        original_user_storage_init = rs_user_storage_init

        async def switching_user_storage_init(data_base_dir: Path, device: LocalDevice) -> None:
            nonlocal use_rust_storage_impl

            if use_rust_storage_impl:
                print("[user_storage_non_speculative_init] Using rust impl")
                await original_user_storage_init(data_base_dir=data_base_dir, device=device)
            else:
                print("[user_storage_non_speculative_init] Using python impl")
                await py_user_storage_init(data_base_dir=data_base_dir, device=device)

        for path in [
            "parsec.core.fs.storage",
            "parsec.core.fs.storage.user_storage",
        ]:
            monkeypatch.setattr(
                path + ".user_storage_non_speculative_init", switching_user_storage_init
            )

    return UserFSOfflineStateMachine


@pytest.fixture
def user_fs_online_state_machine(
    user_fs_offline_state_machine,
    backend_factory,
    running_backend_factory,
    reset_testbed,
):
    class UserFSOnlineStateMachine(user_fs_offline_state_machine):
        async def start_backend(self, **kwargs):
            async def _backend_controlled_cb(started_cb):
                async with backend_factory(**kwargs) as backend:
                    async with running_backend_factory(backend) as server:
                        await started_cb(backend=backend, server=server)

            self.backend_controller = await self.get_root_nursery().start(
                call_with_control, _backend_controlled_cb
            )

        async def stop_backend(self):
            try:
                await self.backend_controller.stop()
            except AttributeError:
                pass

        async def reset_all(self):
            await self.stop_user_fs()
            await self.stop_backend()
            await reset_testbed(keep_logs=True)

        @property
        def backend(self):
            return self.backend_controller.backend

        @property
        def correct_addr(self):
            return self.backend_controller.server.correct_addr

    return UserFSOnlineStateMachine
