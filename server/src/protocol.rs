// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    create_exception,
    exceptions::{PyAttributeError, PyException, PyValueError},
    pyclass, pymethods,
    types::{PyModule, PyType},
    IntoPy, PyAny, PyErr, PyObject, PyResult, Python,
};

use libparsec::low_level::serialization_format::python_bindings_parsec_protocol_cmds_family;

use crate::binding_utils::gen_proto;

/*
 * ProtocolError
 */

create_exception!(_parsec, ProtocolError, PyException);

#[pyclass]
pub struct ProtocolErrorFields(pub libparsec::low_level::protocol::ProtocolError);

#[pymethods]
impl ProtocolErrorFields {
    #[classmethod]
    #[pyo3(name = "EncodingError")]
    fn encoding_error(_cls: &PyType, exc: String) -> Self {
        Self(libparsec::low_level::protocol::ProtocolError::EncodingError { exc })
    }

    #[classattr]
    #[pyo3(name = "NotHandled")]
    fn not_handled() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    ProtocolErrorFields(libparsec::low_level::protocol::ProtocolError::NotHandled).into_py(py)
                })
            };
        }

        &VALUE
    }

    #[classmethod]
    #[pyo3(name = "BadRequest")]
    fn bad_request(_cls: &PyType, exc: String) -> Self {
        Self(libparsec::low_level::protocol::ProtocolError::BadRequest { exc })
    }

    #[getter]
    fn exc(&self) -> PyResult<&str> {
        match &self.0 {
            libparsec::low_level::protocol::ProtocolError::EncodingError { exc } => Ok(exc),
            libparsec::low_level::protocol::ProtocolError::DecodingError { exc } => Ok(exc),
            libparsec::low_level::protocol::ProtocolError::BadRequest { exc } => Ok(exc),
            _ => Err(PyAttributeError::new_err("No such attribute `exc`")),
        }
    }
}

gen_proto!(ProtocolErrorFields, __richcmp__, eq);
gen_proto!(ProtocolErrorFields, __str__); // Needed for python's exceptions
gen_proto!(ProtocolErrorFields, __repr__);
gen_proto!(ProtocolErrorFields, __copy__);
gen_proto!(ProtocolErrorFields, __deepcopy__);

impl From<ProtocolErrorFields> for PyErr {
    fn from(err: ProtocolErrorFields) -> Self {
        ProtocolError::new_err(err)
    }
}

impl From<libparsec::low_level::protocol::ProtocolError> for ProtocolErrorFields {
    fn from(err: libparsec::low_level::protocol::ProtocolError) -> Self {
        ProtocolErrorFields(err)
    }
}

pub type ProtocolResult<T> = Result<T, ProtocolErrorFields>;

/*
 * Custom types `ReencryptionBatchEntry`
 */

#[pyclass]
#[derive(Clone)]
pub(crate) struct ReencryptionBatchEntry(pub libparsec::low_level::types::ReencryptionBatchEntry);

crate::binding_utils::gen_proto!(ReencryptionBatchEntry, __repr__);
crate::binding_utils::gen_proto!(ReencryptionBatchEntry, __copy__);
crate::binding_utils::gen_proto!(ReencryptionBatchEntry, __deepcopy__);
crate::binding_utils::gen_proto!(ReencryptionBatchEntry, __richcmp__, eq);

#[pymethods]
impl ReencryptionBatchEntry {
    #[new]
    fn new(
        vlob_id: crate::ids::VlobID,
        version: u64,
        blob: crate::binding_utils::BytesWrapper,
    ) -> PyResult<Self> {
        crate::binding_utils::unwrap_bytes!(blob);
        let vlob_id = vlob_id.0;
        Ok(Self(libparsec::low_level::types::ReencryptionBatchEntry {
            vlob_id,
            version,
            blob,
        }))
    }

    #[getter]
    fn vlob_id(&self) -> PyResult<crate::ids::VlobID> {
        Ok(crate::ids::VlobID(self.0.vlob_id))
    }

    #[getter]
    fn version(&self) -> PyResult<u64> {
        Ok(self.0.version)
    }

    #[getter]
    fn blob(&self) -> PyResult<&[u8]> {
        Ok(&self.0.blob)
    }
}

/*
 * Custom types `ActiveUsersLimit`
 */

#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct ActiveUsersLimit(pub libparsec::low_level::types::ActiveUsersLimit);

#[pymethods]
impl ActiveUsersLimit {
    #[classmethod]
    #[pyo3(name = "LimitedTo")]
    fn limited_to(_cls: &PyType, user_count_limit: u64) -> Self {
        Self(libparsec::low_level::types::ActiveUsersLimit::LimitedTo(
            user_count_limit,
        ))
    }

    #[classattr]
    #[pyo3(name = "NO_LIMIT")]
    fn no_limit() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    ActiveUsersLimit(libparsec::low_level::types::ActiveUsersLimit::NoLimit)
                        .into_py(py)
                })
            };
        };

        &VALUE
    }

    #[classmethod]
    #[pyo3(name = "FromOptionalInt")]
    fn from_optional_int<'py>(cls: &'py PyType, py: Python<'py>, count: Option<u64>) -> &'py PyAny {
        match count {
            Some(x) => Self::limited_to(cls, x).into_py(py).into_ref(py),
            None => Self::no_limit().as_ref(py),
        }
    }

    fn to_int(&self) -> Option<u64> {
        match self.0 {
            libparsec::low_level::types::ActiveUsersLimit::LimitedTo(user_count_limit) => {
                Some(user_count_limit)
            }
            libparsec::low_level::types::ActiveUsersLimit::NoLimit => None,
        }
    }
}

crate::binding_utils::gen_proto!(ActiveUsersLimit, __richcmp__, ord);
crate::binding_utils::gen_proto!(ActiveUsersLimit, __repr__);
crate::binding_utils::gen_proto!(ActiveUsersLimit, __copy__);
crate::binding_utils::gen_proto!(ActiveUsersLimit, __deepcopy__);

python_bindings_parsec_protocol_cmds_family!(
    "../oxidation/libparsec/crates/protocol/schema/invited_cmds"
);
python_bindings_parsec_protocol_cmds_family!(
    "../oxidation/libparsec/crates/protocol/schema/authenticated_cmds"
);
python_bindings_parsec_protocol_cmds_family!(
    "../oxidation/libparsec/crates/protocol/schema/anonymous_cmds"
);

pub(crate) fn add_mod(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    invited_cmds_populate_mod(py, m)?;
    authenticated_cmds_populate_mod(py, m)?;
    anonymous_cmds_populate_mod(py, m)?;

    m.add_class::<ReencryptionBatchEntry>()?;
    m.add_class::<ActiveUsersLimit>()?;

    // Error type
    m.add_class::<ProtocolErrorFields>()?;
    m.add("ProtocolError", py.get_type::<ProtocolError>())?;

    Ok(())
}
