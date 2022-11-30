mod certif;
mod error;
mod invite;
mod local_manifest;
mod manifest;
mod message;
mod pki;

pub(crate) use certif::*;
pub(crate) use error::*;
pub(crate) use invite::*;
pub(crate) use local_manifest::*;
pub(crate) use manifest::*;
pub(crate) use message::*;
pub(crate) use pki::*;

use pyo3::{types::PyModule, wrap_pyfunction, PyResult, Python};

pub(crate) fn add_mod(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Error
    m.add("DataError", py.get_type::<DataError>())?;
    m.add("EntryNameError", py.get_type::<EntryNameError>())?;

    // Certif
    m.add_class::<UserCertificate>()?;
    m.add_class::<RevokedUserCertificate>()?;
    m.add_class::<DeviceCertificate>()?;
    m.add_class::<RealmRoleCertificate>()?;
    m.add_class::<SequesterAuthorityCertificate>()?;
    m.add_class::<SequesterServiceCertificate>()?;

    // Invite
    m.add_class::<SASCode>()?;
    m.add_function(wrap_pyfunction!(generate_sas_codes, m)?)?;
    m.add_function(wrap_pyfunction!(generate_sas_code_candidates, m)?)?;
    m.add_class::<InviteUserData>()?;
    m.add_class::<InviteUserConfirmation>()?;
    m.add_class::<InviteDeviceData>()?;
    m.add_class::<InviteDeviceConfirmation>()?;

    // Local Manifest
    m.add_class::<Chunk>()?;
    m.add_class::<LocalFileManifest>()?;
    m.add_class::<LocalFolderManifest>()?;
    m.add_class::<LocalWorkspaceManifest>()?;
    m.add_class::<LocalUserManifest>()?;
    m.add_function(wrap_pyfunction!(local_manifest_decrypt_and_load, m)?)?;

    // Manifest
    m.add_class::<EntryName>()?;
    m.add_class::<WorkspaceEntry>()?;
    m.add_class::<BlockAccess>()?;
    m.add_class::<FolderManifest>()?;
    m.add_class::<FileManifest>()?;
    m.add_class::<WorkspaceManifest>()?;
    m.add_class::<UserManifest>()?;
    m.add_function(wrap_pyfunction!(manifest_decrypt_and_load, m)?)?;
    m.add_function(wrap_pyfunction!(manifest_decrypt_verify_and_load, m)?)?;
    m.add_function(wrap_pyfunction!(manifest_verify_and_load, m)?)?;
    m.add_function(wrap_pyfunction!(manifest_unverified_load, m)?)?;

    // Message
    m.add_class::<MessageContent>()?;
    m.add_class::<SharingGrantedMessageContent>()?;
    m.add_class::<SharingReencryptedMessageContent>()?;
    m.add_class::<SharingRevokedMessageContent>()?;
    m.add_class::<PingMessageContent>()?;

    // Pki
    m.add_class::<PkiEnrollmentAnswerPayload>()?;
    m.add_class::<PkiEnrollmentSubmitPayload>()?;

    Ok(())
}
