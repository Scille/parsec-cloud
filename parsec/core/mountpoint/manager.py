# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import logging
import re
import sys
from contextlib import asynccontextmanager
from importlib import __import__ as import_function
from pathlib import Path, PurePath
from subprocess import CalledProcessError
from typing import Any, AsyncContextManager, AsyncGenerator, Callable, Sequence, Union, cast

import trio
from structlog import get_logger

from parsec._parsec import CoreEvent, DateTime, RealmArchivingConfiguration
from parsec.core.fs import FsPath, UserFS, WorkspaceFS, WorkspaceFSTimestamped
from parsec.core.fs.exceptions import FSWorkspaceNotFoundError, FSWorkspaceTimestampedTooEarly
from parsec.core.logged_core import WorkspaceEntry
from parsec.core.mountpoint.exceptions import (
    MountpointAlreadyMounted,
    MountpointConfigurationError,
    MountpointConfigurationWorkspaceFSTimestampedError,
    MountpointError,
    MountpointFuseNotAvailable,
    MountpointNotMounted,
    MountpointWinfspNotAvailable,
)
from parsec.core.mountpoint.winify import winify_entry_name
from parsec.core.types import EntryID
from parsec.core.win_registry import cleanup_parsec_drive_icons
from parsec.event_bus import EventBus
from parsec.utils import TaskStatus, open_service_nursery, start_task

RunnerType = Callable[
    [UserFS, WorkspaceFS, Path, dict[Any, Any], EventBus],
    AsyncContextManager[Union[Path, PurePath]],
]

# Importing winfspy can take some time (about 0.4 seconds)
# Let's import those bindings at module level, in order to
# avoid spending too much time importing them later while the
# trio loop is running.
try:
    if sys.platform == "win32":
        import_function("winfspy")
    else:
        import_function("fuse")
except (ImportError, RuntimeError, OSError):
    pass

logger = get_logger()


def get_mountpoint_runner() -> RunnerType:
    # Windows
    if sys.platform == "win32":
        try:
            # Use import function for easier mock up
            import_function("winfspy")
        except RuntimeError as exc:
            raise MountpointWinfspNotAvailable(exc) from exc

        logging.getLogger("winfspy").setLevel(logging.WARNING)
        from parsec.core.mountpoint.winfsp_runner import winfsp_mountpoint_runner

        return winfsp_mountpoint_runner

    # Linux
    else:
        try:
            # Use import function for easier mock up
            import_function("fuse")
        except (ImportError, OSError) as exc:
            raise MountpointFuseNotAvailable(exc) from exc

        logging.getLogger("fuse").setLevel(logging.WARNING)
        from parsec.core.mountpoint.fuse_runner import fuse_mountpoint_runner

        return fuse_mountpoint_runner


