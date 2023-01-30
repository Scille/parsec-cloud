// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyBytes, PyTuple, PyType},
    IntoPy, PyObject, PyResult, Python,
};

use libparsec::{
    client_types,
    protocol::authenticated_cmds::v2::{invite_delete, invite_new},
};

use crate::protocol::{ProtocolErrorFields, ProtocolResult};

// #[non_exhaustive] macro must be set for every enum,
// because we would like to call `is` in `python`, then
// a static reference should be returned instead of a new object

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct ClientType(pub client_types::ClientType);

crate::binding_utils::gen_proto!(ClientType, __repr__);
crate::binding_utils::gen_proto!(ClientType, __copy__);
crate::binding_utils::gen_proto!(ClientType, __deepcopy__);
crate::binding_utils::gen_proto!(ClientType, __richcmp__, eq);
crate::binding_utils::gen_proto!(ClientType, __hash__);

crate::binding_utils::impl_enum_field!(
    ClientType,
    [
        "AUTHENTICATED",
        authenticated,
        client_types::ClientType::Authenticated
    ],
    ["INVITED", invited, client_types::ClientType::Invited],
    ["ANONYMOUS", anonymous, client_types::ClientType::Anonymous]
);

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct InvitationDeletedReason(pub invite_delete::InvitationDeletedReason);

crate::binding_utils::gen_proto!(InvitationDeletedReason, __repr__);
crate::binding_utils::gen_proto!(InvitationDeletedReason, __copy__);
crate::binding_utils::gen_proto!(InvitationDeletedReason, __deepcopy__);
crate::binding_utils::gen_proto!(InvitationDeletedReason, __richcmp__, eq);

crate::binding_utils::impl_enum_field!(
    InvitationDeletedReason,
    [
        "FINISHED",
        finished,
        invite_delete::InvitationDeletedReason::Finished
    ],
    [
        "CANCELLED",
        cancelled,
        invite_delete::InvitationDeletedReason::Cancelled
    ],
    [
        "ROTTEN",
        rotten,
        invite_delete::InvitationDeletedReason::Rotten
    ]
);

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct InvitationEmailSentStatus(pub invite_new::InvitationEmailSentStatus);

crate::binding_utils::gen_proto!(InvitationEmailSentStatus, __repr__);
crate::binding_utils::gen_proto!(InvitationEmailSentStatus, __copy__);
crate::binding_utils::gen_proto!(InvitationEmailSentStatus, __deepcopy__);
crate::binding_utils::gen_proto!(InvitationEmailSentStatus, __richcmp__, eq);

crate::binding_utils::impl_enum_field!(
    InvitationEmailSentStatus,
    [
        "SUCCESS",
        success,
        invite_new::InvitationEmailSentStatus::Success
    ],
    [
        "NOT_AVAILABLE",
        not_available,
        invite_new::InvitationEmailSentStatus::NotAvailable
    ],
    [
        "BAD_RECIPIENT",
        bad_recipient,
        invite_new::InvitationEmailSentStatus::BadRecipient
    ]
);

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct InvitationStatus(pub libparsec::types::InvitationStatus);

crate::binding_utils::gen_proto!(InvitationStatus, __repr__);
crate::binding_utils::gen_proto!(InvitationStatus, __copy__);
crate::binding_utils::gen_proto!(InvitationStatus, __deepcopy__);
crate::binding_utils::gen_proto!(InvitationStatus, __richcmp__, eq);
crate::binding_utils::gen_proto!(InvitationStatus, __hash__);

crate::binding_utils::impl_enum_field!(
    InvitationStatus,
    ["IDLE", idle, libparsec::types::InvitationStatus::Idle],
    ["READY", ready, libparsec::types::InvitationStatus::Ready],
    [
        "DELETED",
        deleted,
        libparsec::types::InvitationStatus::Deleted
    ]
);

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct InvitationType(pub libparsec::types::InvitationType);

crate::binding_utils::gen_proto!(InvitationType, __repr__);
crate::binding_utils::gen_proto!(InvitationType, __copy__);
crate::binding_utils::gen_proto!(InvitationType, __deepcopy__);
crate::binding_utils::gen_proto!(InvitationType, __richcmp__, eq);
crate::binding_utils::gen_proto!(InvitationType, __hash__);

crate::binding_utils::impl_enum_field!(
    InvitationType,
    ["DEVICE", device, libparsec::types::InvitationType::Device],
    ["USER", user, libparsec::types::InvitationType::User]
);

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct RealmRole(pub libparsec::types::RealmRole);

