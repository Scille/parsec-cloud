// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

//! This crate implement binding for our python front, and those will never be compile on the arch `wasm32`.
//! Trying to compile this crate on the target `wasm32-*` will result in a crash of the `pyo3` build script.
#![cfg(not(target_arch = "wasm32"))]
// Waiting for a fix from pyo3
#![allow(clippy::borrow_deref_ref)]

use pyo3::prelude::{pymodule, wrap_pyfunction, PyModule, PyResult, Python};

mod addrs;
mod api_crypto;
mod binding_utils;
mod file_operations;
mod ids;
mod invite;
mod local_device;
mod local_manifest;
mod manifest;
mod protocol;
mod storage;
mod time;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _libparsec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<local_manifest::Chunk>()?;
    m.add_class::<local_manifest::LocalFileManifest>()?;
    m.add_class::<local_manifest::LocalFolderManifest>()?;
    m.add_class::<local_manifest::LocalWorkspaceManifest>()?;
    m.add_class::<local_manifest::LocalUserManifest>()?;
    // Block
    m.add_class::<protocol::BlockCreateReq>()?;
    m.add_class::<protocol::BlockCreateRep>()?;
    m.add_class::<protocol::BlockReadReq>()?;
    m.add_class::<protocol::BlockReadRep>()?;
    // Cmd
    m.add_class::<protocol::AuthenticatedAnyCmdReq>()?;
    m.add_class::<protocol::InvitedAnyCmdReq>()?;
    // Events
    m.add_class::<protocol::EventsListenReq>()?;
    m.add_class::<protocol::EventsListenRep>()?;
    m.add_class::<protocol::EventsSubscribeReq>()?;
    m.add_class::<protocol::EventsSubscribeRep>()?;
    // Invite
    m.add_class::<protocol::InviteNewReq>()?;
    m.add_class::<protocol::InviteNewRep>()?;
    m.add_class::<protocol::InviteDeleteReq>()?;
    m.add_class::<protocol::InviteDeleteRep>()?;
    m.add_class::<protocol::InviteListReq>()?;
    m.add_class::<protocol::InviteListRep>()?;
    m.add_class::<protocol::InviteInfoReq>()?;
    m.add_class::<protocol::InviteInfoRep>()?;
    m.add_class::<protocol::Invite1ClaimerWaitPeerReq>()?;
    m.add_class::<protocol::Invite1ClaimerWaitPeerRep>()?;
    m.add_class::<protocol::Invite1GreeterWaitPeerReq>()?;
    m.add_class::<protocol::Invite1GreeterWaitPeerRep>()?;
    m.add_class::<protocol::Invite2aClaimerSendHashedNonceHashNonceReq>()?;
    m.add_class::<protocol::Invite2aClaimerSendHashedNonceHashNonceRep>()?;
    m.add_class::<protocol::Invite2aGreeterGetHashedNonceReq>()?;
    m.add_class::<protocol::Invite2aGreeterGetHashedNonceRep>()?;
    m.add_class::<protocol::Invite2bClaimerSendNonceReq>()?;
    m.add_class::<protocol::Invite2bClaimerSendNonceRep>()?;
    m.add_class::<protocol::Invite2bGreeterSendNonceReq>()?;
    m.add_class::<protocol::Invite2bGreeterSendNonceRep>()?;
    m.add_class::<protocol::Invite3aClaimerSignifyTrustReq>()?;
    m.add_class::<protocol::Invite3aClaimerSignifyTrustRep>()?;
    m.add_class::<protocol::Invite3aGreeterWaitPeerTrustReq>()?;
    m.add_class::<protocol::Invite3aGreeterWaitPeerTrustRep>()?;
    m.add_class::<protocol::Invite3bClaimerWaitPeerTrustReq>()?;
    m.add_class::<protocol::Invite3bClaimerWaitPeerTrustRep>()?;
    m.add_class::<protocol::Invite3bGreeterSignifyTrustReq>()?;
    m.add_class::<protocol::Invite3bGreeterSignifyTrustRep>()?;
    m.add_class::<protocol::Invite4ClaimerCommunicateReq>()?;
    m.add_class::<protocol::Invite4ClaimerCommunicateRep>()?;
    m.add_class::<protocol::Invite4GreeterCommunicateReq>()?;
    m.add_class::<protocol::Invite4GreeterCommunicateRep>()?;
    m.add_class::<protocol::InviteListItem>()?;
    // Message
    m.add_class::<protocol::MessageGetReq>()?;
    m.add_class::<protocol::MessageGetRep>()?;
    m.add_class::<protocol::Message>()?;
    // Organization
    m.add_class::<protocol::OrganizationStatsReq>()?;
    m.add_class::<protocol::OrganizationStatsRep>()?;
    m.add_class::<protocol::OrganizationConfigReq>()?;
    m.add_class::<protocol::OrganizationConfigRep>()?;
    m.add_class::<protocol::UsersPerProfileDetailItem>()?;
    // Ping
    m.add_class::<protocol::AuthenticatedPingReq>()?;
    m.add_class::<protocol::AuthenticatedPingRep>()?;
    m.add_class::<protocol::InvitedPingReq>()?;
    m.add_class::<protocol::InvitedPingRep>()?;
    // Realm
    m.add_class::<protocol::RealmCreateReq>()?;
    m.add_class::<protocol::RealmCreateRep>()?;
    m.add_class::<protocol::RealmStatusReq>()?;
    m.add_class::<protocol::RealmStatusRep>()?;
    m.add_class::<protocol::RealmStatsReq>()?;
    m.add_class::<protocol::RealmStatsRep>()?;
    m.add_class::<protocol::RealmGetRoleCertificateReq>()?;
    m.add_class::<protocol::RealmGetRoleCertificateRep>()?;
    m.add_class::<protocol::RealmUpdateRolesReq>()?;
    m.add_class::<protocol::RealmUpdateRolesRep>()?;
    m.add_class::<protocol::RealmStartReencryptionMaintenanceReq>()?;
    m.add_class::<protocol::RealmStartReencryptionMaintenanceRep>()?;
    m.add_class::<protocol::RealmFinishReencryptionMaintenanceReq>()?;
    m.add_class::<protocol::RealmFinishReencryptionMaintenanceRep>()?;
    // User
    m.add_class::<protocol::UserGetReq>()?;
    m.add_class::<protocol::UserGetRep>()?;
    m.add_class::<protocol::UserCreateReq>()?;
    m.add_class::<protocol::UserCreateRep>()?;
    m.add_class::<protocol::UserRevokeReq>()?;
    m.add_class::<protocol::UserRevokeRep>()?;
    m.add_class::<protocol::DeviceCreateReq>()?;
    m.add_class::<protocol::DeviceCreateRep>()?;
    m.add_class::<protocol::HumanFindReq>()?;
    m.add_class::<protocol::HumanFindRep>()?;
    m.add_class::<protocol::Trustchain>()?;
    m.add_class::<protocol::HumanFindResultItem>()?;
    // Vlob
    m.add_class::<protocol::VlobCreateReq>()?;
    m.add_class::<protocol::VlobCreateRep>()?;
    m.add_class::<protocol::VlobReadReq>()?;
    m.add_class::<protocol::VlobReadRep>()?;
    m.add_class::<protocol::VlobUpdateReq>()?;
    m.add_class::<protocol::VlobUpdateRep>()?;
    m.add_class::<protocol::VlobPollChangesReq>()?;
    m.add_class::<protocol::VlobPollChangesRep>()?;
    m.add_class::<protocol::VlobListVersionsReq>()?;
    m.add_class::<protocol::VlobListVersionsRep>()?;
    m.add_class::<protocol::VlobMaintenanceGetReencryptionBatchReq>()?;
    m.add_class::<protocol::VlobMaintenanceGetReencryptionBatchRep>()?;
    m.add_class::<protocol::VlobMaintenanceSaveReencryptionBatchReq>()?;
    m.add_class::<protocol::VlobMaintenanceSaveReencryptionBatchRep>()?;
    m.add_class::<protocol::ReencryptionBatchEntry>()?;
    // Storage
    m.add_class::<storage::WorkspaceStorage>()?;
    // File operations
    m.add_function(wrap_pyfunction!(file_operations::prepare_read, m)?)?;
    m.add_function(wrap_pyfunction!(file_operations::prepare_write, m)?)?;
    m.add_function(wrap_pyfunction!(file_operations::prepare_resize, m)?)?;
    m.add_function(wrap_pyfunction!(file_operations::prepare_reshape, m)?)?;
    Ok(())
}
