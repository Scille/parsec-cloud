# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from structlog import get_logger


logger = get_logger()


async def monitor_realms(device, user_fs, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    def _on_listener_started(sender):
        event_bus.send("backend.realm.listen", realm_id=device.user_manifest_access.id)
        user_manifest = user_fs.get_user_manifest()
        for workspace_entry in user_manifest.workspaces:
            event_bus.send("backend.realm.listen", realm_id=workspace_entry.access.id)

    def _on_workspace_created(sender, new_entry):
        event_bus.send("backend.realm.listen", realm_id=new_entry.access.id)

    with event_bus.connect_in_context(
        ("backend.listener.started", _on_listener_started),
        ("fs.workspace.created", _on_workspace_created),
    ):

        task_status.started()
        await trio.sleep_forever()
