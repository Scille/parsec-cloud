// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::prelude::{pyfunction, wrap_pyfunction, PyModule, PyResult, Python};

pub(crate) fn add_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(toggle_local_db_in_memory, m)?)?;
    m.add_function(wrap_pyfunction!(clear_local_db_in_memory, m)?)?;

    Ok(())
}

#[pyfunction]
pub(crate) fn toggle_local_db_in_memory(enabled: bool) {
    libparsec::local_db::toggle_local_db_in_memory(enabled)
}

#[pyfunction]
fn clear_local_db_in_memory() {
    libparsec::local_db::clear_local_db_in_memory()
}
