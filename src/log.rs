// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{types::PyModule, PyResult, Python};

pub(crate) fn add_mod(py: Python, m: &PyModule) -> PyResult<()> {
    #[cfg(feature = "test-utils")]
    {
        let _ = py;
        m.add_function(pyo3::wrap_pyfunction!(test_log_in_lib, m)?)?;
    }

    #[cfg(not(feature = "test-utils"))]
    let _ = (py, m);

    Ok(())
}

pub(crate) fn init_log() {
    pyo3_log::init();
}

#[cfg(feature = "test-utils")]
#[pyo3::pyfunction]
pub(crate) fn test_log_in_lib(level: &str, message: &str) -> PyResult<()> {
    match level {
        "TRACE" => log::trace!("message=`{message}`"),
        "DEBUG" => log::debug!("message=`{message}`"),
        "INFO" => log::info!("message=`{message}`"),
        "WARNING" => log::warn!("message=`{message}`"),
        "ERROR" => log::error!("message=`{message}`"),
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unexpected log level `{level}`"
            )))
        }
    }

    Ok(())
}
