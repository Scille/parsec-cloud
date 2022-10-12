use crate::{
    certif::RealmRole,
    ids::{RealmID, VlobID},
    invite::InvitationToken,
    protocol::{invite::InvitationStatus, Reason},
};
use libparsec::protocol::authenticated_cmds::{
    events_listen::{self, APIEvent},
    events_subscribe,
};
use pyo3::{import_exception, prelude::*, types::PyBytes, PyObject, PyResult, Python};

use super::gen_rep;

import_exception!(parsec.api.protcol, ProtocolError);

#[pyclass]
#[derive(Clone)]
pub(crate) struct EventsListenReq(pub events_listen::Req);

crate::binding_utils::gen_proto!(EventsListenReq, __repr__);
crate::binding_utils::gen_proto!(EventsListenReq, __richcmp__, eq);

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

gen_rep!(
    events_listen,
    EventsListenRep,
    { .. },
    [Cancelled, reason: Reason],
    [NoEvents]
);

#[pymethods]
impl EventsListenRep {
    #[classmethod]
    fn _post_load<'py>(
        _cls: &'py ::pyo3::types::PyType,
        py: Python<'py>,
        loaded: &'py PyAny,
    ) -> PyResult<PyObject> {
        let event = match loaded.extract::<EventsListenRep>()?.0 {
            events_listen::Rep::Ok(event) => event,
            _ => return Ok(loaded.into_py(py)),
        };
        let init = PyClassInitializer::from(EventsListenRepOk::new(event.to_owned()));
        let ret = match event {
            APIEvent::Pinged { .. } => {
                Py::new(py, init.add_subclass(EventsListenRepOkPinged))?.into_py(py)
            }
            APIEvent::MessageReceived { .. } => {
                Py::new(py, init.add_subclass(EventsListenRepOkMessageReceived))?.into_py(py)
            }
            APIEvent::InviteStatusChanged { .. } => {
                Py::new(py, init.add_subclass(EventsListenRepOkInviteStatusChanged))?.into_py(py)
            }
            APIEvent::RealmMaintenanceFinished { .. } => Py::new(
                py,
                init.add_subclass(EventsListenRepOkRealmMaintenanceFinished),
            )?
            .into_py(py),
            APIEvent::RealmMaintenanceStarted { .. } => Py::new(
                py,
                init.add_subclass(EventsListenRepOkRealmMaintenanceStarted),
            )?
            .into_py(py),
            APIEvent::RealmVlobsUpdated { .. } => {
                Py::new(py, init.add_subclass(EventsListenRepOkRealmVlobsUpdated))?.into_py(py)
            }
            APIEvent::RealmRolesUpdated { .. } => {
                Py::new(py, init.add_subclass(EventsListenRepOkRealmRolesUpdated))?.into_py(py)
            }
            APIEvent::PkiEnrollmentUpdated { .. } => {
                Py::new(py, init.add_subclass(EventsListenRepOkPkiEnrollmentUpdated))?.into_py(py)
            }
        };
        Ok(ret)
    }
}

#[pyclass(extends = EventsListenRep, subclass)]
#[derive(Clone)]
pub(crate) struct EventsListenRepOk;

impl EventsListenRepOk {
    fn new(event: events_listen::APIEvent) -> (Self, EventsListenRep) {
        (Self, EventsListenRep(events_listen::Rep::Ok(event)))
    }
}

#[pyclass(extends = EventsListenRepOk)]
#[derive(Clone)]
pub(crate) struct EventsListenRepOkPinged;

#[pymethods]
impl EventsListenRepOkPinged {
    #[new]
    fn new(ping: String) -> PyClassInitializer<Self> {
        let event = events_listen::APIEvent::Pinged { ping };
        PyClassInitializer::from(EventsListenRepOk::new(event)).add_subclass(Self)
    }

    #[getter]
    fn ping(_self: PyRef<'_, Self>) -> String {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::Pinged { ping }) => ping.clone(),
            _ => unreachable!(),
        }
    }
}

#[pyclass(extends = EventsListenRepOk)]
#[derive(Clone)]
pub(crate) struct EventsListenRepOkMessageReceived;

#[pymethods]
impl EventsListenRepOkMessageReceived {
    #[new]
    fn new(index: u64) -> PyClassInitializer<Self> {
        let event = events_listen::APIEvent::MessageReceived { index };
        PyClassInitializer::from(EventsListenRepOk::new(event)).add_subclass(Self)
    }

    #[getter]
    fn index(_self: PyRef<'_, Self>) -> u64 {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::MessageReceived { index }) => *index,
            _ => unreachable!(),
        }
    }
}

#[pyclass(extends = EventsListenRepOk)]
#[derive(Clone)]
pub(crate) struct EventsListenRepOkInviteStatusChanged;

#[pymethods]
impl EventsListenRepOkInviteStatusChanged {
    #[new]
    fn new(
        token: InvitationToken,
        invitation_status: InvitationStatus,
    ) -> PyClassInitializer<Self> {
        let event = events_listen::APIEvent::InviteStatusChanged {
            token: token.0,
            invitation_status: invitation_status.0,
        };
        PyClassInitializer::from(EventsListenRepOk::new(event)).add_subclass(Self)
    }

    #[getter]
    fn token(_self: PyRef<'_, Self>) -> InvitationToken {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::InviteStatusChanged { token, .. }) => {
                InvitationToken(*token)
            }
            _ => unreachable!(),
        }
    }

    #[getter]
    fn invitation_status(_self: PyRef<'_, Self>) -> InvitationStatus {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::InviteStatusChanged {
                invitation_status, ..
            }) => InvitationStatus(invitation_status.clone()),
            _ => unreachable!(),
        }
    }
}

