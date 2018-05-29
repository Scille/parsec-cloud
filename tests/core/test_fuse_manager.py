import pytest
from unittest.mock import patch
import os
import attr
import trio
import blinker

from parsec.core.fuse_manager import (
    FuseManager,
    FuseNotAvailable,
    FuseAlreadyStarted,
    FuseNotStarted,
    FUSE_AVAILABLE,
)


@pytest.fixture
def signal_ns():
    return blinker.Namespace()


@pytest.mark.trio
async def test_fuse_not_available(signal_ns):
    with patch("parsec.core.fuse_manager.FUSE_AVAILABLE", new=False):
        fm = FuseManager("tcp://dummy.com:9999", signal_ns)

        with pytest.raises(FuseNotAvailable):
            await fm.start_mountpoint("/foo/bar")

        with pytest.raises(FuseNotAvailable):
            await fm.stop_mountpoint()

        with pytest.raises(FuseNotAvailable):
            fm.open_file("/foo/bar")


@pytest.mark.trio
@pytest.mark.slow
@pytest.mark.skipif(not FUSE_AVAILABLE, reason="libfuse/fusepy not installed")
@pytest.mark.parametrize("fuse_stop_mode", ["manual", "teardown"])
async def test_mount_fuse(core, alice, tmpdir, fuse_stop_mode):
    mountpoint = "%s/fuse_mountpoint" % tmpdir

    # Must start real server to serve core given fuse will be in a separate process
    async with trio.open_nursery() as nursery:
        listeners = await nursery.start(trio.serve_tcp, core.handle_client, 0)
        core.config = attr.evolve(
            core.config, addr="tcp://localhost:%s" % listeners[0].socket.getsockname()[1]
        )

        # Login into the core and populate it a bit
        await core.login(alice)
        await core.fs.folder_create("/foo")
        await core.fs.file_create("/bar.txt")
        await core.fs.file_write("/bar.txt", b"Hello world !")

        # Now we can start fuse
        await core.fuse_manager.start_mountpoint(mountpoint)

        # TODO: find a better way to wait for fuse init...
        await trio.sleep(1)

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

        # Finally stop fuse manually if needed. Otherwise it should be done
        # by `FuseManager.teardown` during `core.logout`
        if fuse_stop_mode == "manual":
            await core.fuse_manager.stop_mountpoint()

        # Don't forget to explicitly cancel the nursery to close the server !
        nursery.cancel_scope.cancel()
