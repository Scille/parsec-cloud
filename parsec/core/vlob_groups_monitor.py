# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from structlog import get_logger


logger = get_logger()


async def monitor_vlob_groups(device, user_fs, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    user_manifest = user_fs.get_user_manifest()
    for workspace_entry in user_manifest.workspaces:
        event_bus.send("backend.vlob_group.listen", id=workspace_entry.access.id)

    def _on_workspace_created(sender, new_entry):
        event_bus.send("backend.vlob_group.listen", id=new_entry.access.id)

    def _on_vlob_group_updated(sender, id, checkpoint, src_id, src_version):
        # TODO: special event to signify an inbound sync is needed ?
        event_bus.send("fs.entry.updated", id=src_id)

    with event_bus.connect_in_context(
        ("fs.workspace.created", _on_workspace_created),
        ("backend.vlob_group.updated", _on_vlob_group_updated),
    ):

        task_status.started()
        await trio.sleep_forever()
