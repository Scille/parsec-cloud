// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes, PyType};

import_exception!(nacl.exceptions, CryptoError);

#[pyclass]
#[derive(PartialEq, Eq)]
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
        let bytes = match data.extract::<&PyByteArray>(py) {
            // Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
            // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
            // python thread modifying the bytearray behind our back.
            Ok(x) => unsafe { x.as_bytes() },
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

    fn __richcmp__(&self, py: Python, value: &HashDigest, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => (self == value).into_py(py),
            CompareOp::Ne => (self != value).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

#[pyclass]
#[derive(PartialEq, Eq)]
pub(crate) struct SecretKey(parsec_api_crypto::SecretKey);

#[pymethods]
impl SecretKey {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        match parsec_api_crypto::SecretKey::try_from(data) {
            Ok(h) => Ok(Self(h)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    fn generate(_cls: &PyType) -> PyResult<SecretKey> {
        Ok(SecretKey(parsec_api_crypto::SecretKey::generate()))
    }

    #[getter]
    fn secret<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_ref()))
    }

    pub fn encrypt<'p>(&self, py: Python<'p>, data: &[u8]) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.encrypt(data)))
    }

    pub fn decrypt<'p>(&self, py: Python<'p>, ciphered: &[u8]) -> PyResult<&'p PyBytes> {
        match self.0.decrypt(ciphered) {
            Ok(v) => Ok(PyBytes::new(py, &v)),
            Err(err) => Err(CryptoError::new_err(err)),
        }
    }

    pub fn hmac<'p>(&self, py: Python<'p>, data: &[u8]) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.hmac(data)))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(String::from("SecretKey"))
    }

    fn __richcmp__(&self, py: Python, value: &SecretKey, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => (self == value).into_py(py),
            CompareOp::Ne => (self != value).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}
