# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from parsec._parsec import DateTime
from contextlib import asynccontextmanager
from hypothesis_trio.stateful import run_state_machine_as_test, TrioAsyncioRuleBasedStateMachine

from parsec.api.data import EntryName
from parsec.core.fs.storage import WorkspaceStorage
from parsec.core.fs.workspacefs.file_transactions import FileTransactions
from parsec.core.fs.workspacefs.entry_transactions import EntryTransactions
from parsec.core.fs.workspacefs.sync_transactions import SyncTransactions
from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.backend_connection.authenticated import backend_authenticated_cmds_factory
from parsec.core.types import LocalWorkspaceManifest, WorkspaceEntry, EntryID

from tests.common import call_with_control


@pytest.fixture
def transactions_factory(event_bus, remote_devices_manager_factory, core_config):
    @asynccontextmanager
    async def _transactions_factory(device, local_storage, cls=SyncTransactions):
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
    async def _file_transactions_factory(device, local_storage):
        async with transactions_factory(
            device, local_storage=local_storage, cls=FileTransactions
        ) as file_transactions:
            yield file_transactions

    return _file_transactions_factory


@pytest.fixture
async def alice_transaction_local_storage(alice, tmp_path):
    async with WorkspaceStorage.run(
        tmp_path / "alice_transaction_local_storage", alice, EntryID.new()
    ) as storage:
        yield storage


@pytest.fixture
async def alice_file_transactions(
    file_transactions_factory, alice, alice_transaction_local_storage
):
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


@pytest.fixture
def user_fs_offline_state_machine(
    user_fs_factory, persistent_mockup, reset_testbed, hypothesis_settings
):
    class UserFSOfflineStateMachine(TrioAsyncioRuleBasedStateMachine):
        async def start_user_fs(self, device):
            async def _user_fs_controlled_cb(started_cb):
                async with user_fs_factory(device=device) as user_fs:
                    await started_cb(user_fs=user_fs)

            self.user_fs_controller = await self.get_root_nursery().start(
                call_with_control, _user_fs_controlled_cb
            )
            return self.user_fs_controller

        async def stop_user_fs(self):
            try:
                await self.user_fs_controller.stop()
            except AttributeError:
                pass

        async def restart_user_fs(self, device):
            await self.stop_user_fs()
            await self.start_user_fs(device)

        async def reset_user_fs(self, device):
            await self.stop_user_fs()
            persistent_mockup.clear()
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

    return UserFSOfflineStateMachine


@pytest.fixture
def user_fs_online_state_machine(
    user_fs_offline_state_machine, backend_factory, running_backend_factory, reset_testbed
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
