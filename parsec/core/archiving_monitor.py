# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from math import inf
from typing import TYPE_CHECKING, cast

import trio

from parsec._parsec import CoreEvent, DateTime, EntryID
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.fs import FSBackendOfflineError
from parsec.core.fs.userfs import UserFS
from parsec.event_bus import EventBus, EventCallback

if TYPE_CHECKING:
    from parsec.core.backend_connection.authenticated import MonitorTaskStatus


async def freeze_archiving_monitor_mockpoint() -> None:
    """
    Noop function that could be mocked during tests to be able to freeze the
    monitor coroutine running in background
    """


async def monitor_archiving(
    user_fs: UserFS, event_bus: EventBus, task_status: MonitorTaskStatus
) -> None:
    next_deletion = None
    update_sleep_scope = trio.CancelScope()
    interrupt_sleep_scope = trio.CancelScope()

    def _update_next_deletion(next_deletion_date: DateTime | None) -> None:
        nonlocal next_deletion
        if next_deletion_date is None:
            return
        if next_deletion is None:
            next_deletion = next_deletion_date
        else:
            next_deletion = min(next_deletion, next_deletion_date)
        update_sleep_scope.cancel()

    async def _wait_until_next_deletion() -> None:
        nonlocal interrupt_sleep_scope, update_sleep_scope, next_deletion
        try:
            with interrupt_sleep_scope:
                while True:
                    delta = (
                        max(0.0, next_deletion - user_fs.device.time_provider.now())
                        if next_deletion is not None
                        else inf
                    )
                    with trio.CancelScope() as update_sleep_scope:
                        await user_fs.device.time_provider.sleep(delta)
                    if update_sleep_scope.cancelled_caught:
                        continue
                    next_deletion = None
                    return
        finally:
            interrupt_sleep_scope = trio.CancelScope()

    def _on_archiving_or_roles_updated(event: CoreEvent, **kwargs: object) -> None:
        interrupt_sleep_scope.cancel()
        # Don't wait for the *actual* awakening to change the status to
        # avoid having a period of time when the awakening is scheduled but
        # not yet notified to task_status
        task_status.awake()

    def _on_archiving_next_deletion_date(
        event: CoreEvent, workspace_id: EntryID, next_deletion_date: DateTime
    ) -> None:
        _update_next_deletion(next_deletion_date)
        # Don't wait for the *actual* awakening to change the status to
        # avoid having a period of time when the awakening is scheduled but
        # not yet notified to task_status
        task_status.awake()

    with event_bus.connect_in_context(
        (
            CoreEvent.BACKEND_REALM_ARCHIVING_UPDATED,
            cast(EventCallback, _on_archiving_or_roles_updated),
        ),
        (
            CoreEvent.BACKEND_REALM_ROLES_UPDATED,
            cast(EventCallback, _on_archiving_or_roles_updated),
        ),
        (
            CoreEvent.ARCHIVING_NEXT_DELETION_DATE,
            cast(EventCallback, _on_archiving_next_deletion_date),
        ),
    ):
        try:
            _update_next_deletion(await user_fs.update_archiving_status())
            task_status.started()
            while True:
                task_status.idle()
                await _wait_until_next_deletion()
                await freeze_archiving_monitor_mockpoint()
                _update_next_deletion(await user_fs.update_archiving_status())

        except FSBackendOfflineError as exc:
            raise BackendNotAvailable from exc
