import pytest

from parsec.core.fs.local_folder_fs import LocalFolderFS
from parsec.core.fs.local_file_fs import LocalFileFS


@pytest.fixture
def local_folder_fs_factory(alice, event_bus):
    def _local_folder_fs_factory(device=alice, allow_non_workpace_in_root=True):
        return LocalFolderFS(
            device, event_bus, allow_non_workpace_in_root=allow_non_workpace_in_root
        )

    return _local_folder_fs_factory


@pytest.fixture
def local_folder_fs(request, local_folder_fs_factory):
    # Big hack to simplify tests. Otherwise we must create (and
    # potentially synchronize) a workspace everytime we want to
    # test folder/file.
    if request.node.get_closest_marker("only_workpace_in_root"):
        allow_non_workpace_in_root = False
    else:
        allow_non_workpace_in_root = True

    return local_folder_fs_factory(allow_non_workpace_in_root=allow_non_workpace_in_root)


@pytest.fixture
def local_file_fs_factory(alice, local_folder_fs, event_bus):
    def _local_file_fs_factory(device=alice, local_folder_fs=local_folder_fs):
        return LocalFileFS(device, local_folder_fs, event_bus)

    return _local_file_fs_factory


@pytest.fixture
def local_file_fs(local_file_fs_factory):
    return local_file_fs_factory()
