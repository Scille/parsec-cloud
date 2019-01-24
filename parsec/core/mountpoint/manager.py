import os
import warnings
import logging
from pathlib import Path

import trio
import attr
from async_generator import asynccontextmanager

from parsec.types import DeviceID

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


def get_default_mountpoint(device_id: DeviceID):
    if os.name == "nt":
        raise NotImplementedError()
    else:
        return Path(f"/media/{device_id}")


@attr.s
class TaskStatus:

    cancel = attr.ib()
    join = attr.ib()
    value = attr.ib()

    async def cancel_and_join(self):
        self.cancel()
        await self.join()


async def stoppable(corofn, *args, task_status=trio.TASK_STATUS_IGNORED):
    finished = trio.Event()
    try:
        async with trio.open_nursery() as nursery:
            value = await nursery.start(corofn, *args)
            status = TaskStatus(cancel=nursery.cancel_scope.cancel, join=finished.wait, value=value)
            task_status.started(status)
    finally:
        finished.set()


@asynccontextmanager
async def fuse_mountpoint_manager(
    fs,
    event_bus,
    mountpoint,
    nursery,
    *,
    mode="thread",
    debug: bool = False,
    nothreads: bool = False,
):
    # No mountpoint
    if mountpoint is None:
        yield None
        return

    # Fuse not available
    if not FUSE_AVAILABLE:
        raise RuntimeError("Fuse is not available, is fusepy installed ?")

    # Invalid mode
    if mode not in ("thread", "process"):
        raise ValueError("mode param must be `thread` or `process`")

    fuse_config = {"debug": debug, "nothreads": nothreads}
    if mode == "process":
        fuse_runner = run_fuse_in_process
    else:
        fuse_runner = run_fuse_in_thread

    task = await nursery.start(stoppable, fuse_runner, fs, mountpoint, fuse_config, event_bus)
    try:
        yield task
    finally:
        await task.cancel_and_join()


mountpoint_manager = fuse_mountpoint_manager
