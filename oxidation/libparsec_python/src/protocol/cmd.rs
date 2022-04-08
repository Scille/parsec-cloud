// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

use parsec_api_protocol::{authenticated_cmds, invited_cmds};

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct AuthenticatedAnyCmdReq(pub authenticated_cmds::AnyCmdReq);

#[pymethods]
impl AuthenticatedAnyCmdReq {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}>", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        Ok(Self(
            authenticated_cmds::AnyCmdReq::load(&buf).map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct InvitedAnyCmdReq(pub invited_cmds::AnyCmdReq);

#[pymethods]
impl InvitedAnyCmdReq {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}>", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>) -> PyResult<Self> {
        Ok(Self(
            invited_cmds::AnyCmdReq::load(&buf).map_err(ProtocolError::new_err)?,
        ))
    }
}
