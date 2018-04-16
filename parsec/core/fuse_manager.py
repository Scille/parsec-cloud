import os
import trio
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


class FuseManager:

    def __init__(self, core_addr: str, debug: bool = False, nothreads: bool = False):
        self._lock = trio.Lock()
        self._start_fuse_config = {
            "socket_address": core_addr, "debug": debug, "nothreads": nothreads
        }
        self.mountpoint = None
        self.fuse_process = None

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
            fuse_process = Process(
                target=start_fuse,
                kwargs={**self._start_fuse_config, "mountpoint": mountpoint}
            )
            fuse_process.start()
            if os.name == "nt":
                if not os.path.isabs(mountpoint):
                    mountpoint = os.path.join(os.getcwd(), mountpoint)
            # TODO: ask Frossigneux...
            # await trio.sleep(1)
            # subprocess.Popen(
            #     "net use p: \\\\localhost\\"
            #     + mountpoint[0]
            #     + "$"
            #     + mountpoint[2:],
            #     shell=True,
            # )

            self.mountpoint = mountpoint
            self.fuse_process = fuse_process

    async def stop_mountpoint(self):
        _die_if_fuse_not_available()
        async with self._lock:
            if not self.is_started():
                raise FuseNotStarted("Fuse is not started")

            # TODO: multiprocess start/stop can make async loop unresponsive,
            # we should use a thread executor do to this
            self.fuse_process.terminate()
            self.fuse_process.join()
            self.mountpoint = self.fuse_process = None

    # TODO: ask Frossigneux...
    # if name == "nt":
    #     subprocess.call("net use p: /delete /y", shell=True)

    def open_file(self, path: str):
        _die_if_fuse_not_available()
        if not self.is_started():
            raise FuseNotStarted("Fuse is not started")

        # TODO: find a better way to open file with system's default application
        webbrowser.open(os.path.join(self.mountpoint, path))
