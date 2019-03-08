# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.core.fs.local_folder_fs import LocalFolderFS
from parsec.core.fs.local_file_fs import LocalFileFS


@pytest.fixture
def local_folder_fs_factory(event_bus):
    def _local_folder_fs_factory(device, local_storage):
        return LocalFolderFS(device, local_storage, event_bus)

    return _local_folder_fs_factory


@pytest.fixture
def local_folder_fs(local_folder_fs_factory, alice, alice_local_storage):
    return local_folder_fs_factory(alice, alice_local_storage)


@pytest.fixture
def local_file_fs_factory(local_folder_fs_factory, event_bus):
    def _local_file_fs_factory(device, local_storage, local_folder_fs=None):
        local_folder_fs = local_folder_fs or local_folder_fs_factory(device, local_storage)
        return LocalFileFS(device, local_storage, local_folder_fs, event_bus)

    return _local_file_fs_factory


@pytest.fixture
def local_file_fs(local_folder_fs, local_file_fs_factory, alice, alice_local_storage):
    # local_folder_fs contains a cache, so cannot have multiple instances
    # for a single device.
    return local_file_fs_factory(alice, alice_local_storage, local_folder_fs=local_folder_fs)
