# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from enum import Enum


class CoreEvent(Enum):
    pinged = "message.pinged"
    # Backend
    backend_connection_changed = "backend.connection.changed"
    backend_message_received = "backend.message.received"
    backend_pinged = "backend.pinged"
    backend_realm_maintenance_finished = "backend.realm.maintenance_finished"
    backend_realm_maintenance_started = "backend.realm.maintenance_started"
    backend_realm_roles_updated = "backend.realm.roles_updated"
    backend_realm_vlobs_updated = "backend.realm.vlobs_updated"
    # Fs
    fs_entry_remote_changed = "fs.entry.remote_changed"
    fs_entry_synced = "fs.entry.synced"
    fs_entry_downsynced = "fs.entry.downsynced"
    fs_entry_minimal_synced = "fs.entry.minimal_synced"
    fs_entry_updated = "fs.entry.updated"
    fs_entry_file_conflict_resolved = "fs.entry.file_conflict_resolved"
    fs_entry_file_update_conflicted = "fs.entry.file_update_conflicted"
    fs_workspace_created = "fs.workspace.created"
    # Gui
    gui_config_changed = "gui.config.changed"
    # Mountpoint
    mountpoint_remote_error = "mountpoint.remote_error"
    mountpoint_started = "mountpoint.started"
    mountpoint_starting = "mountpoint.starting"
    mountpoint_stopped = "mountpoint.stopped"
    mountpoint_unhandled_error = "mountpoint.unhandled_error"
    # Others
    sharing_updated = "sharing.updated"
    userfs_updated = "userfs.updated"
