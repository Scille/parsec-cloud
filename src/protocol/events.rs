use crate::{
    binding_utils::{gen_proto, py_to_rs_invitation_status, py_to_rs_realm_role},
    ids::{self, RealmID, VlobID},
    invite::InvitationToken,
    protocol::Reason,
};
use libparsec::protocol::authenticated_cmds::{
    events_listen::{self, APIEvent},
    events_subscribe,
};
use pyo3::{
    exceptions::PyAttributeError, import_exception, prelude::*, types::PyBytes, PyObject, PyResult,
    Python,
};

use super::gen_rep;

import_exception!(parsec.api.protcol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct EventsListenReq(pub events_listen::Req);

#[pymethods]
impl EventsListenReq {
    #[new]
    fn new(wait: bool) -> PyResult<Self> {
        Ok(Self(events_listen::Req { wait }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[getter]
    fn wait(&self) -> PyResult<bool> {
        Ok(self.0.wait)
    }
}

gen_proto!(EventsListenReq, __repr__);

gen_rep!(
    events_listen,
    EventsListenRep,
    { .. },
    [Cancelled, reason: Reason],
    [NoEvents]
);

#[pyclass(extends = EventsListenRep)]
#[derive(Clone)]
pub(crate) struct EventsListenRepOk;

#[pyclass]
#[derive(Clone, PartialEq)]
pub(crate) enum BackendEvent {
    DeviceCreated,
    InviteConduiteUpdated,
    UserCreated,
    UserRevoked,
    OrganizationExpired,
    Pinged,
    MessageReceived,
    InviteStatusChanged,
    RealmMaintenanceStarted,
    RealmMaintenanceFinished,
    RealmVlobsUpdated,
    RealmRolesUpdated,
    PkiEnrollmentsUpdated,
}

#[pymethods]
impl EventsListenRepOk {
    #[new]
    fn new(event_data: EventsListenRep) -> PyResult<(Self, EventsListenRep)> {
        if let events_listen::Rep::Ok(_) = event_data.0 {
            Ok((Self, event_data))
        } else {
            Err(PyAttributeError::new_err(
                "Can't create EventsListenRepOk from a response != Rep::Ok",
            ))
        }
    }

    #[getter]
    fn event(_self: PyRef<'_, Self>) -> PyResult<BackendEvent> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => match rep {
                events_listen::APIEvent::Pinged { .. } => Ok(BackendEvent::Pinged),
                events_listen::APIEvent::RealmRolesUpdated { .. } => {
                    Ok(BackendEvent::RealmRolesUpdated)
                }
                events_listen::APIEvent::RealmMaintenanceStarted { .. } => {
                    Ok(BackendEvent::RealmMaintenanceStarted)
                }
                events_listen::APIEvent::RealmMaintenanceFinished { .. } => {
                    Ok(BackendEvent::RealmMaintenanceFinished)
                }
                events_listen::APIEvent::RealmVlobsUpdated { .. } => {
                    Ok(BackendEvent::RealmVlobsUpdated)
                }
                events_listen::APIEvent::MessageReceived { .. } => {
                    Ok(BackendEvent::MessageReceived)
                }
                events_listen::APIEvent::InviteStatusChanged { .. } => {
                    Ok(BackendEvent::InviteStatusChanged)
                }
            },
            _ => unreachable!(),
        }
    }

    #[getter]
    fn invitation_status(_self: PyRef<'_, Self>) -> PyResult<&'static str> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::InviteStatusChanged {
                invitation_status, ..
            }) => match invitation_status {
                libparsec::types::InvitationStatus::Idle => Ok("IDLE"),
                libparsec::types::InvitationStatus::Ready => Ok("READY"),
                libparsec::types::InvitationStatus::Deleted => Ok("DELETED"),
            },
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn ping(_self: PyRef<'_, Self>) -> PyResult<String> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::Pinged { ping }) => Ok(ping.clone()),
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn index(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::MessageReceived { index }) => Ok(*index),
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn token(_self: PyRef<'_, Self>) -> PyResult<InvitationToken> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::InviteStatusChanged { token, .. }) => {
                Ok(InvitationToken(*token))
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> PyResult<RealmID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => match rep {
                APIEvent::RealmMaintenanceStarted { realm_id, .. } => Ok(RealmID(*realm_id)),
                APIEvent::RealmMaintenanceFinished { realm_id, .. } => Ok(RealmID(*realm_id)),
                APIEvent::RealmVlobsUpdated { realm_id, .. } => Ok(RealmID(*realm_id)),
                APIEvent::RealmRolesUpdated { realm_id, .. } => Ok(RealmID(*realm_id)),
                _ => Err(PyAttributeError::new_err("")),
            },
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => match rep {
                APIEvent::RealmMaintenanceFinished {
                    encryption_revision,
                    ..
                } => Ok(*encryption_revision),
                APIEvent::RealmMaintenanceStarted {
                    encryption_revision,
                    ..
                } => Ok(*encryption_revision),
                _ => Err(PyAttributeError::new_err("")),
            },
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn checkpoint(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { checkpoint, .. }) => {
                Ok(*checkpoint)
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn src_id(_self: PyRef<'_, Self>) -> PyResult<VlobID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { src_id, .. }) => {
                Ok(VlobID(*src_id))
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn src_version(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { src_version, .. }) => {
                Ok(*src_version)
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn role(_self: PyRef<'_, Self>) -> PyResult<Option<&'static str>> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmRolesUpdated { role, .. }) => match role {
                Some(role) => match role {
                    libparsec::types::RealmRole::Owner => Ok(Some("OWNER")),
                    libparsec::types::RealmRole::Manager => Ok(Some("MANAGER")),
                    libparsec::types::RealmRole::Contributor => Ok(Some("CONTRIBUTOR")),
                    libparsec::types::RealmRole::Reader => Ok(Some("READER")),
                },
                None => Ok(None),
            },
            _ => Err(PyAttributeError::new_err("")),
        }
    }
}

gen_proto!(EventsListenRepOk, __repr__, pyref);
gen_proto!(EventsListenRepOk, __richcmp__, eq_pyref);

#[pyclass(extends = EventsListenRep)]
#[derive(Clone, PartialEq)]
pub(crate) struct EventsListenRepOkPinged;

#[pymethods]
impl EventsListenRepOkPinged {
    #[new]
    fn new(ping: String) -> PyResult<(Self, EventsListenRep)> {
        Ok((
            Self,
            EventsListenRep(events_listen::Rep::Ok(events_listen::APIEvent::Pinged {
                ping,
            })),
        ))
    }

    #[getter]
    fn ping(_self: PyRef<'_, Self>) -> PyResult<String> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::Pinged { ping }) => Ok(ping.clone()),
            _ => Err(PyAttributeError::new_err("")),
        }
    }
}

#[pyclass(extends = EventsListenRep)]
#[derive(PartialEq, Clone)]
pub(crate) struct EventsListenRepOkMessageReceived;

#[pymethods]
impl EventsListenRepOkMessageReceived {
    #[new]
    fn new(index: u64) -> PyResult<(Self, EventsListenRep)> {
        Ok((
            Self,
            EventsListenRep(events_listen::Rep::Ok(APIEvent::MessageReceived { index })),
        ))
    }

    #[getter]
    fn index(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::MessageReceived { index }) => Ok(*index),
            _ => Err(PyAttributeError::new_err("")),
        }
    }
}

#[pyclass(extends = EventsListenRep)]
#[derive(PartialEq, Clone)]
pub(crate) struct EventsListenRepOkInviteStatusChanged;

#[pymethods]
impl EventsListenRepOkInviteStatusChanged {
    #[new]
    fn new(token: InvitationToken, invitation_status: &PyAny) -> PyResult<(Self, EventsListenRep)> {
        Ok((
            Self,
            EventsListenRep(events_listen::Rep::Ok(APIEvent::InviteStatusChanged {
                token: token.0,
                invitation_status: py_to_rs_invitation_status(invitation_status)?,
            })),
        ))
    }

    #[getter]
    fn token(_self: PyRef<'_, Self>) -> PyResult<InvitationToken> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::InviteStatusChanged { token, .. }) => {
                Ok(InvitationToken(*token))
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn invitation_status(_self: PyRef<'_, Self>) -> PyResult<&'static str> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::InviteStatusChanged {
                invitation_status, ..
            }) => match invitation_status {
                libparsec::types::InvitationStatus::Idle => Ok("IDLE"),
                libparsec::types::InvitationStatus::Ready => Ok("READY"),
                libparsec::types::InvitationStatus::Deleted => Ok("DELETED"),
            },
            _ => Err(PyAttributeError::new_err("")),
        }
    }
}

#[pyclass(extends = EventsListenRep)]
#[derive(PartialEq, Clone)]
pub(crate) struct EventsListenRepOkRealmMaintenanceFinished;

#[pymethods]
impl EventsListenRepOkRealmMaintenanceFinished {
    #[new]
    fn new(realm_id: RealmID, encryption_revision: u64) -> PyResult<(Self, EventsListenRep)> {
        Ok((
            Self,
            EventsListenRep(events_listen::Rep::Ok(APIEvent::RealmMaintenanceFinished {
                realm_id: realm_id.0,
                encryption_revision,
            })),
        ))
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> PyResult<RealmID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmMaintenanceFinished { realm_id, .. }) => {
                Ok(ids::RealmID(*realm_id))
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmMaintenanceFinished {
                encryption_revision,
                ..
            }) => Ok(*encryption_revision),
            _ => Err(PyAttributeError::new_err("")),
        }
    }
}

