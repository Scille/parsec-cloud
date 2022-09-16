use crate::{protocol::Reason, invite::InvitationToken, binding_utils::{py_to_rs_invitation_status, py_to_rs_realm_role}, ids::{RealmID, self, VlobID}};
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

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
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

#[pyclass(extends = EventsListenRep)]
#[derive(Clone)]
pub(crate) struct EventsListenRepPingedOk;

#[pymethods]
impl EventsListenRepPingedOk {
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
            events_listen::Rep::Ok(rep) => {
                if let APIEvent::Pinged { ping } = rep {
                    Ok(ping.clone())
                } else {
                    Err(PyAttributeError::new_err(""))
                }
            }
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
        Ok((Self, EventsListenRep(events_listen::Rep::Ok(APIEvent::MessageReceived { index }))))
    }

    #[getter]
    fn index(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(x) => if let APIEvent::MessageReceived { index } = x {
                Ok(*index)
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
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
        Ok((Self, EventsListenRep(events_listen::Rep::Ok(APIEvent::InviteStatusChanged { token: token.0,
            invitation_status: py_to_rs_invitation_status(invitation_status)? }))))
    }

    #[getter]
    fn token(_self: PyRef<'_, Self>) -> PyResult<InvitationToken> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(x) => if let APIEvent::InviteStatusChanged { token, .. } = x {
                Ok(InvitationToken(*token))
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
    }

    #[getter]
    fn invitation_status(_self: PyRef<'_, Self>) -> PyResult<&'static str> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(x) => if let APIEvent::InviteStatusChanged { invitation_status, .. } = x {
                match invitation_status {
                    libparsec::types::InvitationStatus::Idle => Ok("IDLE"),
                    libparsec::types::InvitationStatus::Ready => Ok("READY"),
                    libparsec::types::InvitationStatus::Deleted => Ok("DELETED"),
}
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
    }
}

#[pyclass(extends = EventsListenRep)]
#[derive(PartialEq, Clone)]
struct EventsListenRepOkRealmMaintenanceFinished;

#[pymethods]
impl EventsListenRepOkRealmMaintenanceFinished {
    #[new]
    fn new(realm_id: RealmID, encryption_revision: u64) -> PyResult<(Self, EventsListenRep)> {
        Ok((Self, EventsListenRep(events_listen::Rep::Ok(APIEvent::RealmMaintenanceFinished { realm_id: realm_id.0, encryption_revision }))))
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> PyResult<RealmID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => if let APIEvent::RealmMaintenanceFinished { realm_id, .. } = rep {
                Ok(ids::RealmID(*realm_id))
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => if let APIEvent::RealmMaintenanceFinished { encryption_revision, .. } = rep {
                Ok(*encryption_revision)
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
    }
}

#[pyclass(extends = EventsListenRep)]
#[derive(PartialEq, Clone)]
struct EventsListenRepOkRealmMaintenanceStarted;

#[pymethods]
impl EventsListenRepOkRealmMaintenanceStarted {
    #[new]
    fn new(realm_id: RealmID, encryption_revision: u64) -> PyResult<(Self, EventsListenRep)> {
        Ok((Self, EventsListenRep(events_listen::Rep::Ok(APIEvent::RealmMaintenanceStarted { realm_id: realm_id.0, encryption_revision }))))
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> PyResult<RealmID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => if let APIEvent::RealmMaintenanceFinished { realm_id, .. } = rep {
                Ok(ids::RealmID(*realm_id))
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => if let APIEvent::RealmMaintenanceFinished { encryption_revision, .. } = rep {
                Ok(*encryption_revision)
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
    }
}

#[pyclass(extends = EventsListenRep)]
#[derive(PartialEq, Clone)]
struct EventsListenRepOkVlobsUpdated;

#[pymethods]
impl EventsListenRepOkVlobsUpdated {
    #[new]
    fn new(realm_id: RealmID, checkpoint: u64, src_id: VlobID, src_version: u64) -> PyResult<(Self, EventsListenRep)> {
        Ok((Self, EventsListenRep(events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { realm_id: realm_id.0, checkpoint, src_id: src_id.0, src_version }))))
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> PyResult<RealmID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => if let APIEvent::RealmVlobsUpdated { realm_id, .. } = rep {
                Ok(RealmID(*realm_id))
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
    }

    #[getter]
    fn checkpoint(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => if let APIEvent::RealmVlobsUpdated { checkpoint, .. } = rep {
                Ok(*checkpoint)
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
    }

    #[getter]
    fn src_id(_self: PyRef<'_, Self>) -> PyResult<VlobID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => if let APIEvent::RealmVlobsUpdated { src_id, .. } = rep {
                Ok(VlobID(*src_id))
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
    }

    #[getter]
    fn src_version(_self: PyRef<'_, Self>) -> PyResult<u64> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => if let APIEvent::RealmVlobsUpdated { src_version, .. } = rep {
                Ok(*src_version)
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
    }
}

#[pyclass(extends = EventsListenRep)]
#[derive(PartialEq, Clone)]
struct EventsListenRepOkRealmRolesUpdated;

#[pymethods]
impl EventsListenRepOkRealmRolesUpdated {
    #[new]
    fn new(realm_id: RealmID, role: &PyAny) -> PyResult<(Self, EventsListenRep)> {
        let r = py_to_rs_realm_role(role)?.expect("Expected valid realm role");
        Ok((Self, EventsListenRep(events_listen::Rep::Ok(APIEvent::RealmRolesUpdated { realm_id: realm_id.0, role: r } ))))
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> PyResult<RealmID> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => if let APIEvent::RealmRolesUpdated { realm_id, .. }  = rep {
                Ok(RealmID(*realm_id))
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
    }

    #[getter]
    fn role(_self: PyRef<'_, Self>) -> PyResult<&'static str> {
        match &_self.as_ref().0 {
            events_listen::Rep::Ok(rep) => if let APIEvent::RealmRolesUpdated { role, .. }  = rep {
                match role {
                    libparsec::types::RealmRole::Owner => Ok("OWNER"),
                    libparsec::types::RealmRole::Manager => Ok("MANAGER"),
                    libparsec::types::RealmRole::Contributor => Ok("CONTRIBUTOR"),
                    libparsec::types::RealmRole::Reader => Ok("READER"),
                }
            } else {
                Err(PyAttributeError::new_err(""))
            },
            _ => Err(PyAttributeError::new_err(""))
        }
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

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

gen_rep!(events_subscribe, EventsSubscribeRep, { .. });

#[pyclass(extends = EventsSubscribeRep)]
pub(crate) struct EventsSubscribeRepOk;
