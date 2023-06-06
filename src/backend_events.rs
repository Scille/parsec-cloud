// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyBytes, PyDict},
};
use serde::{Deserialize, Serialize};

use crate::{
    enumerate::{InvitationStatus, RealmRole, UserProfile},
    ids::{DeviceID, InvitationToken, OrganizationID, RealmID, UserID, VlobID},
    time::DateTime,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
enum RawBackendEvent {
    #[serde(rename = "certificates.updated")]
    CertificatesUpdated {
        organization_id: libparsec::types::OrganizationID,
        timestamp: libparsec::types::DateTime,
    },
    #[serde(rename = "invite.conduit_updated")]
    InviteConduitUpdated {
        organization_id: libparsec::types::OrganizationID,
        token: libparsec::types::InvitationToken,
    },
    #[serde(rename = "user.profile_updated_or_revoked")]
    UserProfileUpdatedOrRevoked {
        organization_id: libparsec::types::OrganizationID,
        user_id: libparsec::types::UserID,
        profile: Option<libparsec::types::UserProfile>,
    },
    #[serde(rename = "organization.expired")]
    OrganizationExpired {
        organization_id: libparsec::types::OrganizationID,
    },
    #[serde(rename = "pinged")]
    Pinged {
        organization_id: libparsec::types::OrganizationID,
        author: libparsec::types::DeviceID,
        ping: String,
    },
    #[serde(rename = "message.received")]
    MessageReceived {
        organization_id: libparsec::types::OrganizationID,
        author: libparsec::types::DeviceID,
        recipient: libparsec::types::UserID,
        index: u64,
        message: Vec<u8>,
    },
    #[serde(rename = "invite.status_changed")]
    InviteStatusChanged {
        organization_id: libparsec::types::OrganizationID,
        greeter: libparsec::types::UserID,
        token: libparsec::types::InvitationToken,
        status: libparsec::types::InvitationStatus,
    },
    #[serde(rename = "realm.maintenance_finished")]
    RealmMaintenanceFinished {
        organization_id: libparsec::types::OrganizationID,
        author: libparsec::types::DeviceID,
        realm_id: libparsec::types::RealmID,
        encryption_revision: u64,
    },
    #[serde(rename = "realm.maintenance_started")]
    RealmMaintenanceStarted {
        organization_id: libparsec::types::OrganizationID,
        author: libparsec::types::DeviceID,
        realm_id: libparsec::types::RealmID,
        encryption_revision: u64,
    },
    #[serde(rename = "realm.vlobs_updated")]
    RealmVlobsUpdated {
        organization_id: libparsec::types::OrganizationID,
        author: libparsec::types::DeviceID,
        realm_id: libparsec::types::RealmID,
        checkpoint: u64,
        src_id: libparsec::types::VlobID,
        src_version: u64,
    },
    #[serde(rename = "realm.roles_updated")]
    RealmRolesUpdated {
        organization_id: libparsec::types::OrganizationID,
        author: libparsec::types::DeviceID,
        realm_id: libparsec::types::RealmID,
        user: libparsec::types::UserID,
        role: Option<libparsec::types::RealmRole>,
    },
    #[serde(rename = "pki_enrollment.updated")]
    PkiEnrollmentUpdated {
        organization_id: libparsec::types::OrganizationID,
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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
            [timestamp: DateTime, "timestamp"],
        );

        Ok((
            BackendEventCertificatesUpdated,
            BackendEvent(RawBackendEvent::CertificatesUpdated {
                organization_id: organization_id.0,
                timestamp: timestamp.0,
            }),
        ))
    }

    #[getter]
    fn timestamp<'py>(_self: PyRef<Self>, py: Python<'py>) -> PyResult<DateTime> {
        match &_self.into_super().0 {
            RawBackendEvent::CertificatesUpdated { timestamp, .. } => Ok(DateTime(*timestamp)),
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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
            [token: InvitationToken, "token"],
        );

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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
            [user_id: UserID, "user_id"],
            [profile: Option<UserProfile>, "profile"],
        );

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
    fn profile(_self: PyRef<Self>) -> PyResult<Option<UserProfile>> {
        match &_self.into_super().0 {
            RawBackendEvent::UserProfileUpdatedOrRevoked { profile, .. } => {
                Ok(profile.map(UserProfile))
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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
        );

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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
            [author: DeviceID, "author"],
            [ping: String, "ping"],
        );

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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
            [author: DeviceID, "author"],
            [recipient: UserID, "recipient"],
            [index: u64, "index"],
            [message: &PyBytes, "message"],
        );

        Ok((
            BackendEventMessageReceived,
            BackendEvent(RawBackendEvent::MessageReceived {
                organization_id: organization_id.0,
                author: author.0,
                recipient: recipient.0,
                index,
                message: message.as_bytes().to_vec(),
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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
            [greeter: UserID, "greeter"],
            [token: InvitationToken, "token"],
            [status: InvitationStatus, "status"],
        );

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
    fn status(_self: PyRef<Self>) -> PyResult<InvitationStatus> {
        match &_self.into_super().0 {
            RawBackendEvent::InviteStatusChanged { status, .. } => {
                Ok(InvitationStatus(status.clone()))
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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
            [author: DeviceID, "author"],
            [realm_id: RealmID, "realm_id"],
            [encryption_revision: u64, "encryption_revision"],
        );

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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
            [author: DeviceID, "author"],
            [realm_id: RealmID, "realm_id"],
            [encryption_revision: u64, "encryption_revision"],
        );

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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
            [author: DeviceID, "author"],
            [realm_id: RealmID, "realm_id"],
            [checkpoint: u64, "checkpoint"],
            [src_id: VlobID, "src_id"],
            [src_version: u64, "src_version"],
        );

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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
            [author: DeviceID, "author"],
            [realm_id: RealmID, "realm_id"],
            [user: UserID, "user"],
            [role: Option<RealmRole>, "role"],
        );

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
    fn role(_self: PyRef<Self>) -> PyResult<Option<RealmRole>> {
        match &_self.into_super().0 {
            RawBackendEvent::RealmRolesUpdated { role, .. } => Ok(role.map(RealmRole)),
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
    #[args(py_kwargs = "**")]
    fn new(py_kwargs: Option<&PyDict>) -> PyResult<(Self, BackendEvent)> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [organization_id: OrganizationID, "organization_id"],
        );

        Ok((
            BackendEventPkiEnrollmentUpdated,
            BackendEvent(RawBackendEvent::PkiEnrollmentUpdated {
                organization_id: organization_id.0,
            }),
        ))
    }
}
