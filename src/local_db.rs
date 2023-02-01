// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::prelude::{pyfunction, wrap_pyfunction, PyModule, PyResult, Python};

pub(crate) fn add_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_toggle_local_db_in_memory_mock, m)?)?;
    m.add_function(wrap_pyfunction!(test_clear_local_db_in_memory_mock, m)?)?;

    Ok(())
}

#[pyfunction]
pub(crate) fn test_toggle_local_db_in_memory_mock(enabled: bool) {
    libparsec::local_db::test_toggle_local_db_in_memory_mock(enabled)
}

#[pyfunction]
fn test_clear_local_db_in_memory_mock() {
    libparsec::local_db::test_clear_local_db_in_memory_mock()
}
