# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import warnings
import logging
from pathlib import Path

from async_generator import asynccontextmanager

from parsec.types import DeviceID
from parsec.utils import start_task


def get_mountpoint_runner():
    if os.name == "nt":
        try:
            import winfspy  # noqa: test import is working

            logging.getLogger("winfspy").setLevel(logging.WARNING)

        except (ImportError, OSError) as exc:
            warnings.warn(f"WinFSP not available: {exc}")
            return None

        else:
            from parsec.core.mountpoint.winfsp_runner import winfsp_mountpoint_runner

            return winfsp_mountpoint_runner

    else:
        try:
            from fuse import FUSE  # noqa: test import is working

            logging.getLogger("fuse").setLevel(logging.WARNING)

        except (ImportError, OSError) as exc:
            warnings.warn(f"FUSE not available: {exc}")
            return None

        else:
            from parsec.core.mountpoint.fuse_runner import fuse_mountpoint_runner

            return fuse_mountpoint_runner


def get_default_mountpoint(device_id: DeviceID):
    if os.name == "nt":
        return Path("Z:")
    else:
        return Path(f"/media/{device_id}")


@asynccontextmanager
async def mountpoint_manager(fs, event_bus, mountpoint, nursery, *, debug: bool = False, **config):
    config["debug"] = debug

    # No mountpoint
    if mountpoint is None:
        yield None
        return

    mountpoint_runner = get_mountpoint_runner()
    if not mountpoint_runner:
        raise RuntimeError("No mountpoint runner available.")

    task = await start_task(nursery, mountpoint_runner, fs, mountpoint, config, event_bus)
    try:
        yield task
    finally:
        await task.cancel_and_join()
