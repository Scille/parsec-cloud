import os
import trio
import time
import threading
import logging
from pathlib import Path

from parsec.core.fuse.operations import FuseOperations

try:
    from fuse import FUSE

    logging.getLogger("fuse").setLevel(logging.WARNING)

    FUSE_AVAILABLE = True
except ImportError:
    FUSE_AVAILABLE = False


class FuseManagerError(Exception):
    pass


class FuseAlreadyStarted(FuseManagerError):
    pass


class FuseNotStarted(FuseManagerError):
    pass


class FuseNotAvailable(FuseManagerError):
    pass


class FuseConfigurationError(FuseManagerError):
    pass


class FuseManager:
    def __init__(self, fs, event_bus, debug: bool = False, nothreads: bool = False):
        if not FUSE_AVAILABLE:
            raise FuseNotAvailable("Fuse is not available")

        self.event_bus = event_bus
        self.mountpoint = None
        self._fs = fs
        self._nursery = None
        self._portal = None
        self._started = False
        self._fuse_config = {"debug": debug, "nothreads": nothreads}
        self._fuse_operations = None
        self._fuse_thread_started = threading.Event()
        self._fuse_thread_stopped = threading.Event()

    def get_abs_mountpoint(self):
        return str(self.mountpoint.absolute())

    async def init(self, nursery):
        self._portal = trio.BlockingTrioPortal()
        self._nursery = nursery

    def is_started(self):
        return self._started

    async def start(self, mountpoint):
        if self.is_started():
            raise FuseAlreadyStarted(f"Fuse already started on mountpoint `{mountpoint}`")

        self.mountpoint = Path(mountpoint)
        abs_mountpoint = self.get_abs_mountpoint()
        self.event_bus.send("fuse.mountpoint.starting", mountpoint=abs_mountpoint)
        self._fuse_operations = FuseOperations(self._fs, self._portal, abs_mountpoint)

        await self._nursery.start(self._run_fuse)
        self._started = True
        self.event_bus.send("fuse.mountpoint.started", mountpoint=abs_mountpoint)

    async def _run_fuse(self, *, task_status=trio.TASK_STATUS_IGNORED):
        if os.name == "posix":
            # On POSIX systems, mounting target must exists
            self.mountpoint.mkdir(exist_ok=True, parents=True)
            initial_st_dev = self.mountpoint.stat().st_dev
        else:
            # On Windows, only parent's mounting target must exists
            self.mountpoint.parent.mkdir(exist_ok=True, parents=True)
            if self.mountpoint.exists():
                raise FuseConfigurationError(
                    f"Mountpoint `{self.get_abs_mountpoint()}` must not exists on windows"
                )
            initial_st_dev = None

        try:
            async with trio.open_nursery() as nursery:
                nursery.start_soon(self._wait_for_fuse_ready, task_status, initial_st_dev)

                def _run_fuse_thread():
                    try:
                        self._fuse_thread_started.set()
                        FUSE(
                            self._fuse_operations,
                            self.get_abs_mountpoint(),
                            foreground=True,
                            **self._fuse_config,
                        )
                    finally:
                        self._fuse_thread_stopped.set()

                await trio.run_sync_in_worker_thread(_run_fuse_thread, cancellable=True)

        finally:
            await self._stop_fuse_thread()

    async def _wait_for_fuse_ready(self, task_status, initial_st_dev):

        # Polling until fuse is ready
        # Note given python fs api is blocking, we must run it inside a thread
        # to avoid blocking the trio loop and ending up in a deadlock

        need_stop = False

        def _wait_for_fuse_ready_thread():
            self._fuse_thread_started.wait()
            while not need_stop:
                time.sleep(0.1)
                try:
                    if self.mountpoint.stat().st_dev != initial_st_dev:
                        break
                except FileNotFoundError:
                    pass

        try:
            await trio.run_sync_in_worker_thread(_wait_for_fuse_ready_thread, cancellable=True)
        finally:
            need_stop = True
        task_status.started()

    async def stop(self):
        if not self.is_started():
            raise FuseNotStarted()

        await self._stop_fuse_thread()

        self._started = False
        self.event_bus.send("fuse.mountpoint.stopped", mountpoint=self.get_abs_mountpoint())

    async def _stop_fuse_thread(self):
        self._fuse_operations.schedule_exit()

        # Ask for dummy file just to force a fuse operation that will
        # process the `fuse_exit` from a valid context
        # Note given python fs api is blocking, we must run it inside a thread
        # to avoid blocking the trio loop and ending up in a deadlock

        def _stop_fuse():
            try:
                (self.mountpoint / "__shutdown_fuse__").exists()
            except OSError:
                pass
            self._fuse_thread_stopped.wait()

        await trio.run_sync_in_worker_thread(_stop_fuse)

        if os.name == "posix":
            try:
                self.mountpoint.rmdir()
            except OSError:
                pass

    async def teardown(self):
        try:
            await self.stop()
        except FuseNotStarted:
            pass
