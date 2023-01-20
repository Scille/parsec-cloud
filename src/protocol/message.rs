// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyNotImplementedError,
    prelude::*,
    types::{PyBytes, PyTuple},
};

use libparsec::protocol::{authenticated_cmds::v2::message_get, Request};

use crate::protocol::{
    error::{ProtocolError, ProtocolErrorFields, ProtocolResult},
    gen_rep,
};
use crate::time::DateTime;
use crate::{binding_utils::BytesWrapper, ids::DeviceID};

#[pyclass]
#[derive(Clone)]
pub(crate) struct Message(pub message_get::Message);

crate::binding_utils::gen_proto!(Message, __repr__);
crate::binding_utils::gen_proto!(Message, __richcmp__, eq);

#[pymethods]
impl Message {
    #[new]
    fn new(
        count: u64,
        sender: DeviceID,
        timestamp: DateTime,
        body: BytesWrapper,
    ) -> PyResult<Self> {
        crate::binding_utils::unwrap_bytes!(body);
        let sender = sender.0;
        let timestamp = timestamp.0;
        Ok(Self(message_get::Message {
            count,
            sender,
            timestamp,
            body,
        }))
    }

    #[getter]
    fn count(&self) -> PyResult<u64> {
        Ok(self.0.count)
    }

    #[getter]
    fn sender(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.sender.clone()))
    }

    #[getter]
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn body<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(py, &self.0.body))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct MessageGetReq(pub message_get::Req);

crate::binding_utils::gen_proto!(MessageGetReq, __repr__);
crate::binding_utils::gen_proto!(MessageGetReq, __richcmp__, eq);

#[pymethods]
impl MessageGetReq {
    #[new]
    fn new(offset: u64) -> PyResult<Self> {
        Ok(Self(message_get::Req { offset }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[getter]
    fn offset(&self) -> PyResult<u64> {
        Ok(self.0.offset)
    }
}

gen_rep!(message_get, MessageGetRep, { .. });

#[pyclass(extends=MessageGetRep)]
pub(crate) struct MessageGetRepOk;

#[pymethods]
impl MessageGetRepOk {
    #[new]
    fn new(messages: Vec<Message>) -> PyResult<(Self, MessageGetRep)> {
        let messages = messages.into_iter().map(|x| x.0).collect();
        Ok((Self, MessageGetRep(message_get::Rep::Ok { messages })))
    }

    #[getter]
    fn messages<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyTuple> {
        Ok(match &_self.as_ref().0 {
            message_get::Rep::Ok { messages, .. } => {
                PyTuple::new(py, messages.iter().cloned().map(|x| Message(x).into_py(py)))
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}
