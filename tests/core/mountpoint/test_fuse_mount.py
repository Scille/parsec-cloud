import os
import subprocess

import trio
import pytest
from pathlib import Path
from unittest.mock import patch

from parsec.core.mountpoint import (
    mountpoint_manager_factory,
    NotAvailableMountpointManager,
    FuseMountpointManager,
    MountpointManagerNotAvailable,
)
from parsec.core import logged_core_factory


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
async def test_mount_fuse(alice_fs, event_bus, tmpdir, fuse_mode):
    # Populate a bit the fs first...

    await alice_fs.workspace_create("/w")
    await alice_fs.folder_create("/w/foo")
    await alice_fs.file_create("/w/bar.txt")
    await alice_fs.file_write("/w/bar.txt", b"Hello world !")

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
                root_children = set(os.listdir(mountpoint))
                assert root_children == {"w"}

                wksp_children = set(os.listdir(f"{mountpoint}/w"))
                assert wksp_children == {"foo", "bar.txt"}

                bar_stat = os.stat(f"{mountpoint}/w/bar.txt")
                assert bar_stat.st_size == len(b"Hello world !")

                with open(f"{mountpoint}/w/bar.txt", "rb") as fd:
                    bar_txt = fd.read()
                assert bar_txt == b"Hello world !"

            # Note given python fs api is blocking, we must run it inside a thread
            # to avoid blocking the trio loop and ending up in a deadlock
            await trio.run_sync_in_worker_thread(inspect_mountpoint)

        finally:
            with event_bus.listen() as spy:
                await manager.teardown()
                await trio.sleep(0.01)  # Synchronization issue - fixed in PR #145
            spy.assert_events_occured([("mountpoint.stopped", {"mountpoint": mountpoint})])


@pytest.mark.trio
@pytest.mark.fuse
async def test_umount_fuse(core_config, alice, tmpdir, fuse_mode):
    core_config = core_config.evolve(mountpoint_enabled=True)
    async with logged_core_factory(core_config, alice) as alice_core:
        assert await trio.Path(alice_core.mountpoint).exists()
    assert not await trio.Path(alice_core.mountpoint).exists()


@pytest.mark.trio
@pytest.mark.fuse
async def test_external_umount_fuse(core_config, alice, tmpdir, fuse_mode):
    core_config = core_config.evolve(mountpoint_enabled=True)
    async with logged_core_factory(core_config, alice) as alice_core:
        assert await trio.Path(alice_core.mountpoint).exists()
        # TODO: use trio.Process once updated to trio 0.10.0
        subprocess.run(f"fusermount -u {alice_core.mountpoint}".split())
        await alice_core.mountpoint_manager.join()
    assert not await trio.Path(alice_core.mountpoint).exists()


@pytest.mark.trio
@pytest.mark.fuse
@pytest.mark.skipif(os.name == "nt", reason="Windows doesn't support threaded fuse")
async def test_hard_crash_in_fuse_thread(core_config, alice, tmpdir, fuse_mode):
    core_config = core_config.evolve(mountpoint_enabled=True)

    class ToughLuckError(Exception):
        pass

    def _crash_fuse(*args, **kwargs):
        raise ToughLuckError()

    with patch("parsec.core.mountpoint.thread.FUSE", new=_crash_fuse):
        with pytest.raises(ToughLuckError):
            async with logged_core_factory(core_config, alice):
                pass


@pytest.mark.trio
@pytest.mark.fuse
async def test_mount_missing_path(core_config, alice, tmpdir):
    # Path should be created if it doesn' exist
    base_mountpoint = Path(f"{tmpdir}/dummy/dummy/dummy")
    core_config = core_config.evolve(mountpoint_enabled=True, mountpoint_base_dir=base_mountpoint)
    async with logged_core_factory(core_config, alice) as alice_core:
        assert str(alice_core.mountpoint) == f"{base_mountpoint}/{alice.device_id}"
        assert trio.Path(alice_core.mountpoint).exists()
