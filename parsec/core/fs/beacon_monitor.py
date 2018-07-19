import trio
import logbook

from parsec.core.base import BaseAsyncComponent
from parsec.core.fs.local_folder_fs import FSManifestLocalMiss


logger = logbook.Logger("parsec.core.fs.beacon_monitor")


# We could also mark the local db entries outdated we update occurs to
# only bother about them when they're really needed


class BeaconMonitor(BaseAsyncComponent):
    def __init__(self, device, local_folder_fs, signal_ns):
        super().__init__()
        self._device = device
        self.local_folder_fs = local_folder_fs
        self.signal_ns = signal_ns
        self._task_info = None
        self._workspaces = {}

    async def _init(self, nursery):
        for beacon_id in self.local_folder_fs.get_local_beacons():
            self.signal_ns.signal("backend.beacon.listen").send(None, beacon_id=beacon_id)

        self._task_info = await nursery.start(self._task)

    async def _teardown(self):
        cancel_scope, closed_event = self._task_info
        cancel_scope.cancel()
        await closed_event.wait()
        self._task_info = None

    def _retreive_workspace_from_beacon(self, beacon_id):
        root_manifest = self.local_folder_fs.get_manifest(self._device.user_manifest_access)
        if root_manifest["beacon_id"] == beacon_id:
            return "/"
        # No need to go recursive given workspaces must be direct root children
        for child_name, child_access in root_manifest["children"].items():
            child_manifest = self.local_folder_fs.get_manifest(child_access)
            if child_manifest.get("beacon_id") == beacon_id:
                return f"/{child_name}"
        raise FSManifestLocalMiss()

    async def _task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        def _on_workspace_loaded(sender, path, id, beacon_id):
            self._workspaces[beacon_id] = path
            self.signal_ns.signal("backend.beacon.listen").send(None, beacon_id=beacon_id)

        def _on_workspace_unloaded(sender, path, id, beacon_id):
            del self._workspaces[beacon_id]
            self.signal_ns.signal("backend.beacon.unlisten").send(None, beacon_id=beacon_id)

        entry_updated_signal = self.signal_ns.signal("fs.entry.updated")

        def _on_beacon_updated(sender, beacon_id, index, src_id, src_version):
            try:
                workspace_path = self._retreive_workspace_from_beacon(beacon_id)
            except FSManifestLocalMiss:
                # This workspace is not present in our local cache, nothing
                # to keep updated then.
                return

            logger.debug(
                "Beacon {} ({}) notifies update of entry {} to version {}",
                beacon_id,
                workspace_path,
                src_id,
                src_version,
            )
            entry_updated_signal.send(id=src_id)

        self.signal_ns.signal("fs.workspace.loaded").connect(_on_workspace_loaded, weak=True)
        self.signal_ns.signal("fs.workspace.unloaded").connect(_on_workspace_unloaded, weak=True)
        self.signal_ns.signal("backend.beacon.updated").connect(_on_beacon_updated, weak=True)

        closed_event = trio.Event()
        try:
            with trio.open_cancel_scope() as cancel_scope:
                task_status.started((cancel_scope, closed_event))
                await trio.sleep_forever()
        finally:
            closed_event.set()
