use pyo3::prelude::{pymodule, wrap_pyfunction, PyModule, PyResult, Python};

mod addrs;
mod api_crypto;
mod binding_utils;
mod data;
mod enumerate;
mod file_operations;
mod ids;
mod local_device;
mod misc;
mod protocol;
mod regex;
mod runtime;
mod time;
mod trustchain;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_parsec")]
fn entrypoint(py: Python, m: &PyModule) -> PyResult<()> {
    crate::data::add_mod(py, m)?;
    crate::protocol::add_mod(py, m)?;

    m.add_class::<addrs::BackendAddr>()?;
    m.add_class::<addrs::BackendOrganizationAddr>()?;
    m.add_class::<addrs::BackendActionAddr>()?;
    m.add_class::<addrs::BackendOrganizationBootstrapAddr>()?;
    m.add_class::<addrs::BackendOrganizationFileLinkAddr>()?;
    m.add_class::<addrs::BackendInvitationAddr>()?;
    m.add_class::<addrs::BackendPkiEnrollmentAddr>()?;
    m.add_function(wrap_pyfunction!(addrs::export_root_verify_key, m)?)?;

    m.add_class::<api_crypto::HashDigest>()?;
    m.add_class::<api_crypto::SigningKey>()?;
    m.add_class::<api_crypto::VerifyKey>()?;
    m.add_class::<api_crypto::SecretKey>()?;
    m.add_class::<api_crypto::PrivateKey>()?;
    m.add_class::<api_crypto::PublicKey>()?;
    m.add_class::<api_crypto::SequesterPrivateKeyDer>()?;
    m.add_class::<api_crypto::SequesterPublicKeyDer>()?;
    m.add_class::<api_crypto::SequesterSigningKeyDer>()?;
    m.add_class::<api_crypto::SequesterVerifyKeyDer>()?;
    m.add_function(wrap_pyfunction!(api_crypto::generate_nonce, m)?)?;

    m.add_class::<enumerate::ClientType>()?;
    m.add_class::<enumerate::InvitationEmailSentStatus>()?;
    m.add_class::<enumerate::InvitationDeletedReason>()?;
    m.add_class::<enumerate::InvitationStatus>()?;
    m.add_class::<enumerate::InvitationType>()?;
    m.add_class::<enumerate::RealmRole>()?;
    m.add_class::<enumerate::UserProfile>()?;
    m.add_class::<enumerate::CoreEvent>()?;

    m.add_function(wrap_pyfunction!(file_operations::prepare_read, m)?)?;
    m.add_function(wrap_pyfunction!(file_operations::prepare_write, m)?)?;
    m.add_function(wrap_pyfunction!(file_operations::prepare_resize, m)?)?;
    m.add_function(wrap_pyfunction!(file_operations::prepare_reshape, m)?)?;

    m.add_class::<ids::OrganizationID>()?;
    m.add_class::<ids::EntryID>()?;
    m.add_class::<ids::BlockID>()?;
    m.add_class::<ids::RealmID>()?;
    m.add_class::<ids::VlobID>()?;
    m.add_class::<ids::ChunkID>()?;
    m.add_class::<ids::SequesterServiceID>()?;
    m.add_class::<ids::EnrollmentID>()?;
    m.add_class::<ids::HumanHandle>()?;
    m.add_class::<ids::DeviceID>()?;
    m.add_class::<ids::DeviceName>()?;
    m.add_class::<ids::DeviceLabel>()?;
    m.add_class::<ids::UserID>()?;
    m.add_class::<ids::InvitationToken>()?;

    m.add_class::<local_device::LocalDevice>()?;
    m.add_class::<local_device::UserInfo>()?;
    m.add_class::<local_device::DeviceInfo>()?;

    // Time
    m.add_function(wrap_pyfunction!(time::mock_time, m)?)?;
    m.add_class::<time::TimeProvider>()?;
    m.add_class::<time::DateTime>()?;
    m.add_class::<time::LocalDateTime>()?;

    // Regex
    m.add_class::<regex::Regex>()?;

    m.add_class::<trustchain::TrustchainContext>()?;
    m.add_class::<trustchain::TrustchainError>()?;
    m.add(
        "TrustchainErrorException",
        py.get_type::<trustchain::TrustchainErrorException>(),
    )?;

    // Registering ABC classes
    m.add_class::<runtime::FutureIntoCoroutine>()?;
    let future_into_coroutine_cls = m.getattr("FutureIntoCoroutine")?;
    py.import("typing")?
        .getattr("Coroutine")?
        .call_method1("register", (future_into_coroutine_cls,))?;

    // Misc
    m.add_class::<misc::ApiVersion>()?;

    Ok(())
}
