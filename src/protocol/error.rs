use libparsec::protocol;
use pyo3::{
    create_exception,
    exceptions::{PyAttributeError, PyException},
    pyclass, pymethods,
    types::PyType,
    PyErr, PyResult,
};

use crate::binding_utils::gen_proto;

create_exception!(_parsec, ProtocolError, PyException);

#[pyclass]
pub struct ProtocolErrorFields(pub protocol::ProtocolError);

#[pymethods]
impl ProtocolErrorFields {
    #[classmethod]
    #[pyo3(name = "EncodingError")]
    fn encoding_error(_cls: &PyType, exc: String) -> Self {
        Self(protocol::ProtocolError::EncodingError { exc })
    }

    #[classmethod]
    #[pyo3(name = "NotHandled")]
    fn not_handled(_cls: &PyType) -> Self {
        Self(protocol::ProtocolError::NotHandled)
    }

    #[classmethod]
    #[pyo3(name = "BadRequest")]
    fn bad_request(_cls: &PyType, exc: String) -> Self {
        Self(protocol::ProtocolError::BadRequest { exc })
    }

    #[getter]
    fn exc(&self) -> PyResult<&str> {
        match &self.0 {
            protocol::ProtocolError::EncodingError { exc } => Ok(exc),
            protocol::ProtocolError::DecodingError { exc } => Ok(exc),
            protocol::ProtocolError::BadRequest { exc } => Ok(exc),
            _ => Err(PyAttributeError::new_err("No such attribute `exc`")),
        }
    }
}

gen_proto!(ProtocolErrorFields, __richcmp__, eq);
gen_proto!(ProtocolErrorFields, __str__); // Needed for python's exceptions
gen_proto!(ProtocolErrorFields, __repr__);

impl From<ProtocolErrorFields> for PyErr {
    fn from(err: ProtocolErrorFields) -> Self {
        ProtocolError::new_err(err)
    }
}

impl From<protocol::ProtocolError> for ProtocolErrorFields {
    fn from(err: protocol::ProtocolError) -> Self {
        ProtocolErrorFields(err)
    }
}

pub type ProtocolResult<T> = Result<T, ProtocolErrorFields>;
