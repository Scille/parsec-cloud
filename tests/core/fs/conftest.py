# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.core.fs.local_folder_fs import LocalFolderFS
from parsec.core.fs.file_transactions import FileTransactions
from parsec.core.fs.remote_loader import RemoteLoader


@pytest.fixture
def local_folder_fs_factory(event_bus):
    def _local_folder_fs_factory(device, local_storage):
        return LocalFolderFS(device, local_storage, event_bus)

    return _local_folder_fs_factory


@pytest.fixture
def local_folder_fs(local_folder_fs_factory, alice, alice_local_storage):
    return local_folder_fs_factory(alice, alice_local_storage)


@pytest.fixture
def file_transactions_factory(event_bus, remote_devices_manager_factory):
    def _file_transactions_factory(device, local_storage, backend_cmds):
        remote_devices_manager = remote_devices_manager_factory(device)
        remote_loader = RemoteLoader(backend_cmds, remote_devices_manager, local_storage)
        return FileTransactions(local_storage, remote_loader, event_bus)

    return _file_transactions_factory


@pytest.fixture
def file_transactions(file_transactions_factory, alice, alice_local_storage, alice_backend_cmds):
    return file_transactions_factory(alice, alice_local_storage, alice_backend_cmds)
