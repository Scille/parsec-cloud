// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

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
    fn from_data(data: &[u8]) -> PyResult<HashDigest> {
        Ok(Self(parsec_api_crypto::HashDigest::from_data(data)))
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
}
