# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import trio
import warnings
import logging
from functools import partial
from pathlib import PurePath

from async_generator import asynccontextmanager

from parsec.utils import start_task
from parsec.core.types import FsPath, EntryID
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
    def __init__(self, user_fs, event_bus, base_mountpoint_path, config, runner, nursery):
        self.user_fs = user_fs
        self.event_bus = event_bus
        self.base_mountpoint_path = base_mountpoint_path
        self.config = config
        self._runner = runner
        self._nursery = nursery
        self._mountpoint_tasks = {}

    def _get_workspace(self, workspace_id: EntryID):
        try:
            return self.user_fs.get_workspace(workspace_id)
        except FSWorkspaceNotFoundError as exc:
            raise MountpointConfigurationError(f"Workspace `{workspace_id}` doesn't exist") from exc

    def get_path_in_mountpoint(self, workspace_id: EntryID, path: FsPath) -> PurePath:
        self._get_workspace(workspace_id)
        try:
            runner_task = self._mountpoint_tasks[workspace_id]
            return runner_task.value / path.relative_to(path.root)

        except KeyError:
            raise MountpointNotMounted(f"Workspace `{workspace_id}` is not mounted")

    async def mount_workspace(self, workspace_id: EntryID) -> PurePath:
        workspace = self._get_workspace(workspace_id)
        if workspace_id in self._mountpoint_tasks:
            raise MountpointAlreadyMounted(f"Workspace `{workspace_id}` already mounted.")

        curried_runner = partial(
            self._runner,
            workspace,
            self.base_mountpoint_path,
            config=self.config,
            event_bus=self.event_bus,
        )
        runner_task = await start_task(self._nursery, curried_runner)
        self._mountpoint_tasks[workspace_id] = runner_task
        return runner_task.value

    async def unmount_workspace(self, workspace_id: EntryID):
        if workspace_id not in self._mountpoint_tasks:
            raise MountpointNotMounted(f"Workspace `{workspace_id}` not mounted.")

        await self._mountpoint_tasks[workspace_id].cancel_and_join()
        del self._mountpoint_tasks[workspace_id]

    async def mount_all(self):
        user_manifest = self.user_fs.get_user_manifest()
        for workspace_entry in user_manifest.workspaces:
            try:
                await self.mount_workspace(workspace_entry.id)
            except MountpointAlreadyMounted:
                pass

    async def unmount_all(self):
        for workspace_id in list(self._mountpoint_tasks.keys()):
            await self.unmount_workspace(workspace_id)


@asynccontextmanager
async def mountpoint_manager_factory(
    user_fs, event_bus, base_mountpoint_path, *, enabled=True, debug: bool = False, **config
):
    config["debug"] = debug

    if not enabled:
        runner = disabled_runner
    else:
        runner = get_mountpoint_runner()
        if not runner:
            raise RuntimeError("Mountpoint support not available.")

    async with trio.open_nursery() as nursery:
        mountpoint_manager = MountpointManager(
            user_fs, event_bus, base_mountpoint_path, config, runner, nursery
        )
        try:
            yield mountpoint_manager

        finally:
            await mountpoint_manager.unmount_all()

        nursery.cancel_scope.cancel()
