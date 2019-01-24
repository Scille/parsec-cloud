import os
import trio
import time
import threading
from pathlib import Path
from fuse import FUSE
from structlog import get_logger

from parsec.core.mountpoint.operations import FuseOperations
from parsec.core.mountpoint.exceptions import MountpointConfigurationError


logger = get_logger()


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


def _teardown_mountpoint(mountpoint):
    if os.name == "posix":
        try:
            mountpoint.rmdir()
        except OSError:
            pass


async def run_fuse_in_thread(
    fs, mountpoint: Path, fuse_config: dict, event_bus, *, task_status=trio.TASK_STATUS_IGNORED
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
        event_bus.send("mountpoint.starting", mountpoint=abs_mountpoint)

        async with trio.open_nursery() as nursery:

            def _run_fuse_thread():
                try:
                    fuse_thread_started.set()
                    FUSE(fuse_operations, abs_mountpoint, foreground=True, **fuse_config)
                finally:
                    fuse_thread_stopped.set()

            nursery.start_soon(
                lambda: trio.run_sync_in_worker_thread(_run_fuse_thread, cancellable=True)
            )

            await _wait_for_fuse_ready(mountpoint, fuse_thread_started, initial_st_dev)

            event_bus.send("mountpoint.started", mountpoint=abs_mountpoint)
            task_status.started(abs_mountpoint)

    finally:
        await _stop_fuse_runner()
        event_bus.send("mountpoint.stopped", mountpoint=abs_mountpoint)
        _teardown_mountpoint(mountpoint)


async def _wait_for_fuse_ready(mountpoint, fuse_thread_started, initial_st_dev):

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


async def _stop_fuse_thread(mountpoint, fuse_operations, fuse_thread_stopped):
    if fuse_thread_stopped.is_set():
        return

    # Ask for dummy file just to force a fuse operation that will
    # process the `fuse_exit` from a valid context
    # Note given python fs api is blocking, we must run it inside a thread
    # to avoid blocking the trio loop and ending up in a deadlock

    def _wakeup_fuse():
        try:
            (mountpoint / "__shutdown_fuse__").exists()
        except OSError:
            pass

    with trio.open_cancel_scope(shield=True):
        logger.info("Stopping fuse thread...")
        fuse_operations.schedule_exit()
        await trio.run_sync_in_worker_thread(_wakeup_fuse)
        await trio.run_sync_in_worker_thread(fuse_thread_stopped.wait)
        logger.info("Fuse thread stopped")


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

    def file_fd_read(self, fh, size, offset):
        return self._portal.run(self.fs.file_fd_read, fh, size, offset)

    def file_fd_write(self, fh, data, offset):
        return self._portal.run(self.fs.file_fd_write, fh, data, offset)

    def file_fd_truncate(self, fh, length):
        return self._portal.run(self.fs.file_fd_truncate, fh, length)

    def file_fd_flush(self, fh):
        return self._portal.run(self.fs.file_fd_flush, fh)
