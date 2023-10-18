# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import TYPE_CHECKING, cast

import structlog
import trio

from parsec._parsec import CoreEvent, DateTime, EntryID, RealmArchivingConfiguration, WorkspaceEntry
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.fs import FSBackendOfflineError, UserFS
from parsec.core.fs.exceptions import FSWorkspaceNoAccess, FSWorkspaceNotFoundError
from parsec.core.fs.workspacefs.remanence_manager import (
    RemanenceManagerTask,
    RemanenceManagerTaskID,
)
from parsec.event_bus import EventBus, EventCallback
from parsec.utils import open_service_nursery

if TYPE_CHECKING:
    from parsec.core.backend_connection.authenticated import MonitorTaskStatus


logger = structlog.get_logger()


async def freeze_remanence_monitor_mockpoint() -> None:
    """
    Noop function that could be mocked during tests to be able to freeze the
    monitor coroutine running in background
    """


async def monitor_remanent_workspaces(
    user_fs: UserFS, event_bus: EventBus, task_status: MonitorTaskStatus
) -> None:
    awake_tasks: set[RemanenceManagerTaskID] = set()
    cancel_scopes: dict[EntryID, trio.CancelScope] = {}

    def _idle(task_id: RemanenceManagerTaskID) -> None:
        awake_tasks.discard(task_id)
        if not awake_tasks:
            logger.info(
                "Remanence monitor workspace idle", workspace_id=task_id[0], task=task_id[1].name
            )
            task_status.idle()

    def _awake(task_id: RemanenceManagerTaskID) -> None:
        logger.info(
            "Remanence monitor workspace awake", workspace_id=task_id[0], task=task_id[1].name
        )
        awake_tasks.add(task_id)
        task_status.awake()

    def _start_remanence_manager(workspace_id: EntryID) -> bool:
        # Make sure the workspace is available
        try:
            workspace_fs = user_fs.get_workspace(workspace_id)
        except FSWorkspaceNotFoundError:
            return False

        async def remanent_task() -> None:
            # Already started
            if workspace_id in cancel_scopes:
                return
            # Make it cancellable when reader rights are revoked
            try:
                with trio.CancelScope() as cancel_scopes[workspace_id]:
                    # Possibly block new tasks while testing
                    await freeze_remanence_monitor_mockpoint()
                    # Run task
                    await workspace_fs.run_remanence_manager(_idle, _awake)
            # Our read rights have been revoked, no reason to collapse the monitor for that
            except FSWorkspaceNoAccess:
                pass
            # Set workspace id as not running
            finally:
                del cancel_scopes[workspace_id]

        # Schedule task
        nursery.start_soon(remanent_task)
        return True

    def _on_read_rights_removed(workspace_id: EntryID) -> bool:
        task_id = (workspace_id, RemanenceManagerTask.CLEANUP)

        # Cancel the remanence manager if it's running
        if workspace_id in cancel_scopes:
            cancel_scopes[workspace_id].cancel()

        # Make sure the workspace is available
        try:
            workspace_fs = user_fs.get_workspace(workspace_id)
        except FSWorkspaceNotFoundError:
            return False

        # TODO: this might be a good place to fully clean up the
        # blocks from the cache even if the workspace is not block remanent
        # since the read rights have been revoked.
        # It might also make sense to remove the manifests here.

        # Nothing to cleanup
        if not workspace_fs.local_storage.is_block_remanent():
            return False

        async def cleanup_task() -> None:
            # Avoid concurrency
            if task_id in awake_tasks:
                return
            # Set task
            _awake(task_id)
            try:
                # Possibly block new tasks while testing
                await freeze_remanence_monitor_mockpoint()
                # Disable block remanence to cleanup disk space
                await workspace_fs.local_storage.disable_block_remanence()
            # Discard task
            finally:
                _idle(task_id)

        # Schedule task
        nursery.start_soon(cleanup_task)
        return True

    def _on_sharing_updated(
        event: CoreEvent,
        new_entry: WorkspaceEntry,
        previous_entry: WorkspaceEntry | None,
    ) -> None:
        # No reader rights
        if new_entry.role is None:
            if new_entry.id in cancel_scopes:
                _on_read_rights_removed(new_entry.id)
        # Reader rights have been granted
        elif previous_entry is None or previous_entry.role is None:
            _start_remanence_manager(new_entry.id)

    def _on_archiving_updated(
        event: CoreEvent,
        workspace_id: EntryID,
        configuration: RealmArchivingConfiguration,
        configured_on: DateTime | None,
        is_deleted: bool,
    ) -> None:
        # No reader rights
        if is_deleted and workspace_id in cancel_scopes:
            _on_read_rights_removed(workspace_id)

    def _fs_workspace_created(
        event: CoreEvent,
        new_entry: WorkspaceEntry,
    ) -> None:
        _start_remanence_manager(new_entry.id)

    try:
        # Nursery used by _start_remanence_manager/_on_sharing_updated/_fs_workspace_created closures
        async with open_service_nursery() as nursery:
            with event_bus.connect_in_context(
                (CoreEvent.SHARING_UPDATED, cast(EventCallback, _on_sharing_updated)),
                (CoreEvent.ARCHIVING_UPDATED, cast(EventCallback, _on_archiving_updated)),
                (CoreEvent.FS_WORKSPACE_CREATED, cast(EventCallback, _fs_workspace_created)),
            ):
                # All workspaces should be processed at startup
                available_workspaces, unavailable_workspaces = user_fs.get_all_workspaces()

                # Clean up unavailable workspaces if necessary
                cleanup_tasks_scheduled = [
                    _on_read_rights_removed(workspace.workspace_id)
                    for workspace in unavailable_workspaces
                ]

                # Each workspace will have it own task that will start awake,
                # then switch to idle
                manager_tasks_scheduled = [
                    _start_remanence_manager(workspace.workspace_id)
                    for workspace in available_workspaces
                ]

                # Set idle status if no task has been scheduled
                if not any(manager_tasks_scheduled) and not any(cleanup_tasks_scheduled):
                    task_status.idle()

                # The monitor is now started
                task_status.started()
                await trio.sleep_forever()

    except FSBackendOfflineError as exc:
        raise BackendNotAvailable from exc
