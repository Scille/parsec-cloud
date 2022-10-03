// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
use pyo3::{exceptions::PyValueError, pyclass, pymethods, types::PyType, PyResult};

use crate::binding_utils::gen_proto;

#[pyclass]
pub(crate) struct Regex(pub libparsec::types::regex::Regex);

#[pymethods]
impl Regex {
    #[classmethod]
    fn from_pattern(_cls: &PyType, pattern: &str) -> PyResult<Self> {
        if let Ok(reg) = libparsec::types::regex::Regex::from_pattern(pattern) {
            Ok(Regex(reg))
        } else {
            Err(PyValueError::new_err(format!(
                "Invalid pattern `{}`",
                pattern
            )))
        }
    }

    #[classmethod]
    fn from_regex_str(_cls: &PyType, regex_str: &str) -> PyResult<Self> {
        if let Ok(reg) = libparsec::types::regex::Regex::from_regex_str(regex_str) {
            Ok(Regex(reg))
        } else {
            Err(PyValueError::new_err(format!(
                "Invalid regex `{}`",
                regex_str
            )))
        }
    }

    fn __str__(&self) -> String {
        self.0 .0.to_string()
    }

    #[getter]
    fn pattern(&self) -> String {
        self.0 .0.to_string()
    }
}

gen_proto!(Regex, __richcmp__, eq);
