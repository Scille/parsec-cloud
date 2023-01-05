use pyo3::prelude::*;

use crate::enumerate::UserProfile;

#[pyclass]
#[derive(Clone)]
pub(crate) struct UsersPerProfileDetailItem(pub libparsec::types::UsersPerProfileDetailItem);

crate::binding_utils::gen_proto!(UsersPerProfileDetailItem, __repr__);
crate::binding_utils::gen_proto!(UsersPerProfileDetailItem, __richcmp__, eq);

#[pymethods]
impl UsersPerProfileDetailItem {
    #[new]
    fn new(profile: UserProfile, active: u64, revoked: u64) -> PyResult<Self> {
        Ok(Self(libparsec::types::UsersPerProfileDetailItem {
            profile: profile.0,
            active,
            revoked,
        }))
    }

    #[getter]
    fn profile(&self) -> PyResult<&'static PyObject> {
        Ok(UserProfile::from_profile(self.0.profile))
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
