use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyTuple, PyType},
    IntoPy, PyObject, PyResult, Python,
};

use libparsec::client_types;
use libparsec::protocol::authenticated_cmds::v2::{invite_delete, invite_new};

#[pyclass]
#[derive(Clone)]
pub(crate) struct ClientType(pub client_types::ClientType);

crate::binding_utils::gen_proto!(ClientType, __repr__);
crate::binding_utils::gen_proto!(ClientType, __richcmp__, eq);
crate::binding_utils::gen_proto!(ClientType, __hash__);

#[pymethods]
impl ClientType {
    #[classattr]
    #[pyo3(name = "AUTHENTICATED")]
    fn authenticated() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     ClientType(client_types::ClientType::Authenticated).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "INVITED")]
    fn invited() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     ClientType(client_types::ClientType::Invited).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "ANONYMOUS")]
    fn anonymous() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     ClientType(client_types::ClientType::Anonymous).into_py(py)
                })
            };
        };

        &VALUE
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitationDeletedReason(pub invite_delete::InvitationDeletedReason);

crate::binding_utils::gen_proto!(InvitationDeletedReason, __repr__);
crate::binding_utils::gen_proto!(InvitationDeletedReason, __richcmp__, eq);

#[pymethods]
impl InvitationDeletedReason {
    #[classattr]
    #[pyo3(name = "FINISHED")]
    fn finished() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationDeletedReason(invite_delete::InvitationDeletedReason::Finished).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "CANCELLED")]
    fn cancelled() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationDeletedReason(invite_delete::InvitationDeletedReason::Cancelled).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "ROTTEN")]
    fn rotten() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationDeletedReason(invite_delete::InvitationDeletedReason::Rotten).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "VALUES")]
    fn values() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUES: PyObject = {
                Python::with_gil(|py| {
                    PyTuple::new(py, [
                        InvitationDeletedReason::finished(),
                        InvitationDeletedReason::cancelled(),
                        InvitationDeletedReason::rotten()
                    ]).into_py(py)
                })
            };
        };

        &VALUES
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<Self> {
        match value {
            "FINISHED" => Ok(Self(invite_delete::InvitationDeletedReason::Finished)),
            "CANCELLED" => Ok(Self(invite_delete::InvitationDeletedReason::Cancelled)),
            "ROTTEN" => Ok(Self(invite_delete::InvitationDeletedReason::Rotten)),
            _ => Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
        }
    }

    #[getter]
    fn str(&self) -> &str {
        match self.0 {
            invite_delete::InvitationDeletedReason::Finished => "FINISHED",
            invite_delete::InvitationDeletedReason::Cancelled => "CANCELLED",
            invite_delete::InvitationDeletedReason::Rotten => "ROTTEN",
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitationEmailSentStatus(pub invite_new::InvitationEmailSentStatus);

crate::binding_utils::gen_proto!(InvitationEmailSentStatus, __repr__);
crate::binding_utils::gen_proto!(InvitationEmailSentStatus, __richcmp__, eq);

#[pymethods]
impl InvitationEmailSentStatus {
    #[classattr]
    #[pyo3(name = "SUCCESS")]
    fn success() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationEmailSentStatus(invite_new::InvitationEmailSentStatus::Success).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "NOT_AVAILABLE")]
    fn not_available() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationEmailSentStatus(invite_new::InvitationEmailSentStatus::NotAvailable).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "BAD_RECIPIENT")]
    fn bad_recipient() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationEmailSentStatus(invite_new::InvitationEmailSentStatus::BadRecipient).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "VALUES")]
    fn values() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUES: PyObject = {
                Python::with_gil(|py| {
                    PyTuple::new(
                        py,
                        [
                            InvitationEmailSentStatus::success(),
                            InvitationEmailSentStatus::not_available(),
                            InvitationEmailSentStatus::bad_recipient(),
                        ],
                    ).into_py(py)
                })
            };
        };

        &VALUES
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<Self> {
        match value {
            "SUCCESS" => Ok(Self(invite_new::InvitationEmailSentStatus::Success)),
            "NOT_AVAILABLE" => Ok(Self(invite_new::InvitationEmailSentStatus::NotAvailable)),
            "BAD_RECIPIENT" => Ok(Self(invite_new::InvitationEmailSentStatus::BadRecipient)),
            _ => Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
        }
    }

    #[getter]
    fn str(&self) -> &str {
        match self.0 {
            invite_new::InvitationEmailSentStatus::Success => "SUCCESS",
            invite_new::InvitationEmailSentStatus::NotAvailable => "NOT_AVAILABLE",
            invite_new::InvitationEmailSentStatus::BadRecipient => "BAD_RECIPIENT",
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitationStatus(pub libparsec::types::InvitationStatus);

crate::binding_utils::gen_proto!(InvitationStatus, __repr__);
crate::binding_utils::gen_proto!(InvitationStatus, __richcmp__, eq);
crate::binding_utils::gen_proto!(InvitationStatus, __hash__);

#[pymethods]
impl InvitationStatus {
    #[classattr]
    #[pyo3(name = "IDLE")]
    fn idle() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationStatus(libparsec::types::InvitationStatus::Idle).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "READY")]
    fn ready() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationStatus(libparsec::types::InvitationStatus::Ready).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "DELETED")]
    fn deleted() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationStatus(libparsec::types::InvitationStatus::Deleted).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "VALUES")]
    fn values() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUES: PyObject = {
                Python::with_gil(|py| {
                    PyTuple::new(
                        py,
                        [
                            InvitationStatus::idle(),
                            InvitationStatus::ready(),
                            InvitationStatus::deleted()
                        ]
                    ).into_py(py)
                })
            };
        };

        &VALUES
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<&'static PyObject> {
        match value {
            "IDLE" => Ok(Self::idle()),
            "READY" => Ok(Self::ready()),
            "DELETED" => Ok(Self::deleted()),
            _ => Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
        }
    }

    #[getter]
    fn str(&self) -> &str {
        match self.0 {
            libparsec::types::InvitationStatus::Idle => "IDLE",
            libparsec::types::InvitationStatus::Ready => "READY",
            libparsec::types::InvitationStatus::Deleted => "DELETED",
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitationType(pub libparsec::types::InvitationType);

crate::binding_utils::gen_proto!(InvitationType, __repr__);
crate::binding_utils::gen_proto!(InvitationType, __richcmp__, eq);
crate::binding_utils::gen_proto!(InvitationType, __hash__);

#[pymethods]
impl InvitationType {
    #[classattr]
    #[pyo3(name = "DEVICE")]
    fn device() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationType(libparsec::types::InvitationType::Device).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classattr]
    #[pyo3(name = "USER")]
    fn user() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationType(libparsec::types::InvitationType::User).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classattr]
    #[pyo3(name = "VALUES")]
    fn values() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUES: PyObject = {
                Python::with_gil(|py| {
                    PyTuple::new(py, [InvitationType::device(), InvitationType::user()])
                        .into_py(py)
                })
            };
        };

        &VALUES
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<&'static PyObject> {
        match value {
            "DEVICE" => Ok(Self::device()),
            "USER" => Ok(Self::user()),
            _ => Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
        }
    }

    #[getter]
    fn str(&self) -> &str {
        match self.0 {
            libparsec::types::InvitationType::Device => "DEVICE",
            libparsec::types::InvitationType::User => "USER",
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct RealmRole(pub libparsec::types::RealmRole);

crate::binding_utils::gen_proto!(RealmRole, __repr__);
crate::binding_utils::gen_proto!(RealmRole, __richcmp__, eq);
crate::binding_utils::gen_proto!(RealmRole, __hash__);

#[pymethods]
impl RealmRole {
    #[classattr]
    #[pyo3(name = "OWNER")]
    fn owner() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     RealmRole(libparsec::types::RealmRole::Owner).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "MANAGER")]
    fn manager() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     RealmRole(libparsec::types::RealmRole::Manager).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "CONTRIBUTOR")]
    fn contributor() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     RealmRole(libparsec::types::RealmRole::Contributor).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "READER")]
    fn reader() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     RealmRole(libparsec::types::RealmRole::Reader).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "VALUES")]
    fn values() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUES: PyObject = {
                Python::with_gil(|py| {
                    PyTuple::new(
                        py,
                        [
                            RealmRole::owner(),
                            RealmRole::manager(),
                            RealmRole::contributor(),
                            RealmRole::reader(),
                        ],
                    ).into_py(py)
                })
            };
        };

        &VALUES
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<&'static PyObject> {
        match value {
            "OWNER" => Ok(Self::owner()),
            "MANAGER" => Ok(Self::manager()),
            "CONTRIBUTOR" => Ok(Self::contributor()),
            "READER" => Ok(Self::reader()),
            _ => Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
        }
    }

    #[getter]
    fn str(&self) -> &str {
        match self.0 {
            libparsec::types::RealmRole::Owner => "OWNER",
            libparsec::types::RealmRole::Manager => "MANAGER",
            libparsec::types::RealmRole::Contributor => "CONTRIBUTOR",
            libparsec::types::RealmRole::Reader => "READER",
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct UserProfile(pub libparsec::types::UserProfile);

crate::binding_utils::gen_proto!(UserProfile, __repr__);
crate::binding_utils::gen_proto!(UserProfile, __richcmp__, eq);
crate::binding_utils::gen_proto!(UserProfile, __hash__);

#[pymethods]
impl UserProfile {
    #[classattr]
    #[pyo3(name = "ADMIN")]
    pub fn admin() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    UserProfile(libparsec::types::UserProfile::Admin).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classattr]
    #[pyo3(name = "STANDARD")]
    pub fn standard() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    UserProfile(libparsec::types::UserProfile::Standard).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classattr]
    #[pyo3(name = "OUTSIDER")]
    pub fn outsider() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    UserProfile(libparsec::types::UserProfile::Outsider).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classattr]
    #[pyo3(name = "VALUES")]
    fn values() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUES: PyObject = {
                Python::with_gil(|py| {
                    PyTuple::new(
                        py,
                        [
                            UserProfile::admin(),
                            UserProfile::standard(),
                            UserProfile::outsider()
                        ]
                    )
                    .into_py(py)
                })
            };
        };

        &VALUES
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<&'static PyObject> {
        Ok(match value {
            "ADMIN" => Self::admin(),
            "STANDARD" => Self::standard(),
            "OUTSIDER" => Self::outsider(),
            _ => return Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
        })
    }

    #[getter]
    fn str(&self) -> &str {
        match self.0 {
            libparsec::types::UserProfile::Admin => "ADMIN",
            libparsec::types::UserProfile::Standard => "STANDARD",
            libparsec::types::UserProfile::Outsider => "OUTSIDER",
        }
    }
}

impl UserProfile {
    pub fn from_profile(profile: libparsec::types::UserProfile) -> &'static PyObject {
        match profile {
            libparsec::types::UserProfile::Admin => UserProfile::admin(),
            libparsec::types::UserProfile::Standard => UserProfile::standard(),
            libparsec::types::UserProfile::Outsider => UserProfile::outsider(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
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

#[pymethods]
impl CoreEvent {
    #[getter]
    fn value(&self) -> &'static str {
        self.0.as_str()
    }
}

crate::binding_utils::gen_proto!(CoreEvent, __hash__);
crate::binding_utils::gen_proto!(CoreEvent, __repr__);
crate::binding_utils::gen_proto!(CoreEvent, __richcmp__, eq);
