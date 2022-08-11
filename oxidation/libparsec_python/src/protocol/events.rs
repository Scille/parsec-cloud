// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

use libparsec::protocol::authenticated_cmds::{events_listen, events_subscribe};

use crate::binding_utils::{py_to_rs_invitation_status, py_to_rs_realm_role};
use crate::ids::{RealmID, VlobID};
use crate::invite::InvitationToken;

import_exception!(parsec.api.protocol, ProtocolError);

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

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct EventsListenRep(pub events_listen::Rep);

#[pymethods]
impl EventsListenRep {
    #[classmethod]
    #[pyo3(name = "OkPinged")]
    fn ok_pinged(_cls: &PyType, ping: String) -> PyResult<Self> {
        Ok(Self(events_listen::Rep::Ok(
            events_listen::APIEvent::Pinged { ping },
        )))
    }

    #[classmethod]
    #[pyo3(name = "OkMessageReceived")]
    fn ok_message_received(_cls: &PyType, index: u64) -> PyResult<Self> {
        Ok(Self(events_listen::Rep::Ok(
            events_listen::APIEvent::MessageReceived { index },
        )))
    }

    #[classmethod]
    #[pyo3(name = "OkInviteStatusChanged")]
    fn ok_invite_status_changed(
        _cls: &PyType,
        token: InvitationToken,
        invitation_status: &PyAny,
    ) -> PyResult<Self> {
        let token = token.0;
        let invitation_status = py_to_rs_invitation_status(invitation_status)?;
        Ok(Self(events_listen::Rep::Ok(
            events_listen::APIEvent::InviteStatusChanged {
                token,
                invitation_status,
            },
        )))
    }

    #[classmethod]
    #[pyo3(name = "OkRealmMaintenanceFinished")]
    fn ok_realm_maintenance_finished(
        _cls: &PyType,
        realm_id: RealmID,
        encryption_revision: u64,
    ) -> PyResult<Self> {
        let realm_id = realm_id.0;
        Ok(Self(events_listen::Rep::Ok(
            events_listen::APIEvent::RealmMaintenanceFinished {
                realm_id,
                encryption_revision,
            },
        )))
    }

    #[classmethod]
    #[pyo3(name = "OkRealmMaintenanceStarted")]
    fn ok_realm_maintenance_started(
        _cls: &PyType,
        realm_id: RealmID,
        encryption_revision: u64,
    ) -> PyResult<Self> {
        let realm_id = realm_id.0;
        Ok(Self(events_listen::Rep::Ok(
            events_listen::APIEvent::RealmMaintenanceStarted {
                realm_id,
                encryption_revision,
            },
        )))
    }

    #[classmethod]
    #[pyo3(name = "OkRealmVlobsUpdated")]
    fn ok_realm_vlob_updated(
        _cls: &PyType,
        realm_id: RealmID,
        checkpoint: u64,
        src_id: VlobID,
        src_version: u64,
    ) -> PyResult<Self> {
        let realm_id = realm_id.0;
        let src_id = src_id.0;
        Ok(Self(events_listen::Rep::Ok(
            events_listen::APIEvent::RealmVlobsUpdated {
                realm_id,
                checkpoint,
                src_id,
                src_version,
            },
        )))
    }

    #[classmethod]
    #[pyo3(name = "OkRealmRolesUpdated")]
    fn ok_realm_roles_updated(_cls: &PyType, realm_id: RealmID, role: &PyAny) -> PyResult<Self> {
        let realm_id = realm_id.0;
        let role =
            py_to_rs_realm_role(role)?.ok_or_else(|| PyValueError::new_err("role is missing"))?;
        Ok(Self(events_listen::Rep::Ok(
            events_listen::APIEvent::RealmRolesUpdated { realm_id, role },
        )))
    }

    #[classmethod]
    #[pyo3(name = "Cancelled")]
    fn cancelled(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(events_listen::Rep::Cancelled { reason }))
    }

    #[classmethod]
    #[pyo3(name = "NoEvents")]
    fn no_events(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(events_listen::Rep::NoEvents))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        events_listen::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
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

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct EventsSubscribeRep(pub events_subscribe::Rep);

#[pymethods]
impl EventsSubscribeRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(events_subscribe::Rep::Ok))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        events_subscribe::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
