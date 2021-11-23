// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::basic::CompareOp;
use pyo3::types::{PyBool, PyByteArray};
use pyo3::{exceptions::PyValueError, prelude::*, types::PyBytes};

#[pyclass(name = "_Rs_HashDigest")]
pub(crate) struct HashDigest(parsec_api_crypto::HashDigest);

#[pymethods]
impl HashDigest {
    #[new]
    fn new(hash: &[u8]) -> PyResult<Self> {
        match parsec_api_crypto::HashDigest::try_from(hash) {
            Ok(h) => Ok(Self(h)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[staticmethod]
    fn from_data(py: Python, data: PyObject) -> PyResult<HashDigest> {
        let copy;
        let bytes = match data.extract::<&PyByteArray>(py) {
            Ok(x) => {
                copy = x.to_vec();
                &copy
            }
            Err(_) => data.extract::<&PyBytes>(py)?.as_bytes(),
        };
        Ok(Self(parsec_api_crypto::HashDigest::from_data(bytes)))
    }

    #[getter]
    fn digest<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_ref()))
    }

    fn hexdigest(&self) -> PyResult<String> {
        Ok(self.0.hexdigest())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("HashDigest({})", self.0.hexdigest()))
    }

    fn __richcmp__(&self, py: Python, value: &HashDigest, op: CompareOp) -> PyResult<PyObject> {
        match op {
            CompareOp::Eq => Ok(PyBool::new(py, self.0 == value.0).into_py(py)),
            CompareOp::Ne => Ok(PyBool::new(py, self.0 != value.0).into_py(py)),
            _ => Ok(py.NotImplemented()),
        }
    }
}
