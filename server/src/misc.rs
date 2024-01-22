// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyBytes, PyType},
    PyResult, Python,
};

crate::binding_utils::gen_py_wrapper_class!(
    ApiVersion,
    libparsec_types::ApiVersion,
    __str__,
    __copy__,
    __deepcopy__,
    __repr__,
    __richcmp__ ord,
);

#[pymethods]
impl ApiVersion {
    #[new]
    fn new(version: u32, revision: u32) -> Self {
        Self(libparsec_types::ApiVersion { version, revision })
    }

    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self
                .0
                .clone()
                .dump()
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        ))
    }

    #[classmethod]
    fn from_bytes(_cls: &PyType, bytes: &[u8]) -> PyResult<Self> {
        Ok(Self(
            libparsec_types::ApiVersion::load(bytes)
                .map_err(|err| PyValueError::new_err(err.to_string()))?,
        ))
    }

    #[classmethod]
    fn from_str(_cls: &PyType, version_str: &str) -> PyResult<Self> {
        libparsec_types::ApiVersion::try_from(version_str)
            .map(Self)
            .map_err(PyValueError::new_err)
    }

    #[getter]
    fn version(&self) -> u32 {
        self.0.version
    }

    #[getter]
    fn revision(&self) -> u32 {
        self.0.revision
    }

    #[classattr]
    #[pyo3(name = "API_V1_VERSION")]
    fn api_v1_version() -> Self {
        const API_V1_VERSION: ApiVersion = Self(*libparsec_protocol::API_V1_VERSION);
        API_V1_VERSION
    }

    #[classattr]
    #[pyo3(name = "API_V2_VERSION")]
    fn api_v2_version() -> Self {
        const API_V2_VERSION: ApiVersion = Self(*libparsec_protocol::API_V2_VERSION);
        API_V2_VERSION
    }

    #[classattr]
    #[pyo3(name = "API_V3_VERSION")]
    fn api_v3_version() -> Self {
        const API_V3_VERSION: ApiVersion = Self(*libparsec_protocol::API_V3_VERSION);
        API_V3_VERSION
    }

    #[classattr]
    #[pyo3(name = "API_V4_VERSION")]
    fn api_v4_version() -> Self {
        const API_V4_VERSION: ApiVersion = Self(*libparsec_protocol::API_V4_VERSION);
        API_V4_VERSION
    }

    #[classattr]
    #[pyo3(name = "API_LATEST_VERSION")]
    fn api_version_number() -> Self {
        const API_LATEST_VERSION: ApiVersion = Self(*libparsec_protocol::API_LATEST_VERSION);
        API_LATEST_VERSION
    }
}
