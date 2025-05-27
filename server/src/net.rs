// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use pyo3::{exceptions::PyValueError, prelude::*, pymethods, PyResult};

crate::binding_utils::gen_py_wrapper_class_for_id!(
    HostAddr,
    libparsec_types::HostAddr,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __hash__,
);

#[pymethods]
impl HostAddr {
    #[new]
    pub fn new(raw: &str) -> PyResult<Self> {
        libparsec_types::HostAddr::from_str(raw)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
