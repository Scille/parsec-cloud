import trio


async def monitor_connection(event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    events = {
        "backend.offline": trio.Event(),
        "backend.online": trio.Event(),
        "sync_monitor.reconnection_sync.done": trio.Event(),
        "message_monitor.reconnection_message_processing.done": trio.Event(),
    }

    def _on_event(event, *args, **kwargs):
        events[event].set()

    event_bus.connect("backend.offline", _on_event, weak=True)
    event_bus.connect("backend.online", _on_event, weak=True)
    event_bus.connect("sync_monitor.reconnection_sync.done", _on_event, weak=True)
    event_bus.connect("message_monitor.reconnection_message_processing.done", _on_event, weak=True)

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
