# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import trio
import warnings
import logging
from functools import partial
from pathlib import PurePath
from pendulum import Pendulum

from async_generator import asynccontextmanager

from parsec.utils import start_task
from parsec.core.types import FsPath, EntryID
from parsec.core.fs.exceptions import FSWorkspaceNotFoundError, FSWorkspaceTimestampedTooEarly
from parsec.core.mountpoint.exceptions import (
    MountpointConfigurationError,
    MountpointConfigurationWorkspaceFSTimestampedError,
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
        self._timestamped_workspacefs = {}

    def _get_workspace(self, workspace_id: EntryID):
        try:
            return self.user_fs.get_workspace(workspace_id)
        except FSWorkspaceNotFoundError as exc:
            raise MountpointConfigurationError(f"Workspace `{workspace_id}` doesn't exist") from exc

    def _get_workspace_timestamped(self, workspace_id: EntryID, timestamp: Pendulum):
        try:
            return self._timestamped_workspacefs[(workspace_id, timestamp)]
        except KeyError:
            try:
                self.user_fs.get_workspace(workspace_id)
                raise MountpointNotMounted(
                    f"Workspace `{workspace_id}` not mounted at timestamped `{timestamp}`"
                )
            except FSWorkspaceNotFoundError as exc:
                raise MountpointConfigurationError(
                    f"Workspace `{workspace_id}` doesn't exist"
                ) from exc

    async def _mount_workspace_helper(self, workspace_fs, timestamp: Pendulum = None):
        curried_runner = partial(
            self._runner,
            workspace_fs,
            self.base_mountpoint_path,
            config=self.config,
            event_bus=self.event_bus,
        )
        runner_task = await start_task(self._nursery, curried_runner)
        self._mountpoint_tasks[(workspace_fs.workspace_id, timestamp)] = runner_task
        return runner_task

    def get_path_in_mountpoint(
        self, workspace_id: EntryID, path: FsPath, timestamp: Pendulum = None
    ) -> PurePath:
        if timestamp is None:
            self._get_workspace(workspace_id)
        else:
            self._get_workspace_timestamped(workspace_id, timestamp)
        try:
            runner_task = self._mountpoint_tasks[(workspace_id, timestamp)]
            return runner_task.value / path.relative_to(path.root)

        except KeyError:
            raise MountpointNotMounted(f"Workspace `{workspace_id}` is not mounted")

    async def mount_workspace(self, workspace_id: EntryID, timestamp: Pendulum = None) -> PurePath:
        if timestamp is None:
            workspace = self._get_workspace(workspace_id)
        else:
            try:
                workspace = self._get_workspace_timestamped(workspace_id, timestamp)
            except MountpointNotMounted:
                return await self.remount_workspace_new_timestamp(workspace_id, None, timestamp)
        if (workspace_id, timestamp) in self._mountpoint_tasks:
            raise MountpointAlreadyMounted(f"Workspace `{workspace_id}` already mounted.")

        runner_task = await self._mount_workspace_helper(workspace)
        return runner_task.value

    async def unmount_workspace(self, workspace_id: EntryID, timestamp: Pendulum = None):
        if (workspace_id, timestamp) not in self._mountpoint_tasks:
            raise MountpointNotMounted(f"Workspace `{workspace_id}` not mounted.")

        await self._mountpoint_tasks[(workspace_id, timestamp)].cancel_and_join()
        del self._mountpoint_tasks[(workspace_id, timestamp)]

    async def remount_workspace_new_timestamp(
        self, workspace_id: EntryID, original_timestamp: Pendulum, target_timestamp: Pendulum
    ) -> PurePath:
        """
        Mount the workspace at target_timestamp, and unmount the workspace at the original
        timestamp if it is not None. If both timestamps are equals, do nothing.
        """
        # TODO : use different workspaces for temp mount
        if original_timestamp == target_timestamp:
            try:
                return self._mountpoint_tasks[(workspace_id, target_timestamp)].value
            except KeyError:
                pass
            try:
                self.user_fs.get_workspace(workspace_id)
                raise MountpointNotMounted(
                    f"Workspace `{workspace_id}` not mounted at timestamped `{target_timestamp}`"
                )
            except FSWorkspaceNotFoundError as exc:
                raise MountpointConfigurationError(
                    f"Workspace `{workspace_id}` doesn't exist"
                ) from exc
        if original_timestamp is None:
            workspace = self._get_workspace(workspace_id)
        else:
            try:
                workspace = self._get_workspace_timestamped(workspace_id, original_timestamp)
            except MountpointNotMounted:
                workspace = self._get_workspace(workspace_id)
        try:
            new_workspace = await workspace.to_timestamped(target_timestamp)
        except FSWorkspaceTimestampedTooEarly as exc:
            raise MountpointConfigurationWorkspaceFSTimestampedError(
                f"Workspace `{workspace_id}` didn't exist at `{target_timestamp}`",
                workspace_id,
                target_timestamp,
                workspace.get_workspace_name(),
            ) from exc
        self._timestamped_workspacefs[(workspace_id, target_timestamp)] = new_workspace

        runner_task = await self._mount_workspace_helper(new_workspace, target_timestamp)
        if original_timestamp is not None:
            if (workspace_id, original_timestamp) not in self._mountpoint_tasks:
                raise MountpointNotMounted(f"Workspace `{workspace_id}` not mounted.")

            await self._mountpoint_tasks[(workspace_id, original_timestamp)].cancel_and_join()
            del self._mountpoint_tasks[(workspace_id, original_timestamp)]
        return runner_task.value

    async def mount_all(self, timestamp: Pendulum = None):
        user_manifest = self.user_fs.get_user_manifest()
        for workspace_entry in user_manifest.workspaces:
            if workspace_entry.role is None:
                continue
            try:
                await self.mount_workspace(workspace_entry.id, timestamp=timestamp)
            except MountpointAlreadyMounted:
                pass

    async def unmount_all(self):
        for workspace_id_ts_combination in list(self._mountpoint_tasks.keys()):
            await self.unmount_workspace(*workspace_id_ts_combination)

    async def get_timestamped_mounted(self):
        return {
            workspace_id_and_ts: self._get_workspace_timestamped(*workspace_id_and_ts)
            for workspace_id_and_ts in self._mountpoint_tasks.keys()
            if workspace_id_and_ts[1] is not None
        }


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
