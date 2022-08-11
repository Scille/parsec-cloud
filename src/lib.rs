// Waiting for a fix from pyo3
#![allow(clippy::borrow_deref_ref)]

use pyo3::prelude::{pymodule, wrap_pyfunction, PyModule, PyResult, Python};

mod addrs;
mod api_crypto;
mod binding_utils;
mod certif;
mod ids;
mod invite;
mod local_device;
mod local_manifest;
mod manifest;
mod time;
mod trustchain;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_parsec")]
fn entrypoint(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<addrs::BackendAddr>()?;
    m.add_class::<addrs::BackendOrganizationAddr>()?;
    m.add_class::<addrs::BackendActionAddr>()?;
    m.add_class::<addrs::BackendOrganizationBootstrapAddr>()?;
    m.add_class::<addrs::BackendOrganizationFileLinkAddr>()?;
    m.add_class::<addrs::BackendInvitationAddr>()?;
    m.add_class::<addrs::BackendPkiEnrollmentAddr>()?;

    m.add_class::<api_crypto::HashDigest>()?;
    m.add_class::<api_crypto::SigningKey>()?;
    m.add_class::<api_crypto::VerifyKey>()?;
    m.add_class::<api_crypto::SecretKey>()?;
    m.add_class::<api_crypto::PrivateKey>()?;
    m.add_class::<api_crypto::PublicKey>()?;

    m.add_class::<certif::UserCertificate>()?;
    m.add_class::<certif::RevokedUserCertificate>()?;
    m.add_class::<certif::DeviceCertificate>()?;
    m.add_class::<certif::RealmRoleCertificate>()?;

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

    m.add_class::<local_device::LocalDevice>()?;

    m.add_class::<local_manifest::Chunk>()?;
    m.add_class::<local_manifest::LocalFileManifest>()?;
    m.add_class::<local_manifest::LocalFolderManifest>()?;
    m.add_class::<local_manifest::LocalWorkspaceManifest>()?;
    m.add_class::<local_manifest::LocalUserManifest>()?;
    m.add_function(wrap_pyfunction!(
        local_manifest::local_manifest_decrypt_and_load,
        m
    )?)?;

    m.add_class::<manifest::EntryName>()?;
    m.add_class::<manifest::WorkspaceEntry>()?;
    m.add_class::<manifest::BlockAccess>()?;
    m.add_class::<manifest::FolderManifest>()?;
    m.add_class::<manifest::FileManifest>()?;
    m.add_class::<manifest::WorkspaceManifest>()?;
    m.add_class::<manifest::UserManifest>()?;

    m.add_function(wrap_pyfunction!(time::mock_time, m)?)?;
    m.add_class::<time::DateTime>()?;
    m.add_class::<time::LocalDateTime>()?;

    m.add_class::<trustchain::TrustchainContext>()?;
    Ok(())
}
