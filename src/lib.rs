use pyo3::prelude::*;

mod api_crypto;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_parsec")]
fn entrypoint(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<api_crypto::HashDigest>()?;
    m.add_class::<api_crypto::SigningKey>()?;
    m.add_class::<api_crypto::VerifyKey>()?;
    m.add_class::<api_crypto::SecretKey>()?;
    m.add_class::<api_crypto::PrivateKey>()?;
    m.add_class::<api_crypto::PublicKey>()?;
    Ok(())
}
