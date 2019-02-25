# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import time
import threading
from pathlib import Path
from fuse import FUSE
from structlog import get_logger

from parsec.core.mountpoint.fuse_operations import FuseOperations
from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess
from parsec.core.mountpoint.exceptions import MountpointConfigurationError


__all__ = ("fuse_mountpoint_runner",)


logger = get_logger()


def _bootstrap_mountpoint(mountpoint):
    try:
        # On POSIX systems, mounting target must exists
        mountpoint.mkdir(exist_ok=True, parents=True)
        initial_st_dev = mountpoint.stat().st_dev

    except OSError as exc:
        # In case of hard crash, it's possible the FUSE mountpoint is still
        # mounted (but point to nothing). In such case access to the mountpoint
        # end up with an error :(
        raise MountpointConfigurationError(
            "Mountpoint is busy, has parsec prevously cleanly exited ?"
        ) from exc

    return initial_st_dev


def _teardown_mountpoint(mountpoint):
    try:
        mountpoint.rmdir()
    except OSError:
        pass


async def fuse_mountpoint_runner(
    fs, mountpoint: Path, config: dict, event_bus, *, task_status=trio.TASK_STATUS_IGNORED
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
    fuse_operations = FuseOperations(fs_access)

    initial_st_dev = _bootstrap_mountpoint(mountpoint)

    try:
        event_bus.send("mountpoint.starting", mountpoint=abs_mountpoint)

        async with trio.open_nursery() as nursery:

            def _run_fuse_thread():
                try:
                    fuse_thread_started.set()
                    FUSE(
                        fuse_operations,
                        abs_mountpoint,
                        foreground=True,
                        auto_unmount=True,
                        **config,
                    )
                finally:
                    fuse_thread_stopped.set()

            nursery.start_soon(
                lambda: trio.run_sync_in_worker_thread(_run_fuse_thread, cancellable=True)
            )

            await _wait_for_fuse_ready(mountpoint, fuse_thread_started, initial_st_dev)

            event_bus.send("mountpoint.started", mountpoint=abs_mountpoint)
            task_status.started(abs_mountpoint)

    finally:
        await _stop_fuse_thread(mountpoint, fuse_operations, fuse_thread_stopped)
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

    with trio.CancelScope(shield=True):
        logger.info("Stopping fuse thread...")
        fuse_operations.schedule_exit()
        await trio.run_sync_in_worker_thread(_wakeup_fuse)
        await trio.run_sync_in_worker_thread(fuse_thread_stopped.wait)
        logger.info("Fuse thread stopped")
