// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// use async_trait::async_trait;
// use std::sync::Arc;

// use libparsec_client_connection::{protocol::authenticated_cmds, ConnectionError};
use libparsec_client_connection::ConnectionError;
// use libparsec_platform_storage::workspace::WorkspaceDataStorage;
use libparsec_types::prelude::*;

// use super::super::WorkspaceOps;
use crate::certif::{
    // CertifOps,
    InvalidCertificateError,
    InvalidKeysBundleError,
    // InvalidManifestError,
    // CertifValidateManifestError,
};

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Client has stopped")]
    Stopped,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("The realm doesn't have any key yet")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] InvalidKeysBundleError),
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    // Note `InvalidManifest` here, this is because we self-repair in case of invalid
    // user manifest (given otherwise the client would be stuck for good !)
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    BadTimestamp {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for SyncError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

// pub async fn refresh_realm_checkpoint(ops: &WorkspaceOps) -> Result<(), SyncError> {
//     let last_checkpoint = ops.data_storage.get_realm_checkpoint().await?;

//     let (changes, current_checkpoint) = {
//         use authenticated_cmds::latest::vlob_poll_changes::{Rep, Req};
//         let req = Req {
//             realm_id: ops.realm_id,
//             last_checkpoint,
//         };
//         let rep = ops.cmds.send(req).await?;
//         match rep {
//             Rep::Ok {
//                 changes,
//                 current_checkpoint,
//             } => (changes.into_iter().collect(), current_checkpoint),
//             // TODO: error handling !
//             Rep::AuthorNotAllowed => todo!(),
//             Rep::RealmNotFound { .. } => todo!(),
//             Rep::UnknownStatus { .. } => todo!(),
//         }
//     };

//     if last_checkpoint != current_checkpoint {
//         ops.data_storage
//             .update_realm_checkpoint(current_checkpoint, changes)
//             .await?;
//     }

//     Ok(())
// }

// pub async fn get_need_inbound_sync(ops: &WorkspaceOps) -> anyhow::Result<Vec<VlobID>> {
//     let need_sync = ops.data_storage.get_need_sync_entries().await?;
//     Ok(need_sync.remote)
// }

#[derive(Debug)]
pub enum InboundSyncOutcome {
    Updated,
    NoChange,
}

// /// Download and merge remote changes from the server.
// ///
// /// If the client contains local changes, an outbound sync is still needed to
// /// have the client fully synchronized with the server.
// pub async fn inbound_sync(
//     ops: &WorkspaceOps,
//     entry_id: VlobID,
// ) -> Result<InboundSyncOutcome, SyncError> {
//     if entry_id == ops.realm_id {
//         inbound_sync_root(ops).await
//     } else {
//         inbound_sync_child(ops, entry_id).await
//     }
//     // TODO: send inbound sync event ?
// }

// async fn inbound_sync_root(ops: &WorkspaceOps) -> Result<InboundSyncOutcome, SyncError> {
//     // Retrieve remote
//     let remote: WorkspaceManifest =
//         match fetch_remote_manifest_with_self_heal(ops, ops.realm_id).await? {
//             FetchWithSelfHealOutcome::LastVersion(manifest) => manifest,
//             FetchWithSelfHealOutcome::SelfHeal {
//                 last_version,
//                 mut last_valid_manifest,
//             } => {
//                 // If we use the manifest as-is, we won't be able to do outbound sync
//                 // given the server will always complain a more recent manifest exists
//                 // (i.e. the one that is corrupted !).
//                 // So we tweak this old manifest into pretending it is the corrupted one.
//                 // TODO: what if the server sends the manifest data to client A, but dummy
//                 // data to client B ? We would end up with clients not agreeing on what
//                 // contains a given version of the manifest...
//                 // TODO: send event or warning log ?
//                 last_valid_manifest.version = last_version;
//                 last_valid_manifest
//             }
//         };

//     // Now merge the remote with the current local manifest
//     update_storage_with_remote_root(ops, remote)
//         .await
//         .map_err(|err| err.into())

//     // TODO: we used to send a SHARING_UPDATED event here, however it would simpler
//     // to send such event from the CertifOps (i.e. when a realm role certificate
//     // is added). The downside of this approach is we don't have the guarantee that
//     // WorkspaceOps have processed the changes (e.g. GUI receive an event about an new
//     // workspace shared with us, but got an error when trying to get the workspace
//     // from WorkspaceOps...)
// }

// pub(super) async fn update_storage_with_remote_root(
//     ops: &WorkspaceOps,
//     remote: WorkspaceManifest,
// ) -> anyhow::Result<InboundSyncOutcome> {
//     let (updater, local) = ops.data_storage.for_update_workspace_manifest().await;
//     // Note merge may end up with nothing new, typically if the remote version is
//     // already the one local is based on
//     if let Some(merged) = super::super::merge::merge_local_workspace_manifests(
//         &ops.device.device_id,
//         ops.device.now(),
//         &local,
//         remote,
//     ) {
//         updater.update_workspace_manifest(Arc::new(merged)).await?;
//         Ok(InboundSyncOutcome::Updated)
//     } else {
//         Ok(InboundSyncOutcome::NoChange)
//     }
// }

// async fn inbound_sync_child(
//     ops: &WorkspaceOps,
//     entry_id: VlobID,
// ) -> Result<InboundSyncOutcome, SyncError> {
//     // Retrieve remote
//     let remote: ChildManifest = match fetch_remote_manifest_with_self_heal(ops, entry_id).await? {
//         FetchWithSelfHealOutcome::LastVersion(manifest) => manifest,
//         FetchWithSelfHealOutcome::SelfHeal {
//             last_version,
//             mut last_valid_manifest,
//         } => {
//             // If we use the manifest as-is, we won't be able to do outbound sync
//             // given the server will always complain a more recent manifest exists
//             // (i.e. the one that is corrupted !).
//             // So we tweak this old manifest into pretending it is the corrupted one.
//             // TODO: what if the server sends the manifest data to client A, but dummy
//             // data to client B ? We would end up with clients not agreeing on what
//             // contains a given version of the manifest...
//             // TODO: send event or warning log ?
//             match &mut last_valid_manifest {
//                 ChildManifest::File(m) => m.version = last_version,
//                 ChildManifest::Folder(m) => m.version = last_version,
//             }
//             last_valid_manifest
//         }
//     };

//     // Now merge the remote with the current local manifest
//     update_storage_with_remote_child(ops, remote)
//         .await
//         .map_err(|err| err.into())
// }

// pub(super) async fn update_storage_with_remote_child(
//     ops: &WorkspaceOps,
//     remote: ChildManifest,
// ) -> Result<InboundSyncOutcome, anyhow::Error> {
//     let entry_id = match &remote {
//         ChildManifest::File(m) => m.id,
//         ChildManifest::Folder(m) => m.id,
//     };

//     let (updater, local) = ops.data_storage.for_update_child_manifest(entry_id).await?;
//     // Note merge may end up with nothing new, typically if the remote version is
//     // already the one local is based on
//     match (local, remote) {
//         // File added remotely
//         (None, ChildManifest::File(remote)) => {
//             let local_manifest = Arc::new(LocalFileManifest::from_remote(remote));
//             updater
//                 .update_as_file_manifest(local_manifest, false, [].into_iter())
//                 .await?;
//             Ok(InboundSyncOutcome::Updated)
//         }

//         // Folder added remotely
//         (None, ChildManifest::Folder(remote)) => {
//             let local_manifest = Arc::new(LocalFolderManifest::from_remote(remote, None));
//             updater.update_as_folder_manifest(local_manifest).await?;
//             Ok(InboundSyncOutcome::Updated)
//         }

//         // Folder present in both remote and local, need to merge them
//         (Some(ArcLocalChildManifest::Folder(local_manifest)), ChildManifest::Folder(remote)) => {
//             // Note merge may end up with nothing to sync, typically if the remote version is
//             // already the one local is based on
//             if let Some(merged_manifest) = super::super::merge::merge_local_folder_manifests(
//                 &ops.device.device_id,
//                 ops.device.now(),
//                 &local_manifest,
//                 remote,
//             ) {
//                 updater
//                     .update_as_folder_manifest(Arc::new(merged_manifest))
//                     .await?;
//                 Ok(InboundSyncOutcome::Updated)
//             } else {
//                 Ok(InboundSyncOutcome::NoChange)
//             }
//         }

//         // TODO: finish this !

//         // File present in both remote and local, either they are the same or we have a file conflict !
//         (Some(ArcLocalChildManifest::File(_)), ChildManifest::File(_)) => todo!(),

//         // The entry has changed it type, this is not expected :/
//         // Solve this by considering this is a file conflict
//         (Some(ArcLocalChildManifest::Folder(_)), ChildManifest::File(_)) => todo!(),
//         (Some(ArcLocalChildManifest::File(_)), ChildManifest::Folder(_)) => todo!(),
//     }
// }

// #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
// #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
// pub trait RemoteManifest: Sized {
//     type LocalManifest: Sized;

//     fn extract_base_from_local(local: Self::LocalManifest) -> Self;

//     async fn get_from_storage(
//         storage: &WorkspaceDataStorage,
//         entry_id: VlobID,
//     ) -> Result<(VersionInt, Self::LocalManifest), anyhow::Error>;

//     #[allow(clippy::too_many_arguments)]
//     async fn validate(
//         certif_ops: &CertifOps,
//         realm_id: VlobID,
//         realm_key: &SecretKey,
//         vlob_id: VlobID,
//         certificate_index: IndexInt,
//         author: &DeviceID,
//         version: VersionInt,
//         timestamp: DateTime,
//         encrypted: &[u8],
//     ) -> Result<Self, CertifValidateManifestError>;
// }

// #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
// #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
// impl RemoteManifest for WorkspaceManifest {
//     type LocalManifest = Arc<LocalWorkspaceManifest>;

//     fn extract_base_from_local(local: Self::LocalManifest) -> Self {
//         local.base.clone()
//     }

//     // TODO: handle entry not found
//     async fn get_from_storage(
//         storage: &WorkspaceDataStorage,
//         _entry_id: VlobID,
//     ) -> Result<(VersionInt, Self::LocalManifest), anyhow::Error> {
//         let manifest = storage.get_workspace_manifest();
//         Ok((manifest.base.version, manifest))
//     }

//     async fn validate(
//         certif_ops: &CertifOps,
//         realm_id: VlobID,
//         realm_key: &SecretKey,
//         _vlob_id: VlobID,
//         certificate_index: IndexInt,
//         author: &DeviceID,
//         version: VersionInt,
//         timestamp: DateTime,
//         encrypted: &[u8],
//     ) -> Result<Self, CertifValidateManifestError> {
//         certif_ops
//             .validate_workspace_manifest(
//                 realm_id,
//                 realm_key,
//                 certificate_index,
//                 author,
//                 version,
//                 timestamp,
//                 encrypted,
//             )
//             .await
//     }
// }

// #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
// #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
// impl RemoteManifest for ChildManifest {
//     type LocalManifest = ArcLocalChildManifest;

//     fn extract_base_from_local(local: Self::LocalManifest) -> Self {
//         match local {
//             Self::LocalManifest::File(m) => Self::File(m.base.clone()),
//             Self::LocalManifest::Folder(m) => Self::Folder(m.base.clone()),
//         }
//     }

//     // TODO: handle entry not found
//     async fn get_from_storage(
//         storage: &WorkspaceDataStorage,
//         entry_id: VlobID,
//     ) -> Result<(VersionInt, Self::LocalManifest), anyhow::Error> {
//         let manifest = storage.get_child_manifest(entry_id).await?;
//         let base_version = match &manifest {
//             Self::LocalManifest::File(m) => m.base.version,
//             Self::LocalManifest::Folder(m) => m.base.version,
//         };
//         Ok((base_version, manifest))
//     }

//     async fn validate(
//         certif_ops: &CertifOps,
//         realm_id: VlobID,
//         realm_key: &SecretKey,
//         vlob_id: VlobID,
//         certificate_index: IndexInt,
//         author: &DeviceID,
//         version: VersionInt,
//         timestamp: DateTime,
//         encrypted: &[u8],
//     ) -> Result<Self, CertifValidateManifestError> {
//         certif_ops
//             .validate_child_manifest(
//                 realm_id,
//                 realm_key,
//                 vlob_id,
//                 certificate_index,
//                 author,
//                 version,
//                 timestamp,
//                 encrypted,
//             )
//             .await
//     }
// }

// enum FetchWithSelfHealOutcome<M> {
//     LastVersion(M),
//     SelfHeal {
//         last_version: VersionInt,
//         last_valid_manifest: M,
//     },
// }

// async fn fetch_remote_manifest_with_self_heal<M: RemoteManifest>(
//     ops: &WorkspaceOps,
//     entry_id: VlobID,
// ) -> Result<FetchWithSelfHealOutcome<M>, SyncError> {
//     // Retrieve remote
//     match fetch_remote_manifest(ops, entry_id, None).await {
//         Ok(manifest) => Ok(FetchWithSelfHealOutcome::LastVersion(manifest)),

//         // The last version of the manifest appear to be invalid (uploaded by
//         // a buggy Parsec client ?), however we cannot just fail here otherwise
//         // the system would be stuck for good !
//         Err(FetchRemoteManifestError::InvalidManifest(err)) => {
//             // Try to find the last valid version of the manifest and continue
//             // from there

//             let last_version = match err {
//                 InvalidManifestError::Corrupted { version, .. } => version,
//                 InvalidManifestError::NonExistentAuthor { version, .. } => version,
//                 InvalidManifestError::RevokedAuthor { version, .. } => version,
//                 InvalidManifestError::AuthorRealmRoleCannotWrite { version, .. } => version,
//                 InvalidManifestError::AuthorNoAccessToRealm { version, .. } => version,
//             };

//             let last_valid_manifest: M =
//                 find_last_valid_manifest(ops, entry_id, last_version).await?;

//             Ok(FetchWithSelfHealOutcome::SelfHeal {
//                 last_version,
//                 last_valid_manifest,
//             })
//         }

//         // We couldn't validate the manifest due to an invalid certificate
//         // provided by the server. Hence there is nothing more we can do :(
//         Err(FetchRemoteManifestError::InvalidCertificate(err)) => {
//             Err(SyncError::InvalidCertificate(err))
//         }

//         Err(FetchRemoteManifestError::Offline) => Err(SyncError::Offline),

//         // We didn't specify a `version` argument in the request
//         Err(FetchRemoteManifestError::BadVersion) => {
//             unreachable!()
//         }

//         // D'Oh :/
//         Err(err @ FetchRemoteManifestError::Internal(_)) => Err(SyncError::Internal(err.into())),
//     }
// }

// async fn find_last_valid_manifest<M: RemoteManifest>(
//     ops: &WorkspaceOps,
//     entry_id: VlobID,
//     last_version: VersionInt,
// ) -> Result<M, SyncError> {
//     let (local_base_version, local_manifest) =
//         M::get_from_storage(&ops.data_storage, entry_id).await?;

//     for candidate_version in (local_base_version + 1..last_version).rev() {
//         match fetch_remote_manifest(ops, entry_id, Some(candidate_version)).await {
//             // Finally found a valid manifest !
//             Ok(manifest) => {
//                 return Ok(manifest)
//             },

//             // Yet another invalid manifest, just skip it
//             Err(FetchRemoteManifestError::InvalidManifest(_)) => continue,

//             // Errors that prevent us from continuing :(

//             Err(FetchRemoteManifestError::Offline) => {
//                 return Err(SyncError::Offline)
//             }
//             Err(FetchRemoteManifestError::InvalidCertificate(err)) => {
//                 return Err(SyncError::InvalidCertificate(err))
//             },
//             // The version we sent was lower than the one of the invalid manifest previously
//             // sent by the server, so this error should not occur in theory (unless the server
//             // have just done a rollback, but this is very unlikely !)
//             Err(FetchRemoteManifestError::BadVersion) => {
//                 return Err(SyncError::Internal(
//                     anyhow::anyhow!(
//                         "Server sent us vlob `{}` with version {} but now complains version {} we ask for doesn't exist",
//                         entry_id,
//                         last_version,
//                         candidate_version
//                     )
//                 ))
//             },
//             Err(err @ FetchRemoteManifestError::Internal(_)) => {
//                 return Err(SyncError::Internal(err.into()))
//             }
//         }
//     }

//     // It seems the last valid manifest is the one we already have
//     let manifest = M::extract_base_from_local(local_manifest);
//     Ok(manifest)
// }

// #[derive(Debug, thiserror::Error)]
// pub enum FetchRemoteManifestError {
//     #[error("Cannot reach the server")]
//     Offline,
//     #[error("Server has no such version for this manifest")]
//     BadVersion,
//     #[error(transparent)]
//     InvalidCertificate(#[from] InvalidCertificateError),
//     #[error(transparent)]
//     InvalidManifest(#[from] InvalidManifestError),
//     #[error(transparent)]
//     Internal(#[from] anyhow::Error),
// }

// impl From<ConnectionError> for FetchRemoteManifestError {
//     fn from(value: ConnectionError) -> Self {
//         match value {
//             ConnectionError::NoResponse(_) => Self::Offline,
//             err => Self::Internal(err.into()),
//         }
//     }
// }

// async fn fetch_remote_manifest<M: RemoteManifest>(
//     ops: &WorkspaceOps,
//     entry_id: VlobID,
//     version: Option<VersionInt>,
// ) -> Result<M, FetchRemoteManifestError> {
//     use authenticated_cmds::latest::vlob_read::{Rep, Req};

//     let req = Req {
//         timestamp: None,
//         version,
//         vlob_id: entry_id,
//     };

//     let rep = ops.cmds.send(req).await?;

//     let outcome = match rep {
//         Rep::Ok { certificate_index, author: expected_author, version: version_according_to_server, timestamp: expected_timestamp, blob } => {
//             let expected_version = match version {
//                 Some(version) => version,
//                 None => version_according_to_server,
//             };
//             let realm_key = {
//                 let config = ops.user_dependant_config.lock().expect("Mutex is poisoned");
//                 config.realm_key.clone()
//             };
//             M::validate(
//                 &ops.certif_ops,
//                 ops.realm_id,
//                 &realm_key,
//                 entry_id,
//                 certificate_index, &expected_author, expected_version, expected_timestamp, &blob
//             ).await
//         }
//         // Expected errors
//         Rep::BadVersion if version.is_some() => {
//             return Err(FetchRemoteManifestError::BadVersion);
//         }
//         // Unexpected errors :(
//         // TODO: error handling is invalid !
//         rep @ (
//             // We didn't specify a `version` argument in the request
//             Rep::BadVersion |
//             // User never loses access to its user manifest's workspace
//             Rep::NotAllowed |
//             // User manifest's workspace never gets reencrypted !
//             Rep::InMaintenance |
//             Rep::BadEncryptionRevision |
//             // User manifest's vlob is supposed to exist !
//             Rep::NotFound { .. } |
//             // Don't know what to do with this status :/
//             Rep::UnknownStatus { .. }
//         ) => {
//             return Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into());
//         }
//     };

//     outcome.map_err(|err| match err {
//         CertifValidateManifestError::InvalidCertificate(err) => {
//             FetchRemoteManifestError::InvalidCertificate(err)
//         }
//         CertifValidateManifestError::InvalidManifest(err) => {
//             FetchRemoteManifestError::InvalidManifest(err)
//         }
//         CertifValidateManifestError::Offline => FetchRemoteManifestError::Offline,
//         err @ CertifValidateManifestError::Internal(_) => FetchRemoteManifestError::Internal(err.into()),
//     })
// }
