// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::Path;

use pyo3::{exceptions::PyValueError, pyclass, pymethods, types::PyType, PyResult};

#[derive(Clone)]
#[pyclass]
pub(crate) struct Regex(pub libparsec::types::Regex);

crate::binding_utils::gen_proto!(Regex, __richcmp__, eq);
crate::binding_utils::gen_proto!(Regex, __str__);

#[pymethods]
impl Regex {
    #[classmethod]
    fn from_file(_cls: &PyType, path: &str) -> PyResult<Self> {
        libparsec::types::Regex::from_file(Path::new(path))
            .map(Self)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    #[classmethod]
    fn from_raw_regexes(_cls: &PyType, raw_regexes: Vec<&str>) -> PyResult<Self> {
        libparsec::types::Regex::from_raw_regexes(&raw_regexes)
            .map(Regex)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    #[classmethod]
    fn from_glob_pattern(_cls: &PyType, glob_pattern: &str) -> PyResult<Self> {
        libparsec::types::Regex::from_glob_pattern(glob_pattern)
            .map(Regex)
            .map_err(|err| {
                PyValueError::new_err(format!(
                    "Failed to convert pattern to regex: `{}` with the following error `{}`",
                    glob_pattern, err
                ))
            })
    }

    #[classmethod]
    fn from_regex_str(_cls: &PyType, regex_str: &str) -> PyResult<Self> {
        libparsec::types::Regex::from_regex_str(regex_str)
            .map(Regex)
            .map_err(|err| {
                PyValueError::new_err(format!(
                    "Regex creation failed: `{}` with the following error `{}`",
                    regex_str, err
                ))
            })
    }

    #[getter]
    fn pattern(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(test)]
mod test {
    use pyo3::{
        exceptions::PyValueError,
        types::{PyModule, PyTuple},
        Py, PyAny, PyResult, Python,
    };

    const IGNORE_PATTERN_FILE_PATH: &str = "parsec/core/resources/default_pattern.ignore";

    fn py_to_rs_regex(regex: &PyAny) -> PyResult<regex::Regex> {
        let regex = regex
            .getattr("pattern")
            .unwrap_or(regex)
            .extract::<&str>()?
            .replace("\\Z", "\\z")
            .replace("\\ ", "\x20");
        regex::Regex::new(&regex).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn rs_to_py_regex<'py>(py: Python<'py>, regex: &regex::Regex) -> PyResult<&'py PyAny> {
        let re = py.import("re")?;
        let args = PyTuple::new(
            py,
            vec![regex.as_str().replace("\\z", "\\Z").replace('\x20', "\\ ")],
        );
        re.call_method1("compile", args)
    }

    fn load_regex() -> String {
        let file_content = std::fs::read_to_string(IGNORE_PATTERN_FILE_PATH).unwrap();
        let mut regex_str = String::new();

        for line in file_content.lines() {
            // It's a comment or empty line
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            let sub_regex_str = libparsec::types::Regex::from_glob_pattern(line.trim())
                .unwrap()
                .to_string();

            if !regex_str.is_empty() {
                regex_str.push('|');
            }
            regex_str.push_str(&sub_regex_str);
        }

        regex_str
    }

    #[test]
    fn test_compatible_with_legacy_py_to_rs() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let regex_str = format!("new_regex = '{}'", load_regex());
            let reg: Py<PyAny> = PyModule::from_code(py, &regex_str, "", "")
                .unwrap()
                .getattr("new_regex")
                .unwrap()
                .into();

            let re = PyModule::import(py, "re").unwrap();
            let compile_fn = re.getattr("compile").unwrap();
            let compile_args = PyTuple::new(py, vec![reg]);

            // Assert re can compile the new regex and transform to rust one
            let py_reg = compile_fn
                .call1(compile_args)
                .expect("Cannot compile new regex with `re` module");
            assert!(py_to_rs_regex(py_reg).is_ok());
        });
    }

    #[test]
    fn test_compatible_with_legacy_rs_to_py() {
        let regex = regex::Regex::new(&load_regex()).expect("Should be valid");
        Python::with_gil(|py| assert!(rs_to_py_regex(py, &regex).is_ok()));
    }
}
