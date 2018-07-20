import pytest

from parsec.core.fs.local_folder_fs import LocalFolderFS
from parsec.core.fs.local_file_fs import LocalFileFS


@pytest.fixture
def local_folder_fs_factory(alice, signal_ns):
    def _local_folder_fs_factory(device=alice):
        return LocalFolderFS(device, signal_ns)

    return _local_folder_fs_factory


@pytest.fixture
def local_folder_fs(local_folder_fs_factory):
    return local_folder_fs_factory()


@pytest.fixture
def local_file_fs_factory(alice, local_folder_fs, signal_ns):
    def _local_file_fs_factory(device=alice, local_folder_fs=local_folder_fs):
        return LocalFileFS(device, local_folder_fs, signal_ns)

    return _local_file_fs_factory


@pytest.fixture
def local_file_fs(local_file_fs_factory):
    return local_file_fs_factory()
