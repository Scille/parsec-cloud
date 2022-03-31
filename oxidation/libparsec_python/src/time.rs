// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use parsec_api_types::DateTime;
use pyo3::prelude::*;
use pyo3::{PyAny, PyResult};

use crate::binding_utils::py_to_rs_datetime;

#[pyfunction]
pub(crate) fn freeze_time(time: &PyAny) -> PyResult<()> {
    let time = py_to_rs_datetime(time)?;
    DateTime::freeze_time(time);
    Ok(())
}
