# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import trio
import warnings
import logging
from functools import partial

from async_generator import asynccontextmanager

from parsec.utils import start_task
from parsec.core.mountpoint.exceptions import (
    MountpointConfigurationError,
    MountpointAlreadyMounted,
    MountpointNotMounted,
    MountpointDisabled,
)


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


async def disabled_runner(*args, **kwargs):
    raise MountpointDisabled()


class MountpointManager:
    def __init__(self, base_mountpoint, fs, runner, nursery):
        self._base_mountpoint = base_mountpoint
        self._fs = fs
        self._runner = runner
        self._nursery = nursery
        self._mountpoint_tasks = {}

    async def mount_workspace(self, workspace: str):
        if workspace in self._mountpoint_tasks:
            raise MountpointAlreadyMounted(f"Workspace `{workspace}` already mounted.")

        try:
            await self._fs.stat(f"/{workspace}")
        except FileNotFoundError as exc:
            raise MountpointConfigurationError(f"Workspace `{workspace}` doesn't exist") from exc

        mountpoint = self._base_mountpoint / f"{self._fs.device.user_id}-{workspace}"
        runner_task = await start_task(self._nursery, self._runner, workspace, mountpoint)
        self._mountpoint_tasks[workspace] = runner_task

    async def unmount_workspace(self, workspace: str):
        if workspace not in self._mountpoint_tasks:
            raise MountpointNotMounted(f"Workspace `{workspace}` not mounted.")

        await self._mountpoint_tasks[workspace].cancel_and_join()
        del self._mountpoint_tasks[workspace]

    async def mount_all(self):
        stat = await self._fs.stat("/")
        for workspace in stat["children"]:
            try:
                await self.mount_workspace(workspace)
            except MountpointAlreadyMounted:
                pass

    async def unmount_all(self):
        for workspace in list(self._mountpoint_tasks.keys()):
            await self.unmount_workspace(workspace)


@asynccontextmanager
async def mountpoint_manager_factory(
    fs, event_bus, base_mountpoint, *, enabled=True, debug: bool = False, **config
):
    config["debug"] = debug

    if not enabled:
        runner = disabled_runner
    else:
        runner = get_mountpoint_runner()
        if not runner:
            raise RuntimeError("Mountpoint support not available.")
    curried_runner = partial(runner, config=config, fs=fs, event_bus=event_bus)

    async with trio.open_nursery() as nursery:
        mountpoint_manager = MountpointManager(base_mountpoint, fs, curried_runner, nursery)
        try:
            yield mountpoint_manager

        finally:
            await mountpoint_manager.unmount_all()

        nursery.cancel_scope.cancel()
