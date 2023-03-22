// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::prelude::{pymodule, wrap_pyfunction, PyModule, PyResult, Python};

mod addrs;
mod api_crypto;
mod backend_connection;
mod binding_utils;
mod core_fs;
mod data;
mod enumerate;
mod file_operations;
mod ids;
mod invite;
#[cfg(feature = "test-utils")]
mod local_db;
mod local_device;
mod misc;
mod protocol;
mod regex;
mod remote_devices_manager;
mod runtime;
#[cfg(feature = "test-utils")]
mod testbed;
mod time;
mod trustchain;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_parsec")]
fn entrypoint(py: Python, m: &PyModule) -> PyResult<()> {
    crate::api_crypto::add_mod(py, m)?;
    crate::backend_connection::add_mod(py, m)?;
    crate::core_fs::add_mod(py, m)?;
    crate::data::add_mod(py, m)?;
    crate::invite::add_mod(py, m)?;
    #[cfg(feature = "test-utils")]
    crate::local_db::add_mod(py, m)?;
    crate::local_device::add_mod(py, m)?;
    crate::protocol::add_mod(py, m)?;
    crate::remote_devices_manager::add_mod(py, m)?;

    m.add_class::<addrs::BackendAddr>()?;
    m.add_class::<addrs::BackendOrganizationAddr>()?;
    m.add_class::<addrs::BackendActionAddr>()?;
    m.add_class::<addrs::BackendOrganizationBootstrapAddr>()?;
    m.add_class::<addrs::BackendOrganizationFileLinkAddr>()?;
    m.add_class::<addrs::BackendInvitationAddr>()?;
    m.add_class::<addrs::BackendPkiEnrollmentAddr>()?;
    m.add_function(wrap_pyfunction!(addrs::export_root_verify_key, m)?)?;

    m.add_class::<enumerate::ClientType>()?;
    m.add_class::<enumerate::CoreEvent>()?;
    m.add_class::<enumerate::DeviceFileType>()?;
    m.add_class::<enumerate::InvitationDeletedReason>()?;
    m.add_class::<enumerate::InvitationEmailSentStatus>()?;
    m.add_class::<enumerate::InvitationStatus>()?;
    m.add_class::<enumerate::InvitationType>()?;
    m.add_class::<enumerate::RealmRole>()?;
    m.add_class::<enumerate::UserProfile>()?;

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

    // Testbed stuff
    #[cfg(feature = "test-utils")]
    {
        m.add_function(wrap_pyfunction!(testbed::test_new_testbed, m)?)?;
        m.add_function(wrap_pyfunction!(testbed::test_drop_testbed, m)?)?;
        m.add_function(wrap_pyfunction!(testbed::test_get_testbed_templates, m)?)?;
        m.add_class::<testbed::TestbedDeviceData>()?;
        m.add_class::<testbed::TestbedUserData>()?;
        m.add_class::<testbed::TestbedTemplate>()?;
    }

    Ok(())
}
