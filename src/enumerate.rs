use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyList, PyType},
    IntoPy, PyObject, PyResult, Python,
};

use libparsec::protocol::authenticated_cmds::v2::invite_new;

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitationType(pub libparsec::types::InvitationType);

crate::binding_utils::gen_proto!(InvitationType, __repr__);
crate::binding_utils::gen_proto!(InvitationType, __richcmp__, eq);
crate::binding_utils::gen_proto!(InvitationType, __hash__);

#[pymethods]
impl InvitationType {
    #[classattr]
    #[pyo3(name = "DEVICE")]
    fn device() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationType(libparsec::types::InvitationType::Device).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classattr]
    #[pyo3(name = "USER")]
    fn user() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    InvitationType(libparsec::types::InvitationType::User).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classmethod]
    fn values<'py>(_cls: &'py PyType, py: Python<'py>) -> &'py PyList {
        PyList::new(py, [Self::device(), Self::user()])
    }

    #[getter]
    fn str(&self) -> &str {
        match self.0 {
            libparsec::types::InvitationType::Device => "DEVICE",
            libparsec::types::InvitationType::User => "USER",
        }
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<&'static PyObject> {
        match value {
            "DEVICE" => Ok(Self::device()),
            "USER" => Ok(Self::user()),
            _ => Err(PyValueError::new_err("")),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitationEmailSentStatus(pub invite_new::InvitationEmailSentStatus);

crate::binding_utils::gen_proto!(InvitationEmailSentStatus, __repr__);
crate::binding_utils::gen_proto!(InvitationEmailSentStatus, __richcmp__, eq);

#[pymethods]
impl InvitationEmailSentStatus {
    #[classattr]
    #[pyo3(name = "SUCCESS")]
    fn success() -> PyResult<&'static PyObject> {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationEmailSentStatus(invite_new::InvitationEmailSentStatus::Success).into_py(py)
                })
            };
        };

        Ok(&VALUE)
    }

    #[classattr]
    #[pyo3(name = "NOT_AVAILABLE")]
    fn not_available() -> PyResult<&'static PyObject> {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationEmailSentStatus(invite_new::InvitationEmailSentStatus::NotAvailable).into_py(py)
                })
            };
        };

        Ok(&VALUE)
    }

    #[classattr]
    #[pyo3(name = "BAD_RECIPIENT")]
    fn bad_recipient() -> PyResult<&'static PyObject> {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     InvitationEmailSentStatus(invite_new::InvitationEmailSentStatus::BadRecipient).into_py(py)
                })
            };
        };

        Ok(&VALUE)
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<Self> {
        match value {
            "SUCCESS" => Ok(Self(invite_new::InvitationEmailSentStatus::Success)),
            "NOT_AVAILABLE" => Ok(Self(invite_new::InvitationEmailSentStatus::NotAvailable)),
            "BAD_RECIPIENT" => Ok(Self(invite_new::InvitationEmailSentStatus::BadRecipient)),
            _ => Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
        }
    }

    fn __str__(&self) -> &str {
        match self.0 {
            invite_new::InvitationEmailSentStatus::Success => "SUCCESS",
            invite_new::InvitationEmailSentStatus::NotAvailable => "NOT_AVAILABLE",
            invite_new::InvitationEmailSentStatus::BadRecipient => "BAD_RECIPIENT",
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct RealmRole(pub libparsec::types::RealmRole);

crate::binding_utils::gen_proto!(RealmRole, __repr__);
crate::binding_utils::gen_proto!(RealmRole, __richcmp__, eq);
crate::binding_utils::gen_proto!(RealmRole, __hash__);

#[pymethods]
impl RealmRole {
    #[classattr]
    #[pyo3(name = "OWNER")]
    fn owner() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     RealmRole(libparsec::types::RealmRole::Owner).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "MANAGER")]
    fn manager() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     RealmRole(libparsec::types::RealmRole::Manager).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "CONTRIBUTOR")]
    fn contributor() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     RealmRole(libparsec::types::RealmRole::Contributor).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classattr]
    #[pyo3(name = "READER")]
    fn reader() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                     RealmRole(libparsec::types::RealmRole::Reader).into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classmethod]
    fn values<'py>(_cls: &'py PyType, py: Python<'py>) -> &'py PyList {
        PyList::new(
            py,
            [
                Self::owner(),
                Self::manager(),
                Self::contributor(),
                Self::reader(),
            ],
        )
    }

    #[getter]
    fn str(&self) -> &str {
        match self.0 {
            libparsec::types::RealmRole::Owner => "OWNER",
            libparsec::types::RealmRole::Manager => "MANAGER",
            libparsec::types::RealmRole::Contributor => "CONTRIBUTOR",
            libparsec::types::RealmRole::Reader => "READER",
        }
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<&'static PyObject> {
        match value {
            "OWNER" => Ok(Self::owner()),
            "MANAGER" => Ok(Self::manager()),
            "CONTRIBUTOR" => Ok(Self::contributor()),
            "READER" => Ok(Self::reader()),
            _ => Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct UserProfile(pub libparsec::types::UserProfile);

crate::binding_utils::gen_proto!(UserProfile, __repr__);
crate::binding_utils::gen_proto!(UserProfile, __richcmp__, eq);
crate::binding_utils::gen_proto!(UserProfile, __hash__);

#[pymethods]
impl UserProfile {
    #[classattr]
    #[pyo3(name = "ADMIN")]
    pub fn admin() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    UserProfile(libparsec::types::UserProfile::Admin).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classattr]
    #[pyo3(name = "STANDARD")]
    pub fn standard() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    UserProfile(libparsec::types::UserProfile::Standard).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classattr]
    #[pyo3(name = "OUTSIDER")]
    pub fn outsider() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    UserProfile(libparsec::types::UserProfile::Outsider).into_py(py)
                })
            };
        };
        &VALUE
    }

    #[classmethod]
    fn values<'py>(_cls: &PyType, py: Python<'py>) -> &'py PyList {
        PyList::new(py, [Self::admin(), Self::standard(), Self::outsider()])
    }

    #[classmethod]
    fn from_str(_cls: &PyType, value: &str) -> PyResult<&'static PyObject> {
        Ok(match value {
            "ADMIN" => Self::admin(),
            "STANDARD" => Self::standard(),
            "OUTSIDER" => Self::outsider(),
            _ => return Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
        })
    }

    #[getter]
    fn str(&self) -> &str {
        match self.0 {
            libparsec::types::UserProfile::Admin => "ADMIN",
            libparsec::types::UserProfile::Standard => "STANDARD",
            libparsec::types::UserProfile::Outsider => "OUTSIDER",
        }
    }
}

impl UserProfile {
    pub fn from_profile(profile: libparsec::types::UserProfile) -> &'static PyObject {
        match profile {
            libparsec::types::UserProfile::Admin => UserProfile::admin(),
            libparsec::types::UserProfile::Standard => UserProfile::standard(),
            libparsec::types::UserProfile::Outsider => UserProfile::outsider(),
        }
    }
}
