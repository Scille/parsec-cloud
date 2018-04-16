import pytest
from unittest.mock import patch
import os
import trio

from parsec.core.fuse_manager import (
    FuseManager, FuseNotAvailable, FuseAlreadyStarted, FuseNotStarted, FUSE_AVAILABLE
)


@pytest.mark.trio
async def test_fuse_not_available():
    with patch("parsec.core.fuse_manager.FUSE_AVAILABLE", new=False):
        fm = FuseManager("tcp://dummy.com:9999")

        with pytest.raises(FuseNotAvailable):
            await fm.start_mountpoint("/foo/bar")

        with pytest.raises(FuseNotAvailable):
            await fm.stop_mountpoint()

        with pytest.raises(FuseNotAvailable):
            fm.open_file("/foo/bar")


@pytest.mark.trio
@pytest.mark.slow
@pytest.mark.skipif(not FUSE_AVAILABLE, reason="libfuse/fusepy not installed")
async def test_mount_fuse(core, alice, tmpdir):
    mountpoint = "%s/fuse_mountpoint" % tmpdir

    # Login into the core and populate it a bit
    await core.login(alice)
    await core.fs.root.create_folder("/foo")
    bar = await core.fs.root.create_file("/bar.txt")
    await bar.write(b"Hello world !")

    # Must start real server to serve core given fuse will be in a separate process
    async with trio.open_nursery() as nursery:
        listeners = await nursery.start(trio.serve_tcp, core.handle_client, 0)
        core_addr = "tcp://localhost:%s" % listeners[0].socket.getsockname()[1]

        # Now we can start fuse
        fm = FuseManager(core_addr, debug=True)
        await fm.start_mountpoint(mountpoint)
        try:

            # TODO: find a better way to wait for fuse init...
            await trio.sleep(1)

            # Finally explore the mountpoint

            def inspect_mountpoint():
                children = set(os.listdir(mountpoint))
                assert children == {"foo", "bar"}

                with open("%s/bar.txt" % mountpoint, "rb") as fd:
                    bar_txt = fd.read()
                assert bar_txt == b"Hello world !"

            # Note given python fs api is blocking, we must run it inside a thread
            # to avoid blocking the trio loop and ending up in a deadlock
            await trio.run_sync_in_worker_thread(inspect_mountpoint)

        finally:
            # TODO: great chances that we endup here due to fuse deadlock...
            await fm.stop_mountpoint()
