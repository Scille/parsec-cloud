# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import errno
import importlib.resources
import os
import signal
import sys
import threading
from contextlib import asynccontextmanager
from itertools import count
from pathlib import Path
from types import FrameType
from typing import AsyncGenerator, Tuple, cast

import trio
from fuse import FUSE
from structlog import get_logger

from parsec._parsec import DateTime, EntryID
from parsec.core import resources as resources_module
from parsec.core.core_events import CoreEvent
from parsec.core.fs.userfs import UserFS
from parsec.core.fs.workspacefs import WorkspaceFS
from parsec.core.mountpoint.exceptions import MountpointDriverCrash
from parsec.core.mountpoint.fuse_operations import FuseOperations
from parsec.core.mountpoint.thread_fs_access import ThreadFSAccess
from parsec.event_bus import EventBus

__all__ = ("fuse_mountpoint_runner",)


logger = get_logger()


def _sig_ign(sig: int, stack: FrameType | None) -> None:
    """A signal handler behaving like signal.SIG_IGN"""


def _sig_dfl(sig: int, stack: FrameType | None) -> None:
    """A signal handler behaving like signal.SIG_DFL"""
    # Restore the DLF handler
    signal.signal(sig, signal.SIG_DFL)
    # Raise either SIGHUP or SIGTERM, which terminates the process
    signal.raise_signal(sig)


def _patch_signals() -> None:
    """Prevent libfuse2 to mess with the default signal handlers in python.

    The fusepy runner (FUSE) relies on the `fuse_main_real` function from libfuse2.
    This function is a high-level helper on top of the libfuse API that is intended
    for simple application. As such, it sets some signal handlers to exit cleanly
    after a SIGINT, a SIGTERM or a SIGHUP. It also switches the SIGPIPE signal to
    IGN. These handlers are meant to be restored at the end of the fuse session.

    This is, however, not compatible with our multi-instance multi-threaded application.
    In order to prevent libfuse to do anything with those 4 signals, we're setting custom
    handlers to let libfuse know that we're already handling those. Since python already
    has a custom SIGINT handler, only the other 3 signals should be patched.

    This function is idempotent.

    Note: python won't let us patched the signals outside of the main thread.
    In practice, this should never be the case. One counter example are the tests
    using the `mountpoint_service` fixture, which starts by calling `_patch_signals`
    directly before delegating to subthreads.
    """
    if sys.platform != "win32":
        # The default value for SIGPIPE in python is IGN
        if signal.getsignal(signal.SIGPIPE) == signal.SIG_IGN:
            signal.signal(signal.SIGPIPE, _sig_ign)
        # The default value for SIGHUP in python is DFL
        if signal.getsignal(signal.SIGHUP) == signal.SIG_DFL:
            signal.signal(signal.SIGHUP, _sig_dfl)
        # The default value for SIGTERM in python is DFL
        if signal.getsignal(signal.SIGTERM) == signal.SIG_DFL:
            signal.signal(signal.SIGTERM, _sig_dfl)
    else:
        raise RuntimeError("Attempted to patch unix signals on windows")


# Mypy reports a missing return statement, however it's not possible to exit this
# function without raising an exception or successfully return
async def _bootstrap_mountpoint(base_mountpoint_path: Path, workspace_fs: WorkspaceFS) -> Tuple[Path, int]:  # type: ignore[return]
    # Find a suitable path where to mount the workspace. The check we are doing
    # here are not atomic (and the mount operation is not itself atomic anyway),
    # hence there is still edgecases where the mount can crash due to concurrent
    # changes on the mountpoint path
    workspace_name = workspace_fs.get_workspace_name()
    for tentative in count(1):
        if tentative == 1:
            dirname = workspace_name.str
        else:
            dirname = f"{workspace_name.str} ({tentative})"
        mountpoint_path = base_mountpoint_path / dirname

        try:
            # On POSIX systems, mounting target must exists
            trio_mountpoint_path = trio.Path(mountpoint_path)
            await trio_mountpoint_path.mkdir(mode=0o700, exist_ok=True, parents=True)
            base_st_dev = (await trio.Path(base_mountpoint_path).stat()).st_dev
            initial_st_dev = (await trio_mountpoint_path.stat()).st_dev
            if initial_st_dev != base_st_dev:
                # mountpoint_path seems to already have a mountpoint on it,
                # hence find another place to setup our own mountpoint
                continue
            if list(await trio_mountpoint_path.iterdir()):
                # mountpoint_path not empty, cannot mount there
                continue
            return mountpoint_path, initial_st_dev

        except OSError:
            # In case of hard crash, it's possible the FUSE mountpoint is still
            # mounted (but points to nothing). In such case just mount in
            # another place
            continue


