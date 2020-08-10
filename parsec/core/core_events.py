# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from enum import Enum


class CoreEvent(Enum):
    MESSAGE_PINGED = "message.pinged"
    # Backend
    BACKEND_CONNECTION_CHANGED = "backend.connection.changed"
    BACKEND_MESSAGE_RECEIVED = "backend.message.received"
    BACKEND_PINGED = "backend.pinged"
    BACKEND_REALM_MAINTENANCE_FINISHED = "backend.realm.maintenance_finished"
    BACKEND_REALM_MAINTENANCE_STARTED = "backend.realm.maintenance_started"
    BACKEND_REALM_ROLES_UPDATED = "backend.realm.roles_updated"
    BACKEND_REALM_VLOBS_UPDATED = "backend.realm.vlobs_updated"
    # Fs
    FS_ENTRY_REMOTE_CHANGED = "fs.entry.remote_changed"
    FS_ENTRY_SYNCED = "fs.entry.synced"
    FS_ENTRY_DOWNSYNCED = "fs.entry.downsynced"
    FS_ENTRY_MINIMAL_SYNCED = "fs.entry.minimal_synced"
    FS_ENTRY_UPDATED = "fs.entry.updated"
    FS_ENTRY_FILE_CONFLICT_RESOLVED = "fs.entry.file_conflict_resolved"
    FS_ENTRY_FILE_UPDATE_CONFLICTED = "fs.entry.file_update_conflicted"
    FS_WORKSPACE_CREATED = "fs.workspace.created"
    # Gui
    GUI_CONFIG_CHANGED = "gui.config.changed"
    # Mountpoint
    MOUNTPOINT_REMOTE_ERROR = "mountpoint.remote_error"
    MOUNTPOINT_STARTED = "mountpoint.started"
    MOUNTPOINT_STARTING = "mountpoint.starting"
    MOUNTPOINT_STOPPED = "mountpoint.stopped"
    MOUNTPOINT_UNHANDLED_ERROR = "mountpoint.unhandled_error"
    # Others
    SHARING_UPDATED = "sharing.updated"
    USERFS_UPDATED = "userfs.updated"