#[pyclass(extends = EventsListenRep)]
#[derive(PartialEq, Clone)]
pub(crate) struct EventsListenRepOkRealmMaintenanceStarted;

#[pymethods]
impl EventsListenRepOkRealmMaintenanceStarted {
    #[new]
    fn new(realm_id: RealmID, encryption_revision: u64) -> PyResult<(Self, EventsListenRep)> {
        Ok((
            Self,
            EventsListenRep(events_listen::Rep::Ok(APIEvent::RealmMaintenanceStarted {
                realm_id: realm_id.0,
                encryption_revision,
            })),
        ))
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> PyResult<RealmID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmMaintenanceFinished { realm_id, .. }) => {
                Ok(ids::RealmID(*realm_id))
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmMaintenanceFinished {
                encryption_revision,
                ..
            }) => Ok(*encryption_revision),
            _ => Err(PyAttributeError::new_err("")),
        }
    }
}

#[pyclass(extends = EventsListenRep)]
#[derive(PartialEq, Clone)]
pub(crate) struct EventsListenRepOkVlobsUpdated;

#[pymethods]
impl EventsListenRepOkVlobsUpdated {
    #[new]
    fn new(
        realm_id: RealmID,
        checkpoint: u64,
        src_id: VlobID,
        src_version: u64,
    ) -> PyResult<(Self, EventsListenRep)> {
        Ok((
            Self,
            EventsListenRep(events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated {
                realm_id: realm_id.0,
                checkpoint,
                src_id: src_id.0,
                src_version,
            })),
        ))
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> PyResult<RealmID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { realm_id, .. }) => {
                Ok(RealmID(*realm_id))
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn checkpoint(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { checkpoint, .. }) => {
                Ok(*checkpoint)
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn src_id(_self: PyRef<'_, Self>) -> PyResult<VlobID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { src_id, .. }) => {
                Ok(VlobID(*src_id))
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn src_version(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { src_version, .. }) => {
                Ok(*src_version)
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }
}

#[pyclass(extends = EventsListenRep)]
#[derive(PartialEq, Clone)]
pub(crate) struct EventsListenRepOkRealmRolesUpdated;

#[pymethods]
impl EventsListenRepOkRealmRolesUpdated {
    #[new]
    fn new(realm_id: RealmID, role: &PyAny) -> PyResult<(Self, EventsListenRep)> {
        let r = py_to_rs_realm_role(role)?;
        Ok((
            Self,
            EventsListenRep(events_listen::Rep::Ok(APIEvent::RealmRolesUpdated {
                realm_id: realm_id.0,
                role: r,
            })),
        ))
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> PyResult<RealmID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmRolesUpdated { realm_id, .. }) => {
                Ok(RealmID(*realm_id))
            }
            _ => Err(PyAttributeError::new_err("")),
        }
    }

    #[getter]
    fn role(_self: PyRef<'_, Self>) -> PyResult<Option<&'static str>> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmRolesUpdated { role, .. }) => match role {
                Some(libparsec::types::RealmRole::Owner) => Ok(Some("OWNER")),
                Some(libparsec::types::RealmRole::Manager) => Ok(Some("MANAGER")),
                Some(libparsec::types::RealmRole::Contributor) => Ok(Some("CONTRIBUTOR")),
                Some(libparsec::types::RealmRole::Reader) => Ok(Some("READER")),
                None => Ok(None),
            },
            _ => Err(PyAttributeError::new_err("")),
        }
    }
}

#[pyclass(extends = EventsListenRep)]
#[derive(Clone, PartialEq)]
pub(crate) struct EventsListenRepOkPkiEnrollment;

#[pymethods]
impl EventsListenRepOkPkiEnrollment {
    #[new]
    fn new() -> PyResult<(Self, EventsListenRep)> {
        Ok((
            Self,
            EventsListenRep(events_listen::Rep::Ok(APIEvent::Pinged {
                ping: String::new(),
            })),
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct EventsSubscribeReq(pub events_subscribe::Req);

#[pymethods]
impl EventsSubscribeReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(events_subscribe::Req))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

gen_proto!(EventsSubscribeReq, __repr__);

gen_rep!(events_subscribe, EventsSubscribeRep, { .. });

#[pyclass(extends = EventsSubscribeRep)]
pub(crate) struct EventsSubscribeRepOk;

#[pymethods]
impl EventsSubscribeRepOk {
    #[new]
    fn new() -> PyResult<(Self, EventsSubscribeRep)> {
        Ok((Self {}, EventsSubscribeRep(events_subscribe::Rep::Ok)))
    }
}
