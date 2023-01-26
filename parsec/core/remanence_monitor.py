# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import TYPE_CHECKING, cast

import trio

from parsec._parsec import CoreEvent, EntryID, WorkspaceEntry
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.fs import FSBackendOfflineError, UserFS
from parsec.core.fs.exceptions import FSWorkspaceNotFoundError
from parsec.event_bus import EventBus, EventCallback
from parsec.utils import open_service_nursery

if TYPE_CHECKING:
    from parsec.core.backend_connection.authenticated import MonitorTaskStatus


async def freeze_remanence_monitor_mockpoint() -> None:
    """
    Noop function that could be mocked during tests to be able to freeze the
    monitor coroutine running in background
    """


async def monitor_remanent_workspaces(
    user_fs: UserFS, event_bus: EventBus, task_status: MonitorTaskStatus
) -> None:
    on_going_tasks: int = 0

    def start_downsync_manager(workspace_id: EntryID) -> None:
        # Make sure the workspace is available
        try:
            workspace_fs = user_fs.get_workspace(workspace_id)
        except FSWorkspaceNotFoundError:
            return

        async def sleep_forever() -> None:
            nonlocal on_going_tasks
            on_going_tasks -= 1
            if on_going_tasks == 0:
                task_status.idle()
            try:
                await trio.sleep_forever()
            finally:
                task_status.awake()
                on_going_tasks += 1

        async def remanent_task() -> None:
            nonlocal on_going_tasks
            # Possibly block new tasks while testing
            await freeze_remanence_monitor_mockpoint()
            # Register both tasks (the remanence manager has two tasks)
            on_going_tasks += 2
            task_status.awake()
            # Run task
            try:
                await workspace_fs.run_remanence_manager(sleep_forever)
            # Cleanup task
            finally:
                on_going_tasks -= 2
                if on_going_tasks == 0:
                    task_status.idle()

        # Schedule task
        nursery.start_soon(remanent_task)

    def _on_sharing_updated(
        event: CoreEvent,
        new_entry: WorkspaceEntry,
        previous_entry: WorkspaceEntry | None,
    ) -> None:
        # Not a new workspace
        if previous_entry is not None:
            return
        start_downsync_manager(new_entry.id)

    def _fs_workspace_created(
        event: CoreEvent,
        new_entry: WorkspaceEntry,
    ) -> None:
        start_downsync_manager(new_entry.id)

    try:
        async with open_service_nursery() as nursery:

            with event_bus.connect_in_context(
                (CoreEvent.SHARING_UPDATED, cast(EventCallback, _on_sharing_updated)),
                (CoreEvent.FS_WORKSPACE_CREATED, cast(EventCallback, _fs_workspace_created)),
            ):
                # Set a brief idle state useful for testing
                task_status.started()
                task_status.idle()

                # All workspaces should be processed at startup
                for entry in user_fs.get_user_manifest().workspaces:
                    start_downsync_manager(entry.id)

                await trio.sleep_forever()

    except FSBackendOfflineError as exc:
        raise BackendNotAvailable from exc
