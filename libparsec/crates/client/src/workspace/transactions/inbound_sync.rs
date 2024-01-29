// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use async_trait::async_trait;
use std::sync::Arc;

use libparsec_client_connection::{protocol::authenticated_cmds, ConnectionError};
use libparsec_platform_storage::workspace::WorkspaceDataStorage;
use libparsec_types::prelude::*;

use super::super::WorkspaceOps;
use crate::certif::{
    CertifOps, CertifValidateManifestError, InvalidCertificateError, InvalidKeysBundleError,
    InvalidManifestError,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceSyncError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Client has stopped")]
    Stopped,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("The realm doesn't have any key yet")]
    NoKey,
    #[error("The workspace's realm hasn't been created yet on server")]
    NoRealm,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    // Note `InvalidManifest` here, this is because we self-repair in case of invalid
    // user manifest (given otherwise the client would be stuck for good !)
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for WorkspaceSyncError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn refresh_realm_checkpoint(ops: &WorkspaceOps) -> Result<(), WorkspaceSyncError> {
    let last_checkpoint = ops.data_storage.get_realm_checkpoint().await?;

    let (changes, current_checkpoint) = {
        use authenticated_cmds::latest::vlob_poll_changes::{Rep, Req};
        let req = Req {
            realm_id: ops.realm_id,
            last_checkpoint,
        };
        let rep = ops.cmds.send(req).await?;
        match rep {
            Rep::Ok {
                changes,
                current_checkpoint,
            } => (changes.into_iter().collect(), current_checkpoint),
            // TODO: error handling !
            Rep::AuthorNotAllowed => return Err(WorkspaceSyncError::NotAllowed),
            Rep::RealmNotFound { .. } => return Err(WorkspaceSyncError::NoRealm),
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }
    };

    if last_checkpoint != current_checkpoint {
        ops.data_storage
            .update_realm_checkpoint(current_checkpoint, changes)
            .await?;
    }

    Ok(())
}

pub async fn get_need_inbound_sync(ops: &WorkspaceOps) -> anyhow::Result<Vec<VlobID>> {
    let need_sync = ops.data_storage.get_need_sync_entries().await?;
    Ok(need_sync.remote)
}

#[derive(Debug)]
pub enum InboundSyncOutcome {
    Updated,
    NoChange,
}

