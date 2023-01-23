// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyNotImplementedError,
    prelude::*,
    types::{PyBytes, PyString},
};

use libparsec::protocol::{
    authenticated_cmds::v2 as authenticated_cmds, invited_cmds::v2 as invited_cmds, Request,
};

use crate::protocol::{
    error::{ProtocolError, ProtocolErrorFields, ProtocolResult},
    gen_rep,
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitedPingReq(pub invited_cmds::ping::Req);

crate::binding_utils::gen_proto!(InvitedPingReq, __repr__);
crate::binding_utils::gen_proto!(InvitedPingReq, __copy__);
crate::binding_utils::gen_proto!(InvitedPingReq, __deepcopy__);
crate::binding_utils::gen_proto!(InvitedPingReq, __richcmp__, eq);

#[pymethods]
impl InvitedPingReq {
    #[new]
    fn new(ping: String) -> PyResult<Self> {
        Ok(Self(invited_cmds::ping::Req { ping }))
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
    fn ping(&self) -> PyResult<&str> {
        Ok(&self.0.ping)
    }
}

gen_rep!(invited_cmds::ping, InvitedPingRep, { .. });

#[pyclass(extends=InvitedPingRep)]
pub(crate) struct InvitedPingRepOk;

#[pymethods]
impl InvitedPingRepOk {
    #[new]
    fn new(pong: String) -> PyResult<(Self, InvitedPingRep)> {
        Ok((Self, InvitedPingRep(invited_cmds::ping::Rep::Ok { pong })))
    }

    #[getter]
    fn pong<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyString> {
        Ok(match &_self.as_ref().0 {
            invited_cmds::ping::Rep::Ok { pong, .. } => PyString::new(py, pong),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct AuthenticatedPingReq(pub authenticated_cmds::ping::Req);

crate::binding_utils::gen_proto!(AuthenticatedPingReq, __repr__);
crate::binding_utils::gen_proto!(AuthenticatedPingReq, __copy__);
crate::binding_utils::gen_proto!(AuthenticatedPingReq, __deepcopy__);
crate::binding_utils::gen_proto!(AuthenticatedPingReq, __richcmp__, eq);

#[pymethods]
impl AuthenticatedPingReq {
    #[new]
    fn new(ping: String) -> PyResult<Self> {
        Ok(Self(authenticated_cmds::ping::Req { ping }))
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
    fn ping(&self) -> PyResult<&str> {
        Ok(&self.0.ping)
    }
}

gen_rep!(authenticated_cmds::ping, AuthenticatedPingRep, { .. });

#[pyclass(extends=AuthenticatedPingRep)]
pub(crate) struct AuthenticatedPingRepOk;

#[pymethods]
impl AuthenticatedPingRepOk {
    #[new]
    fn new(pong: String) -> PyResult<(Self, AuthenticatedPingRep)> {
        Ok((
            Self,
            AuthenticatedPingRep(authenticated_cmds::ping::Rep::Ok { pong }),
        ))
    }

    #[getter]
    fn pong<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyString> {
        Ok(match &_self.as_ref().0 {
            authenticated_cmds::ping::Rep::Ok { pong, .. } => PyString::new(py, pong),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}
