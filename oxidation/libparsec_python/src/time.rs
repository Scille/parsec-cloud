// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec::types::DateTime;
use pyo3::prelude::*;
use pyo3::{PyAny, PyResult};

use crate::binding_utils::py_to_rs_datetime;

#[pyfunction]
pub(crate) fn freeze_time(time: &PyAny) -> PyResult<()> {
    let time = py_to_rs_datetime(time).ok();
    DateTime::freeze_time(time);
    Ok(())
}
