# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import os
import sys
import errno
import pytest
import trio
from pathlib import Path

from parsec.core.mountpoint import mountpoint_manager_factory
from parsec.test_utils import create_inconsistent_workspace


# winerror codes corresponding to ntstatus errors
WINDOWS_ERROR_PERMISSION_DENIED = 5  # ntstatus.ERROR_ACCESS_DENIED
WINDOWS_ERROR_HOST_UNREACHABLE = 1232  # ntstatus.STATUS_HOST_UNREACHABLE


@pytest.mark.trio
@pytest.mark.mountpoint
@pytest.mark.skipif(sys.platform == "darwin", reason="TODO : crash on macOS")
async def test_inconsistent_folder_no_network(base_mountpoint, running_backend, alice_user_fs):
    async with mountpoint_manager_factory(
        alice_user_fs, alice_user_fs.event_bus, base_mountpoint
    ) as alice_mountpoint_manager:
        workspace = await create_inconsistent_workspace(alice_user_fs)
        mountpoint_path = Path(
            await alice_mountpoint_manager.mount_workspace(workspace.workspace_id)
        )
        with running_backend.offline():
            await trio.to_thread.run_sync(
                _os_tests, mountpoint_path, errno.EHOSTUNREACH, WINDOWS_ERROR_HOST_UNREACHABLE
            )


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_inconsistent_folder_with_network(base_mountpoint, running_backend, alice_user_fs):
    async with mountpoint_manager_factory(
        alice_user_fs, alice_user_fs.event_bus, base_mountpoint
    ) as alice_mountpoint_manager:
        workspace = await create_inconsistent_workspace(alice_user_fs)
        mountpoint_path = Path(
            await alice_mountpoint_manager.mount_workspace(workspace.workspace_id)
        )
        await trio.to_thread.run_sync(
            _os_tests, mountpoint_path, errno.EACCES, WINDOWS_ERROR_PERMISSION_DENIED
        )


def _os_tests(mountpoint_path, error_code, winerror):

    # Check stat of inconsistent dir counts one file on Windows, 2 on Linux
    assert ((mountpoint_path / "rep").stat()).st_nlink == 1 if sys.platform == "win32" else 2

    # Check listdir on workspace dir still works
    os.listdir(mountpoint_path)

    # Check listdir of inconsistent dir fails on Windows, works on Linux
    if sys.platform == "win32":
        with pytest.raises(OSError) as exc:
            os.listdir(mountpoint_path / "rep")
        assert exc.value.winerror == winerror
        assert exc.value.filename[-3:] == "rep"
    else:
        assert os.listdir(mountpoint_path / "rep") == ["foo.txt", "newfail.txt"]

    # Check scandir of inconsistent dir fails on Windows, works on Linux
    # But check that accessing stats of the inconsistent child is failing as expected on Linux
    if sys.platform == "win32":
        with pytest.raises(OSError) as exc:
            [dir_entry for dir_entry in os.scandir(mountpoint_path / "rep")]
        assert exc.value.winerror == winerror
        assert exc.value.filename[-3:] == "rep"
    else:
        entries = [dir_entry for dir_entry in os.scandir(mountpoint_path / "rep")]
        with pytest.raises(OSError) as exc:
            [os.stat(entry) for entry in entries]
        assert exc.value.errno == error_code
        assert exc.value.filename[-16:] == "/rep/newfail.txt"
