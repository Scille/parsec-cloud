import os
import subprocess
import tempfile
import trio
import blinker
from multiprocessing import Process
import webbrowser

try:
    from parsec.ui.fuse import start_fuse

    FUSE_AVAILABLE = True
except ImportError:
    FUSE_AVAILABLE = False


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


class FuseManager:

    def __init__(
        self,
        core_addr: str,
        signal_ns: blinker.Namespace,
        debug: bool = False,
        nothreads: bool = False,
    ):
        self._fuse_mountpoint_started = signal_ns.signal("fuse_mountpoint_started")
        self._fuse_mountpoint_need_stop = signal_ns.signal("fuse_mountpoint_need_stop")
        self._fuse_mountpoint_stopped = signal_ns.signal("fuse_mountpoint_stopped")
        self._lock = trio.Lock()
        self._start_fuse_config = {
            "socket_address": core_addr, "debug": debug, "nothreads": nothreads
        }
        self.mountpoint = None
        self.drive_letter = None
        self.fuse_process = None

    async def teardown(self):
        try:
            await self.stop_mountpoint()
        except FuseNotStarted:
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

            if os.name == "nt":
                self.drive_letter = mountpoint
                mountpoint = tempfile.mkdtemp(prefix="parsec-mountpoint-")
                mountpoint = os.path.join(mountpoint, "Parsec")

            if os.name == "posix":
                try:
                    os.makedirs(mountpoint)
                except FileExistsError:
                    pass
            fuse_process = Process(
                target=start_fuse, kwargs={**self._start_fuse_config, "mountpoint": mountpoint}
            )
            fuse_process.start()
            if os.name == "nt":
                await trio.sleep(1)
                subprocess.Popen(
                    "net use "
                    + self.drive_letter
                    + ": \\\\localhost\\"
                    + mountpoint[0]
                    + "$"
                    + mountpoint[2:]
                    + " /persistent:no",
                    shell=True,
                )

            self.mountpoint = mountpoint
            self.fuse_process = fuse_process
            self._fuse_mountpoint_started.send(mountpoint)

    async def stop_mountpoint(self):
        _die_if_fuse_not_available()
        async with self._lock:
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

            await trio.run_sync_in_worker_thread(close_fuse_process)

            # TODO: multiprocess start/stop can make async loop unresponsive,
            # we should use a thread executor do to this
            old_mountpoint = self.mountpoint
            self.mountpoint = self.fuse_process = None

        # TODO: remove mountpoint dir

        if os.name == "nt":
            subprocess.call("net use " + self.drive_letter + ": /delete /y", shell=True)
        self._fuse_mountpoint_stopped.send(old_mountpoint)

    def open_file(self, path: str):
        _die_if_fuse_not_available()
        if not self.is_started():
            raise FuseNotStarted("Fuse is not started")

        # TODO: find a better way to open file with system's default application
        webbrowser.open(os.path.join(self.mountpoint, path))
