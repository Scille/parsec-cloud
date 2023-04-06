# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import errno
import os
import sys
from itertools import count
from pathlib import Path, PurePath

import pytest
import trio

from parsec._parsec import CoreEvent, DateTime
from parsec.api.data import EntryID, EntryName
from parsec.core import logged_core_factory
from parsec.core.fs import FsPath
from parsec.core.mountpoint import (
    MountpointAlreadyMounted,
    MountpointConfigurationError,
    MountpointFuseNotAvailable,
    MountpointNotMounted,
    MountpointWinfspNotAvailable,
    mountpoint_manager_factory,
)
from parsec.core.types import WorkspaceRole
from tests.common import create_shared_workspace, real_clock_timeout


# Helper


def get_path_in_mountpoint(manager, wid, path, timestamp=None):
    return trio.Path(manager.get_path_in_mountpoint(wid, FsPath(path), timestamp=timestamp))


@pytest.mark.trio
async def test_runner_not_available(monkeypatch, alice_user_fs, event_bus):
    base_mountpoint = Path("/foo")

    def _import(name):
        if name == "winfspy":
            raise RuntimeError()
        else:
            raise ImportError()

    monkeypatch.setattr("parsec.core.mountpoint.manager.import_function", _import)
    with pytest.raises((MountpointFuseNotAvailable, MountpointWinfspNotAvailable)):
        async with mountpoint_manager_factory(alice_user_fs, event_bus, base_mountpoint):
            pass


@pytest.mark.trio
async def test_mount_unknown_workspace(base_mountpoint, alice_user_fs, event_bus):
    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        wid = EntryID.new()
        with pytest.raises(MountpointConfigurationError) as exc:
            await mountpoint_manager.mount_workspace(wid)

        assert exc.value.args == (f"Workspace `{wid.hex}` doesn't exist",)


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_base_mountpoint_not_created(base_mountpoint, alice_user_fs, event_bus):
    # Path should be created if it doesn' exist
    base_mountpoint = base_mountpoint / "dummy/dummy/dummy"

    wid = await alice_user_fs.workspace_create(EntryName("w"))
    workspace = alice_user_fs.get_workspace(wid)
    await workspace.touch("/bar.txt")

    # Now we can start fuse

    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        await mountpoint_manager.mount_workspace(wid)
        bar_txt = get_path_in_mountpoint(mountpoint_manager, wid, "/bar.txt")
        assert await bar_txt.exists()


@pytest.mark.trio
@pytest.mark.mountpoint
@pytest.mark.skipif(sys.platform == "win32", reason="Windows uses drive")
async def test_mountpoint_path_already_in_use(
    base_mountpoint, running_backend, alice_user_fs, alice2_user_fs
):
    # Create a workspace and make it available in two devices
    wid = await alice_user_fs.workspace_create(EntryName("w"))
    await alice_user_fs.sync()
    await alice2_user_fs.sync()

    # Easily differentiate alice&alice2
    await alice2_user_fs.get_workspace(wid).touch("/I_am_alice2.txt")
    await alice_user_fs.get_workspace(wid).touch("/I_am_alice.txt")

    naive_workspace_path = (base_mountpoint / "w").absolute()

    # Default workspace path already exists, shouldn't be able to use it
    await trio.Path(base_mountpoint / "w").mkdir(parents=True)
    await trio.Path(base_mountpoint / "w" / "bar.txt").touch()

    async with mountpoint_manager_factory(
        alice_user_fs, alice_user_fs.event_bus, base_mountpoint
    ) as alice_mountpoint_manager, mountpoint_manager_factory(
        alice2_user_fs, alice2_user_fs.event_bus, base_mountpoint
    ) as alice2_mountpoint_manager:
        # Alice mount the workspace first
        alice_mountpoint_path = await alice_mountpoint_manager.mount_workspace(wid)
        assert str(alice_mountpoint_path) == f"{naive_workspace_path} (2)"

        # Alice2 should also be able to mount the workspace without name clashing
        alice2_mountpoint_path = await alice2_mountpoint_manager.mount_workspace(wid)
        assert str(alice2_mountpoint_path) == f"{naive_workspace_path} (3)"

        # Finally make sure each workspace is well mounted
        assert await trio.Path(alice_mountpoint_path / "I_am_alice.txt").exists()
        assert await trio.Path(alice2_mountpoint_path / "I_am_alice2.txt").exists()


