# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from math import inf
from typing import TYPE_CHECKING, cast

import trio

from parsec._parsec import CoreEvent
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
    wakeup = trio.Event()

    def _on_archiving_updated(event: CoreEvent, **kwargs: object) -> None:
        nonlocal wakeup
        wakeup.set()
        # Don't wait for the *actual* awakening to change the status to
        # avoid having a period of time when the awakening is scheduled but
        # not yet notified to task_status
        task_status.awake()

    with event_bus.connect_in_context(
        (CoreEvent.BACKEND_REALM_ARCHIVING_UPDATED, cast(EventCallback, _on_archiving_updated))
    ):
        try:
            next_deletion = await user_fs.update_archiving_status()
            task_status.started()
            while True:
                task_status.idle()
                delta = (
                    max(0.0, next_deletion - user_fs.device.time_provider.now())
                    if next_deletion is not None
                    else inf
                )
                with trio.move_on_after(delta):
                    await wakeup.wait()
                wakeup = trio.Event()
                await freeze_archiving_monitor_mockpoint()
                next_deletion = await user_fs.update_archiving_status()

        except FSBackendOfflineError as exc:
            raise BackendNotAvailable from exc
