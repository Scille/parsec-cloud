# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import threading


@pytest.mark.win32
@pytest.mark.mountpoint
def test_rename_to_another_drive(mountpoint_service):
    async def _bootstrap(user_fs, mountpoint_manager):
        xid = await user_fs.workspace_create("x")
        xworkspace = user_fs.get_workspace(xid)
        await xworkspace.touch("/foo.txt")
        await user_fs.workspace_create("y")
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
@pytest.mark.mountpoint
def test_close_trio_loop_during_fs_access(mountpoint_service, monkeypatch):
    mountpoint_needed_stop = threading.Event()
    mountpoint_stopped = threading.Event()

    from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess

    vanilla_entry_info = ThreadFSAccess.entry_info

    def _entry_info_maybe_stop_loop(self, path):
        if str(path) == "/stop_loop":
            mountpoint_needed_stop.set()
            mountpoint_stopped.wait()
        return vanilla_entry_info(self, path)

    monkeypatch.setattr(
        "parsec.core.mountpoint.thread_fs_access.ThreadFSAccess.entry_info",
        _entry_info_maybe_stop_loop,
    )

    def _wait_and_stop_mountpoint():
        mountpoint_needed_stop.wait()
        try:
            mountpoint_service.stop(reset_testbed=False)
        finally:
            mountpoint_stopped.set()

    thread = threading.Thread(target=_wait_and_stop_mountpoint)
    thread.setName("MountpointService")
    thread.start()
    try:

        mountpoint_service.start()
        mountpoint = mountpoint_service.get_default_workspace_mountpoint()
        with pytest.raises(OSError) as exc:
            (mountpoint / "stop_loop").stat()

        assert str(exc.value).startswith(
            "[WinError 995] The I/O operation has been aborted because of either a thread exit or an application request"
        )

    finally:
        mountpoint_needed_stop.set()
        mountpoint_stopped.wait()
        thread.join()


@pytest.mark.win32
@pytest.mark.mountpoint
def test_mount_workspace_with_non_win32_friendly_name(mountpoint_service):
    # see https://docs.microsoft.com/en-us/windows/win32/fileio/naming-a-file
    bad_names = (
        '<>:"\\|?*',  # Invalid chars (except `/` which is not allowed in Parsec)
        "".join(chr(x) for x in range(1, 32)),  # Invalid control characters
        "foo.",  # Trailing dot not allowed
        "foo ",  # Trailing space not allowed
        "CON",  # Invalid name
        "COM1.foo",  # Invalid name with extension
    )

    async def _bootstrap(user_fs, mountpoint_manager):

        for bad_name in bad_names:
            # Apply bad name to both the mountpoint folder and data inside it
            wid = await user_fs.workspace_create(bad_name)
            workspace = user_fs.get_workspace(wid)
            await workspace.touch(f"/{bad_name}")
        await mountpoint_manager.mount_all()

    mountpoint_service.start()
    mountpoint_service.execute(_bootstrap)

    workspaces = list(mountpoint_service.base_mountpoint.iterdir())

    # mountpoint_service creates a `w` workspace by default
    assert len(workspaces) == len(bad_names) + 1

    for workspace in workspaces:
        if workspace.name == "w":
            continue
        assert workspace.exists()
        entries = list(workspace.iterdir())
        assert len(entries) == 1
        assert "~" in entries[0].name  # Tild escape the invalid part of the name
        assert entries[0].exists()