@pytest.mark.trio
@pytest.mark.mountpoint
@pytest.mark.parametrize(
    "snapshot_workspace", [False, True], ids=["current_workspace", "snapshot_workspace"]
)
@pytest.mark.parametrize(
    "manual_unmount", [False, True], ids=["automatic_unmount", "manual_unmount"]
)
async def test_mount_and_explore_workspace(
    base_mountpoint, alice_user_fs, event_bus, running_backend, snapshot_workspace, manual_unmount
):
    # Populate a bit the fs first...

    wid = await alice_user_fs.workspace_create(EntryName("w"))
    workspace = alice_user_fs.get_workspace(wid)
    await workspace.mkdir("/foo")
    await workspace.touch("/bar.txt")
    await workspace.write_bytes("/bar.txt", b"Hello world !")

    if snapshot_workspace:
        await workspace.sync()
        timestamp = DateTime.now()
        await workspace.mkdir("/too_early")  # Should not be visible in the snapshot
        await workspace.sync()
    else:
        timestamp = None

    # Now we can start fuse

    with event_bus.listen() as spy:
        async with mountpoint_manager_factory(
            alice_user_fs, event_bus, base_mountpoint
        ) as mountpoint_manager:
            await mountpoint_manager.mount_workspace(wid, timestamp=timestamp)
            mountpoint_path = get_path_in_mountpoint(mountpoint_manager, wid, "/", timestamp)
            expected = {"mountpoint": mountpoint_path, "workspace_id": wid, "timestamp": timestamp}

            spy.assert_events_occurred(
                [
                    (CoreEvent.MOUNTPOINT_STARTING, expected),
                    (CoreEvent.MOUNTPOINT_STARTED, expected),
                ]
            )

            # Finally explore the mountpoint

            def inspect_mountpoint():
                wksp_children = set(os.listdir(mountpoint_path))
                assert wksp_children == {"foo", "bar.txt"}

                bar_stat = os.stat(f"{mountpoint_path}/bar.txt")
                assert bar_stat.st_size == len(b"Hello world !")

                with open(f"{mountpoint_path}/bar.txt", "rb") as fd:
                    bar_txt = fd.read()
                assert bar_txt == b"Hello world !"

            # Note given python fs api is blocking, we must run it inside a thread
            # to avoid blocking the trio loop and ending up in a deadlock
            await trio.to_thread.run_sync(inspect_mountpoint)

            if manual_unmount:
                await mountpoint_manager.unmount_workspace(wid, timestamp=timestamp)
                # Mountpoint should be stopped by now
                spy.assert_events_occurred([(CoreEvent.MOUNTPOINT_STOPPED, expected)])

        if not manual_unmount:
            # Mountpoint should be stopped by now
            spy.assert_events_occurred([(CoreEvent.MOUNTPOINT_STOPPED, expected)])


@pytest.mark.trio
@pytest.mark.mountpoint
@pytest.mark.parametrize(
    "snapshot_workspace", [False, True], ids=["current_workspace", "snapshot_workspace"]
)
@pytest.mark.parametrize(
    "manual_unmount", [False, True], ids=["automatic_unmount", "manual_unmount"]
)
async def test_idempotent_mount(
    base_mountpoint, alice_user_fs, event_bus, running_backend, snapshot_workspace, manual_unmount
):
    # Populate a bit the fs first...

    wid = await alice_user_fs.workspace_create(EntryName("w"))
    workspace = alice_user_fs.get_workspace(wid)
    await workspace.touch("/bar.txt")

    if snapshot_workspace:
        await workspace.sync()
        timestamp = DateTime.now()
        await workspace.mkdir("/too_early")  # Should not be visible in the snapshot
        await workspace.sync()
    else:
        timestamp = None

    # Now we can start fuse

    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        await mountpoint_manager.mount_workspace(wid, timestamp=timestamp)
        bar_txt = get_path_in_mountpoint(mountpoint_manager, wid, "/bar.txt", timestamp)

        assert await bar_txt.exists()

        with pytest.raises(MountpointAlreadyMounted):
            await mountpoint_manager.mount_workspace(wid, timestamp=timestamp)
        assert await bar_txt.exists()

        await mountpoint_manager.unmount_workspace(wid, timestamp=timestamp)
        assert not await bar_txt.exists()

        with pytest.raises(MountpointNotMounted):
            await mountpoint_manager.unmount_workspace(wid, timestamp=timestamp)
        assert not await bar_txt.exists()

        await mountpoint_manager.mount_workspace(wid, timestamp=timestamp)
        assert await bar_txt.exists()


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_work_within_logged_core(base_mountpoint, core_config, alice):
    core_config = core_config.evolve(mountpoint_base_dir=base_mountpoint)

    async with logged_core_factory(core_config, alice) as alice_core:
        manager = alice_core.mountpoint_manager
        wid = await alice_core.user_fs.workspace_create(EntryName("w"))
        workspace = alice_core.user_fs.get_workspace(wid)
        await workspace.touch("/bar.txt")

        with pytest.raises(MountpointNotMounted):
            get_path_in_mountpoint(manager, wid, "/bar.txt")
        await manager.mount_workspace(wid)
        bar_txt = get_path_in_mountpoint(manager, wid, "/bar.txt")
        assert await bar_txt.exists()

    assert not await bar_txt.exists()


