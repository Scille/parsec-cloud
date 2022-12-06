// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyNotImplementedError,
    prelude::*,
    types::{PyBytes, PyTuple},
};

use libparsec::{
    protocol::{
        anonymous_cmds::v2::organization_bootstrap,
        authenticated_cmds::v2::{organization_config, organization_stats},
    },
    types::Maybe,
};

use crate::{
    data::UsersPerProfileDetailItem,
    api_crypto::VerifyKey,
    enumerate::UserProfile,
    protocol::{
        error::{ProtocolError, ProtocolErrorFields, ProtocolResult},
        gen_rep, OptionalDateTime, OptionalFloat, Reason,
    },
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct OrganizationStatsReq(pub organization_stats::Req);

crate::binding_utils::gen_proto!(OrganizationStatsReq, __repr__);
crate::binding_utils::gen_proto!(OrganizationStatsReq, __richcmp__, eq);

#[pymethods]
impl OrganizationStatsReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(organization_stats::Req))
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

crate::binding_utils::gen_proto!(OrganizationConfigReq, __repr__);
crate::binding_utils::gen_proto!(OrganizationConfigReq, __richcmp__, eq);

#[pymethods]
impl OrganizationConfigReq {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(organization_config::Req))
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

#[pyclass]
#[derive(Clone)]
pub(crate) struct OrganizationBootstrapReq(pub organization_bootstrap::Req);

crate::binding_utils::gen_proto!(OrganizationBootstrapReq, __repr__);
crate::binding_utils::gen_proto!(OrganizationBootstrapReq, __richcmp__, eq);

#[pymethods]
impl OrganizationBootstrapReq {
    #[new]
    fn new(
        bootstrap_token: String,
        root_verify_key: VerifyKey,
        user_certificate: Vec<u8>,
        device_certificate: Vec<u8>,
        redacted_user_certificate: Vec<u8>,
        redacted_device_certificate: Vec<u8>,
        sequester_authority_certificate: Option<Vec<u8>>,
    ) -> PyResult<Self> {
        Ok(Self(organization_bootstrap::Req {
            bootstrap_token,
            root_verify_key: root_verify_key.0,
            user_certificate,
            device_certificate,
            redacted_user_certificate,
            redacted_device_certificate,
            sequester_authority_certificate: Maybe::Present(sequester_authority_certificate),
        }))
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self
                .0
                .clone()
                .dump()
                .map_err(|e| ProtocolError::new_err(format!("encoding error: {e}")))?,
        ))
    }

    #[getter]
    fn bootstrap_token(&self) -> &str {
        &self.0.bootstrap_token
    }

    #[getter]
    fn root_verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.root_verify_key.clone())
    }

    #[getter]
    fn user_certificate(&self) -> &[u8] {
        &self.0.user_certificate
    }

    #[getter]
    fn device_certificate(&self) -> &[u8] {
        &self.0.device_certificate
    }

    #[getter]
    fn redacted_user_certificate(&self) -> &[u8] {
        &self.0.redacted_user_certificate
    }

    #[getter]
    fn redacted_device_certificate(&self) -> &[u8] {
        &self.0.redacted_device_certificate
    }

    #[getter]
    fn sequester_authority_certificate(&self) -> Option<&[u8]> {
        match &self.0.sequester_authority_certificate {
            Maybe::Present(x) => x.as_ref().map(|x| &x[..]),
            Maybe::Absent => None,
        }
    }
}

gen_rep!(
    organization_bootstrap,
    OrganizationBootstrapRep,
    [InvalidCertification, reason: Reason],
    [InvalidData, reason: Reason],
    [
        BadTimestamp,
        reason: Reason,
        ballpark_client_early_offset: OptionalFloat,
        ballpark_client_late_offset: OptionalFloat,
        backend_timestamp: OptionalDateTime,
        client_timestamp: OptionalDateTime,
    ],
    [AlreadyBootstrapped],
    [NotFound],
);

#[pyclass(extends=OrganizationBootstrapRep)]
pub(crate) struct OrganizationBootstrapRepOk;

#[pymethods]
impl OrganizationBootstrapRepOk {
    #[new]
    fn new() -> (Self, OrganizationBootstrapRep) {
        (
            Self,
            OrganizationBootstrapRep(organization_bootstrap::Rep::Ok),
        )
    }
}
