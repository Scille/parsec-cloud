// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(feature = "test-utils")]
use pyo3::pyfunction;
use pyo3::{types::PyModule, PyResult, Python};

pub(crate) fn add_mod(py: Python, m: &PyModule) -> PyResult<()> {
    #[cfg(feature = "test-utils")]
    {
        let _ = py;
        m.add_function(pyo3::wrap_pyfunction!(test_log_in_lib, m)?)?;
        m.add_function(pyo3::wrap_pyfunction!(test_log_deadlock, m)?)?;
    }

    #[cfg(not(feature = "test-utils"))]
    let _ = (py, m);

    Ok(())
}

pub(crate) fn init_log() {
    pyo3_log::init();
}

#[cfg(feature = "test-utils")]
#[pyfunction]
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

#[cfg(feature = "test-utils")]
#[pyfunction]
pub(crate) fn test_log_deadlock() {
    log::info!("This logs fine");

    let thread1 = std::thread::spawn(|| log::info!("But this ?"));
    let thread2 = std::thread::spawn(|| log::info!("Let's try again !"));

    thread1.join().unwrap();
    thread2.join().unwrap();
}
