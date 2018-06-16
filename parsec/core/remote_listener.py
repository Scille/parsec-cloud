import trio
import json
import logbook

from parsec.core.base import BaseAsyncComponent
from parsec.core.backend_connection import BackendError


logger = logbook.Logger("parsec.core.remote_listener")


class RemoteListener(BaseAsyncComponent):
    def __init__(self, device, backend_connection, backend_event_manager, signal_ns):
        super().__init__()
        self._device = device
        self._backend_connection = backend_connection
        self._backend_event_manager = backend_event_manager
        self.signal_ns = signal_ns
        self._new_notify_sinks = trio.Queue(100)
        self._notify_sink_updated = trio.Queue(100)
        self._new_notify_sink_task_info = None
        self._notify_sink_updated_task_info = None

    async def _init(self, nursery):
        self.signal_ns.signal("fs_local_notify_sink_loaded").connect(
            self._on_fs_local_notify_sink_loaded, weak=True
        )
        self._new_notify_sink_task_info = await nursery.start(self._new_notify_sink_task)
        self._notify_sink_updated_task_info = await nursery.start(self._notify_sink_updated_task)

    async def _teardown(self):
        cancel_scope, closed_event = self._new_notify_sink_task_info
        cancel_scope.cancel()
        await closed_event.wait()

        cancel_scope, closed_event = self._notify_sink_updated_task_info
        cancel_scope.cancel()
        await closed_event.wait()

    def _on_fs_local_notify_sink_loaded(self, sender, notify_sink):
        self._new_notify_sinks.put_nowait(notify_sink)

    def _on_notify_sink_vlob_updated(self, sender, notify_sink):
        self._notify_sink_updated.put_nowait(notify_sink)

    async def _new_notify_sink_task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        try:
            closed_event = trio.Event()
            with trio.open_cancel_scope() as cancel_scope:
                task_status.started((cancel_scope, closed_event))
                while True:
                    new_notify_sink = await self._new_notify_sinks.get()
                    await self._backend_events_manager.subscribe_backend_event(
                        "vlob_updated", new_notify_sink
                    )
        finally:
            closed_event.set()

    async def _notify_sink_updated_task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        try:
            closed_event = trio.Event()
            with trio.open_cancel_scope() as cancel_scope:
                task_status.started((cancel_scope, closed_event))
                while True:
                    notify_sink = await self._notify_sink_updated.get()
                    rep = await self._backend_connection.send(
                        {"cmd": "vlob_get", "id": notify_sink, "trust_seed": notify_sink}
                    )
                    if rep["status"] != "ok":
                        # TODO
                        raise BackendError()
                    data = json.loads(rep["blob"].decode("utf-8"))
                    if data["author"] != self._device.device_id:
                        # TODO: we should only re-sync what's needed
                        self.signal_ns.signal("fs_entry_updated").send(self, "/")

        finally:
            closed_event.set()
