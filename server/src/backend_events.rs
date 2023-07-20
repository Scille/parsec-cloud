// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{exceptions::PyValueError, prelude::*, types::PyBytes};
use serde::{Deserialize, Serialize};

use crate::{
    BytesWrapper, DeviceID, InvitationStatus, InvitationToken, OrganizationID, RealmID, RealmRole,
    UserID, UserProfile, VlobID,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
enum RawBackendEvent {
    #[serde(rename = "certificates.updated")]
    CertificatesUpdated {
        organization_id: libparsec::low_level::types::OrganizationID,
        index: libparsec::low_level::types::IndexInt,
    },
    #[serde(rename = "invite.conduit_updated")]
    InviteConduitUpdated {
        organization_id: libparsec::low_level::types::OrganizationID,
        token: libparsec::low_level::types::InvitationToken,
    },
    #[serde(rename = "user.profile_updated_or_revoked")]
    UserProfileUpdatedOrRevoked {
        organization_id: libparsec::low_level::types::OrganizationID,
        user_id: libparsec::low_level::types::UserID,
        profile: Option<libparsec::low_level::types::UserProfile>,
    },
    #[serde(rename = "organization.expired")]
    OrganizationExpired {
        organization_id: libparsec::low_level::types::OrganizationID,
    },
    #[serde(rename = "pinged")]
    Pinged {
        organization_id: libparsec::low_level::types::OrganizationID,
        author: libparsec::low_level::types::DeviceID,
        ping: String,
    },
    #[serde(rename = "message.received")]
    MessageReceived {
        organization_id: libparsec::low_level::types::OrganizationID,
        author: libparsec::low_level::types::DeviceID,
        recipient: libparsec::low_level::types::UserID,
        index: u64,
        message: Vec<u8>,
    },
    #[serde(rename = "invite.status_changed")]
    InviteStatusChanged {
        organization_id: libparsec::low_level::types::OrganizationID,
        greeter: libparsec::low_level::types::UserID,
        token: libparsec::low_level::types::InvitationToken,
        status: libparsec::low_level::types::InvitationStatus,
    },
    #[serde(rename = "realm.maintenance_finished")]
    RealmMaintenanceFinished {
        organization_id: libparsec::low_level::types::OrganizationID,
        author: libparsec::low_level::types::DeviceID,
        realm_id: libparsec::low_level::types::RealmID,
        encryption_revision: u64,
    },
    #[serde(rename = "realm.maintenance_started")]
    RealmMaintenanceStarted {
        organization_id: libparsec::low_level::types::OrganizationID,
        author: libparsec::low_level::types::DeviceID,
        realm_id: libparsec::low_level::types::RealmID,
        encryption_revision: u64,
    },
    #[serde(rename = "realm.vlobs_updated")]
    RealmVlobsUpdated {
        organization_id: libparsec::low_level::types::OrganizationID,
        author: libparsec::low_level::types::DeviceID,
        realm_id: libparsec::low_level::types::RealmID,
        checkpoint: u64,
        src_id: libparsec::low_level::types::VlobID,
        src_version: u64,
    },
    #[serde(rename = "realm.roles_updated")]
    RealmRolesUpdated {
        organization_id: libparsec::low_level::types::OrganizationID,
        author: libparsec::low_level::types::DeviceID,
        realm_id: libparsec::low_level::types::RealmID,
        user: libparsec::low_level::types::UserID,
        role: Option<libparsec::low_level::types::RealmRole>,
    },
    #[serde(rename = "pki_enrollment.updated")]
    PkiEnrollmentUpdated {
        organization_id: libparsec::low_level::types::OrganizationID,
    },
}

#[pyclass(subclass)]
#[derive(Debug, Clone)]
pub(crate) struct BackendEvent(RawBackendEvent);

crate::binding_utils::gen_proto!(BackendEvent, __repr__);
crate::binding_utils::gen_proto!(BackendEvent, __copy__);
crate::binding_utils::gen_proto!(BackendEvent, __deepcopy__);
crate::binding_utils::gen_proto!(BackendEvent, __richcmp__, eq);

#[pymethods]
impl BackendEvent {
    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        match ::rmp_serde::to_vec_named(&self.0) {
            Ok(buff) => Ok(PyBytes::new(py, &buff)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    #[staticmethod]
    fn load(py: Python, raw: &[u8]) -> PyResult<PyObject> {
        match ::rmp_serde::from_slice::<RawBackendEvent>(raw) {
            Ok(obj) => Ok(match &obj {
                RawBackendEvent::CertificatesUpdated { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventCertificatesUpdated);
                    Py::new(py, init)?.into_py(py)
                }
                RawBackendEvent::InviteConduitUpdated { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventInviteConduitUpdated);
                    Py::new(py, init)?.into_py(py)
                }
                RawBackendEvent::UserProfileUpdatedOrRevoked { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventUserUpdatedOrRevoked);
                    Py::new(py, init)?.into_py(py)
                }
                RawBackendEvent::OrganizationExpired { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventOrganizationExpired);
                    Py::new(py, init)?.into_py(py)
                }
                RawBackendEvent::Pinged { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventPinged);
                    Py::new(py, init)?.into_py(py)
                }
                RawBackendEvent::MessageReceived { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventMessageReceived);
                    Py::new(py, init)?.into_py(py)
                }
                RawBackendEvent::InviteStatusChanged { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventInviteStatusChanged);
                    Py::new(py, init)?.into_py(py)
                }
                RawBackendEvent::RealmMaintenanceFinished { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventRealmMaintenanceFinished);
                    Py::new(py, init)?.into_py(py)
                }
                RawBackendEvent::RealmMaintenanceStarted { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventRealmMaintenanceStarted);
                    Py::new(py, init)?.into_py(py)
                }
                RawBackendEvent::RealmVlobsUpdated { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventRealmVlobsUpdated);
                    Py::new(py, init)?.into_py(py)
                }
                RawBackendEvent::RealmRolesUpdated { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventRealmRolesUpdated);
                    Py::new(py, init)?.into_py(py)
                }
                RawBackendEvent::PkiEnrollmentUpdated { .. } => {
                    let init = PyClassInitializer::from(BackendEvent(obj));
                    let init = init.add_subclass(BackendEventPkiEnrollmentUpdated);
                    Py::new(py, init)?.into_py(py)
                }
            }),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    #[getter]
    fn organization_id(&self) -> OrganizationID {
        let organization_id = match &self.0 {
            RawBackendEvent::CertificatesUpdated {
                organization_id, ..
            } => organization_id,
            RawBackendEvent::InviteConduitUpdated {
                organization_id, ..
            } => organization_id,
            RawBackendEvent::UserProfileUpdatedOrRevoked {
                organization_id, ..
            } => organization_id,
            RawBackendEvent::OrganizationExpired {
                organization_id, ..
            } => organization_id,
            RawBackendEvent::Pinged {
                organization_id, ..
            } => organization_id,
            RawBackendEvent::MessageReceived {
                organization_id, ..
            } => organization_id,
            RawBackendEvent::InviteStatusChanged {
                organization_id, ..
            } => organization_id,
            RawBackendEvent::RealmMaintenanceFinished {
                organization_id, ..
            } => organization_id,
            RawBackendEvent::RealmMaintenanceStarted {
                organization_id, ..
            } => organization_id,
            RawBackendEvent::RealmVlobsUpdated {
                organization_id, ..
            } => organization_id,
            RawBackendEvent::RealmRolesUpdated {
                organization_id, ..
            } => organization_id,
            RawBackendEvent::PkiEnrollmentUpdated {
                organization_id, ..
            } => organization_id,
        };
        OrganizationID(organization_id.clone())
    }
}

