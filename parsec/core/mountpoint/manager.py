import os
import warnings
import logging
from pathlib import Path

from async_generator import asynccontextmanager

from parsec.types import DeviceID
from parsec.utils import start_task

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

    task = await start_task(nursery, fuse_runner, fs, mountpoint, fuse_config, event_bus)
    try:
        yield task
    finally:
        await task.cancel_and_join()


mountpoint_manager = fuse_mountpoint_manager
