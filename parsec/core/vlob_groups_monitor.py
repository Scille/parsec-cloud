# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from structlog import get_logger


logger = get_logger()


# We could also mark the local db entries outdated we update occurs to
# only bother about them when they're really needed


async def monitor_vlob_groups(device, fs, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    # TODO: stop using private attribute `fs._local_folder_fs`
    for vlob_groups_id in fs._local_folder_fs.get_local_vlob_groups():
        event_bus.send("backend.vlob_group.listen", id=vlob_groups_id)

    def _on_workspace_loaded(sender, path, id):
        event_bus.send("backend.vlob_group.listen", id=id)

    def _on_workspace_unloaded(sender, path, id):
        event_bus.send("backend.vlob_group.unlisten", id=id)

    def _on_vlob_group_updated(sender, id, checkpoint, src_id, src_version):
        workspace_path = _retreive_workspace_from_vlob_groups(device, fs, id)
        if not workspace_path:
            # This workspace is not present in our local cache, nothing
            # to keep updated then.
            return

        event_bus.send("fs.entry.updated", id=src_id)

    with event_bus.connect_in_context(
        ("fs.workspace.loaded", _on_workspace_loaded),
        ("fs.workspace.unloaded", _on_workspace_unloaded),
        ("backend.vlob_group.updated", _on_vlob_group_updated),
    ):

        task_status.started()
        await trio.sleep_forever()


def _retreive_workspace_from_vlob_groups(device, fs, vlob_groups_id):
    # vlob_groups_id is either the id of the user manifest or of a workpace manifest
    if device.user_manifest_access.id == vlob_groups_id:
        return "/"

    root_manifest = fs._local_folder_fs.get_manifest(device.user_manifest_access)
    # No need to go recursive given workspaces must be direct root children
    for child_name, child_access in root_manifest.children.items():
        if child_access.id == vlob_groups_id:
            return f"/{child_name}"

    return None