/// Download and merge remote changes from the server.
///
/// If the client contains local changes, an outbound sync is still needed to
/// have the client fully synchronized with the server.
pub async fn inbound_sync(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<InboundSyncOutcome, WorkspaceSyncError> {
    if entry_id == ops.realm_id {
        inbound_sync_root(ops).await
    } else {
        inbound_sync_child(ops, entry_id).await
    }
    // TODO: send inbound sync event ?
}

async fn inbound_sync_root(ops: &WorkspaceOps) -> Result<InboundSyncOutcome, WorkspaceSyncError> {
    // Retrieve remote
    let remote: WorkspaceManifest =
        match fetch_remote_manifest_with_self_heal(ops, ops.realm_id).await? {
            FetchWithSelfHealOutcome::LastVersion(manifest) => manifest,
            FetchWithSelfHealOutcome::SelfHeal {
                last_version,
                mut last_valid_manifest,
            } => {
                // If we use the manifest as-is, we won't be able to do outbound sync
                // given the server will always complain a more recent manifest exists
                // (i.e. the one that is corrupted !).
                // So we tweak this old manifest into pretending it is the corrupted one.
                // TODO: what if the server sends the manifest data to client A, but dummy
                // data to client B ? We would end up with clients not agreeing on what
                // contains a given version of the manifest...
                // TODO: send event or warning log ?
                last_valid_manifest.version = last_version;
                last_valid_manifest
            }
        };

    // Now merge the remote with the current local manifest
    update_storage_with_remote_root(ops, remote)
        .await
        .map_err(|err| err.into())

    // TODO: we used to send a SHARING_UPDATED event here, however it would simpler
    // to send such event from the CertifOps (i.e. when a realm role certificate
    // is added). The downside of this approach is we don't have the guarantee that
    // WorkspaceOps have processed the changes (e.g. GUI receive an event about an new
    // workspace shared with us, but got an error when trying to get the workspace
    // from WorkspaceOps...)
}

pub(super) async fn update_storage_with_remote_root(
    ops: &WorkspaceOps,
    remote: WorkspaceManifest,
) -> anyhow::Result<InboundSyncOutcome> {
    let (updater, local) = ops.data_storage.for_update_workspace_manifest().await;
    // Note merge may end up with nothing new, typically if the remote version is
    // already the one local is based on
    if let Some(merged) = super::super::merge::merge_local_workspace_manifests(
        &ops.device.device_id,
        ops.device.now(),
        &local,
        remote,
    ) {
        updater.update_workspace_manifest(Arc::new(merged)).await?;
        Ok(InboundSyncOutcome::Updated)
    } else {
        Ok(InboundSyncOutcome::NoChange)
    }
}

async fn inbound_sync_child(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<InboundSyncOutcome, WorkspaceSyncError> {
    // Retrieve remote
    let remote: ChildManifest = match fetch_remote_manifest_with_self_heal(ops, entry_id).await? {
        FetchWithSelfHealOutcome::LastVersion(manifest) => manifest,
        FetchWithSelfHealOutcome::SelfHeal {
            last_version,
            mut last_valid_manifest,
        } => {
            // If we use the manifest as-is, we won't be able to do outbound sync
            // given the server will always complain a more recent manifest exists
            // (i.e. the one that is corrupted !).
            // So we tweak this old manifest into pretending it is the corrupted one.
            // TODO: what if the server sends the manifest data to client A, but dummy
            // data to client B ? We would end up with clients not agreeing on what
            // contains a given version of the manifest...
            // TODO: send event or warning log ?
            match &mut last_valid_manifest {
                ChildManifest::File(m) => m.version = last_version,
                ChildManifest::Folder(m) => m.version = last_version,
            }
            last_valid_manifest
        }
    };

    // Now merge the remote with the current local manifest
    update_storage_with_remote_child(ops, remote)
        .await
        .map_err(|err| err.into())
}

pub(super) async fn update_storage_with_remote_child(
    ops: &WorkspaceOps,
    remote: ChildManifest,
) -> Result<InboundSyncOutcome, anyhow::Error> {
    let entry_id = match &remote {
        ChildManifest::File(m) => m.id,
        ChildManifest::Folder(m) => m.id,
    };

    let (updater, local) = ops.data_storage.for_update_child_manifest(entry_id).await?;
    // Note merge may end up with nothing new, typically if the remote version is
    // already the one local is based on
    match (local, remote) {
        // File added remotely
        (None, ChildManifest::File(remote)) => {
            let local_manifest = Arc::new(LocalFileManifest::from_remote(remote));
            updater
                .update_as_file_manifest(local_manifest, false, [].into_iter())
                .await?;
            Ok(InboundSyncOutcome::Updated)
        }

        // Folder added remotely
        (None, ChildManifest::Folder(remote)) => {
            let local_manifest = Arc::new(LocalFolderManifest::from_remote(remote, None));
            updater.update_as_folder_manifest(local_manifest).await?;
            Ok(InboundSyncOutcome::Updated)
        }

        // Folder present in both remote and local, need to merge them
        (Some(ArcLocalChildManifest::Folder(local_manifest)), ChildManifest::Folder(remote)) => {
            // Note merge may end up with nothing to sync, typically if the remote version is
            // already the one local is based on
            if let Some(merged_manifest) = super::super::merge::merge_local_folder_manifests(
                &ops.device.device_id,
                ops.device.now(),
                &local_manifest,
                remote,
            ) {
                updater
                    .update_as_folder_manifest(Arc::new(merged_manifest))
                    .await?;
                Ok(InboundSyncOutcome::Updated)
            } else {
                Ok(InboundSyncOutcome::NoChange)
            }
        }

        // TODO: finish this !

        // File present in both remote and local, either they are the same or we have a file conflict !
        (Some(ArcLocalChildManifest::File(_)), ChildManifest::File(_)) => todo!(),

        // The entry has changed it type, this is not expected :/
        // Solve this by considering this is a file conflict
        (Some(ArcLocalChildManifest::Folder(_)), ChildManifest::File(_)) => todo!(),
        (Some(ArcLocalChildManifest::File(_)), ChildManifest::Folder(_)) => todo!(),
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait RemoteManifest: Sized {
    type LocalManifest: Sized;

    fn extract_base_from_local(local: Self::LocalManifest) -> Self;

    async fn get_from_storage(
        storage: &WorkspaceDataStorage,
        entry_id: VlobID,
    ) -> Result<(VersionInt, Self::LocalManifest), anyhow::Error>;

    #[allow(clippy::too_many_arguments)]
    async fn validate(
        certif_ops: &CertifOps,
        needed_realm_certificate_timestamp: DateTime,
        needed_common_certificate_timestamp: DateTime,
        realm_id: VlobID,
        key_index: IndexInt,
        vlob_id: VlobID,
        author: &DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<Self, CertifValidateManifestError>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl RemoteManifest for WorkspaceManifest {
    type LocalManifest = Arc<LocalWorkspaceManifest>;

    fn extract_base_from_local(local: Self::LocalManifest) -> Self {
        local.base.clone()
    }

    // TODO: handle entry not found
    async fn get_from_storage(
        storage: &WorkspaceDataStorage,
        _entry_id: VlobID,
    ) -> Result<(VersionInt, Self::LocalManifest), anyhow::Error> {
        let manifest = storage.get_workspace_manifest();
        Ok((manifest.base.version, manifest))
    }

    async fn validate(
        certif_ops: &CertifOps,
        needed_realm_certificate_timestamp: DateTime,
        needed_common_certificate_timestamp: DateTime,
        realm_id: VlobID,
        key_index: IndexInt,
        vlob_id: VlobID,
        author: &DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<Self, CertifValidateManifestError> {
        assert!(realm_id == vlob_id); // The workspace manifest always have the realm's ID
        certif_ops
            .validate_workspace_manifest(
                needed_realm_certificate_timestamp,
                needed_common_certificate_timestamp,
                realm_id,
                key_index,
                author,
                version,
                timestamp,
                encrypted,
            )
            .await
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl RemoteManifest for ChildManifest {
    type LocalManifest = ArcLocalChildManifest;

    fn extract_base_from_local(local: Self::LocalManifest) -> Self {
        match local {
            Self::LocalManifest::File(m) => Self::File(m.base.clone()),
            Self::LocalManifest::Folder(m) => Self::Folder(m.base.clone()),
        }
    }

    // TODO: handle entry not found
    async fn get_from_storage(
        storage: &WorkspaceDataStorage,
        entry_id: VlobID,
    ) -> Result<(VersionInt, Self::LocalManifest), anyhow::Error> {
        let manifest = storage.get_child_manifest(entry_id).await?;
        let base_version = match &manifest {
            Self::LocalManifest::File(m) => m.base.version,
            Self::LocalManifest::Folder(m) => m.base.version,
        };
        Ok((base_version, manifest))
    }

    async fn validate(
        certif_ops: &CertifOps,
        needed_realm_certificate_timestamp: DateTime,
        needed_common_certificate_timestamp: DateTime,
        realm_id: VlobID,
        key_index: IndexInt,
        vlob_id: VlobID,
        author: &DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<Self, CertifValidateManifestError> {
        certif_ops
            .validate_child_manifest(
                needed_realm_certificate_timestamp,
                needed_common_certificate_timestamp,
                realm_id,
                key_index,
                vlob_id,
                author,
                version,
                timestamp,
                encrypted,
            )
            .await
    }
}

enum FetchWithSelfHealOutcome<M> {
    LastVersion(M),
    SelfHeal {
        last_version: VersionInt,
        last_valid_manifest: M,
    },
}

async fn fetch_remote_manifest_with_self_heal<M: RemoteManifest>(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<FetchWithSelfHealOutcome<M>, WorkspaceSyncError> {
    // Retrieve remote
    let outcome = fetch_remote_manifest_last_version(ops, entry_id).await?;
    match outcome {
        FetchRemoteManifestOutcome::Valid(manifest) => {
            Ok(FetchWithSelfHealOutcome::LastVersion(manifest))
        }
        // The last version of the manifest appear to be invalid (uploaded by
        // a buggy Parsec client ?), however we cannot just fail here otherwise
        // the system would be stuck for good !
        FetchRemoteManifestOutcome::Invalid(err) => {
            // Try to find the last valid version of the manifest and continue
            // from there
            let last_version = match *err {
                InvalidManifestError::Corrupted { version, .. } => version,
                InvalidManifestError::NonExistentKeyIndex { version, .. } => version,
                InvalidManifestError::CorruptedKey { version, .. } => version,
                InvalidManifestError::NonExistentAuthor { version, .. } => version,
                InvalidManifestError::RevokedAuthor { version, .. } => version,
                InvalidManifestError::AuthorRealmRoleCannotWrite { version, .. } => version,
                InvalidManifestError::AuthorNoAccessToRealm { version, .. } => version,
            };

            let last_valid_manifest: M =
                find_last_valid_manifest(ops, entry_id, last_version).await?;

            Ok(FetchWithSelfHealOutcome::SelfHeal {
                last_version,
                last_valid_manifest,
            })
        }
    }
}

async fn find_last_valid_manifest<M: RemoteManifest>(
    ops: &WorkspaceOps,
    entry_id: VlobID,
    last_version: VersionInt,
) -> Result<M, WorkspaceSyncError> {
    let (local_base_version, local_manifest) =
        M::get_from_storage(&ops.data_storage, entry_id).await?;

    for candidate_version in (local_base_version + 1..last_version).rev() {
        let outcome = fetch_remote_manifest_version(ops, entry_id, candidate_version).await?;
        match outcome {
            // Finally found a valid manifest !
            FetchRemoteManifestOutcome::Valid(manifest) => return Ok(manifest),
            // Yet another invalid manifest, just skip it
            FetchRemoteManifestOutcome::Invalid(_) => continue,
        }
    }

    // It seems the last valid manifest is the one we already have
    let manifest = M::extract_base_from_local(local_manifest);
    Ok(manifest)
}

enum FetchRemoteManifestOutcome<M> {
    Valid(M),
    Invalid(Box<InvalidManifestError>),
}

async fn fetch_remote_manifest_last_version<M: RemoteManifest>(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<FetchRemoteManifestOutcome<M>, WorkspaceSyncError> {
    use authenticated_cmds::latest::vlob_read_batch::{Rep, Req};

    let req = Req {
        at: None,
        realm_id: ops.realm_id(),
        vlobs: vec![entry_id],
    };

    let rep = ops.cmds.send(req).await?;

    let (
        needed_realm_certificate_timestamp,
        needed_common_certificate_timestamp,
        key_index,
        author,
        version,
        timestamp,
        encrypted,
    ) = match rep {
        Rep::Ok { items, needed_common_certificate_timestamp, needed_realm_certificate_timestamp } => {
            if items.len() != 1 || items[0].0 != entry_id {
                return Err(anyhow::anyhow!("Unexpected server response: return ok status but with invalid items").into());
            }
            let (_, key_index, author, version, timestamp, encrypted) =  items.into_iter().last().expect("already checked");

            (needed_realm_certificate_timestamp, needed_common_certificate_timestamp, key_index, author, version, timestamp, encrypted)
        },
        Rep::AuthorNotAllowed => return Err(WorkspaceSyncError::NotAllowed),
        Rep::RealmNotFound => return Err(WorkspaceSyncError::NoRealm),
        // Unexpected errors :(
        rep @ (
            // We provided only a single item...
            Rep::TooManyElements
            // Don't know what to do with this status :/
            | Rep::UnknownStatus { .. }
        ) => return Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into()),
    };

    let outcome = M::validate(
        &ops.certificates_ops,
        needed_realm_certificate_timestamp,
        needed_common_certificate_timestamp,
        ops.realm_id(),
        key_index,
        entry_id,
        &author,
        version,
        timestamp,
        &encrypted,
    )
    .await;

    match outcome {
        Ok(manifest) => Ok(FetchRemoteManifestOutcome::Valid(manifest)),
        Err(err) => match err {
            CertifValidateManifestError::InvalidManifest(err) => {
                Ok(FetchRemoteManifestOutcome::Invalid(err))
            }
            CertifValidateManifestError::Offline => Err(WorkspaceSyncError::Offline),
            CertifValidateManifestError::Stopped => Err(WorkspaceSyncError::Stopped),
            CertifValidateManifestError::NotAllowed => Err(WorkspaceSyncError::NotAllowed),
            CertifValidateManifestError::InvalidCertificate(err) => {
                Err(WorkspaceSyncError::InvalidCertificate(err))
            }
            CertifValidateManifestError::InvalidKeysBundle(err) => {
                Err(WorkspaceSyncError::InvalidKeysBundle(err))
            }
            CertifValidateManifestError::Internal(err) => {
                Err(err.context("Cannot validate remote manifest").into())
            }
        },
    }
}

async fn fetch_remote_manifest_version<M: RemoteManifest>(
    ops: &WorkspaceOps,
    entry_id: VlobID,
    version: VersionInt,
) -> Result<FetchRemoteManifestOutcome<M>, WorkspaceSyncError> {
    use authenticated_cmds::latest::vlob_read_versions::{Rep, Req};

    let req = Req {
        realm_id: ops.realm_id(),
        items: vec![(entry_id, version)],
    };

    let rep = ops.cmds.send(req).await?;

    let (
        needed_realm_certificate_timestamp,
        needed_common_certificate_timestamp,
        key_index,
        author,
        version,
        timestamp,
        encrypted,
    ) = match rep {
        Rep::Ok { items, needed_common_certificate_timestamp, needed_realm_certificate_timestamp } => {
            if items.len() != 1 || items[0].0 != entry_id {
                return Err(anyhow::anyhow!("Unexpected server response: return ok status but with invalid items").into());
            }
            let (_, key_index, author, version, timestamp, encrypted) =  items.into_iter().last().expect("already checked");

            (needed_realm_certificate_timestamp, needed_common_certificate_timestamp, key_index, author, version, timestamp, encrypted)
        },
        Rep::AuthorNotAllowed => return Err(WorkspaceSyncError::NotAllowed),
        Rep::RealmNotFound => return Err(WorkspaceSyncError::NoRealm),
        // Unexpected errors :(
        rep @ (
            // We provided only a single item...
            Rep::TooManyElements
            // Don't know what to do with this status :/
            | Rep::UnknownStatus { .. }
        ) => return Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into()),
    };

    let outcome = M::validate(
        &ops.certificates_ops,
        needed_realm_certificate_timestamp,
        needed_common_certificate_timestamp,
        ops.realm_id(),
        key_index,
        entry_id,
        &author,
        version,
        timestamp,
        &encrypted,
    )
    .await;

    match outcome {
        Ok(manifest) => Ok(FetchRemoteManifestOutcome::Valid(manifest)),
        Err(err) => match err {
            CertifValidateManifestError::InvalidManifest(err) => {
                Ok(FetchRemoteManifestOutcome::Invalid(err))
            }
            CertifValidateManifestError::Offline => Err(WorkspaceSyncError::Offline),
            CertifValidateManifestError::Stopped => Err(WorkspaceSyncError::Stopped),
            CertifValidateManifestError::NotAllowed => Err(WorkspaceSyncError::NotAllowed),
            CertifValidateManifestError::InvalidCertificate(err) => {
                Err(WorkspaceSyncError::InvalidCertificate(err))
            }
            CertifValidateManifestError::InvalidKeysBundle(err) => {
                Err(WorkspaceSyncError::InvalidKeysBundle(err))
            }
            CertifValidateManifestError::Internal(err) => {
                Err(err.context("Cannot validate remote manifest").into())
            }
        },
    }
}
