// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    create_exception,
    exceptions::{PyAttributeError, PyException, PyValueError},
    pyclass, pymethods,
    types::{PyModule, PyType},
    IntoPy, PyAny, PyErr, PyObject, PyResult, Python,
};

use libparsec_serialization_format::python_bindings_parsec_protocol_cmds_family;

/*
 * ProtocolError
 */

create_exception!(_parsec, ProtocolError, PyException);

crate::binding_utils::gen_py_wrapper_class!(
    ProtocolErrorFields,
    libparsec_protocol::ProtocolError,
    __repr__,
    __str__, // Needed for python's exceptions
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl ProtocolErrorFields {
    #[classmethod]
    #[pyo3(name = "EncodingError")]
    fn encoding_error(_cls: &PyType, exc: String) -> Self {
        Self(libparsec_protocol::ProtocolError::EncodingError { exc })
    }

    #[classattr]
    #[pyo3(name = "NotHandled")]
    fn not_handled() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    ProtocolErrorFields(libparsec_protocol::ProtocolError::NotHandled).into_py(py)
                })
            };
        }

        &VALUE
    }

    #[classmethod]
    #[pyo3(name = "BadRequest")]
    fn bad_request(_cls: &PyType, exc: String) -> Self {
        Self(libparsec_protocol::ProtocolError::BadRequest { exc })
    }

    #[getter]
    fn exc(&self) -> PyResult<&str> {
        match &self.0 {
            libparsec_protocol::ProtocolError::EncodingError { exc } => Ok(exc),
            libparsec_protocol::ProtocolError::DecodingError { exc } => Ok(exc),
            libparsec_protocol::ProtocolError::BadRequest { exc } => Ok(exc),
            _ => Err(PyAttributeError::new_err("No such attribute `exc`")),
        }
    }
}

impl From<ProtocolErrorFields> for PyErr {
    fn from(err: ProtocolErrorFields) -> Self {
        ProtocolError::new_err(err)
    }
}

pub(crate) type ProtocolResult<T> = Result<T, ProtocolErrorFields>;

/*
 * Custom types `ReencryptionBatchEntry`
 */

crate::binding_utils::gen_py_wrapper_class!(
    ReencryptionBatchEntry,
    libparsec_types::ReencryptionBatchEntry,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

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
        Ok(Self(libparsec_types::ReencryptionBatchEntry {
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

crate::binding_utils::gen_py_wrapper_class!(
    ActiveUsersLimit,
    libparsec_types::ActiveUsersLimit,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ ord,
);

#[pymethods]
impl ActiveUsersLimit {
    #[classmethod]
    #[pyo3(name = "LimitedTo")]
    fn limited_to(_cls: &PyType, user_count_limit: u64) -> Self {
        Self(libparsec_types::ActiveUsersLimit::LimitedTo(
            user_count_limit,
        ))
    }

    #[classattr]
    #[pyo3(name = "NO_LIMIT")]
    fn no_limit() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    ActiveUsersLimit(libparsec_types::ActiveUsersLimit::NoLimit)
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
            libparsec_types::ActiveUsersLimit::LimitedTo(user_count_limit) => {
                Some(user_count_limit)
            }
            libparsec_types::ActiveUsersLimit::NoLimit => None,
        }
    }
}

python_bindings_parsec_protocol_cmds_family!("../libparsec/crates/protocol/schema/invited_cmds");
python_bindings_parsec_protocol_cmds_family!(
    "../libparsec/crates/protocol/schema/authenticated_cmds"
);
python_bindings_parsec_protocol_cmds_family!("../libparsec/crates/protocol/schema/anonymous_cmds");

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
