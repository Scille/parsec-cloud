import os
import trio
import time
import threading
from pathlib import Path
from fuse import FUSE

from parsec.core.mountpoint.operations import FuseOperations
from parsec.core.mountpoint.exceptions import MountpointConfigurationError


def _bootstrap_mountpoint(mountpoint):
    try:
        if os.name == "posix":
            # On POSIX systems, mounting target must exists
            mountpoint.mkdir(exist_ok=True, parents=True)
            initial_st_dev = mountpoint.stat().st_dev
        else:
            # On Windows, only parent's mounting target must exists
            mountpoint.parent.mkdir(exist_ok=True, parents=True)
            if mountpoint.exists():
                raise MountpointConfigurationError(
                    f"Mountpoint `{mountpoint.absolute()}` must not exists on windows"
                )
            initial_st_dev = None

    except OSError as exc:
        # In case of hard crash, it's possible the FUSE mountpoint is still
        # mounted (but point to nothing). In such case access to the mountpoint
        # end up with an error :(
        raise MountpointConfigurationError(
            "Mountpoint is busy, has parsec prevously cleanly exited ?"
        ) from exc

    return initial_st_dev


async def run_fuse_in_thread(
    fs, mountpoint: Path, fuse_config: dict, *, task_status=trio.TASK_STATUS_IGNORED
):
    """
    Raises:
        MountpointConfigurationError
    """
    fuse_thread_started = threading.Event()
    fuse_thread_stopped = threading.Event()
    portal = trio.BlockingTrioPortal()
    abs_mountpoint = str(mountpoint.absolute())
    fs_access = ThreadFSAccess(portal, fs)
    fuse_operations = FuseOperations(fs_access, abs_mountpoint)

    async def _stop_fuse_runner():
        await _stop_fuse_thread(mountpoint, fuse_operations, fuse_thread_stopped)

    initial_st_dev = _bootstrap_mountpoint(mountpoint)

    try:
        async with trio.open_nursery() as nursery:

            nursery.start_soon(
                _wait_for_fuse_ready,
                mountpoint,
                fuse_thread_started,
                initial_st_dev,
                lambda: task_status.started(_stop_fuse_runner),
            )

            def _run_fuse_thread():
                try:
                    fuse_thread_started.set()
                    FUSE(fuse_operations, abs_mountpoint, foreground=True, **fuse_config)
                finally:
                    fuse_thread_stopped.set()

            await trio.run_sync_in_worker_thread(_run_fuse_thread, cancellable=True)

    finally:
        await _stop_fuse_runner()


async def _wait_for_fuse_ready(mountpoint, fuse_thread_started, initial_st_dev, started_cb):

    # Polling until fuse is ready
    # Note given python fs api is blocking, we must run it inside a thread
    # to avoid blocking the trio loop and ending up in a deadlock

    need_stop = False

    def _wait_for_fuse_ready_thread():
        fuse_thread_started.wait()
        while not need_stop:
            time.sleep(0.1)
            try:
                if mountpoint.stat().st_dev != initial_st_dev:
                    break
            except FileNotFoundError:
                pass

    try:
        await trio.run_sync_in_worker_thread(_wait_for_fuse_ready_thread, cancellable=True)
    finally:
        need_stop = True
    started_cb()


async def _stop_fuse_thread(mountpoint, fuse_operations, fuse_thread_stopped):
    fuse_operations.schedule_exit()

    # Ask for dummy file just to force a fuse operation that will
    # process the `fuse_exit` from a valid context
    # Note given python fs api is blocking, we must run it inside a thread
    # to avoid blocking the trio loop and ending up in a deadlock

    def _stop_fuse():
        try:
            (mountpoint / "__shutdown_fuse__").exists()
        except OSError:
            pass
        fuse_thread_stopped.wait()

    await trio.run_sync_in_worker_thread(_stop_fuse)

    if os.name == "posix":
        try:
            mountpoint.rmdir()
        except OSError:
            pass


class ThreadFSAccess:
    def __init__(self, portal, fs):
        self.fs = fs
        self._portal = portal

    def stat(self, path):
        return self._portal.run(self.fs.stat, path)

    def delete(self, path):
        return self._portal.run(self.fs.delete, path)

    def move(self, src, dst):
        return self._portal.run(self.fs.move, src, dst)

    def file_create(self, path):
        async def _do(path):
            await self.fs.file_create(path)
            return await self.fs.file_fd_open(path)

        return self._portal.run(_do, path)

    def folder_create(self, path):
        return self._portal.run(self.fs.folder_create, path)

    def file_truncate(self, path, length):
        return self._portal.run(self.fs.file_truncate, path, length)

    def file_fd_open(self, path):
        return self._portal.run(self.fs.file_fd_open, path)

    def file_fd_close(self, fh):
        return self._portal.run(self.fs.file_fd_close, fh)

    def file_fd_seek(self, fh, offset):
        return self._portal.run(self.fs.file_fd_seek, fh, offset)

    def file_fd_read(self, fh, size):
        return self._portal.run(self.fs.file_fd_read, fh, size)

    def file_fd_seek_and_read(self, fh, size, offset):
        async def _do(fh, size, offset):
            await self.fs.file_fd_seek(fh, offset)
            return await self.fs.file_fd_read(fh, size)

        return self._portal.run(_do, fh, size, offset)

    def file_fd_seek_and_write(self, fh, data, offset):
        async def _do(fh, data, offset):
            await self.fs.file_fd_seek(fh, offset)
            return await self.fs.file_fd_write(fh, data)

        return self._portal.run(_do, fh, data, offset)

    def file_fd_write(self, fh, data):
        return self._portal.run(self.fs.file_fd_write, fh, data)

    def file_fd_truncate(self, fh, length):
        return self._portal.run(self.fs.file_fd_truncate, fh, length)

    def file_fd_flush(self, fh):
        return self._portal.run(self.fs.file_fd_flush, fh)
