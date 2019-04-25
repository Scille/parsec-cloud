# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import trio
import warnings
import logging
from functools import partial
from pathlib import PurePath

from async_generator import asynccontextmanager

from parsec.utils import start_task
from parsec.core.types import FsPath, AccessID
from parsec.core.fs.exceptions import FSWorkspaceNotFoundError
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
    def __init__(self, base_mountpoint: PurePath, userfs, runner, nursery):
        self._base_mountpoint = base_mountpoint
        self._userfs = userfs
        self._runner = runner
        self._nursery = nursery
        self._mountpoint_tasks = {}

    def _get_workspace(self, workspace_id: AccessID):
        try:
            return self._userfs.get_workspace(workspace_id)
        except FSWorkspaceNotFoundError as exc:
            raise MountpointConfigurationError(f"Workspace `{workspace_id}` doesn't exist") from exc

    def get_path_in_mountpoint(self, workspace_id: AccessID, path: FsPath) -> PurePath:
        workspace = self._get_workspace(workspace_id)
        if workspace_id not in self._mountpoint_tasks:
            raise MountpointNotMounted(
                f"Workspace `{workspace_id}` ({workspace.workspace_name}) is not mounted"
            )
        return self._get_mountpoint_path(workspace) / path.relative_to(path.root)

    def _get_mountpoint_path(self, workspace) -> PurePath:
        return self._base_mountpoint / f"{self._userfs.device.user_id}-{workspace.workspace_name}"

    async def mount_workspace(self, workspace_id: AccessID):
        workspace = self._get_workspace(workspace_id)
        if workspace_id in self._mountpoint_tasks:
            raise MountpointAlreadyMounted(
                f"Workspace `{workspace_id}` ({workspace.workspace_name}) already mounted."
            )

        mountpoint = self._get_mountpoint_path(workspace)
        runner_task = await start_task(self._nursery, self._runner, workspace, mountpoint)
        self._mountpoint_tasks[workspace_id] = runner_task

    async def unmount_workspace(self, workspace_id: AccessID):
        if workspace_id not in self._mountpoint_tasks:
            raise MountpointNotMounted(f"Workspace `{workspace_id}` not mounted.")

        await self._mountpoint_tasks[workspace_id].cancel_and_join()
        del self._mountpoint_tasks[workspace_id]

    async def mount_all(self):
        user_manifest = self._userfs.get_user_manifest()
        for workspace_entry in user_manifest.workspaces:
            try:
                await self.mount_workspace(workspace_entry.access.id)
            except MountpointAlreadyMounted:
                pass

    async def unmount_all(self):
        for workspace_id in list(self._mountpoint_tasks.keys()):
            await self.unmount_workspace(workspace_id)


@asynccontextmanager
async def mountpoint_manager_factory(
    user_fs, event_bus, base_mountpoint, *, enabled=True, debug: bool = False, **config
):
    config["debug"] = debug

    if not enabled:
        runner = disabled_runner
    else:
        runner = get_mountpoint_runner()
        if not runner:
            raise RuntimeError("Mountpoint support not available.")
    curried_runner = partial(runner, config=config, event_bus=event_bus)

    async with trio.open_nursery() as nursery:
        mountpoint_manager = MountpointManager(base_mountpoint, user_fs, curried_runner, nursery)
        try:
            yield mountpoint_manager

        finally:
            await mountpoint_manager.unmount_all()

        nursery.cancel_scope.cancel()