async def _teardown_mountpoint(mountpoint_path: Path) -> None:
    try:
        await trio.Path(mountpoint_path).rmdir()
    except OSError:
        pass


@asynccontextmanager
async def fuse_mountpoint_runner(
    user_fs: UserFS,
    workspace_fs: WorkspaceFS,
    base_mountpoint_path: Path,
    config: dict[object, object],
    event_bus: EventBus,
) -> AsyncGenerator[Path, None]:
    """
    Raises:
        MountpointDriverCrash
    """
    fuse_thread_started = threading.Event()
    fuse_thread_stopped = threading.Event()
    trio_token = trio.lowlevel.current_trio_token()
    fs_access = ThreadFSAccess(trio_token, workspace_fs, event_bus)

    mountpoint_path, initial_st_dev = await _bootstrap_mountpoint(
        base_mountpoint_path, workspace_fs
    )

    # Prepare event information
    event_kwargs: dict[str, Path | EntryID | DateTime | None] = {
        "mountpoint": mountpoint_path,
        "workspace_id": workspace_fs.workspace_id,
        "timestamp": cast(DateTime, getattr(workspace_fs, "timestamp", None)),
    }

    # Prevent mypy arg-type check to fail because of dict unpacking (e.g **Dict[...] incompatible with ...)
    fuse_operations = FuseOperations(fs_access, **event_kwargs)  # type: ignore[arg-type]

    try:
        teardown_cancel_scope = None
        event_bus.send(CoreEvent.MOUNTPOINT_STARTING, **event_kwargs)

        # `open_service_nursery` does not exists in trio according to mypy
        async with trio.open_service_nursery() as nursery:  # type: ignore[attr-defined]

            # Let fusepy decode the paths using the current file system encoding
            # Note that this does not prevent the user from using a certain encoding
            # in the context of the parsec app and another encoding in the context of
            # an application accessing the mountpoint. In this case, an encoding error
            # might be raised while fusepy tries to decode the path. If that happens,
            # fusepy will log the error and simply return EINVAL, which is acceptable.
            encoding = sys.getfilesystemencoding()

            def _run_fuse_thread() -> None:
                parsec_icns_path = importlib.resources.files(resources_module).joinpath(
                    "parsec.icns"
                )
                fuse_platform_options = {}
                if sys.platform == "darwin":
                    fuse_platform_options = {
                        "local": True,
                        "defer_permissions": True,
                        "volname": workspace_fs.get_workspace_name(),
                        "volicon": str(parsec_icns_path.absolute()),
                    }
                    # osxfuse-specific options :
                    # local: allows mountpoint to show up correctly in finder (+ desktop)
                    # volname: specify volume name (default is OSXFUSE [...])
                    # volicon: specify volume icon (default is macOS drive icon)

                # On defer_permissions: "The defer_permissions option [...] causes macFUSE to assume that all
                # accesses are allowed; it will forward all operations to the file system, and it is up to
                # somebody else to eventually allow or deny the operations." See
                # https://github.com/osxfuse/osxfuse/wiki/Mount-options#default_permissions-and-defer_permissions
                # This option is necessary on MacOS to give the right permissions to files inside FUSE drives,
                # otherwise it impedes upon saving and auto-saving from Apple software (TextEdit, Preview...)
                # due to the gid of files seemingly not having writing rights from the software perspective.
                # One other solution found for this issue was to change the gid of the mountpoint and its files
                # from staff (default) to wheel (admin gid). Checking defer_permissions allows to ignore the gid
                # issue entirely and lets Parsec itself handle read/write rights inside workspaces.

                else:
                    fuse_platform_options = {"auto_unmount": True}

                logger.info("Starting fuse thread...", mountpoint=mountpoint_path)
                try:
                    # Do not let fuse start if the runner is stopping
                    # It's important that `fuse_thread_started` is set before the check
                    # in order to avoid race conditions
                    fuse_thread_started.set()
                    if teardown_cancel_scope is not None:
                        return
                    FUSE(
                        fuse_operations,
                        # this method exists in concrete instance (Posix or Windows path)
                        str(mountpoint_path.resolve(strict=False)),
                        foreground=True,
                        encoding=encoding,
                        **fuse_platform_options,
                        **config,
                    )

                except Exception as exc:
                    try:
                        errcode = errno.errorcode[exc.args[0]]
                    except (KeyError, IndexError):
                        errcode = f"Unknown error code: {exc}"
                    raise MountpointDriverCrash(
                        f"Fuse has crashed on {mountpoint_path}: {errcode}"
                    ) from exc

                finally:
                    fuse_thread_stopped.set()

            # We're about to call the `fuse_main_real` function from libfuse, so let's make sure
            # the signals are correctly patched before that (`_path_signals` is idempotent)
            _patch_signals()

            nursery.start_soon(lambda: trio.to_thread.run_sync(_run_fuse_thread, cancellable=True))
            await _wait_for_fuse_ready(mountpoint_path, fuse_thread_started, initial_st_dev)

            # Indicate the mountpoint is now started
            yield mountpoint_path

    finally:
        event_bus.send(CoreEvent.MOUNTPOINT_STOPPING, **event_kwargs)
        with trio.CancelScope(shield=True) as teardown_cancel_scope:
            await _stop_fuse_thread(
                mountpoint_path, fuse_operations, fuse_thread_started, fuse_thread_stopped
            )
            await _teardown_mountpoint(mountpoint_path)


