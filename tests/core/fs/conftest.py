# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.core.fs.workspacefs.file_transactions import FileTransactions
from parsec.core.fs.workspacefs.entry_transactions import EntryTransactions
from parsec.core.fs.workspacefs.sync_transactions import SyncTransactions
from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.types import WorkspaceEntry, LocalWorkspaceManifest

from tests.common import freeze_time


@pytest.fixture
def transactions_factory(event_bus, remote_devices_manager_factory):
    async def _transactions_factory(device, local_storage, backend_cmds, cls=SyncTransactions):
        def _get_workspace_entry():
            return workspace_entry

        with freeze_time("2000-01-01"):
            workspace_entry = WorkspaceEntry("test")
            workspace_manifest = LocalWorkspaceManifest.make_placeholder(
                entry_id=workspace_entry.id, author=device.device_id
            )
        async with local_storage.lock_entry_id(workspace_entry.id):
            local_storage.set_manifest(workspace_entry.id, workspace_manifest)

        remote_devices_manager = remote_devices_manager_factory(device)
        remote_loader = RemoteLoader(
            device,
            workspace_entry.id,
            _get_workspace_entry,
            backend_cmds,
            remote_devices_manager,
            local_storage,
        )

        return cls(
            workspace_entry.id,
            _get_workspace_entry,
            device,
            local_storage,
            remote_loader,
            event_bus,
        )

    return _transactions_factory


@pytest.fixture
def file_transactions_factory(event_bus, remote_devices_manager_factory, transactions_factory):
    async def _file_transactions_factory(device, local_storage, backend_cmds):
        return await transactions_factory(device, local_storage, backend_cmds, cls=FileTransactions)

    return _file_transactions_factory


@pytest.fixture
async def file_transactions(
    file_transactions_factory, alice, alice_local_storage, alice_backend_cmds
):
    return await file_transactions_factory(alice, alice_local_storage, alice_backend_cmds)


@pytest.fixture
def entry_transactions_factory(event_bus, remote_devices_manager_factory, transactions_factory):
    async def _entry_transactions_factory(device, local_storage, backend_cmds):
        return await transactions_factory(
            device, local_storage, backend_cmds, cls=EntryTransactions
        )

    return _entry_transactions_factory


@pytest.fixture
async def entry_transactions(
    entry_transactions_factory, alice, alice_local_storage, alice_backend_cmds
):
    return await entry_transactions_factory(alice, alice_local_storage, alice_backend_cmds)


@pytest.fixture
async def sync_transactions(transactions_factory, alice, alice_local_storage, alice_backend_cmds):
    return await transactions_factory(alice, alice_local_storage, alice_backend_cmds)