@pytest.mark.mountpoint
def test_manifest_not_available(mountpoint_service_factory):
    x_path = None

    async def _bootstrap(user_fs, mountpoint_manager):
        nonlocal x_path
        wid = await user_fs.workspace_create(EntryName("x"))
        workspace = user_fs.get_workspace(wid)
        await workspace.touch("/foo.txt")
        foo_id = await workspace.path_id("/foo.txt")
        async with workspace.local_storage.lock_entry_id(foo_id):
            await workspace.local_storage.clear_manifest(foo_id)
        x_path = await mountpoint_manager.mount_workspace(wid)

    mountpoint_service_factory(_bootstrap)

    with pytest.raises(OSError) as exc:
        Path(x_path / "foo.txt").stat()
    if sys.platform == "win32":
        # This winerror code corresponds to ntstatus.STATUS_HOST_UNREACHABLE
        ERROR_HOST_UNREACHABLE = 1232
        assert exc.value.winerror == ERROR_HOST_UNREACHABLE
    else:
        assert exc.value.errno == errno.EHOSTUNREACH


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_get_path_in_mountpoint(base_mountpoint, alice_user_fs, event_bus):
    # Populate a bit the fs first...
    wid = await alice_user_fs.workspace_create(EntryName("mounted_wksp"))
    wid2 = await alice_user_fs.workspace_create(EntryName("not_mounted_wksp"))
    workspace1 = alice_user_fs.get_workspace(wid)
    workspace2 = alice_user_fs.get_workspace(wid2)
    await workspace1.touch("/bar.txt")
    await workspace2.touch("/foo.txt")

    # Now we can start fuse
    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        await mountpoint_manager.mount_workspace(wid)

        bar_path = mountpoint_manager.get_path_in_mountpoint(wid, FsPath("/bar.txt"))

        assert isinstance(bar_path, PurePath)
        # Windows uses drives, not base_mountpoint
        if sys.platform != "win32":
            expected = base_mountpoint / "mounted_wksp" / "bar.txt"
            assert str(bar_path) == str(expected.absolute())
        assert await trio.Path(bar_path).exists()

        with pytest.raises(MountpointNotMounted):
            mountpoint_manager.get_path_in_mountpoint(wid2, FsPath("/foo.txt"))


@pytest.mark.mountpoint
def test_unhandled_crash_in_fs_operation(caplog, mountpoint_service, monkeypatch):
    from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess

    vanilla_entry_info = ThreadFSAccess.entry_info

    def _entry_info_crash(self, path):
        if str(path) == "/crash_me":
            raise RuntimeError("Crashed !")
        else:
            return vanilla_entry_info(self, path)

    monkeypatch.setattr(
        "parsec.core.mountpoint.thread_fs_access.ThreadFSAccess.entry_info", _entry_info_crash
    )

    with pytest.raises(OSError) as exc:
        Path(mountpoint_service.wpath / "crash_me").stat()

    assert exc.value.errno == errno.EINVAL
    if sys.platform == "win32":
        caplog.assert_occurred_once(
            "[error    ] Unhandled exception in winfsp mountpoint [parsec.core.mountpoint.winfsp_operations]"
        )

    else:
        caplog.assert_occurred_once(
            "[error    ] Unhandled exception in fuse mountpoint [parsec.core.mountpoint.fuse_operations]"
        )


