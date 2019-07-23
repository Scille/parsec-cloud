# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import threading


@pytest.mark.win32
@pytest.mark.mountpoint
def test_rename_to_another_drive(mountpoint_service):
    async def _bootstrap(user_fs, mountpoint_manager):
        xid = await user_fs.workspace_create("x")
        xworkspace = await user_fs.get_workspace(xid)
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
            mountpoint_service.stop()
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
