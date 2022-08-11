// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyType};

use libparsec::protocol::authenticated_cmds::{organization_config, organization_stats};

use crate::binding_utils::py_to_rs_user_profile;

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct OrganizationStatsReq(pub organization_stats::Req);

#[pymethods]
impl OrganizationStatsReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(organization_stats::Req))
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
pub(crate) struct UsersPerProfileDetailItem(pub organization_stats::UsersPerProfileDetailItem);

#[pymethods]
impl UsersPerProfileDetailItem {
    #[new]
    fn new(profile: &PyAny, active: u64, revoked: u64) -> PyResult<Self> {
        let profile = py_to_rs_user_profile(profile)?;
        Ok(Self(organization_stats::UsersPerProfileDetailItem {
            profile,
            active,
            revoked,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct OrganizationStatsRep(pub organization_stats::Rep);

#[pymethods]
impl OrganizationStatsRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(
        _cls: &PyType,
        data_size: u64,
        metadata_size: u64,
        realms: u64,
        users: u64,
        active_users: u64,
        users_per_profile_detail: Vec<UsersPerProfileDetailItem>,
    ) -> PyResult<Self> {
        let users_per_profile_detail = users_per_profile_detail
            .into_iter()
            .map(|detail| detail.0)
            .collect();
        Ok(Self(organization_stats::Rep::Ok {
            data_size,
            metadata_size,
            realms,
            users,
            active_users,
            users_per_profile_detail,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotAllowed")]
    fn not_allowed(_cls: &PyType, reason: Option<String>) -> PyResult<Self> {
        Ok(Self(organization_stats::Rep::NotAllowed { reason }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(organization_stats::Rep::NotFound))
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
        organization_stats::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(PartialEq, Clone)]
pub(crate) struct OrganizationConfigReq(pub organization_config::Req);

#[pymethods]
impl OrganizationConfigReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(organization_config::Req))
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
pub(crate) struct OrganizationConfigRep(pub organization_config::Rep);

#[pymethods]
impl OrganizationConfigRep {
    #[classmethod]
    #[pyo3(name = "Ok")]
    fn ok(
        _cls: &PyType,
        user_profile_outsider_allowed: bool,
        active_users_limit: Option<u64>,
    ) -> PyResult<Self> {
        Ok(Self(organization_config::Rep::Ok {
            user_profile_outsider_allowed,
            active_users_limit,
        }))
    }

    #[classmethod]
    #[pyo3(name = "NotFound")]
    fn not_found(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(organization_config::Rep::NotFound))
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
        organization_config::Rep::load(&buf)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
