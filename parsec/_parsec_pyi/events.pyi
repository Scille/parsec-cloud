# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

class CoreEvent:
    MESSAGE_PINGED: CoreEvent
    # Backend
    BACKEND_CONNECTION_CHANGED: CoreEvent
    BACKEND_MESSAGE_RECEIVED: CoreEvent
    BACKEND_PINGED: CoreEvent
    BACKEND_REALM_MAINTENANCE_FINISHED: CoreEvent
    BACKEND_REALM_MAINTENANCE_STARTED: CoreEvent
    BACKEND_REALM_ROLES_UPDATED: CoreEvent
    BACKEND_REALM_VLOBS_UPDATED: CoreEvent
    # Fs
    FS_ENTRY_REMOTE_CHANGED: CoreEvent
    FS_ENTRY_SYNCED: CoreEvent
    FS_ENTRY_DOWNSYNCED: CoreEvent
    FS_ENTRY_MINIMAL_SYNCED: CoreEvent
    FS_ENTRY_CONFINED: CoreEvent
    FS_ENTRY_UPDATED: CoreEvent
    FS_ENTRY_FILE_CONFLICT_RESOLVED: CoreEvent
    FS_WORKSPACE_CREATED: CoreEvent
    FS_ENTRY_SYNC_REJECTED_BY_SEQUESTER_SERVICE: CoreEvent
    USERFS_SYNC_REJECTED_BY_SEQUESTER_SERVICE: CoreEvent
    # Gui
    GUI_CONFIG_CHANGED: CoreEvent
    # Mountpoint
    MOUNTPOINT_REMOTE_ERROR: CoreEvent
    MOUNTPOINT_STARTING: CoreEvent
    MOUNTPOINT_STARTED: CoreEvent
    MOUNTPOINT_STOPPING: CoreEvent
    MOUNTPOINT_STOPPED: CoreEvent
    MOUNTPOINT_READONLY: CoreEvent
    MOUNTPOINT_UNHANDLED_ERROR: CoreEvent
    MOUNTPOINT_TRIO_DEADLOCK_ERROR: CoreEvent
    # Others
    SHARING_UPDATED: CoreEvent
    USERFS_UPDATED: CoreEvent
    # Pki enrollment
    PKI_ENROLLMENTS_UPDATED: CoreEvent

    @property
    def value(self) -> str: ...
