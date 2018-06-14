import os
import click
import trio
import multiprocessing

from parsec.signals import get_signal
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
    def __init__(self, core_addr: str, debug: bool = False, nothreads: bool = False):
        super().__init__()
        # TODO: make fuse process send events to synchronise with the manager
        self._fuse_mountpoint_started = get_signal("fuse_mountpoint_started")
        self._fuse_mountpoint_need_stop = get_signal("fuse_mountpoint_need_stop")
        self._fuse_mountpoint_stopped = get_signal("fuse_mountpoint_stopped")
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
            fuse_process.start()

            # TODO: waiting event fuse_started
            await trio.sleep(1)

            self.mountpoint = mountpoint
            self.fuse_process = fuse_process
            self._fuse_mountpoint_started.send(mountpoint)

    async def stop_mountpoint(self):
        _die_if_fuse_not_available()
        async with self._lock:
            await self._stop_mountpoint_no_lock()

    async def _stop_mountpoint_no_lock(self):
        if not self.is_started():
            raise FuseNotStarted("Fuse is not started")

        # Fuse process should be listening to this event
        self._fuse_mountpoint_need_stop.send(self.mountpoint)

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

        await trio.run_sync_in_worker_thread(close_fuse_process)

        # TODO: multiprocess start/stop can make async loop unresponsive,
        # we should use a thread executor do to this
        old_mountpoint = self.mountpoint
        self.mountpoint = self.fuse_process = None

        self._fuse_mountpoint_stopped.send(old_mountpoint)

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
