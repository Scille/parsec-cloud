# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from structlog import get_logger


logger = get_logger()


# We could also mark the local db entries outdated we update occurs to
# only bother about them when they're really needed


async def monitor_beacons(device, fs, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    # TODO: stop using private attribute `fs._local_folder_fs`
    for beacon_id in fs._local_folder_fs.get_local_beacons():
        event_bus.send("backend.beacon.listen", beacon_id=beacon_id)

    def _on_workspace_loaded(sender, path, id):
        event_bus.send("backend.beacon.listen", beacon_id=id)

    def _on_workspace_unloaded(sender, path, id):
        event_bus.send("backend.beacon.unlisten", beacon_id=id)

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

    task_status.started()
    await trio.sleep_forever()


def _retreive_workspace_from_beacon(device, fs, beacon_id):
    # beacon_id is either the id of the user manifest or of a workpace manifest
    if device.user_manifest_access.id == beacon_id:
        return "/"

    root_manifest = fs._local_folder_fs.get_manifest(device.user_manifest_access)
    # No need to go recursive given workspaces must be direct root children
    for child_name, child_access in root_manifest.children.items():
        if child_access.id == beacon_id:
            return f"/{child_name}"

    return None
