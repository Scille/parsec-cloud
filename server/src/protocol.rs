// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    prelude::{PyAnyMethods, PyModuleMethods},
    pyclass, pymethods,
    types::{PyInt, PyModule, PyType},
    Bound, IntoPyObjectExt, PyObject, PyResult, Python,
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
    fn limited_to(_cls: Bound<'_, PyType>, user_count_limit: Bound<'_, PyInt>) -> PyResult<Self> {
        // We cannot let PyO3 handles `u64` conversion since it raises an `OverflowError` (which
        // doesn't inherit from `ValueError`) if `user_count_limit` is negative or too big :(
        let user_count_limit: u64 = user_count_limit
            .extract()
            .map_err(|_| PyValueError::new_err("Expected u64"))?;
        Ok(Self(libparsec_types::ActiveUsersLimit::LimitedTo(
            user_count_limit,
        )))
    }

    #[classattr]
    #[pyo3(name = "NO_LIMIT")]
    fn no_limit() -> &'static PyObject {
        lazy_static::lazy_static! {
            static ref VALUE: PyObject = {
                Python::with_gil(|py| {
                    ActiveUsersLimit(libparsec_types::ActiveUsersLimit::NoLimit)
                        .into_py_any(py).expect("Failed to generate static value for ActiveUsersLimit::no_limit")
                })
            };
        };

        &VALUE
    }

    #[classmethod]
    #[pyo3(signature = (count=None))]
    fn from_maybe_int<'py>(
        cls: Bound<'py, PyType>,
        py: Python<'py>,
        count: Option<Bound<'py, PyInt>>,
    ) -> PyResult<PyObject> {
        match count {
            Some(x) => Self::limited_to(cls, x).and_then(|x| x.into_py_any(py)),
            None => Self::no_limit().into_py_any(py),
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

python_bindings_parsec_protocol_cmds_family!("../libparsec/crates/protocol/schema/anonymous_cmds");
python_bindings_parsec_protocol_cmds_family!("../libparsec/crates/protocol/schema/invited_cmds");
python_bindings_parsec_protocol_cmds_family!(
    "../libparsec/crates/protocol/schema/authenticated_cmds"
);
python_bindings_parsec_protocol_cmds_family!("../libparsec/crates/protocol/schema/tos_cmds");
python_bindings_parsec_protocol_cmds_family!(
    "../libparsec/crates/protocol/schema/anonymous_server_cmds"
);
python_bindings_parsec_protocol_cmds_family!(
    "../libparsec/crates/protocol/schema/authenticated_account_cmds"
);

pub(crate) fn add_mod(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    anonymous_cmds_populate_mod(py, m)?;
    invited_cmds_populate_mod(py, m)?;
    authenticated_cmds_populate_mod(py, m)?;
    tos_cmds_populate_mod(py, m)?;
    anonymous_server_cmds_populate_mod(py, m)?;
    authenticated_account_cmds_populate_mod(py, m)?;

    m.add_class::<ActiveUsersLimit>()?;

    Ok(())
}