crate::binding_utils::gen_proto!(RealmRole, __repr__);
crate::binding_utils::gen_proto!(RealmRole, __copy__);
crate::binding_utils::gen_proto!(RealmRole, __deepcopy__);
crate::binding_utils::gen_proto!(RealmRole, __richcmp__, eq);
crate::binding_utils::gen_proto!(RealmRole, __hash__);

crate::binding_utils::impl_enum_field!(
    RealmRole,
    ["OWNER", owner, libparsec::types::RealmRole::Owner],
    ["MANAGER", manager, libparsec::types::RealmRole::Manager],
    [
        "CONTRIBUTOR",
        contributor,
        libparsec::types::RealmRole::Contributor
    ],
    ["READER", reader, libparsec::types::RealmRole::Reader]
);

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct UserProfile(pub libparsec::types::UserProfile);

crate::binding_utils::gen_proto!(UserProfile, __repr__);
crate::binding_utils::gen_proto!(UserProfile, __copy__);
crate::binding_utils::gen_proto!(UserProfile, __deepcopy__);
crate::binding_utils::gen_proto!(UserProfile, __richcmp__, eq);
crate::binding_utils::gen_proto!(UserProfile, __hash__);

crate::binding_utils::impl_enum_field!(
    UserProfile,
    ["ADMIN", admin, libparsec::types::UserProfile::Admin],
    [
        "STANDARD",
        standard,
        libparsec::types::UserProfile::Standard
    ],
    [
        "OUTSIDER",
        outsider,
        libparsec::types::UserProfile::Outsider
    ]
);

