// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    prelude::*,
    types::{PyBytes, PyTuple},
};

use crate::{
    binding_utils::BytesWrapper, data::UsersPerProfileDetailItem, protocol::ActiveUsersLimit,
};

#[pyclass]
pub(crate) struct OrganizationStats(pub libparsec::types::OrganizationStats);

crate::binding_utils::gen_proto!(OrganizationStats, __repr__);
crate::binding_utils::gen_proto!(OrganizationStats, __copy__);
crate::binding_utils::gen_proto!(OrganizationStats, __deepcopy__);
crate::binding_utils::gen_proto!(OrganizationStats, __richcmp__, eq);

#[pymethods]
impl OrganizationStats {
    #[new]
    fn new(
        users: u64,
        active_users: u64,
        realms: u64,
        data_size: u64,
        metadata_size: u64,
        users_per_profile_detail: Vec<UsersPerProfileDetailItem>,
    ) -> Self {
        Self(libparsec::types::OrganizationStats {
            users,
            active_users,
            realms,
            data_size,
            metadata_size,
            users_per_profile_detail: users_per_profile_detail.into_iter().map(|x| x.0).collect(),
        })
    }

    #[getter]
    fn users(&self) -> u64 {
        self.0.users
    }

    #[getter]
    fn active_users(&self) -> u64 {
        self.0.active_users
    }

    #[getter]
    fn realms(&self) -> u64 {
        self.0.realms
    }

    #[getter]
    fn data_size(&self) -> u64 {
        self.0.data_size
    }

    #[getter]
    fn metadata_size(&self) -> u64 {
        self.0.metadata_size
    }

    #[getter]
    fn users_per_profile_detail<'py>(&self, py: Python<'py>) -> &'py PyTuple {
        PyTuple::new(
            py,
            self.0
                .users_per_profile_detail
                .iter()
                .map(|x| UsersPerProfileDetailItem(*x).into_py(py)),
        )
    }
}

#[pyclass]
pub(crate) struct OrganizationConfig(libparsec::types::OrganizationConfig);

crate::binding_utils::gen_proto!(OrganizationConfig, __repr__);
crate::binding_utils::gen_proto!(OrganizationConfig, __copy__);
crate::binding_utils::gen_proto!(OrganizationConfig, __deepcopy__);
crate::binding_utils::gen_proto!(OrganizationConfig, __richcmp__, eq);

#[pymethods]
impl OrganizationConfig {
    #[new]
    fn new(
        user_profile_outsider_allowed: bool,
        active_users_limit: ActiveUsersLimit,
        sequester_authority: Option<BytesWrapper>,
        sequester_services: Option<Vec<BytesWrapper>>,
    ) -> Self {
        crate::binding_utils::unwrap_bytes!(sequester_authority, sequester_services);
        Self(libparsec::types::OrganizationConfig {
            user_profile_outsider_allowed,
            active_users_limit: active_users_limit.0,
            sequester_authority,
            sequester_services,
        })
    }

    #[getter]
    fn user_profile_outsider_allowed(&self) -> bool {
        self.0.user_profile_outsider_allowed
    }

    #[getter]
    fn active_users_limit(&self) -> ActiveUsersLimit {
        ActiveUsersLimit(self.0.active_users_limit)
    }

    #[getter]
    fn sequester_authority<'py>(&self, py: Python<'py>) -> Option<&'py PyBytes> {
        self.0
            .sequester_authority
            .as_ref()
            .map(|x| PyBytes::new(py, x))
    }

    #[getter]
    fn sequester_services<'py>(&self, py: Python<'py>) -> Option<&'py PyTuple> {
        self.0
            .sequester_services
            .as_ref()
            .map(|x| PyTuple::new(py, x.iter().map(|bytes| PyBytes::new(py, bytes))))
    }

    #[getter]
    fn is_sequestered_organization(&self) -> bool {
        self.0.is_sequestered_organization()
    }
}
