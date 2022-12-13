use pyo3::{import_exception, prelude::PyModule, wrap_pyfunction, PyErr, PyResult, Python};

use libparsec::core_fs::FSError;

pub(crate) mod workspace_storage;

pub(crate) fn add_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<workspace_storage::WorkspaceStorage>()?;
    m.add_class::<workspace_storage::WorkspaceStorageSnapshot>()?;
    m.add_function(wrap_pyfunction!(
        workspace_storage::workspace_storage_non_speculative_init,
        m
    )?)?;

    Ok(())
}

import_exception!(parsec.core.fs.exceptions, FSLocalStorageOperationalError);
import_exception!(parsec.core.fs.exceptions, FSLocalStorageClosedError);
import_exception!(parsec.core.fs.exceptions, FSInternalError);

pub(super) fn fs_to_python_error(e: FSError) -> PyErr {
    match e {
        FSError::DatabaseQueryError(_) | FSError::DatabaseOperationalError(_) => {
            FSLocalStorageOperationalError::new_err(e.to_string())
        }
        FSError::DatabaseClosed(_) => FSLocalStorageClosedError::new_err(e.to_string()),
        _ => FSInternalError::new_err(e.to_string()),
    }
}
