// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::prelude::*;

use crate::enumerate::UserProfile;

crate::binding_utils::gen_py_wrapper_class!(
    UsersPerProfileDetailItem,
    libparsec::low_level::types::UsersPerProfileDetailItem,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl UsersPerProfileDetailItem {
    #[new]
    fn new(profile: UserProfile, active: u64, revoked: u64) -> PyResult<Self> {
        Ok(Self(
            libparsec::low_level::types::UsersPerProfileDetailItem {
                profile: profile.0,
                active,
                revoked,
            },
        ))
    }

    #[getter]
    fn profile(&self) -> PyResult<&'static PyObject> {
        Ok(UserProfile::convert(self.0.profile))
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
