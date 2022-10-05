// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{exceptions::PyValueError, pyclass, pymethods, types::PyType, PyResult};

#[pyclass]
pub(crate) struct Regex(pub libparsec::types::regex::Regex);

crate::binding_utils::gen_proto!(Regex, __richcmp__, eq);
crate::binding_utils::gen_proto!(Regex, __str__);

#[pymethods]
impl Regex {
    #[classmethod]
    fn from_pattern(_cls: &PyType, pattern: &str) -> PyResult<Self> {
        libparsec::types::regex::Regex::from_pattern(pattern)
            .map(Regex)
            .map_err(|_| PyValueError::new_err(format!("Invalid pattern `{}`", pattern)))
    }

    #[classmethod]
    fn from_regex_str(_cls: &PyType, regex_str: &str) -> PyResult<Self> {
        libparsec::types::regex::Regex::from_regex_str(regex_str)
            .map(Regex)
            .map_err(|_| PyValueError::new_err(format!("Invalid regex `{}`", regex_str)))
    }

    #[getter]
    fn pattern(&self) -> PyResult<&str> {
        Ok(self.0.as_ref())
    }
}
