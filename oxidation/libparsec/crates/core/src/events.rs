// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum CoreEvent {
    // Backend
    BackendConnectionChanged,
    BackendMessageReceived,
    BackendPinged,
    BackendRealmMaintenanceFinished,
    BackendRealmMaintenanceStarted,
    BackendRealmRolesUpdated,
    BackendRealmVlobsUpdated,

    // Fs
    FsEntryConfined,
    FsEntryDownsynced,
    FsEntryFileConflictResolved,
    FsEntryMinimalSynced,
    FsEntryRemoteChanged,
    FsEntrySynced,
    FsEntryUpdated,
    FsWorkspaceCreated,
    FsEntrySyncRejectedBySequesterService,
    FsBlockDownloaded,
    FsBlockRemoved,

    // Gui
    GuiConfigChanged,

    // Message
    MessagePinged,

    // Mountpoint
    MountpointReadonly,
    MountpointRemoteError,
    MountpointStarted,
    MountpointStarting,
    MountpointStopped,
    MountpointStopping,
    MountpointTrioDeadlockError,
    MountpointUnhandledError,
    PkiEnrollmentsUpdated,
    SharingUpdated,

    // Userfs
    UserfsSyncRejectedBySequesterService,
    UserfsUpdated,
}

impl CoreEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            CoreEvent::BackendConnectionChanged => "backend.connection.changed",
            CoreEvent::BackendMessageReceived => "backend.message.received",
            CoreEvent::BackendPinged => "backend.pinged",
            CoreEvent::BackendRealmMaintenanceFinished => "backend.realm.maintenance_finished",
            CoreEvent::BackendRealmMaintenanceStarted => "backend.realm.maintenance_started",
            CoreEvent::BackendRealmRolesUpdated => "backend.realm.roles_updated",
            CoreEvent::BackendRealmVlobsUpdated => "backend.realm.vlobs_updated",
            CoreEvent::FsEntryConfined => "fs.entry.confined",
            CoreEvent::FsEntryDownsynced => "fs.entry.downsynced",
            CoreEvent::FsEntryFileConflictResolved => "fs.entry.file_conflict_resolved",
            CoreEvent::FsEntryMinimalSynced => "fs.entry.minimal_synced",
            CoreEvent::FsEntryRemoteChanged => "fs.entry.remote_changed",
            CoreEvent::FsEntrySynced => "fs.entry.synced",
            CoreEvent::FsEntryUpdated => "fs.entry.updated",
            CoreEvent::FsWorkspaceCreated => "fs.workspace.created",
            CoreEvent::FsEntrySyncRejectedBySequesterService => {
                "fs.entry.sync_refused_by_sequester_service"
            }
            CoreEvent::FsBlockDownloaded => "fs.block.downloaded",
            CoreEvent::FsBlockRemoved => "fs.block.removed",
            CoreEvent::GuiConfigChanged => "gui.config.changed",
            CoreEvent::MessagePinged => "message.pinged",
            CoreEvent::MountpointReadonly => "mountpoint.readonly",
            CoreEvent::MountpointRemoteError => "mountpoint.remote_error",
            CoreEvent::MountpointStarted => "mountpoint.started",
            CoreEvent::MountpointStarting => "mountpoint.starting",
            CoreEvent::MountpointStopped => "mountpoint.stopped",
            CoreEvent::MountpointStopping => "mountpoint.stopping",
            CoreEvent::MountpointUnhandledError => "mountpoint.unhandled_error",
            CoreEvent::MountpointTrioDeadlockError => "mountpoint.trio_deadlock_error",
            CoreEvent::PkiEnrollmentsUpdated => "pki_enrollment.updated",
            CoreEvent::SharingUpdated => "sharing.updated",
            CoreEvent::UserfsSyncRejectedBySequesterService => {
                "userfs.sync_refused_by_sequester_service"
            }
            CoreEvent::UserfsUpdated => "userfs.updated",
        }
    }
}
