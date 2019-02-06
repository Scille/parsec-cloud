import os
import subprocess

import trio
import pytest
from pathlib import Path
from unittest.mock import patch

from parsec.core.mountpoint import mountpoint_manager
from parsec.core import logged_core_factory


@pytest.mark.trio
async def test_fuse_not_available(alice_fs, event_bus):
    mountpoint = Path("/foo")

    with patch("parsec.core.mountpoint.manager.FUSE_AVAILABLE", new=False):
        async with trio.open_nursery() as nursery:
            with pytest.raises(RuntimeError):
                async with mountpoint_manager(alice_fs, event_bus, mountpoint, nursery):
                    pass


@pytest.mark.trio
@pytest.mark.fuse
async def test_mount_fuse(alice_fs, event_bus, tmpdir, fuse_mode):
    # Populate a bit the fs first...

    await alice_fs.workspace_create("/w")
    await alice_fs.folder_create("/w/foo")
    await alice_fs.file_create("/w/bar.txt")
    await alice_fs.file_write("/w/bar.txt", b"Hello world !")

    # Now we can start fuse

    mountpoint = Path(f"{tmpdir}/fuse_mountpoint")
    async with trio.open_nursery() as nursery:
        with event_bus.listen() as spy:
            async with mountpoint_manager(
                alice_fs, event_bus, mountpoint, nursery, mode=fuse_mode
            ) as task:
                abs_mountpoint = str(mountpoint.absolute())
                assert task.value == abs_mountpoint

                spy.assert_events_occured(
                    [
                        ("mountpoint.starting", {"mountpoint": abs_mountpoint}),
                        ("mountpoint.started", {"mountpoint": abs_mountpoint}),
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

            # Mountpoint should be stopped by now
            spy.assert_events_occured([("mountpoint.stopped", {"mountpoint": abs_mountpoint})])


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
        await alice_core.mountpoint_task.join()
    assert not await trio.Path(alice_core.mountpoint).exists()


@pytest.mark.trio
@pytest.mark.fuse
async def test_cancellable_join_umount_fuse(core_config, alice, tmpdir, fuse_mode):
    core_config = core_config.evolve(mountpoint_enabled=True)
    async with logged_core_factory(core_config, alice) as alice_core:
        assert await trio.Path(alice_core.mountpoint).exists()
        with trio.move_on_after(0.01):
            await alice_core.mountpoint_task.join()
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
