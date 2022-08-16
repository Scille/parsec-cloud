// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

use libparsec::protocol::{authenticated_cmds, invited_cmds};

use crate::protocol::BlockReadReq;

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct AuthenticatedAnyCmdReq(pub authenticated_cmds::AnyCmdReq);

#[pymethods]
impl AuthenticatedAnyCmdReq {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump().map_err(ProtocolError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, buf: Vec<u8>, py: Python) -> PyResult<PyObject> {
        use authenticated_cmds::AnyCmdReq;
        Ok(
            match AnyCmdReq::load(&buf).map_err(ProtocolError::new_err)? {
                AnyCmdReq::BlockRead(x) => BlockReadReq(x).into_py(py),
                _ => unimplemented!(),
            },
        )
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct InvitedAnyCmdReq(pub invited_cmds::AnyCmdReq);

#[pymethods]
impl InvitedAnyCmdReq {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
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
