use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyBytes, PyType},
    PyResult, Python,
};

use crate::{
    binding_utils::gen_proto,
    protocol::{ProtocolErrorFields, ProtocolResult},
};

#[pyclass]
pub(crate) struct ApiVersion(libparsec::protocol::ApiVersion);

#[pymethods]
impl ApiVersion {
    #[new]
    fn new(version: u32, revision: u32) -> Self {
        Self(libparsec::protocol::ApiVersion { version, revision })
    }

    fn dump<'py>(&self, py: Python<'py>) -> ProtocolResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.clone().dump().map_err(|e| {
                ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError {
                    exc: e.to_string(),
                })
            })?,
        ))
    }

    #[classmethod]
    fn from_str(_cls: &PyType, version_str: &str) -> PyResult<Self> {
        libparsec::protocol::ApiVersion::try_from(version_str)
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
        const API_V1_VERSION: ApiVersion = Self(libparsec::protocol::API_V1_VERSION);
        API_V1_VERSION
    }

    #[classattr]
    #[pyo3(name = "API_V2_VERSION")]
    fn api_v2_version() -> Self {
        const API_V2_VERSION: ApiVersion = Self(libparsec::protocol::API_V2_VERSION);
        API_V2_VERSION
    }

    #[classattr]
    #[pyo3(name = "API_VERSION")]
    fn api_version_number() -> Self {
        const API_VERSION: ApiVersion = Self(libparsec::protocol::API_VERSION);
        API_VERSION
    }
}

gen_proto!(ApiVersion, __str__);
gen_proto!(ApiVersion, __repr__);
gen_proto!(ApiVersion, __richcmp__, ord);