@pytest.mark.trio
@pytest.mark.mountpoint
@pytest.mark.parametrize("revoking", ["read", "write"])
@pytest.mark.skipif(
    sys.platform == "darwin", reason="Does not raise PermissionError in assert_cannot_read"
)
async def test_mountpoint_revoke_access(
    base_mountpoint,
    alice_user_fs,
    alice2_user_fs,
    bob_user_fs,
    event_bus,
    running_backend,
    revoking,
):
    # Parametrization
    new_role = None if revoking == "read" else WorkspaceRole.READER

    # Bob creates and share two files with Alice
    wid = await create_shared_workspace(EntryName("w"), bob_user_fs, alice_user_fs, alice2_user_fs)
    workspace = bob_user_fs.get_workspace(wid)
    await workspace.touch("/foo.txt")
    await workspace.touch("/bar.txt")
    await workspace.touch("/to_delete.txt")
    await workspace.sync()
    # Retrieve workspace manifest v1 to replace the default empty speculative placeholder
    await alice_user_fs.get_workspace(wid).sync()
    await alice2_user_fs.get_workspace(wid).sync()

    def get_root_path(mountpoint_manager):
        root_path = mountpoint_manager.get_path_in_mountpoint(wid, FsPath("/"))
        # A trio path is required here, otherwise we risk a messy deadlock!
        return trio.Path(root_path)

    async def assert_cannot_read(mountpoint_manager, root_is_cached=False):
        root_path = get_root_path(mountpoint_manager)
        foo_path = root_path / "foo.txt"
        bar_path = root_path / "bar.txt"
        # For some reason, root_path.stat() does not trigger a new getattr call
        # to fuse operations if there has been a prior recent call to stat.
        if not root_is_cached:
            with pytest.raises(PermissionError):
                await root_path.stat()
        with pytest.raises(PermissionError):
            await foo_path.exists()
        with pytest.raises(PermissionError):
            await foo_path.read_bytes()
        with pytest.raises(PermissionError):
            await bar_path.exists()
        with pytest.raises(PermissionError):
            await bar_path.read_bytes()

    async def assert_cannot_write(mountpoint_manager, new_role):
        expected_error, expected_errno = PermissionError, errno.EACCES
        # On linux, errno.EROFS is not translated to a PermissionError
        if new_role is WorkspaceRole.READER and sys.platform != "win32":
            expected_error, expected_errno = OSError, errno.EROFS
        root_path = get_root_path(mountpoint_manager)
        foo_path = root_path / "foo.txt"
        bar_path = root_path / "bar.txt"
        with pytest.raises(expected_error) as ctx:
            await (root_path / "new_file.txt").touch()
        assert ctx.value.errno == expected_errno
        with pytest.raises(expected_error) as ctx:
            await (root_path / "new_directory").mkdir()
        assert ctx.value.errno == expected_errno
        with pytest.raises(expected_error) as ctx:
            await foo_path.write_bytes(b"foo contents")
        assert ctx.value.errno == expected_errno
        with pytest.raises(expected_error) as ctx:
            await foo_path.unlink()
        assert ctx.value.errno == expected_errno
        with pytest.raises(expected_error) as ctx:
            await bar_path.write_bytes(b"bar contents")
        assert ctx.value.errno == expected_errno
        with pytest.raises(expected_error) as ctx:
            await bar_path.unlink()

        def sync_open():
            for flag in (os.O_WRONLY, os.O_RDWR, os.O_RDWR | os.O_APPEND, os.O_WRONLY | os.O_EXCL):
                with pytest.raises(expected_error) as ctx:
                    os.open(foo_path, flag)
                assert ctx.value.errno == expected_errno

        await trio.to_thread.run_sync(sync_open)

    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        # Mount Bob workspace on Alice's side
        await mountpoint_manager.mount_workspace(wid)
        root_path = get_root_path(mountpoint_manager)

        # Alice can read
        await (root_path / "bar.txt").read_bytes()

        # Alice can write
        await (root_path / "bar.txt").write_bytes(b"test")

        # Alice can delete
        await (root_path / "to_delete.txt").unlink()
        assert not await (root_path / "to_delete.txt").exists()

        # Bob revokes Alice's read or write rights from her workspace
        await bob_user_fs.workspace_share(wid, alice_user_fs.device.user_id, new_role)

        # Let Alice process the info
        await alice_user_fs.process_last_messages()
        await alice2_user_fs.process_last_messages()

        # Alice still has read access
        if new_role is WorkspaceRole.READER:
            await (root_path / "bar.txt").read_bytes()

        # Alice no longer has read access
        else:
            await assert_cannot_read(mountpoint_manager, root_is_cached=True)

        # Alice no longer has write access
        await assert_cannot_write(mountpoint_manager, new_role)

    # Try again with Alice first device

    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        # Mount alice workspace on bob's side once again
        await mountpoint_manager.mount_workspace(wid)
        root_path = get_root_path(mountpoint_manager)

        # Alice still has read access
        if new_role is WorkspaceRole.READER:
            await (root_path / "bar.txt").read_bytes()

        # Alice no longer has read access
        else:
            await assert_cannot_read(mountpoint_manager, root_is_cached=True)

        # Alice no longer has write access
        await assert_cannot_write(mountpoint_manager, new_role)

    # Try again with Alice second device

    async with mountpoint_manager_factory(
        alice2_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        # Mount alice workspace on bob's side once again
        await mountpoint_manager.mount_workspace(wid)
        root_path = get_root_path(mountpoint_manager)

        # Alice still has read access
        if new_role is WorkspaceRole.READER:
            await (root_path / "bar.txt").read_bytes()

        # Alice no longer has read access
        else:
            await assert_cannot_read(mountpoint_manager, root_is_cached=True)

        # Alice no longer has write access
        await assert_cannot_write(mountpoint_manager, new_role)


@pytest.mark.mountpoint
@pytest.mark.skipif(sys.platform == "win32", reason="TODO: crash with WinFSP :'(")
def test_stat_mountpoint(mountpoint_service):
    async def _bootstrap(user_fs, mountpoint_manager):
        workspace = user_fs.get_workspace(mountpoint_service.wid)
        await workspace.touch("/foo.txt")

    mountpoint_service.execute(_bootstrap)

    assert os.listdir(str(mountpoint_service.base_mountpoint)) == [mountpoint_service.wpath.name]
    # Just make sure stats don't lead to a crash
    assert os.stat(str(mountpoint_service.base_mountpoint))
    assert os.stat(str(mountpoint_service.wpath))
    assert os.stat(str(mountpoint_service.wpath / "foo.txt"))


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_mountpoint_access_unicode(base_mountpoint, alice_user_fs, event_bus):
    weird_name = "ÉŸ奇怪😀🔫🐍"

    wid = await alice_user_fs.workspace_create(EntryName(weird_name))
    workspace = alice_user_fs.get_workspace(wid)
    await workspace.touch(f"/{weird_name}")

    # Now we can start fuse
    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        await mountpoint_manager.mount_workspace(wid)

        root_path = mountpoint_manager.get_path_in_mountpoint(wid, FsPath(f"/"))

        # Work around trio issue #1308 (https://github.com/python-trio/trio/issues/1308)
        items = await trio.to_thread.run_sync(
            lambda: [path.name for path in Path(root_path).iterdir()]
        )
        assert items == [weird_name]

        item_path = mountpoint_manager.get_path_in_mountpoint(wid, FsPath(f"/{weird_name}"))
        assert await trio.Path(item_path).exists()


@pytest.mark.mountpoint
def test_nested_rw_access(mountpoint_service):
    fpath = mountpoint_service.wpath / "foo.txt"

    with open(str(fpath), "ab") as f:
        f.write(b"whatever")
        f.flush()
        with open(str(fpath), "rb") as fin:
            data = fin.read()
            assert data == b"whatever"


@pytest.mark.trio
@pytest.mark.slow
@pytest.mark.mountpoint
@pytest.mark.parametrize("n", [10, 100, 1000])
@pytest.mark.parametrize("base_path", ["/", "/foo"])
async def test_mountpoint_iterdir_with_many_files(
    n, base_path, base_mountpoint, alice_user_fs, event_bus
):
    wid = await alice_user_fs.workspace_create(EntryName("w"))
    workspace = alice_user_fs.get_workspace(wid)
    await workspace.mkdir(base_path, parents=True, exist_ok=True)
    names = [f"some_file_{i:03d}.txt" for i in range(n)]
    path_list = [FsPath(f"{base_path}/{name}") for name in names]

    for path in path_list:
        await workspace.touch(path)

    # Now we can start fuse
    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        await mountpoint_manager.mount_workspace(wid)

        test_path = mountpoint_manager.get_path_in_mountpoint(wid, FsPath(base_path))

        # Work around trio issue #1308 (https://github.com/python-trio/trio/issues/1308)
        items = await trio.to_thread.run_sync(
            lambda: [path.name for path in Path(test_path).iterdir()]
        )
        assert items == names

        for path in path_list:
            item_path = mountpoint_manager.get_path_in_mountpoint(wid, path)
            assert await trio.Path(item_path).exists()


@pytest.mark.trio
@pytest.mark.mountpoint
async def test_cancel_mount_workspace(base_mountpoint, alice_user_fs, event_bus):
    """
    This function tests the race conditions between the mounting of a workspace
    and trio cancellation. In particular, it produces interesting results when trying to
    unmount a workspace while it's still initializing.

    The following timeout values are useful for more thorough testing:

        [x * 0.00001 for x in range(2000, 2500)]
    """
    wid = await alice_user_fs.workspace_create(EntryName("w"))

    # Reuse the same mountpoint manager for all the mountings to
    # make sure state is not populated by previous mount attempts
    async with mountpoint_manager_factory(
        alice_user_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:
        for timeout in count(0, 0.002):
            print(f"timeout: {timeout}")

            async with real_clock_timeout():
                with trio.move_on_after(timeout) as cancel_scope:
                    await mountpoint_manager.mount_workspace(wid)

                if cancel_scope.cancelled_caught:
                    with pytest.raises(MountpointNotMounted):
                        mountpoint_manager.get_path_in_mountpoint(wid, FsPath("/"))
                else:
                    # Sanity check
                    path = trio.Path(mountpoint_manager.get_path_in_mountpoint(wid, FsPath("/")))
                    await path.exists()
                    # Timeout has become too high to be useful, time to stop the test
                    break


@pytest.mark.mountpoint
def test_deadlock_detection(mountpoint_service, caplog, monkeypatch):
    async def _in_trio_land(user_fs, mountpoint_manager):
        # We're in trio-land, performing a sync-call to the mountpoint
        # This creates a deadlock as the file system thread is going to try
        # to reach the trio thread which is stuck in the sync call
        # Let's make sure the deadlock detection works
        with pytest.raises(OSError) as ctx:
            os.open(mountpoint_service.wpath / "foo.txt", os.O_RDONLY)

        # TODO: Inconsistent status returned by macOS, it might be because the
        # mountpoint is not fully ready when we try our access (though it is
        # unlikely given we do active stat polling as part of the mount
        # operation especially to avoid such situation... on top of that it
        # seems retrying the os access always return a ENXIO errno)
        if sys.platform == "darwin" and ctx.value.errno == errno.ENXIO:
            pytest.xfail("TODO: macOS inconsistent ENXIO errno")

        # It is possible to have the OS trying to access files in our back
        # (e.g. macOS doing statis on the infamous `._file` files), which is
        # going to create extra errors logs. Hence we check we have at least one
        # "trio thread is unreachable" error (and not exactly one !).
        if sys.platform == "win32":
            caplog.assert_occurred(
                "[error    ] The trio thread is unreachable, a deadlock might have occurred [parsec.core.mountpoint.winfsp_operations]"
            )
        else:
            assert ctx.value.errno == errno.EINVAL
            caplog.assert_occurred(
                "[error    ] The trio thread is unreachable, a deadlock might have occurred [parsec.core.mountpoint.fuse_operations]"
            )

    # Lower the deadlock timeout detection to 100 ms to make the test faster
    monkeypatch.setattr(
        "parsec.core.mountpoint.thread_fs_access.ThreadFSAccess.DEADLOCK_TIMEOUT", 0.1
    )
    mountpoint_service.execute(_in_trio_land)
