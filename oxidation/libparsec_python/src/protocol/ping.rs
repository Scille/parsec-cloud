// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

use libparsec::protocol::{authenticated_cmds, invited_cmds};

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct InvitedPingReq(pub invited_cmds::ping::Req);

#[pymethods]
impl InvitedPingReq {
    #[new]
    fn new(ping: String) -> PyResult<Self> {
        Ok(Self(invited_cmds::ping::Req { ping }))
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
    fn ping(&self) -> PyResult<&str> {
        Ok(&self.0.ping)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct InvitedPingRep(pub invited_cmds::ping::Rep);

#[pymethods]
impl InvitedPingRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, pong: String) -> PyResult<Self> {
        Ok(Self(invited_cmds::ping::Rep::Ok { pong }))
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

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        invited_cmds::ping::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct AuthenticatedPingReq(pub authenticated_cmds::ping::Req);

#[pymethods]
impl AuthenticatedPingReq {
    #[new]
    fn new(ping: String) -> PyResult<Self> {
        Ok(Self(authenticated_cmds::ping::Req { ping }))
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
    fn ping(&self) -> PyResult<&str> {
        Ok(&self.0.ping)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct AuthenticatedPingRep(pub authenticated_cmds::ping::Rep);

#[pymethods]
impl AuthenticatedPingRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, pong: String) -> PyResult<Self> {
        Ok(Self(authenticated_cmds::ping::Rep::Ok { pong }))
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

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        authenticated_cmds::ping::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
