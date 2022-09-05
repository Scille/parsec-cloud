use pyo3::prelude::{pymodule, wrap_pyfunction, PyModule, PyResult, Python};

mod addrs;
mod api_crypto;
mod binding_utils;
mod certif;
mod file_operations;
mod ids;
mod invite;
mod local_device;
mod local_manifest;
mod manifest;
mod protocol;
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
    m.add_class::<local_device::UserInfo>()?;
    m.add_class::<local_device::DeviceInfo>()?;

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
    m.add_function(wrap_pyfunction!(manifest::manifest_decrypt_and_load, m)?)?;
    m.add_function(wrap_pyfunction!(
        manifest::manifest_decrypt_verify_and_load,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(manifest::manifest_verify_and_load, m)?)?;

    // Block
    m.add_class::<protocol::BlockCreateReq>()?;
    m.add_class::<protocol::BlockCreateRep>()?;
    m.add_class::<protocol::BlockCreateRepOk>()?;
    m.add_class::<protocol::BlockCreateRepAlreadyExists>()?;
    m.add_class::<protocol::BlockCreateRepNotFound>()?;
    m.add_class::<protocol::BlockCreateRepTimeout>()?;
    m.add_class::<protocol::BlockCreateRepNotAllowed>()?;
    m.add_class::<protocol::BlockCreateRepInMaintenance>()?;
    m.add_class::<protocol::BlockCreateRepUnknownStatus>()?;
    m.add_class::<protocol::BlockReadReq>()?;
    m.add_class::<protocol::BlockReadRep>()?;
    m.add_class::<protocol::BlockReadRepOk>()?;
    m.add_class::<protocol::BlockReadRepNotFound>()?;
    m.add_class::<protocol::BlockReadRepTimeout>()?;
    m.add_class::<protocol::BlockReadRepNotAllowed>()?;
    m.add_class::<protocol::BlockReadRepInMaintenance>()?;
    m.add_class::<protocol::BlockReadRepUnknownStatus>()?;

    // Message
    m.add_class::<protocol::MessageGetReq>()?;
    m.add_class::<protocol::MessageGetRep>()?;
    m.add_class::<protocol::MessageGetRepOk>()?;
    m.add_class::<protocol::MessageGetRepUnknownStatus>()?;
    m.add_class::<protocol::Message>()?;

    // Vlob
    m.add_class::<protocol::VlobCreateReq>()?;
    m.add_class::<protocol::VlobCreateRep>()?;
    m.add_class::<protocol::VlobCreateRepOk>()?;
    m.add_class::<protocol::VlobCreateRepAlreadyExists>()?;
    m.add_class::<protocol::VlobCreateRepNotAllowed>()?;
    m.add_class::<protocol::VlobCreateRepBadEncryptionRevision>()?;
    m.add_class::<protocol::VlobCreateRepInMaintenance>()?;
    m.add_class::<protocol::VlobCreateRepRequireGreaterTimestamp>()?;
    m.add_class::<protocol::VlobCreateRepBadTimestamp>()?;
    m.add_class::<protocol::VlobCreateRepNotASequesteredOrganization>()?;
    m.add_class::<protocol::VlobCreateRepSequesterInconsistency>()?;
    m.add_class::<protocol::VlobCreateRepUnknownStatus>()?;
    m.add_class::<protocol::VlobReadReq>()?;
    m.add_class::<protocol::VlobReadRep>()?;
    m.add_class::<protocol::VlobReadRepOk>()?;
    m.add_class::<protocol::VlobReadRepNotFound>()?;
    m.add_class::<protocol::VlobReadRepNotAllowed>()?;
    m.add_class::<protocol::VlobReadRepBadVersion>()?;
    m.add_class::<protocol::VlobReadRepBadEncryptionRevision>()?;
    m.add_class::<protocol::VlobReadRepInMaintenance>()?;
    m.add_class::<protocol::VlobReadRepUnknownStatus>()?;
    m.add_class::<protocol::VlobUpdateReq>()?;
    m.add_class::<protocol::VlobUpdateRep>()?;
    m.add_class::<protocol::VlobUpdateRepOk>()?;
    m.add_class::<protocol::VlobUpdateRepNotFound>()?;
    m.add_class::<protocol::VlobUpdateRepNotAllowed>()?;
    m.add_class::<protocol::VlobUpdateRepBadVersion>()?;
    m.add_class::<protocol::VlobUpdateRepBadEncryptionRevision>()?;
    m.add_class::<protocol::VlobUpdateRepInMaintenance>()?;
    m.add_class::<protocol::VlobUpdateRepRequireGreaterTimestamp>()?;
    m.add_class::<protocol::VlobUpdateRepRequireGreaterTimestamp>()?;
    m.add_class::<protocol::VlobUpdateRepBadTimestamp>()?;
    m.add_class::<protocol::VlobUpdateRepNotASequesteredOrganization>()?;
    m.add_class::<protocol::VlobUpdateRepSequesterInconsistency>()?;
    m.add_class::<protocol::VlobUpdateRepUnknownStatus>()?;
    m.add_class::<protocol::VlobPollChangesReq>()?;
    m.add_class::<protocol::VlobPollChangesRep>()?;
    m.add_class::<protocol::VlobPollChangesRepOk>()?;
    m.add_class::<protocol::VlobPollChangesRepNotFound>()?;
    m.add_class::<protocol::VlobPollChangesRepNotAllowed>()?;
    m.add_class::<protocol::VlobPollChangesRepInMaintenance>()?;
    m.add_class::<protocol::VlobPollChangesRepUnknownStatus>()?;
    m.add_class::<protocol::VlobListVersionsReq>()?;
    m.add_class::<protocol::VlobListVersionsRep>()?;
    m.add_class::<protocol::VlobListVersionsRepOk>()?;
    m.add_class::<protocol::VlobListVersionsRepNotFound>()?;
    m.add_class::<protocol::VlobListVersionsRepNotAllowed>()?;
    m.add_class::<protocol::VlobListVersionsRepInMaintenance>()?;
    m.add_class::<protocol::VlobListVersionsRepUnknownStatus>()?;
    m.add_class::<protocol::VlobMaintenanceGetReencryptionBatchReq>()?;
    m.add_class::<protocol::VlobMaintenanceGetReencryptionBatchRep>()?;
    m.add_class::<protocol::VlobMaintenanceGetReencryptionBatchRepOk>()?;
    m.add_class::<protocol::VlobMaintenanceGetReencryptionBatchRepNotFound>()?;
    m.add_class::<protocol::VlobMaintenanceGetReencryptionBatchRepNotAllowed>()?;
    m.add_class::<protocol::VlobMaintenanceGetReencryptionBatchRepNotInMaintenance>()?;
    m.add_class::<protocol::VlobMaintenanceGetReencryptionBatchRepBadEncryptionRevision>()?;
    m.add_class::<protocol::VlobMaintenanceGetReencryptionBatchRepMaintenanceError>()?;
    m.add_class::<protocol::VlobMaintenanceGetReencryptionBatchRepUnknownStatus>()?;
    m.add_class::<protocol::VlobMaintenanceSaveReencryptionBatchReq>()?;
    m.add_class::<protocol::VlobMaintenanceSaveReencryptionBatchRep>()?;
    m.add_class::<protocol::VlobMaintenanceSaveReencryptionBatchRepOk>()?;
    m.add_class::<protocol::VlobMaintenanceSaveReencryptionBatchRepNotFound>()?;
    m.add_class::<protocol::VlobMaintenanceSaveReencryptionBatchRepNotAllowed>()?;
    m.add_class::<protocol::VlobMaintenanceSaveReencryptionBatchRepNotInMaintenance>()?;
    m.add_class::<protocol::VlobMaintenanceSaveReencryptionBatchRepBadEncryptionRevision>()?;
    m.add_class::<protocol::VlobMaintenanceSaveReencryptionBatchRepMaintenanceError>()?;
    m.add_class::<protocol::VlobMaintenanceSaveReencryptionBatchRepUnknownStatus>()?;
    m.add_class::<protocol::ReencryptionBatchEntry>()?;

    // Cmd
    m.add_class::<protocol::AuthenticatedAnyCmdReq>()?;
    m.add_class::<protocol::InvitedAnyCmdReq>()?;

    m.add_function(wrap_pyfunction!(time::mock_time, m)?)?;
    m.add_class::<time::TimeProvider>()?;
    m.add_class::<time::DateTime>()?;
    m.add_class::<time::LocalDateTime>()?;

    m.add_class::<trustchain::TrustchainContext>()?;
    Ok(())
}
