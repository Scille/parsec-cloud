import pytest

from parsec.core.fs.local_folder_fs import LocalFolderFS
from parsec.core.fs.local_file_fs import LocalFileFS


@pytest.fixture
def local_folder_fs_factory(alice, event_bus):
    def _local_folder_fs_factory(device=alice):
        return LocalFolderFS(device, event_bus)

    return _local_folder_fs_factory


@pytest.fixture
def local_folder_fs(request, local_folder_fs_factory):
    return local_folder_fs_factory()


@pytest.fixture
def local_file_fs_factory(alice, local_folder_fs, event_bus):
    def _local_file_fs_factory(device=alice, local_folder_fs=local_folder_fs):
        return LocalFileFS(device, local_folder_fs, event_bus)

    return _local_file_fs_factory


@pytest.fixture
def local_file_fs(local_file_fs_factory):
    return local_file_fs_factory()
