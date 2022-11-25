// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyNotImplementedError,
    import_exception,
    prelude::*,
    types::{PyBytes, PyTuple},
};
use std::num::NonZeroU64;

use libparsec::protocol::{
    authenticated_cmds::v3::{device_create, human_find, user_create, user_get, user_revoke},
    IntegerBetween1And100,
};

use crate::{
    ids::{HumanHandle, UserID},
    protocol::{
        error::{ProtocolError, ProtocolErrorFields, ProtocolResult},
        gen_rep, Reason,
    },
};

import_exception!(parsec.api.protocol, InvalidMessageError);

#[pyclass]
#[derive(Clone)]
pub(crate) struct Trustchain(pub user_get::Trustchain);

crate::binding_utils::gen_proto!(Trustchain, __repr__);
crate::binding_utils::gen_proto!(Trustchain, __richcmp__, eq);

#[pymethods]
impl Trustchain {
    #[new]
    fn new(
        devices: Vec<Vec<u8>>,
        users: Vec<Vec<u8>>,
        revoked_users: Vec<Vec<u8>>,
    ) -> PyResult<Self> {
        Ok(Self(user_get::Trustchain {
            devices,
            users,
            revoked_users,
        }))
    }

    #[getter]
    fn devices<'py>(&self, py: Python<'py>) -> PyResult<&'py PyTuple> {
        Ok(PyTuple::new(
            py,
            self.0.devices.iter().map(|x| PyBytes::new(py, x)),
        ))
    }

    #[getter]
    fn users<'py>(&self, py: Python<'py>) -> PyResult<&'py PyTuple> {
        Ok(PyTuple::new(
            py,
            self.0.users.iter().map(|x| PyBytes::new(py, x)),
        ))
    }

    #[getter]
    fn revoked_users<'py>(&self, py: Python<'py>) -> PyResult<&'py PyTuple> {
        Ok(PyTuple::new(
            py,
            self.0.revoked_users.iter().map(|x| PyBytes::new(py, x)),
        ))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct HumanFindResultItem(pub human_find::HumanFindResultItem);

crate::binding_utils::gen_proto!(HumanFindResultItem, __repr__);
crate::binding_utils::gen_proto!(HumanFindResultItem, __richcmp__, eq);

#[pymethods]
impl HumanFindResultItem {
    #[new]
    fn new(user_id: UserID, human_handle: Option<HumanHandle>, revoked: bool) -> PyResult<Self> {
        let user_id = user_id.0;
        let human_handle = human_handle.map(|hh| hh.0);
        Ok(Self(human_find::HumanFindResultItem {
            user_id,
            human_handle,
            revoked,
        }))
    }

    #[getter]
    fn user_id(&self) -> PyResult<UserID> {
        Ok(UserID(self.0.user_id.clone()))
    }

    #[getter]
    fn human_handle(&self) -> PyResult<Option<HumanHandle>> {
        Ok(self.0.human_handle.clone().map(HumanHandle))
    }

