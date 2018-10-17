import trio
from structlog import get_logger


logger = get_logger()


# We could also mark the local db entries outdated we update occurs to
# only bother about them when they're really needed


async def monitor_beacons(device, fs, event_bus):
    workspaces = set()

    # TODO: stop using private attribute `fs._local_folder_fs`
    for beacon_id in fs._local_folder_fs.get_local_beacons():
        event_bus.send("backend.beacon.listen", beacon_id=beacon_id)

    def _on_workspace_loaded(sender, path, id, beacon_id):
        workspaces.add(beacon_id)
        event_bus.send("backend.beacon.listen", beacon_id=beacon_id)

    def _on_workspace_unloaded(sender, path, id, beacon_id):
        workspaces.remove(beacon_id)
        event_bus.send("backend.beacon.unlisten", beacon_id=beacon_id)

    def _on_beacon_updated(sender, beacon_id, index, src_id, src_version):
        workspace_path = _retreive_workspace_from_beacon(device, fs, beacon_id)
        if not workspace_path:
            # This workspace is not present in our local cache, nothing
            # to keep updated then.
            return

        logger.debug(
            "Beacon notifies entry update",
            beacon_id=beacon_id,
            workspace_path=workspace_path,
            src_id=src_id,
            src_version=src_version,
        )
        event_bus.send("fs.entry.updated", id=src_id)

    event_bus.connect("fs.workspace.loaded", _on_workspace_loaded, weak=True)
    event_bus.connect("fs.workspace.unloaded", _on_workspace_unloaded, weak=True)
    event_bus.connect("backend.beacon.updated", _on_beacon_updated, weak=True)

    await trio.sleep_forever()


def _retreive_workspace_from_beacon(device, fs, beacon_id):
    root_manifest = fs._local_folder_fs.get_manifest(device.user_manifest_access)
    if root_manifest["beacon_id"] == beacon_id:
        return "/"

    # No need to go recursive given workspaces must be direct root children
    for child_name, child_access in root_manifest["children"].items():
        child_manifest = fs._local_folder_fs.get_manifest(child_access)
        if child_manifest.get("beacon_id") == beacon_id:
            return f"/{child_name}"

    return None
