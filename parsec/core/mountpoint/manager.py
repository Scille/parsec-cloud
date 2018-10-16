import warnings
import trio
import logging
from pathlib import Path

from parsec.core.mountpoint.exceptions import MountpointAlreadyStarted, MountpointNotStarted

try:
    from fuse import FUSE

    del FUSE

    logging.getLogger("fuse").setLevel(logging.WARNING)

    FUSE_AVAILABLE = True
except (ImportError, OSError) as exc:
    warnings.warn(f"FUSE not available: {exc}")
    FUSE_AVAILABLE = False
else:
    from parsec.core.mountpoint.thread import run_fuse_in_thread
    from parsec.core.mountpoint.process import run_fuse_in_process


class FuseMountpointManager:
    def __init__(self, fs, event_bus, mode="thread", debug: bool = False, nothreads: bool = False):
        if not FUSE_AVAILABLE:
            raise RuntimeError("Fuse is not available, is fusepy installed ?")

        self.event_bus = event_bus
        if mode not in ("thread", "process"):
            raise ValueError("mode param must be `thread` or `process`")
        self.mode = mode
        self.mountpoint = None
        self._lock = trio.Lock()
        self._fs = fs
        self._nursery = None
        self._stop_fuse_runner = None
        self._fuse_config = {"debug": debug, "nothreads": nothreads}

    def get_abs_mountpoint(self):
        return str(self.mountpoint.absolute())

    async def init(self, nursery):
        self._nursery = nursery

    def is_started(self):
        return self._stop_fuse_runner is not None

    async def start(self, mountpoint):
        async with self._lock:
            if self.is_started():
                raise MountpointAlreadyStarted(f"Fuse already started on mountpoint `{mountpoint}`")

            self.mountpoint = Path(mountpoint)
            abs_mountpoint = self.get_abs_mountpoint()
            self.event_bus.send("mountpoint.starting", mountpoint=abs_mountpoint)

            if self.mode == "process":
                fuse_runner = run_fuse_in_process
            else:
                fuse_runner = run_fuse_in_thread
            self._stop_fuse_runner = await self._nursery.start(
                fuse_runner, self._fs, self.mountpoint, self._fuse_config
            )

            self.event_bus.send("mountpoint.started", mountpoint=abs_mountpoint)

    async def stop(self):
        async with self._lock:
            if not self.is_started():
                raise MountpointNotStarted()

            await self._stop_fuse_runner()

            self._stop_fuse_runner = None
            self.event_bus.send("mountpoint.stopped", mountpoint=self.get_abs_mountpoint())

    async def teardown(self):
        try:
            await self.stop()
        except MountpointNotStarted:
            pass