class MountpointManager:
    def __init__(
        self,
        user_fs: UserFS,
        event_bus: EventBus,
        base_mountpoint_path: Path,
        config: dict[Any, Any],
        runner: RunnerType,
        nursery: trio.Nursery,
        personal_workspace_base_path: Path | None = None,
        personal_workspace_name_pattern: str | None = None,
    ) -> None:
        self.user_fs = user_fs
        self.event_bus = event_bus
        self.base_mountpoint_path = base_mountpoint_path
        self.config = config
        self._runner = runner
        self._nursery = nursery
        self._mountpoint_tasks: dict[tuple[EntryID, DateTime | None], TaskStatus[Any]] = {}
        self._mounting_events: dict[tuple[EntryID, DateTime | None], trio.Event] = {}
        self._timestamped_workspacefs: dict[EntryID, dict[DateTime, WorkspaceFSTimestamped]] = {}
        self.personal_workspace_base_path = personal_workspace_base_path
        self.personal_workspace_name_regex = (
            re.compile(personal_workspace_name_pattern) if personal_workspace_name_pattern else None
        )

        if sys.platform == "win32":
            self._build_mountpoint_path = lambda base_path, parts: base_path / "\\".join(
                winify_entry_name(x) for x in parts
            )
        else:
            self._build_mountpoint_path = lambda base_path, parts: base_path / "/".join(
                [part.str for part in parts]
            )

    def _get_base_mountpoint_path(self, workspace_fs: WorkspaceFS) -> Path:
        if (
            self.personal_workspace_base_path
            and self.personal_workspace_name_regex
            and self.personal_workspace_name_regex.fullmatch(workspace_fs.get_workspace_name().str)
        ):
            # Personal workspace, should be mounted on a different path
            return self.personal_workspace_base_path
        else:
            # Regular workspace, mount on regular base mountpoint path
            return self.base_mountpoint_path

    def _get_workspace(self, workspace_id: EntryID) -> WorkspaceFS:
        try:
            return self.user_fs.get_workspace(workspace_id)
        except FSWorkspaceNotFoundError as exc:
            raise MountpointConfigurationError(
                f"Workspace `{workspace_id.hex}` doesn't exist"
            ) from exc

    def _get_workspace_timestamped(
        self, workspace_id: EntryID, timestamp: DateTime
    ) -> WorkspaceFSTimestamped:
        try:
            return self._timestamped_workspacefs[workspace_id][timestamp]
        except KeyError:
            try:
                self.user_fs.get_workspace(workspace_id)
                raise MountpointNotMounted(
                    f"Workspace `{workspace_id.hex}` not mounted at timestamped `{timestamp}`"
                )
            except FSWorkspaceNotFoundError as exc:
                raise MountpointConfigurationError(
                    f"Workspace `{workspace_id.hex}` doesn't exist"
                ) from exc

    async def _load_workspace_timestamped(
        self, workspace_id: EntryID, timestamp: DateTime
    ) -> WorkspaceFSTimestamped:
        try:
            return self._timestamped_workspacefs[workspace_id][timestamp]
        except KeyError:
            pass
        try:
            # Get a random WorkspaceFSTimestamped if possible, as all WorkspaceFSTimestamped from
            # the same WorkspaceFS will share the same cache when implemented
            source_workspace: WorkspaceFS = next(
                v for v in self._timestamped_workspacefs[workspace_id].values()
            )
        except (StopIteration, KeyError):  # No WorkspaceFSTimestamped found for this workspace_id
            source_workspace = self._get_workspace(workspace_id)
        try:
            new_workspace = await source_workspace.to_timestamped(timestamp)
        except FSWorkspaceTimestampedTooEarly as exc:
            raise MountpointConfigurationWorkspaceFSTimestampedError(
                f"Workspace `{workspace_id.hex}` didn't exist at `{timestamp}`",
                workspace_id,
                timestamp,
                source_workspace.get_workspace_name(),
            ) from exc
        try:
            self._timestamped_workspacefs[workspace_id][timestamp] = new_workspace
        except KeyError:
            self._timestamped_workspacefs[workspace_id] = {timestamp: new_workspace}
        return new_workspace

    async def _mount_workspace_helper(
        self, workspace_fs: WorkspaceFS, timestamp: DateTime | None = None
    ) -> TaskStatus[Path]:
        workspace_id = workspace_fs.workspace_id
        key = workspace_id, timestamp

        async def curried_runner(task_status: TaskStatus[PurePath]) -> None:
            event_kwargs = {}

            try:
                async with self._runner(
                    self.user_fs,
                    workspace_fs,
                    self._get_base_mountpoint_path(workspace_fs),
                    self.config,
                    self.event_bus,
                ) as mountpoint_path:
                    # Another runner started before us
                    if key in self._mountpoint_tasks:
                        raise MountpointAlreadyMounted(
                            f"Workspace `{workspace_id.hex}` already mounted."
                        )

                    # Prepare kwargs for both started and stopped events
                    event_kwargs = {
                        "mountpoint": mountpoint_path,
                        "workspace_id": workspace_fs.workspace_id,
                        "timestamp": timestamp,
                    }

                    # Set the mountpoint as mounted THEN send the corresponding event
                    task_status.started(mountpoint_path)
                    self._mountpoint_tasks[key] = task_status
                    self._mounting_events.pop(key, trio.Event()).set()
                    self.event_bus.send(CoreEvent.MOUNTPOINT_STARTED, **event_kwargs)

                    # It is the reponsability of the runner context teardown to wait
                    # for cancellation. This is done to avoid adding an extra nursery
                    # into the winfsp runner, for simplicity. This could change in the
                    # future in which case we'll simply add a `sleep_forever` below.

            finally:
                # Make sure to notify we are done mounting
                self._mounting_events.pop(key, trio.Event()).set()
                # Pop the mountpoint task if its ours
                if self._mountpoint_tasks.get(key) == task_status:
                    del self._mountpoint_tasks[key]
                # Send stopped event if started has been previously sent
                if event_kwargs:
                    self.event_bus.send(CoreEvent.MOUNTPOINT_STOPPED, **event_kwargs)

        # Start the mountpoint runner task
        runner_task = cast(TaskStatus[Path], await start_task(self._nursery, curried_runner))
        return runner_task

    def get_path_in_mountpoint(
        self, workspace_id: EntryID, path: FsPath, timestamp: DateTime | None = None
    ) -> Path:
        if timestamp is None:
            self._get_workspace(workspace_id)
        else:
            self._get_workspace_timestamped(workspace_id, timestamp)
        try:
            runner_task = self._mountpoint_tasks[(workspace_id, timestamp)]
            return self._build_mountpoint_path(runner_task.value, path.parts)

        except KeyError:
            raise MountpointNotMounted(f"Workspace `{workspace_id.hex}` is not mounted")

    async def mount_workspace(
        self, workspace_id: EntryID, timestamp: DateTime | None = None
    ) -> Path:
        if (workspace_id, timestamp) in self._mountpoint_tasks:
            raise MountpointAlreadyMounted(f"Workspace `{workspace_id.hex}` already mounted.")

        if (workspace_id, timestamp) in self._mounting_events:
            raise MountpointAlreadyMounted(f"Workspace `{workspace_id.hex}` is being mounted.")

        # Set a mounting event to prevent race conditions
        mounting = trio.Event()
        self._mounting_events[(workspace_id, timestamp)] = mounting

        if timestamp is not None:
            return await self.remount_workspace_new_timestamp(workspace_id, None, timestamp)

        workspace = self._get_workspace(workspace_id)
        runner_task = await self._mount_workspace_helper(workspace)
        assert runner_task.value is not None
        return runner_task.value

    async def unmount_workspace(
        self, workspace_id: EntryID, timestamp: DateTime | None = None
    ) -> None:
        # Wait for the workspace to stop being mounted
        while (workspace_id, timestamp) in self._mounting_events:
            await self._mounting_events[(workspace_id, timestamp)].wait()

        if (workspace_id, timestamp) not in self._mountpoint_tasks:
            raise MountpointNotMounted(f"Workspace `{workspace_id.hex}` not mounted.")

        await self._mountpoint_tasks[(workspace_id, timestamp)].cancel_and_join()

    async def remount_workspace_new_timestamp(
        self,
        workspace_id: EntryID,
        original_timestamp: DateTime | None,
        target_timestamp: DateTime,
    ) -> Path:
        """
        Mount the workspace at target_timestamp, and unmount the workspace at the original
        timestamp if it is not None. If both timestamps are equals, do nothing.
        """
        # TODO : use different workspaces for temp mount
        if original_timestamp == target_timestamp:
            try:
                task_result = self._mountpoint_tasks[(workspace_id, target_timestamp)].value
                assert task_result is not None
                return task_result
            except KeyError:
                pass
        new_workspace = await self._load_workspace_timestamped(workspace_id, target_timestamp)

        runner_task = await self._mount_workspace_helper(new_workspace, target_timestamp)
        if original_timestamp is not None:
            if (workspace_id, original_timestamp) not in self._mountpoint_tasks:
                raise MountpointNotMounted(f"Workspace `{workspace_id.hex}` not mounted.")

            await self._mountpoint_tasks[(workspace_id, original_timestamp)].cancel_and_join()
        assert runner_task.value is not None
        return runner_task.value

    async def get_timestamped_mounted(
        self,
    ) -> dict[tuple[EntryID, DateTime | None], WorkspaceFSTimestamped]:
        return {
            # workspace_id_and_ts[1] will never be `None` because of the filter check
            workspace_id_and_ts: self._get_workspace_timestamped(*workspace_id_and_ts)  # type: ignore[arg-type]
            for workspace_id_and_ts in self._mountpoint_tasks.keys()
            if workspace_id_and_ts[1] is not None
        }

    def is_workspace_mounted(
        self, workspace_id: EntryID, timestamp: DateTime | None = None
    ) -> bool:
        return (workspace_id, timestamp) in self._mountpoint_tasks

    async def safe_mount(self, workspace_id: EntryID) -> None:
        try:
            await self.mount_workspace(workspace_id)

        # The workspace could not be mounted
        # Maybe be a `MountpointAlreadyMounted` or `MountpointNoDriveAvailable`
        except MountpointError:
            pass

        # Unexpected exception is not a reason to crash the mountpoint manager
        except Exception:
            logger.exception("Unexpected error while mounting a new workspace")

    async def safe_unmount(self, workspace_id: EntryID, timestamp: DateTime | None = None) -> None:
        try:
            await self.unmount_workspace(workspace_id, timestamp)

        # The workspace could not be unmounted
        # Maybe be a `MountpointAlreadyMounted` or `MountpointNoDriveAvailable`
        except MountpointError:
            pass

        # Unexpected exception is not a reason to crash the mountpoint manager
        except Exception:
            logger.exception("Unexpected error while unmounting a revoked workspace")

    async def safe_mount_all(self, exclude: Sequence[EntryID] = ()) -> None:
        exclude_set = set(exclude)
        for workspace in self.user_fs.get_available_workspaces():
            if workspace.workspace_id in exclude_set:
                continue
            # Do not mount archived workspaces unless we're asked to
            if workspace.is_archived():
                continue
            # Do not mount deletion-planned workspaces unless we're asked to
            if workspace.is_deletion_planned():
                continue
            await self.safe_mount(workspace.workspace_id)

    async def safe_unmount_all(self) -> None:
        for workspace_id, timestamp in list(self._mountpoint_tasks.keys()):
            await self.safe_unmount(workspace_id, timestamp=timestamp)


