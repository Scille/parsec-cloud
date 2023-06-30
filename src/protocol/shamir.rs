// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyNotImplementedError,
    prelude::*,
    types::{PyBytes, PyTuple},
};

use libparsec::{
    protocol::{
        authenticated_cmds::v2::{
            shamir_recovery_others_list, shamir_recovery_self_info, shamir_recovery_setup,
        },
        invited_cmds::v2::invite_shamir_recovery_reveal,
    },
    types::ProtocolRequest,
};

use crate::{
    binding_utils::BytesWrapper,
    ids::ShamirRevealToken,
    protocol::{
        error::{ProtocolError, ProtocolErrorFields, ProtocolResult},
        gen_rep,
    },
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct ShamirRecoveryOthersListReq(pub shamir_recovery_others_list::Req);

crate::binding_utils::gen_proto!(ShamirRecoveryOthersListReq, __repr__);
crate::binding_utils::gen_proto!(ShamirRecoveryOthersListReq, __copy__);
crate::binding_utils::gen_proto!(ShamirRecoveryOthersListReq, __deepcopy__);
crate::binding_utils::gen_proto!(ShamirRecoveryOthersListReq, __richcmp__, eq);

#[pymethods]
impl ShamirRecoveryOthersListReq {
    #[new]
    fn new() -> Self {
        Self(shamir_recovery_others_list::Req)
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
}

gen_rep!(
    shamir_recovery_others_list,
    ShamirRecoveryOthersListRep,
    { .. },
    [NotAllowed],
);

#[pyclass(extends=ShamirRecoveryOthersListRep)]
pub(crate) struct ShamirRecoveryOthersListRepOk;

#[pymethods]
impl ShamirRecoveryOthersListRepOk {
    #[new]
    fn new(
        brief_certificates: Vec<BytesWrapper>,
        share_certificates: Vec<BytesWrapper>,
    ) -> PyResult<(Self, ShamirRecoveryOthersListRep)> {
        crate::binding_utils::unwrap_bytes!(brief_certificates, share_certificates);
        Ok((
            Self,
            ShamirRecoveryOthersListRep(shamir_recovery_others_list::Rep::Ok {
                brief_certificates,
                share_certificates,
            }),
        ))
    }

    #[getter]
    fn brief_certificates<'py>(_self: PyRef<'_, Self>, py: Python<'py>) -> PyResult<&'py PyTuple> {
        Ok(match &_self.as_ref().0 {
            shamir_recovery_others_list::Rep::Ok {
                brief_certificates,
                share_certificates: _,
            } => PyTuple::new(py, brief_certificates.iter().map(|x| PyBytes::new(py, x))),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn share_certificates<'py>(_self: PyRef<'_, Self>, py: Python<'py>) -> PyResult<&'py PyTuple> {
        Ok(match &_self.as_ref().0 {
            shamir_recovery_others_list::Rep::Ok {
                brief_certificates: _,
                share_certificates,
            } => PyTuple::new(py, share_certificates.iter().map(|x| PyBytes::new(py, x))),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct ShamirRecoverySelfInfoReq(pub shamir_recovery_self_info::Req);

crate::binding_utils::gen_proto!(ShamirRecoverySelfInfoReq, __repr__);
crate::binding_utils::gen_proto!(ShamirRecoverySelfInfoReq, __copy__);
crate::binding_utils::gen_proto!(ShamirRecoverySelfInfoReq, __deepcopy__);
crate::binding_utils::gen_proto!(ShamirRecoverySelfInfoReq, __richcmp__, eq);

#[pymethods]
impl ShamirRecoverySelfInfoReq {
    #[new]
    fn new() -> Self {
        Self(shamir_recovery_self_info::Req)
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
}

gen_rep!(shamir_recovery_self_info, ShamirRecoverySelfInfoRep, { .. },);

#[pyclass(extends=ShamirRecoverySelfInfoRep)]
pub(crate) struct ShamirRecoverySelfInfoRepOk;

#[pymethods]
impl ShamirRecoverySelfInfoRepOk {
    #[new]
    fn new(self_info: Option<BytesWrapper>) -> PyResult<(Self, ShamirRecoverySelfInfoRep)> {
        crate::binding_utils::unwrap_bytes!(self_info);
        Ok((
            Self,
            ShamirRecoverySelfInfoRep(shamir_recovery_self_info::Rep::Ok { self_info }),
        ))
    }

    #[getter]
    fn self_info<'py>(_self: PyRef<'_, Self>, py: Python<'py>) -> PyResult<Option<&'py PyBytes>> {
        Ok(match &_self.as_ref().0 {
            shamir_recovery_self_info::Rep::Ok { self_info } => {
                self_info.as_ref().map(|x| PyBytes::new(py, x))
            }
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct ShamirRecoverySetup(pub shamir_recovery_setup::ShamirRecoverySetup);

crate::binding_utils::gen_proto!(ShamirRecoverySetup, __repr__);
crate::binding_utils::gen_proto!(ShamirRecoverySetup, __copy__);
crate::binding_utils::gen_proto!(ShamirRecoverySetup, __deepcopy__);
crate::binding_utils::gen_proto!(ShamirRecoverySetup, __richcmp__, eq);

#[pymethods]
impl ShamirRecoverySetup {
    #[new]
    fn new(
        ciphered_data: BytesWrapper,
        reveal_token: ShamirRevealToken,
        brief: BytesWrapper,
        shares: Vec<BytesWrapper>,
    ) -> Self {
        crate::binding_utils::unwrap_bytes!(ciphered_data, brief, shares);
        Self(shamir_recovery_setup::ShamirRecoverySetup {
            ciphered_data,
            reveal_token: reveal_token.0,
            brief,
            shares,
        })
    }

    #[getter]
    fn ciphered_data(&self) -> &[u8] {
        &self.0.ciphered_data
    }

    #[getter]
    fn reveal_token(&self) -> ShamirRevealToken {
        ShamirRevealToken(self.0.reveal_token)
    }

    #[getter]
    fn brief(&self) -> &[u8] {
        &self.0.brief
    }

    #[getter]
    fn shares<'py>(&self, py: Python<'py>) -> &'py PyTuple {
        PyTuple::new(py, self.0.shares.iter().map(|x| PyBytes::new(py, x)))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct ShamirRecoverySetupReq(pub shamir_recovery_setup::Req);

crate::binding_utils::gen_proto!(ShamirRecoverySetupReq, __repr__);
crate::binding_utils::gen_proto!(ShamirRecoverySetupReq, __copy__);
crate::binding_utils::gen_proto!(ShamirRecoverySetupReq, __deepcopy__);
crate::binding_utils::gen_proto!(ShamirRecoverySetupReq, __richcmp__, eq);

#[pymethods]
impl ShamirRecoverySetupReq {
    #[new]
    fn new(setup: Option<ShamirRecoverySetup>) -> Self {
        Self(shamir_recovery_setup::Req {
            setup: setup.map(|x| x.0),
        })
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
    fn setup(&self) -> Option<ShamirRecoverySetup> {
        self.0.setup.clone().map(ShamirRecoverySetup)
    }
}

gen_rep!(
    shamir_recovery_setup,
    ShamirRecoverySetupRep,
    [InvalidCertification],
    [InvalidData],
    [AlreadySet],
);

#[pyclass(extends=ShamirRecoverySetupRep)]
pub(crate) struct ShamirRecoverySetupRepOk;

#[pymethods]
impl ShamirRecoverySetupRepOk {
    #[new]
    fn new() -> (Self, ShamirRecoverySetupRep) {
        (Self, ShamirRecoverySetupRep(shamir_recovery_setup::Rep::Ok))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InviteShamirRecoveryRevealReq(pub invite_shamir_recovery_reveal::Req);

crate::binding_utils::gen_proto!(InviteShamirRecoveryRevealReq, __repr__);
crate::binding_utils::gen_proto!(InviteShamirRecoveryRevealReq, __copy__);
crate::binding_utils::gen_proto!(InviteShamirRecoveryRevealReq, __deepcopy__);
crate::binding_utils::gen_proto!(InviteShamirRecoveryRevealReq, __richcmp__, eq);

#[pymethods]
impl InviteShamirRecoveryRevealReq {
    #[new]
    fn new(reveal_token: ShamirRevealToken) -> Self {
        Self(invite_shamir_recovery_reveal::Req {
            reveal_token: reveal_token.0,
        })
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
    fn reveal_token(&self) -> ShamirRevealToken {
        ShamirRevealToken(self.0.reveal_token)
    }
}

gen_rep!(
    invite_shamir_recovery_reveal,
    InviteShamirRecoveryRevealRep,
    { .. },
    [NotFound],
);

#[pyclass(extends=InviteShamirRecoveryRevealRep)]
pub(crate) struct InviteShamirRecoveryRevealRepOk;

#[pymethods]
impl InviteShamirRecoveryRevealRepOk {
    #[new]
    fn new(ciphered_data: &[u8]) -> (Self, InviteShamirRecoveryRevealRep) {
        (
            Self,
            InviteShamirRecoveryRevealRep(invite_shamir_recovery_reveal::Rep::Ok {
                ciphered_data: ciphered_data.into(),
            }),
        )
    }

    #[getter]
    fn ciphered_data<'py>(_self: PyRef<'_, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        match &_self.as_ref().0 {
            invite_shamir_recovery_reveal::Rep::Ok { ciphered_data } => {
                Ok(PyBytes::new(py, ciphered_data))
            }
            _ => Err(PyNotImplementedError::new_err("")),
        }
    }
}