/*
 * BackendEventCertificatesUpdated
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventCertificatesUpdated;

#[pymethods]
impl BackendEventCertificatesUpdated {
    #[new]
    #[pyo3(signature = (organization_id, index))]
    fn new(organization_id: OrganizationID, index: u64) -> PyResult<(Self, BackendEvent)> {
        Ok((
            BackendEventCertificatesUpdated,
            BackendEvent(RawBackendEvent::CertificatesUpdated {
                organization_id: organization_id.0,
                index,
            }),
        ))
    }

    #[getter]
    fn index(_self: PyRef<Self>) -> PyResult<u64> {
        match &_self.into_super().0 {
            RawBackendEvent::CertificatesUpdated { index, .. } => Ok(*index),
            _ => unreachable!(),
        }
    }
}

/*
 * BackendEventInviteConduitUpdated
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventInviteConduitUpdated;

#[pymethods]
impl BackendEventInviteConduitUpdated {
    #[new]
    #[pyo3(signature = (organization_id, token))]
    fn new(
        organization_id: OrganizationID,
        token: InvitationToken,
    ) -> PyResult<(Self, BackendEvent)> {
        Ok((
            BackendEventInviteConduitUpdated,
            BackendEvent(RawBackendEvent::InviteConduitUpdated {
                organization_id: organization_id.0,
                token: token.0,
            }),
        ))
    }

    #[getter]
    fn token(_self: PyRef<Self>) -> PyResult<InvitationToken> {
        match &_self.into_super().0 {
            RawBackendEvent::InviteConduitUpdated { token, .. } => Ok(InvitationToken(*token)),
            _ => unreachable!(),
        }
    }
}

/*
 * BackendEventUserUpdatedOrRevoked
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventUserUpdatedOrRevoked;

#[pymethods]
impl BackendEventUserUpdatedOrRevoked {
    #[new]
    #[pyo3(signature = (organization_id, user_id, profile))]
    fn new(
        organization_id: OrganizationID,
        user_id: UserID,
        profile: Option<UserProfile>,
    ) -> PyResult<(Self, BackendEvent)> {
        Ok((
            BackendEventUserUpdatedOrRevoked,
            BackendEvent(RawBackendEvent::UserProfileUpdatedOrRevoked {
                organization_id: organization_id.0,
                user_id: user_id.0,
                profile: profile.map(|p| p.0),
            }),
        ))
    }

    #[getter]
    fn user_id(_self: PyRef<Self>) -> PyResult<UserID> {
        match &_self.into_super().0 {
            RawBackendEvent::UserProfileUpdatedOrRevoked { user_id, .. } => {
                Ok(UserID(user_id.clone()))
            }
            _ => unreachable!(),
        }
    }

    #[getter]
    fn profile(_self: PyRef<Self>) -> PyResult<Option<&'static PyObject>> {
        match &_self.into_super().0 {
            RawBackendEvent::UserProfileUpdatedOrRevoked { profile, .. } => {
                Ok(profile.map(UserProfile::convert))
            }
            _ => unreachable!(),
        }
    }
}

/*
 * BackendEventOrganizationExpired
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventOrganizationExpired;

#[pymethods]
impl BackendEventOrganizationExpired {
    #[new]
    #[pyo3(signature = (organization_id))]
    fn new(organization_id: OrganizationID) -> PyResult<(Self, BackendEvent)> {
        Ok((
            BackendEventOrganizationExpired,
            BackendEvent(RawBackendEvent::OrganizationExpired {
                organization_id: organization_id.0,
            }),
        ))
    }
}

/*
 * BackendEventPinged
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventPinged;

#[pymethods]
impl BackendEventPinged {
    #[new]
    #[pyo3(signature = (organization_id, author, ping))]
    fn new(
        organization_id: OrganizationID,
        author: DeviceID,
        ping: String,
    ) -> PyResult<(Self, BackendEvent)> {
        Ok((
            BackendEventPinged,
            BackendEvent(RawBackendEvent::Pinged {
                organization_id: organization_id.0,
                author: author.0,
                ping,
            }),
        ))
    }

    #[getter]
    fn author(_self: PyRef<Self>) -> PyResult<DeviceID> {
        match &_self.into_super().0 {
            RawBackendEvent::Pinged { author, .. } => Ok(DeviceID(author.clone())),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn ping(_self: PyRef<Self>) -> PyResult<String> {
        match &_self.into_super().0 {
            RawBackendEvent::Pinged { ping, .. } => Ok(ping.clone()),
            _ => unreachable!(),
        }
    }
}

/*
 * BackendEventMessageReceived
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventMessageReceived;

#[pymethods]
impl BackendEventMessageReceived {
    #[new]
    #[pyo3(signature = (organization_id, author, recipient, index, message))]
    fn new(
        organization_id: OrganizationID,
        author: DeviceID,
        recipient: UserID,
        index: u64,
        message: BytesWrapper,
    ) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::unwrap_bytes!(message);

        Ok((
            BackendEventMessageReceived,
            BackendEvent(RawBackendEvent::MessageReceived {
                organization_id: organization_id.0,
                author: author.0,
                recipient: recipient.0,
                index,
                message: message.to_vec(),
            }),
        ))
    }

    #[getter]
    fn author(_self: PyRef<Self>) -> PyResult<DeviceID> {
        match &_self.into_super().0 {
            RawBackendEvent::MessageReceived { author, .. } => Ok(DeviceID(author.clone())),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn recipient(_self: PyRef<Self>) -> PyResult<UserID> {
        match &_self.into_super().0 {
            RawBackendEvent::MessageReceived { recipient, .. } => Ok(UserID(recipient.clone())),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn index(_self: PyRef<Self>) -> PyResult<u64> {
        match &_self.into_super().0 {
            RawBackendEvent::MessageReceived { index, .. } => Ok(*index),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn message<'py>(_self: PyRef<Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        match &_self.into_super().0 {
            RawBackendEvent::MessageReceived { message, .. } => Ok(PyBytes::new(py, message)),
            _ => unreachable!(),
        }
    }
}

/*
 * BackendEventInviteStatusChanged
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventInviteStatusChanged;

#[pymethods]
impl BackendEventInviteStatusChanged {
    #[new]
    #[pyo3(signature = (organization_id, greeter, token, status))]
    fn new(
        organization_id: OrganizationID,
        greeter: UserID,
        token: InvitationToken,
        status: InvitationStatus,
    ) -> PyResult<(Self, BackendEvent)> {
        Ok((
            BackendEventInviteStatusChanged,
            BackendEvent(RawBackendEvent::InviteStatusChanged {
                organization_id: organization_id.0,
                greeter: greeter.0,
                token: token.0,
                status: status.0,
            }),
        ))
    }

    #[getter]
    fn greeter(_self: PyRef<Self>) -> PyResult<UserID> {
        match &_self.into_super().0 {
            RawBackendEvent::InviteStatusChanged { greeter, .. } => Ok(UserID(greeter.clone())),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn token(_self: PyRef<Self>) -> PyResult<InvitationToken> {
        match &_self.into_super().0 {
            RawBackendEvent::InviteStatusChanged { token, .. } => Ok(InvitationToken(*token)),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn status(_self: PyRef<Self>) -> PyResult<&'static PyObject> {
        match &_self.into_super().0 {
            RawBackendEvent::InviteStatusChanged { status, .. } => {
                Ok(InvitationStatus::convert(status.clone()))
            }
            _ => unreachable!(),
        }
    }
}

/*
 * BackendEventRealmMaintenanceFinished
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventRealmMaintenanceFinished;

#[pymethods]
impl BackendEventRealmMaintenanceFinished {
    #[new]
    #[pyo3(signature = (organization_id, author, realm_id, encryption_revision))]
    fn new(
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: u64,
    ) -> PyResult<(Self, BackendEvent)> {
        Ok((
            BackendEventRealmMaintenanceFinished,
            BackendEvent(RawBackendEvent::RealmMaintenanceFinished {
                organization_id: organization_id.0,
                author: author.0,
                realm_id: realm_id.0,
                encryption_revision,
            }),
        ))
    }

    #[getter]
    fn author(_self: PyRef<Self>) -> PyResult<DeviceID> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmMaintenanceFinished { author, .. } => {
                Ok(DeviceID(author.clone()))
            }
            _ => unreachable!(),
        }
    }

    #[getter]
    fn realm_id(_self: PyRef<Self>) -> PyResult<RealmID> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmMaintenanceFinished { realm_id, .. } => Ok(RealmID(*realm_id)),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn encryption_revision(_self: PyRef<Self>) -> PyResult<u64> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmMaintenanceFinished {
                encryption_revision,
                ..
            } => Ok(*encryption_revision),
            _ => unreachable!(),
        }
    }
}

/*
 * BackendEventRealmMaintenanceStarted
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventRealmMaintenanceStarted;

#[pymethods]
impl BackendEventRealmMaintenanceStarted {
    #[new]
    #[pyo3(signature = (organization_id, author, realm_id, encryption_revision))]
    fn new(
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: u64,
    ) -> PyResult<(Self, BackendEvent)> {
        Ok((
            BackendEventRealmMaintenanceStarted,
            BackendEvent(RawBackendEvent::RealmMaintenanceStarted {
                organization_id: organization_id.0,
                author: author.0,
                realm_id: realm_id.0,
                encryption_revision,
            }),
        ))
    }

    #[getter]
    fn author(_self: PyRef<Self>) -> PyResult<DeviceID> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmMaintenanceStarted { author, .. } => Ok(DeviceID(author.clone())),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn realm_id(_self: PyRef<Self>) -> PyResult<RealmID> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmMaintenanceStarted { realm_id, .. } => Ok(RealmID(*realm_id)),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn encryption_revision(_self: PyRef<Self>) -> PyResult<u64> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmMaintenanceStarted {
                encryption_revision,
                ..
            } => Ok(*encryption_revision),
            _ => unreachable!(),
        }
    }
}

/*
 * BackendEventRealmVlobsUpdated
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventRealmVlobsUpdated;

#[pymethods]
impl BackendEventRealmVlobsUpdated {
    #[new]
    #[pyo3(signature = (organization_id, author, realm_id, checkpoint, src_id, src_version))]
    fn new(
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        checkpoint: u64,
        src_id: VlobID,
        src_version: u64,
    ) -> PyResult<(Self, BackendEvent)> {
        Ok((
            BackendEventRealmVlobsUpdated,
            BackendEvent(RawBackendEvent::RealmVlobsUpdated {
                organization_id: organization_id.0,
                author: author.0,
                realm_id: realm_id.0,
                checkpoint,
                src_id: src_id.0,
                src_version,
            }),
        ))
    }

    #[getter]
    fn author(_self: PyRef<Self>) -> PyResult<DeviceID> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmVlobsUpdated { author, .. } => Ok(DeviceID(author.clone())),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn realm_id(_self: PyRef<Self>) -> PyResult<RealmID> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmVlobsUpdated { realm_id, .. } => Ok(RealmID(*realm_id)),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn checkpoint(_self: PyRef<Self>) -> PyResult<u64> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmVlobsUpdated { checkpoint, .. } => Ok(*checkpoint),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn src_id(_self: PyRef<Self>) -> PyResult<VlobID> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmVlobsUpdated { src_id, .. } => Ok(VlobID(*src_id)),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn src_version(_self: PyRef<Self>) -> PyResult<u64> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmVlobsUpdated { src_version, .. } => Ok(*src_version),
            _ => unreachable!(),
        }
    }
}

/*
 * BackendEventRealmRolesUpdated
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventRealmRolesUpdated;

#[pymethods]
impl BackendEventRealmRolesUpdated {
    #[new]
    #[pyo3(signature = (organization_id, author, realm_id, user, role))]
    fn new(
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        user: UserID,
        role: Option<RealmRole>,
    ) -> PyResult<(Self, BackendEvent)> {
        Ok((
            BackendEventRealmRolesUpdated,
            BackendEvent(RawBackendEvent::RealmRolesUpdated {
                organization_id: organization_id.0,
                author: author.0,
                realm_id: realm_id.0,
                user: user.0,
                role: role.map(|r| r.0),
            }),
        ))
    }

    #[getter]
    fn author(_self: PyRef<Self>) -> PyResult<DeviceID> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmRolesUpdated { author, .. } => Ok(DeviceID(author.clone())),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn realm_id(_self: PyRef<Self>) -> PyResult<RealmID> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmRolesUpdated { realm_id, .. } => Ok(RealmID(*realm_id)),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn user(_self: PyRef<Self>) -> PyResult<UserID> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmRolesUpdated { user, .. } => Ok(UserID(user.clone())),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn role(_self: PyRef<Self>) -> PyResult<Option<&'static PyObject>> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmRolesUpdated { role, .. } => Ok(role.map(RealmRole::convert)),
            _ => unreachable!(),
        }
    }
}

/*
 * BackendEventPkiEnrollmentUpdated
 */

#[pyclass(extends=BackendEvent)]
pub(crate) struct BackendEventPkiEnrollmentUpdated;

#[pymethods]
impl BackendEventPkiEnrollmentUpdated {
    #[new]
    #[pyo3(signature = (organization_id))]
    fn new(organization_id: OrganizationID) -> PyResult<(Self, BackendEvent)> {
        Ok((
            BackendEventPkiEnrollmentUpdated,
            BackendEvent(RawBackendEvent::PkiEnrollmentUpdated {
                organization_id: organization_id.0,
            }),
        ))
    }
}
