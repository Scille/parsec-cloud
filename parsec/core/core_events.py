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
    FS_ENTRY_CONFINED = "fs.entry.confined"
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


class FSEntryUpdatedReason(Enum):
    ENTRY_RENAME = "ENTRY_RENAME"
    FOLDER_DELETE = "FOLDER_DELETE"
    # Creation of the parent manifest when creating folder, this is the main event
    FOLDER_CREATE = "FOLDER_CREATE"
    # Creation of the child
    FOLDER_CREATE_ENTRY_CREATION = "FOLDER_CREATE_ENTRY_CREATION"
    FILE_DELETE = "FILE_DELETE"
    # Creation of the parent manifest when creating file, this is the main event
    FILE_CREATE = "FILE_CREATE"
    # Creation of the child
    FILE_CREATE_ENTRY_CREATION = "FILE_CREATE_ENTRY_CREATION"
    FILE_WRITE = "FILE_WRITE"
    FILE_RESIZE = "FILE_RESIZE"
    # Sync parent of file, this is the main event
    SYNC_FILE_CONFLICT = "SYNC_FILE_CONFLICT"
    # Sync child
    SYNC_FILE_CONFLICT_ENTRY_CREATION = "SYNC_FILE_CONFLICT_ENTRY_CREATION"
    WORKSPACE_CREATE = "WORKSPACE_CREATE"
    WORKSPACE_RENAME = "WORKSPACE_RENAME"
    PROCESS_LAST_MESSAGES = "PROCESS_LAST_MESSAGES"
