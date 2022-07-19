// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

use libparsec::protocol::authenticated_cmds::message_get;

use crate::ids::DeviceID;
use crate::time::DateTime;

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct MessageGetReq(pub message_get::Req);

#[pymethods]
impl MessageGetReq {
    #[new]
    fn new(offset: u64) -> PyResult<Self> {
        Ok(Self(message_get::Req { offset }))
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
    fn offset(&self) -> PyResult<u64> {
        Ok(self.0.offset)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Message(pub message_get::Message);

#[pymethods]
impl Message {
    #[new]
    fn new(count: u64, sender: DeviceID, timestamp: DateTime, body: Vec<u8>) -> PyResult<Self> {
        let sender = sender.0;
        let timestamp = timestamp.0;
        Ok(Self(message_get::Message {
            count,
            sender,
            timestamp,
            body,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct MessageGetRep(pub message_get::Rep);

#[pymethods]
impl MessageGetRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType, messages: Vec<Message>) -> PyResult<Self> {
        let messages = messages.into_iter().map(|msg| msg.0).collect();
        Ok(Self(message_get::Rep::Ok { messages }))
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
        message_get::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
