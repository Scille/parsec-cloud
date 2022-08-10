use pyo3::prelude::{pymodule, wrap_pyfunction, PyModule, PyResult, Python};

mod api_crypto;
mod binding_utils;
mod ids;
mod invite;

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

    m.add_class::<ids::OrganizationID>()?;
    m.add_class::<ids::EntryID>()?;
    m.add_class::<ids::BlockID>()?;
    m.add_class::<ids::RealmID>()?;
    m.add_class::<ids::VlobID>()?;
    m.add_class::<ids::ChunkID>()?;
    m.add_class::<ids::HumanHandle>()?;
    m.add_class::<ids::DeviceID>()?;
    m.add_class::<ids::DeviceName>()?;
    m.add_class::<ids::DeviceLabel>()?;
    m.add_class::<ids::UserID>()?;

    m.add_class::<invite::InvitationToken>()?;
    m.add_class::<invite::SASCode>()?;
    m.add_function(wrap_pyfunction!(invite::generate_sas_codes, m)?)?;
    m.add_function(wrap_pyfunction!(invite::generate_sas_code_candidates, m)?)?;
    m.add_class::<invite::InviteUserData>()?;
    m.add_class::<invite::InviteUserConfirmation>()?;
    m.add_class::<invite::InviteDeviceData>()?;
    m.add_class::<invite::InviteDeviceConfirmation>()?;
    Ok(())
}
