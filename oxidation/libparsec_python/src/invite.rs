// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyBytes, PyString, PyType};
use uuid::Uuid;

use crate::binding_utils::comp_op;
use crate::binding_utils::hash_generic;

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct InvitationToken(pub parsec_api_types::InvitationToken);

#[pymethods]
impl InvitationToken {
    #[new]
    pub fn new(uuid: &PyAny) -> PyResult<Self> {
        match Uuid::parse_str(uuid.getattr("hex")?.extract::<&str>()?) {
            Ok(u) => Ok(Self(parsec_api_types::InvitationToken::from(u))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[getter]
    fn uuid<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let uuid = PyModule::import(py, "uuid")?;
        let kwargs = vec![("hex", self.hex().unwrap())].into_py_dict(py);
        match uuid.getattr("UUID")?.call((), Some(kwargs)) {
            Ok(any) => Ok(any),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(parsec_api_types::InvitationToken::default()))
    }

    #[getter]
    fn bytes<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_bytes()))
    }

    #[getter]
    fn hex(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __richcmp__(&self, py: Python, other: &InvitationToken, op: CompareOp) -> PyResult<bool> {
        let h1 = self.__hash__(py).unwrap();
        let h2 = other.__hash__(py).unwrap();
        comp_op(op, h1, h2)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.as_hyphenated())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<EntryID '{}'>", self.0.as_hyphenated()))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.as_hyphenated(), py)
    }

    #[classmethod]
    fn from_bytes(_cls: &PyType, bytes: &PyBytes) -> PyResult<Self> {
        let b = bytes.as_bytes();
        match uuid::Uuid::from_slice(b) {
            Ok(uuid) => Ok(Self(parsec_api_types::InvitationToken::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    fn from_hex(_cls: &PyType, hex: &PyString) -> PyResult<Self> {
        match hex.to_string().parse::<parsec_api_types::InvitationToken>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}