async def cleanup_macos_mountpoint_folder(base_mountpoint_path: Path) -> None:
    # In case of a crash on macOS, workspaces don't unmount correctly and leave empty
    # mountpoints or directories in the default mount folder. This function is used to
    # unmount and/or delete these anytime a login occurs. In some rare and so far unknown
    # conditions, the unmount call can fail, raising a CalledProcessError, hence the
    # exception below. In such cases, the issue stops occurring after a system reboot,
    # making the exception catching a temporary solution.
    base_mountpoint_path = trio.Path(base_mountpoint_path)
    try:
        mountpoint_names = await base_mountpoint_path.iterdir()
    except FileNotFoundError:
        # Unlike with `pathlib.Path.iterdir` which returns a lazy iterator,
        # `trio.Path.iterdir` does FS access and may raise FileNotFoundError
        return
    for mountpoint_name in mountpoint_names:
        mountpoint_path = base_mountpoint_path / mountpoint_name
        stats = await trio.Path(mountpoint_path).stat()
        if (
            stats.st_size == 0
            and stats.st_blocks == 0
            and stats.st_atime == 0
            and stats.st_mtime == 0
            and stats.st_ctime == 0
        ):
            try:
                await trio.run_process(["diskutil", "unmount", "force", mountpoint_path])
            except CalledProcessError as exc:
                logger.warning(
                    "Error during mountpoint cleanup: diskutil unmount failed",
                    exc_info=exc,
                    mountpoint_path=mountpoint_path,
                )
            try:
                await trio.Path(mountpoint_path).rmdir()
            except FileNotFoundError:
                pass


