import pytest
from unittest.mock import patch
import os
import attr
import trio

from parsec.core.fuse import FuseManager, FuseNotAvailable, FUSE_AVAILABLE


@pytest.mark.trio
async def test_fuse_not_available(alice_fs, event_bus):
    with patch("parsec.core.fuse.manager.FUSE_AVAILABLE", new=False):
        with pytest.raises(FuseNotAvailable):
            FuseManager(alice_fs, event_bus)


@pytest.mark.trio
@pytest.mark.skipif(not FUSE_AVAILABLE, reason="libfuse/fusepy not installed")
async def test_mount_fuse(alice_fs, event_bus, tmpdir):

    # Populate a bit the fs first...

    await alice_fs.folder_create("/foo")
    await alice_fs.file_create("/bar.txt")
    await alice_fs.file_write("/bar.txt", b"Hello world !")

    try:
        # Now we can start fuse

        mountpoint = "%s/fuse_mountpoint" % tmpdir
        manager = FuseManager(alice_fs, event_bus)
        with event_bus.listen() as spy:
            await manager.start(mountpoint)
        spy.assert_events_occured(
            [
                ("fuse.mountpoint.starting", {"mountpoint": mountpoint}),
                ("fuse.mountpoint.started", {"mountpoint": mountpoint}),
            ]
        )

        # Finally explore the mountpoint

        def inspect_mountpoint():
            children = set(os.listdir(mountpoint))
            assert children == {"foo", "bar.txt"}

            with open("%s/bar.txt" % mountpoint, "rb") as fd:
                bar_txt = fd.read()
            assert bar_txt == b"Hello world !"

        # Note given python fs api is blocking, we must run it inside a thread
        # to avoid blocking the trio loop and ending up in a deadlock
        await trio.run_sync_in_worker_thread(inspect_mountpoint)

    finally:
        await manager.stop()


@pytest.mark.trio
@pytest.mark.skipif(not FUSE_AVAILABLE, reason="libfuse/fusepy not installed")
@pytest.mark.parametrize("fuse_stop_mode", ["manual", "logout"])
async def test_umount_fuse(alice_core, tmpdir, fuse_stop_mode):
    mountpoint = "%s/fuse_mountpoint" % tmpdir

    await alice_core.fuse_manager.start(mountpoint)
    assert alice_core.fuse_manager.is_started()

    if fuse_stop_mode == "manual":
        await alice_core.fuse_manager.stop()
        assert not alice_core.fuse_manager.is_started()
    else:
        await alice_core.logout()
