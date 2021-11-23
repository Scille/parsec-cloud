// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::{prelude::*, py_run};

mod crypto;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn libparsec(py: Python, m: &PyModule) -> PyResult<()> {
    // Rust-analyzer produce a false positive "unresolved macro `proc_macro_call`"
    // which makes the entire function pretty unreadable without this hack
    // (see. https://github.com/rust-analyzer/rust-analyzer/issues/9606)
    _libparsec(py, m)
}

fn _libparsec(py: Python, m: &PyModule) -> PyResult<()> {
    // Submodule to store all the stuff that shouldn't be exposed by libparsec
    // but must be for the moment
    let submodule = PyModule::new(py, "hazmat")?;

    submodule.add_class::<crypto::HashDigest>()?;

    // py_run! is quick-and-dirty; should be replaced by PyO3 API calls in actual code
    py_run!(
        py,
        submodule,
        "import sys; sys.modules['libparsec.hazmat'] = submodule"
    );
    m.add_submodule(submodule)?;

    Ok(())
}