async def _wait_for_fuse_ready(
    mountpoint_path: Path, fuse_thread_started: threading.Event, initial_st_dev: int
) -> None:
    trio_mountpoint_path = trio.Path(mountpoint_path)
    # Polling until fuse is ready
    while True:
        # Tick for 10 ms
        await trio.sleep(0.01)
        # Stat the root of the workspace
        try:
            stat = await trio_mountpoint_path.stat()
        # The directory does not exist for some reason
        except FileNotFoundError:
            raise
        # Looks like a revoked workspace has been mounted
        except PermissionError:
            break
        # Might be another OSError like errno 113 (No route to host)
        except OSError:
            break
        # The device field differs, the mountpoint is up and running
        if stat.st_dev != initial_st_dev:
            break


async def _stop_fuse_thread(
    mountpoint_path: Path,
    fuse_operations: FuseOperations,
    fuse_thread_started: threading.Event,
    fuse_thread_stopped: threading.Event,
) -> None:
    # Nothing to do if the fuse thread is not running
    if fuse_thread_stopped.is_set() or not fuse_thread_started.is_set():
        return
    logger.info("Stopping fuse thread...", mountpoint=mountpoint_path)

    # Fuse exit in macOS
    if sys.platform == "darwin":

        # The schedule_exit() solution doesn't work on macOS, instead freezes the application for
        # 120 seconds before a timeout occurs. The solution used is to call this function (macOS
        # equivalent to fusermount) in a subprocess to unmount.
        process_args = ["diskutil", "unmount", "force", str(mountpoint_path)]
        process = await trio.lowlevel.open_process(process_args)

        # Perform 300 attempts of 10 ms, i.e a 3 second timeout
        # A while loop wouldn't be ideal here, especially since this code is protected against cancellation
        for _ in range(300):
            # Check if the attempt succeeded for 10 ms
            if await trio.to_thread.run_sync(fuse_thread_stopped.wait, 0.01):
                break
            # Restart the unmount process if necessary, this is needed in
            # case the stop is ordered before fuse has finished started (hence
            # the unmount command can run before the OS mount has occurred).
            # Of course this means in theory we could be umounting by mistake
            # an unrelated mountpoint that took our path, but it's a really
            # unlikely event.
            if process.poll() is not None:
                process = await trio.lowlevel.open_process(process_args)
        else:
            logger.error("Fuse thread stop timeout", mountpoint=mountpoint_path)

        # Wait for unmount process to complete if necessary
        await process.wait()

    # Fuse exit in linux
    else:

        # Schedule an exit in the fuse operations
        fuse_operations.schedule_exit()

        # Ping fuse by performing an access from a daemon thread
        def _ping_fuse_thread_target() -> None:
            try:
                os.stat(mountpoint_path / ".__shutdown_fuse__")
            except OSError:
                pass

        # All kind of stuff can go wrong here if the access is performed while the fuse FS is already shutting down,
        # from exotic exceptions to blocking forever. For this reason, we don't use `await trio.Path(...).exists()`
        # but simply fire and forget a ping to the fuse FS. Either this ping succeeds in waking the FS up and thus
        # starting its shutdown procedure, or it fails because the FS is already shutting down.
        thread = threading.Thread(target=_ping_fuse_thread_target, daemon=True)
        thread.start()

        # Perform 100 attempts of 10 ms, i.e a 1 second timeout
        # A while loop wouldn't be ideal here, especially since this code is protected against cancellation
        for _ in range(100):
            # Check if the attempt succeeded for 10 ms
            if await trio.to_thread.run_sync(fuse_thread_stopped.wait, 0.01):
                break
            # Restart the daemon thread if necessary
            if not thread.is_alive():
                thread = threading.Thread(target=_ping_fuse_thread_target, daemon=True)
                thread.start()
        else:
            logger.error("Fuse thread stop timeout", mountpoint=mountpoint_path)

    # The thread has now stopped
    logger.info("Fuse thread stopped", mountpoint=mountpoint_path)
