// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyNotImplementedError,
    import_exception,
    prelude::*,
    pyclass::CompareOp,
    types::{PyBytes, PyTuple},
};

use libparsec::protocol::authenticated_cmds::{organization_config, organization_stats};

use crate::{
    binding_utils::{py_to_rs_user_profile, rs_to_py_user_profile},
    protocol::{gen_rep, Reason},
};

import_exception!(parsec.api.protocol, ProtocolError);

#[pyclass]
#[derive(Clone)]
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

    fn __richcmp__(&self, other: Self, op: CompareOp) -> PyResult<bool> {
        Ok(match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

#[pyclass]
#[derive(Clone)]
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

    fn __richcmp__(&self, other: Self, op: CompareOp) -> PyResult<bool> {
        Ok(match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn profile(&self) -> PyResult<PyObject> {
        rs_to_py_user_profile(&self.0.profile)
    }

    #[getter]
    fn active(&self) -> PyResult<u64> {
        Ok(self.0.active)
    }

    #[getter]
    fn revoked(&self) -> PyResult<u64> {
        Ok(self.0.revoked)
    }
}

gen_rep!(
    organization_stats,
    OrganizationStatsRep,
    { .. },
    [NotAllowed, reason: Reason],
    [NotFound],
);

#[pyclass(extends=OrganizationStatsRep)]
pub(crate) struct OrganizationStatsRepOk;

#[pymethods]
impl OrganizationStatsRepOk {
    #[new]
    fn new(
        data_size: u64,
        metadata_size: u64,
        realms: u64,
        users: u64,
        active_users: u64,
        users_per_profile_detail: Vec<UsersPerProfileDetailItem>,
    ) -> PyResult<(Self, OrganizationStatsRep)> {
        let users_per_profile_detail = users_per_profile_detail
            .into_iter()
            .map(|detail| detail.0)
            .collect();
        Ok((
            Self,
            OrganizationStatsRep(organization_stats::Rep::Ok {
                data_size,
                metadata_size,
                realms,
                users,
                active_users,
                users_per_profile_detail,
            }),
        ))
    }

    #[getter]
    fn data_size(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            organization_stats::Rep::Ok { data_size, .. } => data_size,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn metadata_size(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            organization_stats::Rep::Ok { metadata_size, .. } => metadata_size,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn realms(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            organization_stats::Rep::Ok { realms, .. } => realms,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn users(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            organization_stats::Rep::Ok { users, .. } => users,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn active_users(_self: PyRef<'_, Self>) -> PyResult<u64> {
        Ok(match _self.as_ref().0 {
            organization_stats::Rep::Ok { active_users, .. } => active_users,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn users_per_profile_detail<'py>(
        _self: PyRef<'py, Self>,
        py: Python<'py>,
    ) -> PyResult<&'py PyTuple> {
        Ok(match &_self.as_ref().0 {
            organization_stats::Rep::Ok {
                users_per_profile_detail,
                ..
            } => PyTuple::new(
                py,
                users_per_profile_detail
                    .iter()
                    .cloned()
                    .map(|x| UsersPerProfileDetailItem(x).into_py(py)),
            ),
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}

#[pyclass]
#[derive(Clone)]
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

    fn __richcmp__(&self, other: Self, op: CompareOp) -> PyResult<bool> {
        Ok(match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(ProtocolError::new_err)?,
        ))
    }
}

gen_rep!(
    organization_config,
    OrganizationConfigRep,
    { .. },
    [NotFound],
);

#[pyclass(extends=OrganizationConfigRep)]
pub(crate) struct OrganizationConfigRepOk;

#[pymethods]
impl OrganizationConfigRepOk {
    #[new]
    fn new(
        user_profile_outsider_allowed: bool,
        active_users_limit: Option<u64>,
        sequester_authority_certificate: Option<Vec<u8>>,
        sequester_services_certificates: Option<Vec<Vec<u8>>>,
    ) -> PyResult<(Self, OrganizationConfigRep)> {
        Ok((
            Self,
            OrganizationConfigRep(organization_config::Rep::Ok {
                user_profile_outsider_allowed,
                active_users_limit,
                sequester_authority_certificate: libparsec::types::Maybe::Present(
                    sequester_authority_certificate,
                ),
                sequester_services_certificates: libparsec::types::Maybe::Present(
                    sequester_services_certificates,
                ),
            }),
        ))
    }

    #[getter]
    fn user_profile_outsider_allowed(_self: PyRef<'_, Self>) -> PyResult<bool> {
        Ok(match _self.as_ref().0 {
            organization_config::Rep::Ok {
                user_profile_outsider_allowed,
                ..
            } => user_profile_outsider_allowed,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn active_users_limit(_self: PyRef<'_, Self>) -> PyResult<Option<u64>> {
        Ok(match _self.as_ref().0 {
            organization_config::Rep::Ok {
                active_users_limit, ..
            } => active_users_limit,
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn sequester_authority_certificate<'py>(
        _self: PyRef<'py, Self>,
        py: Python<'py>,
    ) -> PyResult<Option<&'py PyBytes>> {
        Ok(match &_self.as_ref().0 {
            organization_config::Rep::Ok {
                sequester_authority_certificate,
                ..
            } => match sequester_authority_certificate {
                libparsec::types::Maybe::Present(x) => x.as_ref().map(|x| PyBytes::new(py, x)),
                _ => None,
            },
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }

    #[getter]
    fn sequester_services_certificates<'py>(
        _self: PyRef<'_, Self>,
        py: Python<'py>,
    ) -> PyResult<Option<&'py PyTuple>> {
        Ok(match &_self.as_ref().0 {
            organization_config::Rep::Ok {
                sequester_services_certificates,
                ..
            } => match sequester_services_certificates {
                libparsec::types::Maybe::Present(x) => x
                    .as_ref()
                    .map(|x| PyTuple::new(py, x.iter().map(|x| PyBytes::new(py, x)))),
                _ => None,
            },
            _ => return Err(PyNotImplementedError::new_err("")),
        })
    }
}
