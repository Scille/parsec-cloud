import trio
from trio.hazmat import current_clock

from parsec.core.base import BaseAsyncComponent
from parsec.core.backend_connection import BackendNotAvailable


MIN_WAIT = 1
MAX_WAIT = 60


def timestamp():
    # Use time from trio clock to easily mock it
    return current_clock().current_time()


class SyncMonitor(BaseAsyncComponent):
    def __init__(self, fs, event_bus):
        super().__init__()
        self.fs = fs
        self.event_bus = event_bus

        self._running = False

        self._backend_online_event = trio.Event()
        self._monitoring_cancel_scope = None

        self.event_bus.connect("backend.online", self._on_backend_online, weak=True)
        self.event_bus.connect("backend.offline", self._on_backend_offline, weak=True)

    def _on_backend_online(self, event):
        self._backend_online_event.set()

    def _on_backend_offline(self, event):
        self._backend_online_event.clear()
        if self._monitoring_cancel_scope:
            self._monitoring_cancel_scope.cancel()

    @property
    def running(self):
        return self._running

    async def run(self):
        if self.running:
            raise RuntimeError("Already running")
        self._running = True

        while True:
            await self._backend_online_event.wait()
            try:
                await self._monitoring()
            except BackendNotAvailable:
                pass
            self._monitoring_cancel_scope = None
            self.event_bus.send("sync_monitor.disconnected")

    async def _monitoring(self):
        with trio.open_cancel_scope() as self._monitoring_cancel_scope:
            self.event_bus.send("sync_monitor.reconnection_sync.started")
            try:
                await self.fs.full_sync()
            finally:
                self.event_bus.send("sync_monitor.reconnection_sync.done")

            await self._monitoring_loop()

    async def _monitoring_loop(self):
        updated_entries = {}
        new_event = trio.Event()

        def _on_entry_updated(sender, id):
            try:
                first_updated, _ = updated_entries[id]
                last_updated = timestamp()
            except KeyError:
                first_updated = last_updated = timestamp()
            updated_entries[id] = (first_updated, last_updated)
            new_event.set()

        self.event_bus.connect("fs.entry.updated", _on_entry_updated, weak=True)

        async with trio.open_nursery() as nursery:
            while True:
                self.event_bus.send("sync_monitor.ready")

                await new_event.wait()
                new_event.clear()

                await self._monitoring_tick(updated_entries)

                if updated_entries:

                    async def _wait():
                        await trio.sleep(MIN_WAIT)
                        new_event.set()

                    nursery.start_soon(_wait)

    async def _monitoring_tick(self, updated_entries):
        now = timestamp()

        for id, (first_updated, last_updated) in updated_entries.items():
            if now - first_updated > MAX_WAIT:
                await self.fs.sync_by_id(id)
                break

        else:
            for id, (_, last_updated) in updated_entries.items():
                if now - last_updated > MIN_WAIT:
                    await self.fs.sync_by_id(id)
                    break

            else:
                id = None

        if id:
            _, new_last_updated = updated_entries[id]
            # This entry has been modified again during the sync
            if new_last_updated != last_updated:
                updated_entries[id] = (last_updated, new_last_updated)
            else:
                del updated_entries[id]
