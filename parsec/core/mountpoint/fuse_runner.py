# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import time
import errno
import signal
import threading
from pathlib import Path
from fuse import FUSE
from structlog import get_logger
from contextlib import contextmanager

from parsec.core.mountpoint.fuse_operations import FuseOperations
from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess
from parsec.core.mountpoint.exceptions import MountpointConfigurationError, MountpointDriverCrash


__all__ = ("fuse_mountpoint_runner",)


logger = get_logger()


@contextmanager
def _reset_signals(signals=None):
    """A context that save the current signal handlers restore them when leaving.

    By default, it does so for SIGINT, SIGTERM, SIGHUP and SIGPIPE.
    """
    if signals is None:
        signals = (signal.SIGINT, signal.SIGTERM, signal.SIGHUP, signal.SIGPIPE)
    saved = {sig: signal.getsignal(sig) for sig in signals}
    try:
        yield
    finally:
        for sig, handler in saved.items():
            try:
                signal.signal(sig, handler)
            except ValueError:
                pass


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
    workspace: str,
    mountpoint: Path,
    config: dict,
    fs,
    event_bus,
    *,
    task_status=trio.TASK_STATUS_IGNORED,
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
    fuse_operations = FuseOperations(workspace, fs_access)

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

                except Exception as exc:
                    try:
                        errcode = errno.errorcode[exc.args[0]]
                    except (KeyError, IndexError):
                        errcode = f"Unknown error code: {exc}"
                    raise MountpointDriverCrash(
                        f"Fuse has crashed on {abs_mountpoint}: {errcode}"
                    ) from exc

                finally:
                    fuse_thread_stopped.set()

            # The fusepy runner (FUSE) relies on the `fuse_main_real` function from libfuse
            # This function is high-level helper on top of the libfuse API that is intended
            # for simple application. As such, it sets some signal handlers to exit cleanly
            # after a SIGTINT, a SIGTERM or a SIGHUP. This is, however, not compatible with
            # our multi-instance multi-threaded application. A simple workaround here is to
            # restore the signals to their previous state once the fuse instance is started.
            with _reset_signals():
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
            time.sleep(0.01)
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
        logger.info("Stopping fuse thread...", mountpoint=mountpoint)
        fuse_operations.schedule_exit()
        await trio.run_sync_in_worker_thread(_wakeup_fuse)
        await trio.run_sync_in_worker_thread(fuse_thread_stopped.wait)
        logger.info("Fuse thread stopped", mountpoint=mountpoint)
