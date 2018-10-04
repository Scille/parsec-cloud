import os
import click
import trio
import time
import threading
from async_generator import asynccontextmanager

from parsec.core.base import BaseAsyncComponent
from parsec.core.fuse.operations import FuseOperations

try:
    from fuse import FUSE

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
    def __init__(self, fs, event_bus, debug: bool = True, nothreads: bool = False):
        if not FUSE_AVAILABLE:
            raise FuseNotAvailable("Fuse is not available")

        self.event_bus = event_bus
        self.mountpoint = None
        self._fs = fs
        self._fuse_config = {"debug": debug, "nothreads": nothreads}
        self._fuse_thread = None
        self._fuse_operations = None

    def is_started(self):
        return self._fuse_thread is not None

    async def start(self, mountpoint):
        if self.is_started():
            raise FuseAlreadyStarted(f"Fuse already started on mountpoint `{mountpoint}`")

        self.mountpoint = mountpoint
        self.event_bus.send("fuse.mountpoint.starting", mountpoint=mountpoint)

        if os.name == "posix":
            wait_for_fuse_ready = self._wait_for_fuse_ready_posix
        else:
            wait_for_fuse_ready = self._wait_for_fuse_ready_windows
        async with wait_for_fuse_ready(mountpoint):

            portal = trio.BlockingTrioPortal()
            self._fuse_operations = FuseOperations(self._fs, portal, mountpoint)

            def _run_fuse():
                FUSE(self._fuse_operations, mountpoint, foreground=True, **self._fuse_config)

            self._fuse_thread = threading.Thread(target=_run_fuse)

            self._fuse_thread.start()

        self.event_bus.send("fuse.mountpoint.started", mountpoint=mountpoint)

    @asynccontextmanager
    async def _wait_for_fuse_ready_posix(self, mountpoint):
        # On POSIX systems, mounting target must exists
        os.makedirs(mountpoint, exist_ok=True)
        initial_st_dev = os.stat(mountpoint).st_dev

        yield

        # Polling until fuse is ready
        # Note given python fs api is blocking, we must run it inside a thread
        # to avoid blocking the trio loop and ending up in a deadlock

        def _wait_for_fuse_ready():
            while True:
                time.sleep(0.1)
                if os.stat(mountpoint).st_dev != initial_st_dev:
                    break

        await trio.run_sync_in_worker_thread(_wait_for_fuse_ready)

    @asynccontextmanager
    async def _wait_for_fuse_ready_windows(self, mountpoint):
        # On Windows, mounting target should not exists

        yield

        # Polling until fuse is ready
        # Note given python fs api is blocking, we must run it inside a thread
        # to avoid blocking the trio loop and ending up in a deadlock

        def _wait_for_fuse_ready():
            while True:
                time.sleep(0.1)
                try:
                    os.stat(mountpoint)
                    break
                except FileNotFoundError:
                    pass

        await trio.run_sync_in_worker_thread(_wait_for_fuse_ready)

    async def stop(self):
        if not self.is_started():
            raise FuseNotStarted()

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
            self._fuse_thread.join()
            self._fuse_thread = None

        await trio.run_sync_in_worker_thread(_stop_fuse)

        if os.name == "posix":
            try:
                os.rmdir(self.mountpoint)
            except OSError:
                pass
        self.event_bus.send("fuse.mountpoint.stopped", mountpoint=self.mountpoint)

    async def teardown(self):
        try:
            await self.stop()
        except FuseNotStarted:
            pass
