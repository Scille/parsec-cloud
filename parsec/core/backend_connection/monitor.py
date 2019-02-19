# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio


async def monitor_backend_connection(
    backend_online, event_bus, *, task_status=trio.TASK_STATUS_IGNORED
):
    events = {
        "backend.offline": trio.Event(),
        "backend.online": trio.Event(),
        "sync_monitor.reconnection_sync.done": trio.Event(),
        "message_monitor.reconnection_message_processing.done": trio.Event(),
    }

    def _on_event(event, *args, **kwargs):
        events[event].set()

    with event_bus.connect_in_context(
        ("backend.offline", _on_event),
        ("backend.online", _on_event),
        ("sync_monitor.reconnection_sync.done", _on_event),
        ("message_monitor.reconnection_message_processing.done", _on_event),
    ):

        if backend_online:
            events["backend.online"].set()

        task_status.started()
        while True:
            await events["backend.online"].wait()
            event_bus.send("backend.connection.bootstrapping")

            await events["sync_monitor.reconnection_sync.done"].wait()
            await events["message_monitor.reconnection_message_processing.done"].wait()
            event_bus.send("backend.connection.ready")

            await events["backend.offline"].wait()
            for e in events.values():
                e.clear()
            event_bus.send("backend.connection.lost")
