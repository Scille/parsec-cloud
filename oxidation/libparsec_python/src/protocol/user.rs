// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};
use std::num::NonZeroU64;

use libparsec::protocol::authenticated_cmds::{
    device_create, human_find, user_create, user_get, user_revoke,
};

use crate::ids::{HumanHandle, UserID};

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct UserGetReq(pub user_get::Req);

#[pymethods]
impl UserGetReq {
    #[new]
    fn new(user_id: UserID) -> PyResult<Self> {
        let user_id = user_id.0;
        Ok(Self(user_get::Req { user_id }))
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
    fn user_id(&self) -> PyResult<UserID> {
        Ok(UserID(self.0.user_id.clone()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct Trustchain(pub user_get::Trustchain);

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

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct UserGetRep(pub user_get::Rep);

#[pymethods]
impl UserGetRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(
        _cls: &PyType,
        user_certificate: Vec<u8>,
        revoked_user_certificate: Vec<u8>,
        device_certificates: Vec<Vec<u8>>,
        trustchain: Trustchain,
    ) -> PyResult<Self> {
        let trustchain = trustchain.0;
        Ok(Self(user_get::Rep::Ok {
            user_certificate,
            revoked_user_certificate,
            device_certificates,
            trustchain,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(user_get::Rep::NotFound))
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
        user_get::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct UserCreateReq(pub user_create::Req);

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

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct UserCreateRep(pub user_create::Rep);

#[pymethods]
impl UserCreateRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(user_create::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(user_create::Rep::NotAllowed { reason }))
    }

    #[classmethod]
    #[pyo3(name = "InvalidCertification")]
    fn invalid_certification(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(user_create::Rep::InvalidCertification { reason }))
    }

    #[classmethod]
    #[pyo3(name = "InvalidData")]
    fn invalid_data(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(user_create::Rep::InvalidData { reason }))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyExists")]
    fn already_exists(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(user_create::Rep::AlreadyExists { reason }))
    }

    #[classmethod]
    #[pyo3(name = "ActiveUsersLimitReached")]
    fn active_users_limit_reached(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(user_create::Rep::ActiveUsersLimitReached { reason }))
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
        user_create::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct UserRevokeReq(pub user_revoke::Req);

#[pymethods]
impl UserRevokeReq {
    #[new]
    fn new(revoked_user_certificate: Vec<u8>) -> PyResult<Self> {
        Ok(Self(user_revoke::Req {
            revoked_user_certificate,
        }))
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
    fn revoked_user_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.revoked_user_certificate)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct UserRevokeRep(pub user_revoke::Rep);

#[pymethods]
impl UserRevokeRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(user_revoke::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "InvalidCertification")]
    fn invalid_certification(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(user_revoke::Rep::InvalidCertification { reason }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(user_revoke::Rep::NotFound))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyRevoked")]
    fn already_revoked(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(user_revoke::Rep::AlreadyRevoked { reason }))
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
        user_revoke::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct DeviceCreateReq(pub device_create::Req);

#[pymethods]
impl DeviceCreateReq {
    #[new]
    fn new(device_certificate: Vec<u8>, redacted_device_certificate: Vec<u8>) -> PyResult<Self> {
        Ok(Self(device_create::Req {
            device_certificate,
            redacted_device_certificate,
        }))
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
    fn device_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.device_certificate)
    }

    #[getter]
    fn redacted_device_certificate(&self) -> PyResult<&[u8]> {
        Ok(&self.0.redacted_device_certificate)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct DeviceCreateRep(pub device_create::Rep);

#[pymethods]
impl DeviceCreateRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(device_create::Rep::Ok))
    }

    #[classmethod]
    #[pyo3(name = "InvalidCertification")]
    fn invalid_certification(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(device_create::Rep::InvalidCertification { reason }))
    }

    #[classmethod]
    #[pyo3(name = "BadUserId")]
    fn bad_user_id(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(device_create::Rep::BadUserId { reason }))
    }

    #[classmethod]
    #[pyo3(name = "InvalidData")]
    fn invalid_data(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(device_create::Rep::InvalidData { reason }))
    }

    #[classmethod]
    #[pyo3(name = "AlreadyExists")]
    fn already_exists(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(device_create::Rep::AlreadyExists { reason }))
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
        device_create::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct HumanFindReq(pub human_find::Req);

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
        let page = NonZeroU64::try_from(page).map_err(ProtocolError::new_err)?;
        let per_page = NonZeroU64::try_from(per_page).map_err(ProtocolError::new_err)?;
        Ok(Self(human_find::Req {
            query,
            omit_revoked,
            omit_non_human,
            page,
            per_page,
        }))
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

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct HumanFindResultItem(pub human_find::HumanFindResultItem);

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

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
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
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct HumanFindRep(pub human_find::Rep);

#[pymethods]
impl HumanFindRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(
        _cls: &PyType,
        results: Vec<HumanFindResultItem>,
        page: u64,
        per_page: u64,
        total: u64,
    ) -> PyResult<Self> {
        let results = results.into_iter().map(|item| item.0).collect();
        let page = NonZeroU64::try_from(page).map_err(ProtocolError::new_err)?;
        let per_page = NonZeroU64::try_from(per_page).map_err(ProtocolError::new_err)?;
        Ok(Self(human_find::Rep::Ok {
            results,
            page,
            per_page,
            total,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(human_find::Rep::NotAllowed { reason }))
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
        human_find::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
