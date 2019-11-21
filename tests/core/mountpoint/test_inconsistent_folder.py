# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import errno
import pytest
import trio

from parsec.core.mountpoint import mountpoint_manager_factory

from tests.core.fs.workspacefs.conftest import create_inconsistent_workspace

# This winerror code corresponds to ntstatus.STATUS_HOST_UNREACHABLE
WINDOWS_ERROR_HOST_UNREACHABLE = 1232


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_inconsistent_folder_no_network(base_mountpoint, running_backend, alice_user_fs):
    async with mountpoint_manager_factory(
        alice_user_fs, alice_user_fs.event_bus, base_mountpoint
    ) as alice_mountpoint_manager:
        workspace = await create_inconsistent_workspace(alice_user_fs)
        mountpoint_path = await alice_mountpoint_manager.mount_workspace(workspace.workspace_id)
        assert mountpoint_path == (base_mountpoint / "w").absolute()
        with running_backend.offline():
            await trio.to_thread.run_sync(
                _os_tests,
                mountpoint_path,
                WINDOWS_ERROR_HOST_UNREACHABLE if os.name == "nt" else errno.EHOSTUNREACH,
                "No route to host",
            )


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_inconsistent_folder_with_network(base_mountpoint, running_backend, alice_user_fs):
    async with mountpoint_manager_factory(
        alice_user_fs, alice_user_fs.event_bus, base_mountpoint
    ) as alice_mountpoint_manager:
        workspace = await create_inconsistent_workspace(alice_user_fs)
        mountpoint_path = await alice_mountpoint_manager.mount_workspace(workspace.workspace_id)
        assert mountpoint_path == (base_mountpoint / "w").absolute()
        await trio.to_thread.run_sync(
            _os_tests,
            mountpoint_path,
            errno.EIO if os.name == "nt" else errno.EACCES,
            "Permission denied",
        )


def _os_tests(mountpoint_path, error_code, error_str):
    assert ((mountpoint_path / "rep").stat()).st_nlink == 1 if os.name == "nt" else 2
    os.listdir(mountpoint_path)

    if os.name == "nt":
        with pytest.raises(OSError) as exc:
            os.listdir(mountpoint_path / "rep")
        assert exc.value.winerror == error_code
        assert exc.value.filename[-3:] == "rep"
    else:
        assert os.listdir(mountpoint_path / "rep") == ["foo.txt", "newfail.txt"]

    if os.name == "nt":
        with pytest.raises(OSError) as exc:
            [dir_entry for dir_entry in os.scandir(mountpoint_path / "rep")]
        assert exc.value.winerror == error_code
        assert exc.value.filename[-3:] == "rep"
    else:
        entries = [dir_entry for dir_entry in os.scandir(mountpoint_path / "rep")]
        with pytest.raises(OSError) as exc:
            [os.stat(entry) for entry in entries]
        assert exc.value.errno == error_code
        assert exc.value.filename[-16:] == "/rep/newfail.txt"
        assert exc.value.strerror == error_str