    #[getter]
    fn revoked(&self) -> PyResult<bool> {
        Ok(self.0.revoked)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct UserGetReq(pub user_get::Req);

crate::binding_utils::gen_proto!(UserGetReq, __repr__);
crate::binding_utils::gen_proto!(UserGetReq, __richcmp__, eq);

#[pymethods]
impl UserGetReq {
    #[new]
    fn new(user_id: UserID) -> PyResult<Self> {
        let user_id = user_id.0;
        Ok(Self(user_get::Req { user_id }))
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
    fn user_id(&self) -> PyResult<UserID> {
        Ok(UserID(self.0.user_id.clone()))
    }
}

gen_rep!(user_get, UserGetRep, { .. }, [NotFound]);

#[pyclass(extends=UserGetRep)]
pub(crate) struct UserGetRepOk;

#[pymethods]
impl UserGetRepOk {
    #[new]
    fn new(
        user_certificate: Vec<u8>,
        revoked_user_certificate: Option<Vec<u8>>,
        device_certificates: Vec<Vec<u8>>,
        trustchain: Trustchain,
    ) -> PyResult<(Self, UserGetRep)> {
        Ok((
            Self,
            UserGetRep(user_get::Rep::Ok {
                user_certificate,
                revoked_user_certificate,
                device_certificates,
                trustchain: trustchain.0,
            }),
        ))
    }

    #[getter]
    fn user_certificate<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(match &_self.as_ref().0 {
            user_get::Rep::Ok {
                user_certificate, ..
            } => PyBytes::new(py, user_certificate),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn revoked_user_certificate<'py>(
        _self: PyRef<'py, Self>,
        py: Python<'py>,
    ) -> PyResult<Option<&'py PyBytes>> {
        Ok(match &_self.as_ref().0 {
            user_get::Rep::Ok {
                revoked_user_certificate,
                ..
            } => revoked_user_certificate
                .as_ref()
                .map(|x| PyBytes::new(py, x)),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn device_certificates<'py>(
        _self: PyRef<'py, Self>,
        py: Python<'py>,
    ) -> PyResult<&'py PyTuple> {
        Ok(match &_self.as_ref().0 {
            user_get::Rep::Ok {
                device_certificates,
                ..
            } => PyTuple::new(py, device_certificates.iter().map(|x| PyBytes::new(py, x))),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn trustchain(_self: PyRef<'_, Self>) -> PyResult<Trustchain> {
        Ok(match &_self.as_ref().0 {
            user_get::Rep::Ok { trustchain, .. } => Trustchain(trustchain.clone()),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct UserCreateReq(pub user_create::Req);

crate::binding_utils::gen_proto!(UserCreateReq, __repr__);
crate::binding_utils::gen_proto!(UserCreateReq, __richcmp__, eq);

#[pymethods]
impl UserCreateReq {
    #[new]
    fn new(
        user_certificate: Vec<u8>,
        device_certificate: Vec<u8>,
        redacted_user_certificate: Vec<u8>,
        redacted_device_certificate: Vec<u8>,
    ) -> PyResult<Self> {
        Ok(Self(user_create::Req {
            user_certificate,
            device_certificate,
            redacted_user_certificate,
            redacted_device_certificate,
        }))
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
    fn user_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.user_certificate)
    }

    #[getter]
    fn device_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.device_certificate)
    }

    #[getter]
    fn redacted_user_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.redacted_user_certificate)
    }

    #[getter]
    fn redacted_device_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.redacted_device_certificate)
    }
}

gen_rep!(
    user_create,
    UserCreateRep,
    [NotAllowed, reason: Reason],
    [InvalidCertification, reason: Reason],
    [InvalidData, reason: Reason],
    [AlreadyExists, reason: Reason],
    [ActiveUsersLimitReached, reason: Reason],
);

#[pyclass(extends=UserCreateRep)]
pub(crate) struct UserCreateRepOk;

#[pymethods]
impl UserCreateRepOk {
    #[new]
    fn new() -> PyResult<(Self, UserCreateRep)> {
        Ok((Self, UserCreateRep(user_create::Rep::Ok)))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct UserRevokeReq(pub user_revoke::Req);

crate::binding_utils::gen_proto!(UserRevokeReq, __repr__);
crate::binding_utils::gen_proto!(UserRevokeReq, __richcmp__, eq);

#[pymethods]
impl UserRevokeReq {
    #[new]
    fn new(revoked_user_certificate: Vec<u8>) -> PyResult<Self> {
        Ok(Self(user_revoke::Req {
            revoked_user_certificate,
        }))
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
    fn revoked_user_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.revoked_user_certificate)
    }
}

gen_rep!(
    user_revoke,
    UserRevokeRep,
    [NotAllowed, reason: Reason],
    [InvalidCertification, reason: Reason],
    [NotFound],
    [AlreadyRevoked, reason: Reason],
);

#[pyclass(extends=UserRevokeRep)]
pub(crate) struct UserRevokeRepOk;

#[pymethods]
impl UserRevokeRepOk {
    #[new]
    fn new() -> PyResult<(Self, UserRevokeRep)> {
        Ok((Self, UserRevokeRep(user_revoke::Rep::Ok)))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct DeviceCreateReq(pub device_create::Req);

crate::binding_utils::gen_proto!(DeviceCreateReq, __repr__);
crate::binding_utils::gen_proto!(DeviceCreateReq, __richcmp__, eq);

#[pymethods]
impl DeviceCreateReq {
    #[new]
    fn new(device_certificate: Vec<u8>, redacted_device_certificate: Vec<u8>) -> PyResult<Self> {
        Ok(Self(device_create::Req {
            device_certificate,
            redacted_device_certificate,
        }))
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
    fn device_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.device_certificate)
    }

    #[getter]
    fn redacted_device_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.redacted_device_certificate)
    }
}

gen_rep!(
    device_create,
    DeviceCreateRep,
    [InvalidCertification, reason: Reason],
    [BadUserId, reason: Reason],
    [InvalidData, reason: Reason],
    [AlreadyExists, reason: Reason],
);

#[pyclass(extends=DeviceCreateRep)]
pub(crate) struct DeviceCreateRepOk;

#[pymethods]
impl DeviceCreateRepOk {
    #[new]
    fn new() -> PyResult<(Self, DeviceCreateRep)> {
        Ok((Self, DeviceCreateRep(device_create::Rep::Ok)))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct HumanFindReq(pub human_find::Req);

crate::binding_utils::gen_proto!(HumanFindReq, __repr__);
crate::binding_utils::gen_proto!(HumanFindReq, __richcmp__, eq);

#[pymethods]
impl HumanFindReq {
    #[new]
    fn new(
        query: Option<String>,
        omit_revoked: bool,
        omit_non_human: bool,
        page: u64,
        per_page: u64,
    ) -> PyResult<Self> {
        let page = NonZeroU64::try_from(page).map_err(InvalidMessageError::new_err)?;
        let per_page =
            IntegerBetween1And100::try_from(per_page).map_err(InvalidMessageError::new_err)?;
        Ok(Self(human_find::Req {
            query,
            omit_revoked,
            omit_non_human,
            page,
            per_page,
        }))
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
    fn query(&self) -> PyResult<Option<&str>> {
        Ok(self.0.query.as_ref().map(|q| &q[..]))
    }

    #[getter]
    fn omit_revoked(&self) -> PyResult<bool> {
        Ok(self.0.omit_revoked)
    }

    #[getter]
    fn omit_non_human(&self) -> PyResult<bool> {
        Ok(self.0.omit_non_human)
    }

    #[getter]
    fn page(&self) -> PyResult<u64> {
        Ok(self.0.page.into())
    }

    #[getter]
    fn per_page(&self) -> PyResult<u64> {
        Ok(self.0.per_page.into())
    }
}

gen_rep!(
    human_find,
    HumanFindRep,
    { .. },
    [NotAllowed, reason: Reason],
);

#[pyclass(extends=HumanFindRep)]
pub(crate) struct HumanFindRepOk;

#[pymethods]
impl HumanFindRepOk {
    #[new]
    fn new(
        results: Vec<HumanFindResultItem>,
        page: u64,
        per_page: u64,
        total: u64,
    ) -> PyResult<(Self, HumanFindRep)> {
        Ok((
            Self,
            HumanFindRep(human_find::Rep::Ok {
                results: results.into_iter().map(|x| x.0).collect(),
                page: NonZeroU64::try_from(page).map_err(InvalidMessageError::new_err)?,
                per_page: IntegerBetween1And100::try_from(per_page)
                    .map_err(InvalidMessageError::new_err)?,
                total,
            }),
        ))
    }

    #[getter]
    fn results<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyTuple> {
        Ok(match &_self.as_ref().0 {
            human_find::Rep::Ok { results, .. } => PyTuple::new(
                py,
                results
                    .iter()
                    .cloned()
                    .map(|x| HumanFindResultItem(x).into_py(py)),
            ),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn page(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            human_find::Rep::Ok { page, .. } => page.into(),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn per_page(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            human_find::Rep::Ok { per_page, .. } => per_page.into(),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn total(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            human_find::Rep::Ok { total, .. } => total,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}
