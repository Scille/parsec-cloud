// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::prelude::*;

mod crypto;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _libparsec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<crypto::HashDigest>()?;
    m.add_class::<crypto::SigningKey>()?;
    m.add_class::<crypto::VerifyKey>()?;
    Ok(())
}
