use pyo3::{
    exceptions::PyValueError, import_exception, pyclass, pyclass::CompareOp, pymethods, PyResult,
    Python,
};

use crate::binding_utils::hash_generic;

import_exception!(parsec.api.data, EntryNameTooLongError);
import_exception!(parsec.api.data, DataError);
import_exception!(parsec.api.data, DataValidationError);

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct EntryName(pub libparsec::types::EntryName);

#[pymethods]
impl EntryName {
    #[new]
    pub fn new(name: String) -> PyResult<Self> {
        match name.parse::<libparsec::types::EntryName>() {
            Ok(en) => Ok(Self(en)),
            Err(err) => match err {
                libparsec::types::EntryNameError::NameTooLong => {
                    Err(EntryNameTooLongError::new_err("Invalid data"))
                }
                _ => Err(PyValueError::new_err("Invalid data")),
            },
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<EntryName {}>", self.0))
    }

    fn __richcmp__(&self, other: &EntryName, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0.as_ref() == other.0.as_ref(),
            CompareOp::Ne => self.0.as_ref() != other.0.as_ref(),
            CompareOp::Lt => self.0.as_ref() < other.0.as_ref(),
            CompareOp::Gt => self.0.as_ref() > other.0.as_ref(),
            CompareOp::Le => self.0.as_ref() <= other.0.as_ref(),
            CompareOp::Ge => self.0.as_ref() >= other.0.as_ref(),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.to_string(), py)
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}
