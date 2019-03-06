# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import trio
import pytest
from unittest.mock import patch

from parsec.core.mountpoint import mountpoint_manager_factory, MountpointDriverCrash


@pytest.mark.linux  # win32 doesn't allow to remove an opened file
@pytest.mark.mountpoint
def test_delete_then_close_file(mountpoint_service):
    async def _bootstrap(fs, mountpoint_manager):
        await fs.file_create(f"/w/with_fsync.txt")
        await fs.file_create(f"/w/without_fsync.txt")

    mountpoint_service.start()
    mountpoint_service.execute(_bootstrap)

    w_path = mountpoint_service.get_default_workspace_mountpoint()

    path = w_path / "with_fsync.txt"
    fd = os.open(path, os.O_RDWR)
    os.unlink(path)
    os.fsync(fd)
    os.close(fd)

    path = w_path / "without_fsync.txt"
    fd = os.open(path, os.O_RDWR)
    os.unlink(path)
    os.close(fd)


@pytest.mark.linux
@pytest.mark.trio
@pytest.mark.mountpoint
async def test_unmount_with_fusermount(base_mountpoint, alice, alice_fs, event_bus):
    mountpoint = f"{base_mountpoint.absolute()}/{alice.user_id}-w"
    await alice_fs.workspace_create("/w")
    await alice_fs.file_create("/w/bar.txt")

    bar_txt = trio.Path(f"{mountpoint}/bar.txt")

    async with mountpoint_manager_factory(
        alice_fs, event_bus, base_mountpoint
    ) as mountpoint_manager:

        with event_bus.listen() as spy:
            await mountpoint_manager.mount_workspace("w")
            proc = trio.Process(f"fusermount -u {mountpoint}".split())
            await proc.wait()

            with trio.fail_after(1):
                await spy.wait("mountpoint.stopped", kwargs={"mountpoint": mountpoint})
        assert not await bar_txt.exists()


@pytest.mark.linux
@pytest.mark.trio
@pytest.mark.mountpoint
async def test_hard_crash_in_fuse_thread(base_mountpoint, alice_fs, event_bus):
    await alice_fs.workspace_create("/w")

    class ToughLuckError(Exception):
        pass

    def _crash_fuse(*args, **kwargs):
        raise ToughLuckError()

    with patch("parsec.core.mountpoint.fuse_runner.FUSE", new=_crash_fuse):
        async with mountpoint_manager_factory(
            alice_fs, event_bus, base_mountpoint
        ) as mountpoint_manager:

            with pytest.raises(MountpointDriverCrash):
                await mountpoint_manager.mount_workspace("w")
