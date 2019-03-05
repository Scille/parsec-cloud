# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.core.types import FsPath


@pytest.mark.win32
def test_win32_move_to_another_drive(mountpoint_service):
    async def _bootstrap(fs, mountpoint_manager):
        await fs.workspace_create("/x")
        await fs.workspace_create("/y")
        await fs.file_create("/x/foo.txt")
        await mountpoint_manager.mount_all()

    mountpoint_service.start()
    mountpoint_service.execute(_bootstrap)
    x_path = mountpoint_service.get_workspace_mountpoint("x")
    y_path = mountpoint_service.get_workspace_mountpoint("y")

    with pytest.raises(OSError) as exc:
        (x_path / "foo.txt").rename(y_path)
    assert str(exc.value).startswith(
        "[WinError 17] The system cannot move the file to a different disk drive"
    )


@pytest.mark.win32
def test_win32_manifest_not_available(mountpoint_service):
    async def _bootstrap(fs, mountpoint_manager):
        await fs.workspace_create("/x")
        await fs.file_create("/x/foo.txt")
        foo_access = fs._local_folder_fs.get_access(FsPath("/x/foo.txt"))
        fs._local_folder_fs.mark_outdated_manifest(foo_access)
        await mountpoint_manager.mount_all()

    mountpoint_service.start()
    mountpoint_service.execute(_bootstrap)
    x_path = mountpoint_service.get_workspace_mountpoint("x")

    with pytest.raises(OSError) as exc:
        (x_path / "foo.txt").stat()
    assert str(exc.value).startswith("[WinError 1231] The network location cannot be reached.")