@asynccontextmanager
async def mountpoint_manager_factory(
    user_fs: UserFS,
    event_bus: EventBus,
    base_mountpoint_path: Path,
    *,
    debug: bool = False,
    mount_all: bool = False,
    mount_on_workspace_created: bool = False,
    mount_on_workspace_shared: bool = False,
    unmount_on_workspace_revoked: bool = False,
    unmount_on_workspace_archived: bool = False,
    unmount_on_workspace_deletion_planned: bool = False,
    exclude_from_mount_all: frozenset[EntryID] = frozenset(),
    mountpoint_in_directory: bool = False,
    personal_workspace_base_path: Path | None = None,
    personal_workspace_name_pattern: str | None = None,
) -> AsyncGenerator[MountpointManager, Any]:
    config = {"debug": debug}

    runner = get_mountpoint_runner()

    # In order to mount personal workspaces into a directory, the
    # mountpoint_in_directory option should be enabled
    if not mountpoint_in_directory:
        # Fail if personal workspaces option are
        assert (
            personal_workspace_base_path is None
        ), "Option personal_workspace_base_path requires mountpoint_in_directory=True"
        assert (
            personal_workspace_name_pattern is None
        ), "Option personal_workspace_name_pattern requires mountpoint_in_directory=True"

    # Now is a good time to perform some cleanup in the registry
    if sys.platform == "win32":
        cleanup_parsec_drive_icons()

        # Enable mountpoint in directory mode only on Windows (also, note that
        # the config dict is *unpacked* on Linux in fuse_mountpoint_runner)
        config["mountpoint_in_directory"] = mountpoint_in_directory
        config["personal_workspace_base_path"] = personal_workspace_base_path
        config["personal_workspace_name_pattern"] = personal_workspace_name_pattern

    elif sys.platform == "darwin":
        await cleanup_macos_mountpoint_folder(base_mountpoint_path)

    def on_workspace_created_event(
        event: CoreEvent, new_entry: WorkspaceEntry, previous_entry: WorkspaceEntry | None = None
    ) -> None:
        if mount_on_workspace_created:
            mount_nursery.start_soon(mountpoint_manager.safe_mount, new_entry.id)

    def on_sharing_updated_event(
        event: CoreEvent, new_entry: WorkspaceEntry, previous_entry: WorkspaceEntry | None = None
    ) -> None:
        new_role = new_entry.role
        previous_role = previous_entry.role if previous_entry is not None else None

        # Workspace revoked
        if previous_role is not None and new_role is None and unmount_on_workspace_revoked:
            mount_nursery.start_soon(mountpoint_manager.safe_unmount, new_entry.id)

        # Workspace shared
        if previous_role is None and new_role is not None and mount_on_workspace_shared:
            mount_nursery.start_soon(mountpoint_manager.safe_mount, new_entry.id)

    def on_archiving_updated_event(
        event: CoreEvent,
        workspace_id: EntryID,
        configuration: RealmArchivingConfiguration,
        configured_on: DateTime | None,
        is_deleted: bool,
    ) -> None:
        if configuration.is_archived() and unmount_on_workspace_archived:
            mount_nursery.start_soon(mountpoint_manager.safe_unmount, workspace_id)
        if configuration.is_deletion_planned() and unmount_on_workspace_deletion_planned:
            mount_nursery.start_soon(mountpoint_manager.safe_unmount, workspace_id)

    # Instantiate the mountpoint manager with its own nursery
    async with open_service_nursery() as nursery:
        mountpoint_manager = MountpointManager(
            user_fs,
            event_bus,
            base_mountpoint_path,
            config,
            runner,
            nursery,
            personal_workspace_base_path,
            personal_workspace_name_pattern,
        )

        # Exit this context by unmounting all mountpoints
        try:
            # A nursery dedicated to new workspace events
            async with open_service_nursery() as mount_nursery:
                # Setup new workspace events
                with event_bus.connect_in_context(
                    (CoreEvent.FS_WORKSPACE_CREATED, on_workspace_created_event),  # type: ignore[arg-type]
                    (CoreEvent.SHARING_UPDATED, on_sharing_updated_event),  # type: ignore[arg-type]
                    (CoreEvent.ARCHIVING_UPDATED, on_archiving_updated_event),  # type: ignore[arg-type]
                ):
                    # Mount required workspaces
                    if mount_all:
                        await mountpoint_manager.safe_mount_all(
                            exclude=tuple(exclude_from_mount_all)
                        )

                    # Yield point
                    yield mountpoint_manager

                    # Cancel current mount_workspace tasks
                    mount_nursery.cancel_scope.cancel()

        # Unmount all the workspaces
        finally:
            # We need a shield here to guarantee not mountpoints remain behind.
            # Otherwise the mountpoint might be used after it related workspacesfs
            # has been torndown.
            with trio.CancelScope(shield=True):
                await mountpoint_manager.safe_unmount_all()

        # Cancel the mountpoint tasks (although they should all be finished by now)
        nursery.cancel_scope.cancel()
