import pytest
from unittest.mock import patch
import os
import trio

from parsec.core.mountpoint import (
    mountpoint_manager_factory,
    NotAvailableMountpointManager,
    FuseMountpointManager,
    MountpointManagerNotAvailable,
)


@pytest.mark.trio
async def test_fuse_not_available(alice_fs, event_bus):
    with patch("parsec.core.mountpoint.manager.FUSE_AVAILABLE", new=False):
        with pytest.raises(RuntimeError):
            FuseMountpointManager(alice_fs, event_bus)

    with patch("parsec.core.mountpoint.FUSE_AVAILABLE", new=False):
        mm = mountpoint_manager_factory(alice_fs, event_bus)
        assert isinstance(mm, NotAvailableMountpointManager)

        async with trio.open_nursery() as nursery:
            await mm.init(nursery)

            with pytest.raises(MountpointManagerNotAvailable):
                await mm.start("/foo")

            with pytest.raises(MountpointManagerNotAvailable):
                await mm.stop()

            with pytest.raises(MountpointManagerNotAvailable):
                mm.is_started()

            with pytest.raises(MountpointManagerNotAvailable):
                mm.get_abs_mountpoint()

            await mm.teardown()


@pytest.mark.trio
@pytest.mark.fuse
async def test_fuse_available(alice_fs, event_bus):
    mm = mountpoint_manager_factory(alice_fs, event_bus)
    assert isinstance(mm, FuseMountpointManager)


@pytest.mark.trio
@pytest.mark.fuse
async def test_mount_fuse(alice_fs, event_bus, tmpdir, monitor, fuse_mode):
    # Populate a bit the fs first...

    await alice_fs.folder_create("/foo")
    await alice_fs.file_create("/bar.txt")
    await alice_fs.file_write("/bar.txt", b"Hello world !")

    # Now we can start fuse

    mountpoint = f"{tmpdir}/fuse_mountpoint"
    manager = FuseMountpointManager(alice_fs, event_bus, mode=fuse_mode)
    async with trio.open_nursery() as nursery:
        try:
            await manager.init(nursery)
            with event_bus.listen() as spy:
                await manager.start(mountpoint)
            spy.assert_events_occured(
                [
                    ("mountpoint.starting", {"mountpoint": mountpoint}),
                    ("mountpoint.started", {"mountpoint": mountpoint}),
                ]
            )

            # Finally explore the mountpoint

            def inspect_mountpoint():
                children = set(os.listdir(mountpoint))
                assert children == {"foo", "bar.txt"}

                bar_stat = os.stat(f"{mountpoint}/bar.txt")
                assert bar_stat.st_size == len(b"Hello world !")

                with open("%s/bar.txt" % mountpoint, "rb") as fd:
                    bar_txt = fd.read()
                assert bar_txt == b"Hello world !"

            # Note given python fs api is blocking, we must run it inside a thread
            # to avoid blocking the trio loop and ending up in a deadlock
            await trio.run_sync_in_worker_thread(inspect_mountpoint)

        finally:
            await manager.teardown()


@pytest.mark.trio
@pytest.mark.fuse
@pytest.mark.parametrize("fuse_stop_mode", ["manual", "logout"])
async def test_umount_fuse(alice_core, tmpdir, fuse_stop_mode, fuse_mode):
    alice_core.mountpoint_manager.mode = fuse_mode

    mountpoint = f"{tmpdir}/fuse_mountpoint"

    await alice_core.mountpoint_manager.start(mountpoint)
    assert alice_core.mountpoint_manager.is_started()

    if fuse_stop_mode == "manual":
        await alice_core.mountpoint_manager.stop()
        assert not alice_core.mountpoint_manager.is_started()
    else:
        await alice_core.logout()


@pytest.mark.trio
@pytest.mark.fuse
@pytest.mark.skipif(os.name == "nt", reason="Windows doesn't support threaded fuse")
async def test_hard_crash_in_fuse_thread(alice_core, tmpdir):
    alice_core.mountpoint_manager.mode = "thread"

    class ToughLuckError(Exception):
        pass

    def _crash_fuse(*args, **kwargs):
        raise ToughLuckError()

    mountpoint = f"{tmpdir}/fuse_mountpoint"
    with patch("parsec.core.mountpoint.thread.FUSE", new=_crash_fuse):
        with pytest.raises(ToughLuckError):
            await alice_core.mountpoint_manager.start(mountpoint)


@pytest.mark.trio
@pytest.mark.fuse
async def test_hard_crash_in_fuse_process(alice_core, tmpdir):
    alice_core.mountpoint_manager.mode = "process"

    class ToughLuckError(Exception):
        pass

    def _crash_fuse(*args, **kwargs):
        raise ToughLuckError()

    mountpoint = f"{tmpdir}/fuse_mountpoint"
    with patch("parsec.core.mountpoint.process.FUSE", new=_crash_fuse):
        with pytest.raises(RuntimeError):
            await alice_core.mountpoint_manager.start(mountpoint)


@pytest.mark.trio
@pytest.mark.fuse
async def test_mount_missing_path(alice_core, tmpdir, fuse_mode):
    alice_core.mountpoint_manager.mode = fuse_mode
    # Path should be created if it doesn' exist
    mountpoint = f"{tmpdir}/dummy/dummy/dummy"
    await alice_core.mountpoint_manager.start(mountpoint)
