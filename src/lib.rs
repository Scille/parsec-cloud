use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_parsec")]
fn entrypoint(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}
