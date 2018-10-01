import os
import click
import trio
import time
import multiprocessing

from parsec.core.base import BaseAsyncComponent

try:
    from parsec.ui.fuse import start_fuse

    FUSE_AVAILABLE = True
except ImportError:
    FUSE_AVAILABLE = False


multiprocessing.freeze_support()


def _die_if_fuse_not_available():
    if not FUSE_AVAILABLE:
        raise FuseNotAvailable("Fuse is not available")


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


class FuseManager(BaseAsyncComponent):
    def __init__(self, core_addr: str, event_bus, debug: bool = False, nothreads: bool = False):
        super().__init__()
        self.event_bus = event_bus
        # TODO: make fuse process send events to synchronise with the manager
        self._start_fuse_config = {
            "socket_address": core_addr,
            "debug": debug,
            "nothreads": nothreads,
        }
        self.mountpoint = None
        self.drive_letter = None
        self.fuse_process = None

    async def _init(self, nursery):
        pass

    async def _teardown(self):
        try:
            await self._stop_mountpoint_no_lock()
        except (FuseNotStarted, FuseNotAvailable):
            pass

    def is_started(self):
        return self.fuse_process is not None

    async def start_mountpoint(self, mountpoint: str):
        _die_if_fuse_not_available()
        async with self._lock:
            if self.is_started():
                raise FuseAlreadyStarted(
                    "Fuse already started on mountpoint `%s`" % self.mountpoint
                )

            if os.name == "posix":
                try:
                    os.makedirs(mountpoint)
                except FileExistsError:
                    pass
            fuse_process = multiprocessing.Process(
                target=start_fuse, kwargs={**self._start_fuse_config, "mountpoint": mountpoint}
            )

            def start_fuse_process():
                # Given st_dev is a unique id of a drive, it must change
                # once the folder is mounted
                try:
                    original_st_dev = os.stat(mountpoint).st_dev
                except FileNotFoundError:
                    original_st_dev = None

                fuse_process.start()

                while True:
                    try:
                        new_st_dev = os.stat(mountpoint).st_dev
                    except FileNotFoundError:
                        new_st_dev = None
                    if new_st_dev != original_st_dev:
                        return
                    time.sleep(0.01)

            await trio.run_sync_in_worker_thread(start_fuse_process)

            self.mountpoint = mountpoint
            self.fuse_process = fuse_process
            self.event_bus.send("fuse.mountpoint.started", mountpoint=mountpoint)

    async def stop_mountpoint(self):
        _die_if_fuse_not_available()
        async with self._lock:
            await self._stop_mountpoint_no_lock()

    async def _stop_mountpoint_no_lock(self):
        if not self.is_started():
            raise FuseNotStarted("Fuse is not started")

        # Fuse process should be listening to this event
        self.event_bus.send("fuse.mountpoint.need_stop", mountpoint=self.mountpoint)

        def close_fuse_process():
            # Once the need stop event received, fuse should close itself,
            # so we just have to wait for this...
            self.fuse_process.join(1)
            if self.fuse_process.exitcode is None:
                raise FuseStoppingError(
                    "Fuse process (pid: %s) refuse to stop" % self.fuse_process.pid
                )

            elif os.name == "posix":
                os.rmdir(self.mountpoint)

        # Wait the process's end in a separate thread to avoid blocking the event loop
        await trio.run_sync_in_worker_thread(close_fuse_process)

        old_mountpoint = self.mountpoint
        self.mountpoint = self.fuse_process = None

        self.event_bus.send("fuse.mountpoint.stopped", mountpoint=old_mountpoint)

    def open_file(self, path: str):
        _die_if_fuse_not_available()
        if not self.is_started():
            raise FuseNotStarted("Fuse is not started")

        if os.name == "nt":
            path = os.path.join(self.mountpoint + "/", path[1:])
        else:
            path = os.path.join(self.mountpoint, path[1:])
        # Run a separate process, interlocking problem on Windows
        click_process = multiprocessing.Process(target=click.launch, args=(path,))
        click_process.daemon = True
        click_process.start()
