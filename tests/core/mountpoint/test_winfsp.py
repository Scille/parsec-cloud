# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import os
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
def test_teardown_during_fs_access(mountpoint_service, monkeypatch):
    mountpoint_needed_stop = threading.Event()
    mountpoint_winfsp_stop = threading.Event()
    mountpoint_stopped = threading.Event()

    from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess
    from winfspy import FileSystem

    # Monkeypatch to trigger mountpoint teardown during an operation

    vanilla_entry_info = ThreadFSAccess.entry_info

    def _entry_info_maybe_stop_loop(self, path):
        if str(path) == "/stop_loop":
            mountpoint_needed_stop.set()
            # WinFSP's stop waits for the current operations (so including this
            # function !) to finish before returning.
            # Given this is the behavior we are testing here, make sure we
            # have actually reach this point before going further.
            mountpoint_winfsp_stop.wait()
        return vanilla_entry_info(self, path)

    monkeypatch.setattr(
        "parsec.core.mountpoint.thread_fs_access.ThreadFSAccess.entry_info",
        _entry_info_maybe_stop_loop,
    )

    # Monkeypatch WinFSP's stop to know when we are about to wait for the
    # current operations to finish

    vanilla_file_system_stop = FileSystem.stop

    def _patched_file_system_stop(self):
        mountpoint_winfsp_stop.set()
        return vanilla_file_system_stop(self)

    monkeypatch.setattr("winfspy.FileSystem.stop", _patched_file_system_stop)

    # Spawn a thread responsible for the mountpoint teardown when needed

    def _wait_and_stop_mountpoint():
        mountpoint_needed_stop.wait()
        try:
            mountpoint_service.stop(reset_testbed=False)
        finally:
            mountpoint_stopped.set()

    thread = threading.Thread(target=_wait_and_stop_mountpoint)
    thread.setName("MountpointService")
    thread.start()

    # All boilerplates are set, let's do the actual test !

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
    items = (
        (
            # Invalid chars (except `/` which is not allowed in Parsec)
            '<>:"\\|?*',
            "~3c~3e~3a~22~5c~7c~3f~2a",
        ),
        (
            # Invalid control characters (1/2)
            "".join(chr(x) for x in range(1, 16)),
            "".join(f"~{x:02x}" for x in range(1, 16)),
        ),
        (
            # Invalid control characters (2/2)
            "".join(chr(x) for x in range(16, 32)),
            "".join(f"~{x:02x}" for x in range(16, 32)),
        ),
        (
            # Trailing dot not allowed
            "foo.",
            "foo~2e",
        ),
        (
            # Trailing space not allowed
            "foo ",
            "foo~20",
        ),
        (
            # Invalid name
            "CON",
            "CO~4e",
        ),
        (
            # Invalid name with extension
            "COM1.foo",
            "COM~31.foo",
        ),
    )

    async def _bootstrap(user_fs, mountpoint_manager):

        for name, _ in items:
            # Apply bad name to both the mountpoint folder and data inside it
            wid = await user_fs.workspace_create(name)
            workspace = user_fs.get_workspace(wid)
            await workspace.touch(f"/{name}")
        await mountpoint_manager.mount_all()

    mountpoint_service.start()
    mountpoint_service.execute(_bootstrap)

    workspaces = list(mountpoint_service.base_mountpoint.iterdir())

    # mountpoint_service creates a `w` workspace by default
    assert set(x.name for x in workspaces) == {"w", *{cooked_name for _, cooked_name in items}}

    for _, cooked_name in items:
        workspace = mountpoint_service.base_mountpoint / cooked_name
        # TODO: currently failed with a `[WinError 1005] The volume does not contain a recognized file system.`
        # assert workspace.exists()

        entries = list(workspace.iterdir())
        assert [x.name for x in entries] == [cooked_name]
        assert entries[0].exists()


@pytest.mark.win32
@pytest.mark.mountpoint
def test_mount_workspace_with_too_long_name(mountpoint_service):
    # WinFSP volume_label (with no trailing '\0') is stored on 32 WCHAR
    too_long = "x" * 33
    # Smiley takes 4 bytes (2 WCHAR) once encoded in UTF-16
    too_long_once_encoded = "x" + "😀" * 16

    async def _bootstrap(user_fs, mountpoint_manager):
        wid = await user_fs.workspace_create(too_long)
        workspace = user_fs.get_workspace(wid)
        await workspace.touch(f"/foo.txt")

        wid = await user_fs.workspace_create(too_long_once_encoded)
        workspace = user_fs.get_workspace(wid)
        await workspace.touch(f"/foo.txt")

        await mountpoint_manager.mount_all()

    mountpoint_service.start()
    mountpoint_service.execute(_bootstrap)

    # TODO: should be doing a `workspace.exists()` instead
    assert (mountpoint_service.base_mountpoint / too_long / "foo.txt").exists()
    assert (mountpoint_service.base_mountpoint / too_long_once_encoded / "foo.txt").exists()


@pytest.mark.win32
@pytest.mark.mountpoint
def test_iterdir_with_marker(mountpoint_service):
    expected_entries_names = []

    async def _bootstrap(user_fs, mountpoint_manager):
        workspace = user_fs.get_workspace(mountpoint_service.default_workspace_id)
        for i in range(150):
            if i < 50:
                # File name < `..` (`..` is always the first item in our implementation)
                path = f"/.-{i}"
            else:
                path = f"/{i}"
            if i % 2:
                await workspace.touch(path)
            else:
                await workspace.mkdir(path)
            expected_entries_names.append(path[1:])

    mountpoint_service.start()
    mountpoint_service.execute(_bootstrap)
    expected_entries_names = sorted(expected_entries_names)

    mountpoint = mountpoint_service.get_default_workspace_mountpoint()
    # Note `os.listdir()` ignores `.` and `..` entries
    entries_names = os.listdir(mountpoint)
    assert entries_names == expected_entries_names
