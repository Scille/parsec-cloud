import os
import trio
import time
import threading
import logging

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


class FuseStoppingError(FuseManagerError):
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

    async def init(self, nursery):
        self._portal = trio.BlockingTrioPortal()
        self._nursery = nursery

    def is_started(self):
        return self._started

    async def start(self, mountpoint):
        if self.is_started():
            raise FuseAlreadyStarted(f"Fuse already started on mountpoint `{mountpoint}`")

        self.mountpoint = mountpoint
        self.event_bus.send("fuse.mountpoint.starting", mountpoint=mountpoint)
        self._fuse_operations = FuseOperations(self._fs, self._portal, mountpoint)

        await self._nursery.start(self._run_fuse)
        self._started = True
        self.event_bus.send("fuse.mountpoint.started", mountpoint=self.mountpoint)

    async def _run_fuse(self, *, task_status=trio.TASK_STATUS_IGNORED):
        if os.name == "posix":
            # On POSIX systems, mounting target must exists
            os.makedirs(self.mountpoint, exist_ok=True)
            initial_st_dev = os.stat(self.mountpoint).st_dev
        else:
            initial_st_dev = None

        try:
            async with trio.open_nursery() as nursery:
                nursery.start_soon(self._wait_for_fuse_ready, task_status, initial_st_dev)

                def _run_fuse_thread():
                    try:
                        self._fuse_thread_started.set()
                        FUSE(
                            self._fuse_operations,
                            self.mountpoint,
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
                if os.stat(self.mountpoint).st_dev != initial_st_dev:
                    break

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
        self.event_bus.send("fuse.mountpoint.stopped", mountpoint=self.mountpoint)

    async def _stop_fuse_thread(self):
        self._fuse_operations.schedule_exit()

        # Ask for dummy file just to force a fuse operation that will
        # process the `fuse_exit` from a valid context
        # Note given python fs api is blocking, we must run it inside a thread
        # to avoid blocking the trio loop and ending up in a deadlock

        def _stop_fuse():
            try:
                os.path.exists(f"{self.mountpoint}/__shutdown_fuse__")
            except OSError:
                pass
            self._fuse_thread_stopped.wait()

        await trio.run_sync_in_worker_thread(_stop_fuse)

        if os.name == "posix":
            try:
                os.rmdir(self.mountpoint)
            except OSError:
                pass

    async def teardown(self):
        try:
            await self.stop()
        except FuseNotStarted:
            pass