impl UserProfile {
    pub(crate) fn from_profile(profile: libparsec::types::UserProfile) -> &'static PyObject {
        match profile {
            libparsec::types::UserProfile::Admin => UserProfile::admin(),
            libparsec::types::UserProfile::Standard => UserProfile::standard(),
            libparsec::types::UserProfile::Outsider => UserProfile::outsider(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct CoreEvent(libparsec::core::CoreEvent);

crate::binding_utils::impl_enum_field!(
    CoreEvent,
    [
        "BACKEND_CONNECTION_CHANGED",
        backend_connection_changed,
        libparsec::core::CoreEvent::BackendConnectionChanged
    ],
    [
        "MESSAGE_PINGED",
        message_pinged,
        libparsec::core::CoreEvent::MessagePinged
    ],
    [
        "BACKEND_MESSAGE_RECEIVED",
        backend_message_received,
        libparsec::core::CoreEvent::BackendMessageReceived
    ],
    [
        "BACKEND_PINGED",
        backend_pinged,
        libparsec::core::CoreEvent::BackendPinged
    ],
    [
        "BACKEND_REALM_MAINTENANCE_FINISHED",
        backend_realm_maintenance_finished,
        libparsec::core::CoreEvent::BackendRealmMaintenanceFinished
    ],
    [
        "BACKEND_REALM_MAINTENANCE_STARTED",
        backend_realm_maintenance_started,
        libparsec::core::CoreEvent::BackendRealmMaintenanceStarted
    ],
    [
        "BACKEND_REALM_ROLES_UPDATED",
        backend_realm_roles_updated,
        libparsec::core::CoreEvent::BackendRealmRolesUpdated
    ],
    [
        "BACKEND_REALM_VLOBS_UPDATED",
        backend_realm_vlobs_updated,
        libparsec::core::CoreEvent::BackendRealmVlobsUpdated
    ],
    [
        "FS_ENTRY_REMOTE_CHANGED",
        fs_entry_remote_changed,
        libparsec::core::CoreEvent::FsEntryRemoteChanged
    ],
    [
        "FS_ENTRY_SYNCED",
        fs_entry_synced,
        libparsec::core::CoreEvent::FsEntrySynced
    ],
    [
        "FS_ENTRY_DOWNSYNCED",
        fs_entry_downsynced,
        libparsec::core::CoreEvent::FsEntryDownsynced
    ],
    [
        "FS_ENTRY_MINIMAL_SYNCED",
        fs_entry_minimal_synced,
        libparsec::core::CoreEvent::FsEntryMinimalSynced
    ],
    [
        "FS_ENTRY_CONFINED",
        fs_entry_confined,
        libparsec::core::CoreEvent::FsEntryConfined
    ],
    [
        "FS_ENTRY_UPDATED",
        fs_entry_updated,
        libparsec::core::CoreEvent::FsEntryUpdated
    ],
    [
        "FS_ENTRY_FILE_CONFLICT_RESOLVED",
        fs_entry_file_conflict_resolved,
        libparsec::core::CoreEvent::FsEntryFileConflictResolved
    ],
    [
        "FS_ENTRY_SYNC_REJECTED_BY_SEQUESTER_SERVICE",
        fs_entry_sync_rejected_by_sequester_service,
        libparsec::core::CoreEvent::FsEntrySyncRejectedBySequesterService
    ],
    [
        "FS_WORKSPACE_CREATED",
        fs_workspace_created,
        libparsec::core::CoreEvent::FsWorkspaceCreated
    ],
    [
        "FS_BLOCK_DOWNLOADED",
        fs_block_downloaded,
        libparsec::core::CoreEvent::FsBlockDownloaded
    ],
    [
        "FS_BLOCK_PURGED",
        fs_block_purged,
        libparsec::core::CoreEvent::FsBlockPurged
    ],
    [
        "USERFS_SYNC_REJECTED_BY_SEQUESTER_SERVICE",
        userfs_sync_rejected_by_sequester_service,
        libparsec::core::CoreEvent::UserfsSyncRejectedBySequesterService
    ],
    [
        "GUI_CONFIG_CHANGED",
        gui_config_changed,
        libparsec::core::CoreEvent::GuiConfigChanged
    ],
    [
        "MOUNTPOINT_REMOTE_ERROR",
        mountpoint_remote_error,
        libparsec::core::CoreEvent::MountpointRemoteError
    ],
    [
        "MOUNTPOINT_STARTING",
        mountpoint_starting,
        libparsec::core::CoreEvent::MountpointStarting
    ],
    [
        "MOUNTPOINT_STARTED",
        mountpoint_started,
        libparsec::core::CoreEvent::MountpointStarted
    ],
    [
        "MOUNTPOINT_STOPPING",
        mountpoint_stopping,
        libparsec::core::CoreEvent::MountpointStopping
    ],
    [
        "MOUNTPOINT_STOPPED",
        mountpoint_stopped,
        libparsec::core::CoreEvent::MountpointStopped
    ],
    [
        "MOUNTPOINT_READONLY",
        mountpoint_readonly,
        libparsec::core::CoreEvent::MountpointReadonly
    ],
    [
        "MOUNTPOINT_UNHANDLED_ERROR",
        mountpoint_unhandled_error,
        libparsec::core::CoreEvent::MountpointUnhandledError
    ],
    [
        "MOUNTPOINT_TRIO_DEADLOCK_ERROR",
        mountpoint_trio_deadlock_error,
        libparsec::core::CoreEvent::MountpointTrioDeadlockError
    ],
    [
        "SHARING_UPDATED",
        sharing_updated,
        libparsec::core::CoreEvent::SharingUpdated
    ],
    [
        "USERFS_UPDATED",
        userfs_updated,
        libparsec::core::CoreEvent::UserfsUpdated
    ],
    [
        "PKI_ENROLLMENTS_UPDATED",
        pki_enrollments_updated,
        libparsec::core::CoreEvent::PkiEnrollmentsUpdated
    ]
);

crate::binding_utils::gen_proto!(CoreEvent, __hash__);
crate::binding_utils::gen_proto!(CoreEvent, __repr__);
crate::binding_utils::gen_proto!(CoreEvent, __copy__);
crate::binding_utils::gen_proto!(CoreEvent, __deepcopy__);
crate::binding_utils::gen_proto!(CoreEvent, __richcmp__, eq);

#[pyclass]
#[derive(Clone)]
#[non_exhaustive]
pub(crate) struct DeviceFileType(pub client_types::DeviceFileType);

crate::binding_utils::impl_enum_field!(
    DeviceFileType,
    ["PASSWORD", password, client_types::DeviceFileType::Password],
    [
        "SMARTCARD",
        smartcard,
        client_types::DeviceFileType::Smartcard
    ],
    ["RECOVERY", recovery, client_types::DeviceFileType::Recovery]
);

#[pymethods]
impl DeviceFileType {
    pub fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[classmethod]
    pub fn load(_cls: &PyType, bytes: &[u8]) -> PyResult<Self> {
        Ok(Self(client_types::DeviceFileType::load(bytes).map_err(
            |_| PyValueError::new_err("Failed to deserialize"),
        )?))
    }
}

crate::binding_utils::gen_proto!(DeviceFileType, __hash__);
crate::binding_utils::gen_proto!(DeviceFileType, __repr__);
crate::binding_utils::gen_proto!(DeviceFileType, __copy__);
crate::binding_utils::gen_proto!(DeviceFileType, __deepcopy__);
crate::binding_utils::gen_proto!(DeviceFileType, __richcmp__, eq);