#[pyclass(extends = EventsListenRepOk)]
#[derive(Clone)]
pub(crate) struct EventsListenRepOkRealmMaintenanceFinished;

#[pymethods]
impl EventsListenRepOkRealmMaintenanceFinished {
    #[new]
    fn new(realm_id: RealmID, encryption_revision: u64) -> PyClassInitializer<Self> {
        let event = events_listen::APIEvent::RealmMaintenanceFinished {
            realm_id: realm_id.0,
            encryption_revision,
        };
        PyClassInitializer::from(EventsListenRepOk::new(event)).add_subclass(Self)
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> RealmID {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmMaintenanceFinished { realm_id, .. }) => {
                RealmID(*realm_id)
            }
            _ => unreachable!(),
        }
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> u64 {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmMaintenanceFinished {
                encryption_revision,
                ..
            }) => *encryption_revision,
            _ => unreachable!(),
        }
    }
}

#[pyclass(extends = EventsListenRepOk)]
#[derive(Clone)]
pub(crate) struct EventsListenRepOkRealmMaintenanceStarted;

#[pymethods]
impl EventsListenRepOkRealmMaintenanceStarted {
    #[new]
    fn new(realm_id: RealmID, encryption_revision: u64) -> PyClassInitializer<Self> {
        let event = events_listen::APIEvent::RealmMaintenanceStarted {
            realm_id: realm_id.0,
            encryption_revision,
        };
        PyClassInitializer::from(EventsListenRepOk::new(event)).add_subclass(Self)
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> RealmID {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmMaintenanceStarted { realm_id, .. }) => {
                RealmID(*realm_id)
            }
            _ => unreachable!(),
        }
    }

    #[getter]
    fn encryption_revision(_self: PyRef<'_, Self>) -> u64 {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmMaintenanceStarted {
                encryption_revision,
                ..
            }) => *encryption_revision,
            _ => unreachable!(),
        }
    }
}

#[pyclass(extends = EventsListenRepOk)]
#[derive(Clone)]
pub(crate) struct EventsListenRepOkRealmVlobsUpdated;

#[pymethods]
impl EventsListenRepOkRealmVlobsUpdated {
    #[new]
    fn new(
        realm_id: RealmID,
        checkpoint: u64,
        src_id: VlobID,
        src_version: u64,
    ) -> PyClassInitializer<Self> {
        let event = events_listen::APIEvent::RealmVlobsUpdated {
            realm_id: realm_id.0,
            checkpoint,
            src_id: src_id.0,
            src_version,
        };
        PyClassInitializer::from(EventsListenRepOk::new(event)).add_subclass(Self)
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> RealmID {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { realm_id, .. }) => {
                RealmID(*realm_id)
            }
            _ => unreachable!(),
        }
    }

    #[getter]
    fn checkpoint(_self: PyRef<'_, Self>) -> u64 {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { checkpoint, .. }) => *checkpoint,
            _ => unreachable!(),
        }
    }

    #[getter]
    fn src_id(_self: PyRef<'_, Self>) -> VlobID {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { src_id, .. }) => VlobID(*src_id),
            _ => unreachable!(),
        }
    }

    #[getter]
    fn src_version(_self: PyRef<'_, Self>) -> u64 {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmVlobsUpdated { src_version, .. }) => *src_version,
            _ => unreachable!(),
        }
    }
}

#[pyclass(extends = EventsListenRepOk)]
#[derive(Clone)]
pub(crate) struct EventsListenRepOkRealmRolesUpdated;

#[pymethods]
impl EventsListenRepOkRealmRolesUpdated {
    #[new]
    fn new(realm_id: RealmID, role: Option<RealmRole>) -> PyClassInitializer<Self> {
        let event = events_listen::APIEvent::RealmRolesUpdated {
            realm_id: realm_id.0,
            role: role.map(|x| x.0),
        };
        PyClassInitializer::from(EventsListenRepOk::new(event)).add_subclass(Self)
    }

    #[getter]
    fn realm_id(_self: PyRef<'_, Self>) -> RealmID {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmRolesUpdated { realm_id, .. }) => {
                RealmID(*realm_id)
            }
            _ => unreachable!(),
        }
    }

    #[getter]
    fn role(_self: PyRef<'_, Self>) -> Option<RealmRole> {
        match &_self.into_super().as_ref().0 {
            events_listen::Rep::Ok(APIEvent::RealmRolesUpdated { role, .. }) => role.map(RealmRole),
            _ => unreachable!(),
        }
    }
}

#[pyclass(extends = EventsListenRepOk)]
#[derive(Clone)]
pub(crate) struct EventsListenRepOkPkiEnrollmentUpdated;

#[pymethods]
impl EventsListenRepOkPkiEnrollmentUpdated {
    #[new]
    fn new() -> PyClassInitializer<Self> {
        let event = events_listen::APIEvent::PkiEnrollmentUpdated;
        PyClassInitializer::from(EventsListenRepOk::new(event)).add_subclass(Self)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct EventsSubscribeReq(pub events_subscribe::Req);

crate::binding_utils::gen_proto!(EventsSubscribeReq, __repr__);
crate::binding_utils::gen_proto!(EventsSubscribeReq, __richcmp__, eq);

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
