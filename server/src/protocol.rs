// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyModule, PyType},
    IntoPy, PyAny, PyObject, PyResult, Python,
};

use libparsec_serialization_format::python_bindings_parsec_protocol_cmds_family;

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
    fn from_maybe_int<'py>(cls: &'py PyType, py: Python<'py>, count: Option<u64>) -> &'py PyAny {
        match count {
            Some(x) => Self::limited_to(cls, x).into_py(py).into_ref(py),
            None => Self::no_limit().as_ref(py),
        }
    }

    fn to_maybe_int(&self) -> Option<u64> {
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

    m.add_class::<ActiveUsersLimit>()?;

    Ok(())
}
