# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from trio.hazmat import current_clock

from parsec.core.fs import FSBackendOfflineError


MIN_WAIT = 1
MAX_WAIT = 60


def timestamp():
    # Use time from trio clock to easily mock it
    return current_clock().current_time()


class SyncMonitor:
    def __init__(self, fs, event_bus):
        super().__init__()
        self.user_fs = fs.user_fs
        self.event_bus = event_bus

        self._running = False

        self._backend_online_event = trio.Event()
        self._backend_online_event.set()
        self._monitoring_cancel_scope = None

        self.event_bus.connect("backend.online", self._on_backend_online)
        self.event_bus.connect("backend.offline", self._on_backend_offline)

    def _on_backend_online(self, event):
        self._backend_online_event.set()

    def _on_backend_offline(self, event):
        self._backend_online_event.clear()
        if self._monitoring_cancel_scope:
            self._monitoring_cancel_scope.cancel()

    @property
    def running(self):
        return self._running

    async def run(self, *, task_status=trio.TASK_STATUS_IGNORED):
        if self.running:
            raise RuntimeError("Already running")
        self._running = True

        task_status.started()
        while True:
            await self._backend_online_event.wait()
            try:
                await self._monitoring()
            except FSBackendOfflineError:
                self._backend_online_event.clear()
            self._monitoring_cancel_scope = None
            self.event_bus.send("sync_monitor.disconnected")

    async def _monitoring(self):
        with trio.CancelScope() as self._monitoring_cancel_scope:
            self.event_bus.send("sync_monitor.reconnection_sync.started")
            try:
                await self.user_fs.sync()
                user_manifest = self.user_fs.get_user_manifest()
                for entry in user_manifest.workspaces:
                    workspace = self.user_fs.get_workspace(entry.id)
                    await workspace.sync("/")
            finally:
                self.event_bus.send("sync_monitor.reconnection_sync.done")

            await self._monitoring_loop()

    async def _monitoring_loop(self):
        updated_entries = {}
        new_event = trio.Event()

        def _on_entry_updated(event, workspace_id=None, id=None):
            assert id is not None
            try:
                first_updated, _ = updated_entries[workspace_id, id]
                last_updated = timestamp()
            except KeyError:
                first_updated = last_updated = timestamp()
            updated_entries[workspace_id, id] = first_updated, last_updated
            new_event.set()

        def _on_realm_vlobs_updated(sender, realm_id, checkpoint, src_id, src_version):
            updated_entries[realm_id, src_id] = timestamp() - MAX_WAIT, timestamp() - MAX_WAIT
            new_event.set()

        with self.event_bus.connect_in_context(
            ("fs.entry.updated", _on_entry_updated),
            ("backend.realm.vlobs_updated", _on_realm_vlobs_updated),
        ):

            async with trio.open_nursery() as nursery:
                while True:
                    self.event_bus.send("sync_monitor.ready")

                    await new_event.wait()
                    new_event.clear()

                    await self._monitoring_tick(updated_entries)
                    sorted_entries = self._sorted_entries(updated_entries)
                    if sorted_entries:
                        t, _, _ = sorted_entries[0]
                        delta = max(0, t - timestamp())

                        async def _wait():
                            await trio.sleep(delta)
                            new_event.set()

                        nursery.start_soon(_wait)

    def _sorted_entries(self, updated_entries):
        result = []
        for (wid, id), (first_updated, last_updated) in updated_entries.items():
            t = min(first_updated + MAX_WAIT, last_updated + MIN_WAIT)
            result.append((t, id, wid))
        return sorted(result)

    async def _monitoring_tick(self, updated_entries):
        # Loop over entries to synchronize
        for t, id, wid in self._sorted_entries(updated_entries):
            if t > timestamp():
                return

            # Pop from entries before the synchronization
            updated_entries.pop((wid, id), None)

            # Perform the synchronization
            if id == self.user_fs.user_manifest_id:
                await self.user_fs.sync()
            elif wid is not None:
                workspace = self.user_fs.get_workspace(wid)
                await workspace.sync_by_id(id)


async def monitor_sync(fs, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    with event_bus.connection_context() as event_bus_ctx:
        sync_monitor = SyncMonitor(fs, event_bus_ctx)
        await sync_monitor.run(task_status=task_status)
