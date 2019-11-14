# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from enum import Enum


BackendState = Enum("BackendState", "READY LOST INCOMPATIBLE_VERSION INITIALIZING")
connection_states = {}


def current_backend_connection_state(event_bus):
    return connection_states[event_bus]


async def monitor_backend_connection(event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    connection_states[event_bus] = BackendState.LOST
    events = {
        "backend.offline": trio.Event(),
        "backend.online": trio.Event(),
        "backend.incompatible_version": trio.Event(),
        "sync_monitor.reconnection_sync.done": trio.Event(),
        "message_monitor.reconnection_message_processing.done": trio.Event(),
    }

    def _on_event(event, *args, **kwargs):
        events[event].set()

    async def wait_offline(nursery):
        await events["backend.offline"].wait()
        if connection_states[event_bus] != BackendState.INCOMPATIBLE_VERSION:
            event_bus.send("backend.connection.lost")
            connection_states[event_bus] = BackendState.LOST
        for e in events.values():
            e.clear()
        nursery.cancel_scope.cancel()

    async def wait_incompatible_version():
        await events["backend.incompatible_version"].wait()
        if connection_states[event_bus] != BackendState.INCOMPATIBLE_VERSION:
            connection_states[event_bus] = BackendState.INCOMPATIBLE_VERSION
            event_bus.send("backend.connection.incompatible_version")

    async def wait_connect():
        await events["backend.online"].wait()
        event_bus.send("backend.connection.bootstrapping")

        await events["sync_monitor.reconnection_sync.done"].wait()
        await events["message_monitor.reconnection_message_processing.done"].wait()
        connection_states[event_bus] = BackendState.READY
        event_bus.send("backend.connection.ready")

    with event_bus.connect_in_context(
        ("backend.offline", _on_event),
        ("backend.online", _on_event),
        ("backend.incompatible_version", _on_event),
        ("sync_monitor.reconnection_sync.done", _on_event),
        ("message_monitor.reconnection_message_processing.done", _on_event),
    ):

        task_status.started()
        while True:
            async with trio.open_service_nursery() as nursery:
                nursery.start_soon(wait_incompatible_version)
                nursery.start_soon(wait_offline, nursery)
                nursery.start_soon(wait_connect)
