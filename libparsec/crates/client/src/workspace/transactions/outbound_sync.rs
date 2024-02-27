// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;
use std::sync::Arc;

use libparsec_client_connection::protocol::authenticated_cmds;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
// use libparsec_platform_storage::workspace::GetChildManifestError;
use libparsec_types::prelude::*;

use super::super::WorkspaceOps;
use super::{update_store_with_remote_child, update_store_with_remote_root, WorkspaceSyncError};
use crate::certif::{
    CertifBootstrapWorkspaceError, CertifEncryptForRealmError,
    CertifEncryptForSequesterServicesError, CertifPollServerError,
};
use crate::workspace::store::{ForUpdateChildLocalOnlyError, WorkspaceStoreOperationError};

pub type WorkspaceGetNeedOutboundSyncEntriesError = WorkspaceStoreOperationError;

pub async fn get_need_outbound_sync(
    ops: &WorkspaceOps,
    limit: u32,
) -> Result<Vec<VlobID>, WorkspaceGetNeedOutboundSyncEntriesError> {
    ops.store.get_outbound_need_sync_entries(limit).await
}

pub async fn outbound_sync(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<OutboundSyncOutcome, WorkspaceSyncError> {
    if entry_id == ops.realm_id {
        outbound_sync_root(ops).await
    } else {
        outbound_sync_child(ops, entry_id).await
    }
    // TODO: send outbound sync event ?
}

#[derive(Debug)]
pub enum OutboundSyncOutcome {
    Done,
    InboundSyncNeeded,
    /// The entry is already locked, this is typically because it is being modified.
    /// Hence now is not the right time to sync it given we will need to re-sync
    /// it after the modification is done. Instead we should just retry later.
    EntryIsBusy,
}

async fn outbound_sync_root(ops: &WorkspaceOps) -> Result<OutboundSyncOutcome, WorkspaceSyncError> {
    let local = ops.store.get_workspace_manifest();
    if !local.need_sync {
        return Ok(OutboundSyncOutcome::Done);
    }

    // If the manifest's vlob has already been synced we know the realm exists,
    // otherwise we have to make sure it is the case !
    if local.base.version == 0 {
        let name = {
            let guard = ops.workspace_entry.lock().expect("Mutex is poisoned");
            guard.name.clone()
        };
        // Note `bootstrap_workspace` is idempotent
        ops.certificates_ops
            .bootstrap_workspace(ops.realm_id, &name)
            .await
            .map_err(|e| match e {
                CertifBootstrapWorkspaceError::Offline => WorkspaceSyncError::Offline,
                CertifBootstrapWorkspaceError::Stopped => WorkspaceSyncError::Stopped,
                CertifBootstrapWorkspaceError::AuthorNotAllowed => WorkspaceSyncError::NotAllowed,
                CertifBootstrapWorkspaceError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                } => WorkspaceSyncError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                },
                CertifBootstrapWorkspaceError::InvalidKeysBundle(err) => {
                    WorkspaceSyncError::InvalidKeysBundle(err)
                }
                CertifBootstrapWorkspaceError::InvalidCertificate(err) => {
                    WorkspaceSyncError::InvalidCertificate(err)
                }
                CertifBootstrapWorkspaceError::Internal(err) => {
                    err.context("Cannot bootstrap workspace").into()
                }
            })?;
    }

    let remote = match upload_manifest(ops, &local).await? {
        UploadManifestOutcome::VersionConflict => {
            return Ok(OutboundSyncOutcome::InboundSyncNeeded)
        }
        UploadManifestOutcome::Success(remote) => remote,
    };

    // The manifest has been updated on the server ! Now merge it back into the current
    // local manifest that may have been concurrently modified in the meantime.
    update_store_with_remote_root(ops, remote).await?;

    Ok(OutboundSyncOutcome::Done)
}

async fn outbound_sync_child(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<OutboundSyncOutcome, WorkspaceSyncError> {
    let local = {
        let outcome = ops.store.for_update_child_local_only(entry_id).await;
        match outcome {
            // Nothing to sync
            Ok((_, None)) => return Ok(OutboundSyncOutcome::Done),
            Ok((_, Some(manifest))) => manifest,
            Err(ForUpdateChildLocalOnlyError::WouldBlock) => {
                return Ok(OutboundSyncOutcome::EntryIsBusy)
            }
            Err(ForUpdateChildLocalOnlyError::Stopped) => return Err(WorkspaceSyncError::Stopped),
            Err(ForUpdateChildLocalOnlyError::Internal(err)) => {
                return Err(err.context("cannot acces entry in store").into())
            }
        }
    };

    // Note we have dropped the lock on the entry to sync, this is because we don't want
    // our sync to block local operations (e.g. file opening, creating file etc.)
    // Instead we will re-take the lock at the end of the sync to merge the remote manifest
    // with the local changes (and possibly fail to do so if the entry is locked at that point).

    let (need_sync, base_version) = match &local {
        ArcLocalChildManifest::File(m) => (m.need_sync, m.base.version),
        ArcLocalChildManifest::Folder(m) => (m.need_sync, m.base.version),
    };
    if !need_sync {
        return Ok(OutboundSyncOutcome::Done);
    }

    // The only way to be sure the bootstrap occured is to ask the certificate ops, however
    // before that there is two simple tests we can do to filter out most false positive:
    // - If the manifest's vlob has already been synced we know we can do it again !
    // - If name origin is not a placeholder, then the workspace has been bootstrapped
    //   (given initial rename is the last step).
    if base_version == 0
        && matches!(
            ops.workspace_entry
                .lock()
                .expect("Mutex is poisoned")
                .name_origin,
            CertificateBasedInfoOrigin::Placeholder
        )
    {
        let name = {
            let guard = ops.workspace_entry.lock().expect("Mutex is poisoned");
            guard.name.clone()
        };
        // Note `bootstrap_workspace` is idempotent
        ops.certificates_ops
            .bootstrap_workspace(ops.realm_id, &name)
            .await
            .map_err(|e| match e {
                CertifBootstrapWorkspaceError::Offline => WorkspaceSyncError::Offline,
                CertifBootstrapWorkspaceError::Stopped => WorkspaceSyncError::Stopped,
                CertifBootstrapWorkspaceError::AuthorNotAllowed => WorkspaceSyncError::NotAllowed,
                CertifBootstrapWorkspaceError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                } => WorkspaceSyncError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                },
                CertifBootstrapWorkspaceError::InvalidKeysBundle(err) => {
                    WorkspaceSyncError::InvalidKeysBundle(err)
                }
                CertifBootstrapWorkspaceError::InvalidCertificate(err) => {
                    WorkspaceSyncError::InvalidCertificate(err)
                }
                CertifBootstrapWorkspaceError::Internal(err) => {
                    err.context("Cannot bootstrap workspace").into()
                }
            })?;
    }

    let remote: ChildManifest = match upload_manifest(ops, &local).await? {
        UploadManifestOutcome::VersionConflict => {
            return Ok(OutboundSyncOutcome::InboundSyncNeeded)
        }
        UploadManifestOutcome::Success(remote) => remote,
    };

    // The manifest has been updated on the server ! Now merge it back into the current
    // local manifest that may have been concurrently modified in the meantime.
    match update_store_with_remote_child(ops, remote).await? {
        super::InboundSyncOutcome::Updated => Ok(OutboundSyncOutcome::Done),
        super::InboundSyncOutcome::NoChange => Ok(OutboundSyncOutcome::Done),
        super::InboundSyncOutcome::EntryIsBusy => Ok(OutboundSyncOutcome::EntryIsBusy),
    }
}

trait RemoteManifest: Sized {
    type LocalManifest: Sized;

    fn from_local(base: &Self::LocalManifest, device_id: DeviceID, timestamp: DateTime) -> Self;

    fn version(&self) -> VersionInt;

    fn id(&self) -> VlobID;

    fn dump_and_sign(&self, key: &SigningKey) -> Vec<u8>;
}

impl RemoteManifest for WorkspaceManifest {
    type LocalManifest = Arc<LocalWorkspaceManifest>;

    fn from_local(base: &Self::LocalManifest, author: DeviceID, timestamp: DateTime) -> Self {
        base.to_remote(author, timestamp)
    }

    fn version(&self) -> VersionInt {
        self.version
    }

    fn id(&self) -> VlobID {
        self.id
    }

    fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
        self.dump_and_sign(author_signkey)
    }
}

impl RemoteManifest for ChildManifest {
    type LocalManifest = ArcLocalChildManifest;

    fn from_local(base: &Self::LocalManifest, author: DeviceID, timestamp: DateTime) -> Self {
        match base {
            ArcLocalChildManifest::File(_) => {
                todo!()
                // TODO: must handle reshape
                // Self::File(m.to_remote(author, timestamp))
            }
            ArcLocalChildManifest::Folder(m) => Self::Folder(m.to_remote(author, timestamp)),
        }
    }

    fn version(&self) -> VersionInt {
        match self {
            ChildManifest::File(m) => m.version,
            ChildManifest::Folder(m) => m.version,
        }
    }

    fn id(&self) -> VlobID {
        match self {
            ChildManifest::File(m) => m.id,
            ChildManifest::Folder(m) => m.id,
        }
    }

    fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
        match self {
            ChildManifest::File(m) => m.dump_and_sign(author_signkey),
            ChildManifest::Folder(m) => m.dump_and_sign(author_signkey),
        }
    }
}

enum UploadManifestOutcome<M> {
    Success(M),
    VersionConflict,
}

async fn upload_manifest<M: RemoteManifest>(
    ops: &WorkspaceOps,
    local_manifest: &M::LocalManifest,
) -> Result<UploadManifestOutcome<M>, WorkspaceSyncError> {
    let mut timestamp = ops.device.now();

    loop {
        // Build vlob
        let to_sync = M::from_local(local_manifest, ops.device.device_id.to_owned(), timestamp);

        let signed = to_sync.dump_and_sign(&ops.device.signing_key);
        let (encrypted, key_index) = ops
            .certificates_ops
            .encrypt_for_realm(ops.realm_id(), &signed)
            .await
            .map_err(|e| match e {
                CertifEncryptForRealmError::Stopped => WorkspaceSyncError::Stopped,
                CertifEncryptForRealmError::Offline => WorkspaceSyncError::Offline,
                CertifEncryptForRealmError::NotAllowed => WorkspaceSyncError::NotAllowed,
                CertifEncryptForRealmError::NoKey => WorkspaceSyncError::NoKey,
                CertifEncryptForRealmError::InvalidKeysBundle(err) => {
                    WorkspaceSyncError::InvalidKeysBundle(err)
                }
                CertifEncryptForRealmError::Internal(err) => {
                    err.context("Cannot encrypt manifest for realm").into()
                }
            })?;
        let encrypted = encrypted.into();
        let sequester_blob = ops
            .certificates_ops
            .encrypt_for_sequester_services(&signed)
            .await
            .map_err(|e| match e {
                CertifEncryptForSequesterServicesError::Stopped => WorkspaceSyncError::Stopped,
                CertifEncryptForSequesterServicesError::Internal(err) => err
                    .context("Cannot encrypt manifest for sequester services")
                    .into(),
            })?;
        let sequester_blob =
            sequester_blob.map(|sequester_blob| HashMap::from_iter(sequester_blob.into_iter()));

        // Sync the vlob with server

        return if to_sync.version() == 1 {
            use authenticated_cmds::latest::vlob_create::{Rep, Req};
            let req = Req {
                realm_id: ops.realm_id,
                key_index,
                vlob_id: to_sync.id(),
                timestamp,
                blob: encrypted,
                sequester_blob,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_sync)),
                Rep::VlobAlreadyExists => Ok(UploadManifestOutcome::VersionConflict),
                Rep::RequireGreaterTimestamp { strictly_greater_than } => {
                    timestamp =
                        std::cmp::max(strictly_greater_than, ops.device.time_provider.now());
                    continue;
                }
                Rep::AuthorNotAllowed => return Err(WorkspaceSyncError::NotAllowed),
                Rep::TimestampOutOfBallpark { ballpark_client_early_offset, ballpark_client_late_offset, client_timestamp, server_timestamp } => {
                    return Err(WorkspaceSyncError::TimestampOutOfBallpark {
                        server_timestamp,
                        client_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                    })
                },

                // TODO: provide a dedicated error for this exotic behavior ?
                Rep::SequesterServiceUnavailable => Err(WorkspaceSyncError::Offline),
                // TODO: we should send a dedicated event for this, and return an according error
                Rep::RejectedBySequesterService { .. } => todo!(),
                // A key rotation occured concurrently, should poll for new certificates and retry
                Rep::BadKeyIndex { last_realm_certificate_timestamp } => {
                    let latest_known_timestamps = PerTopicLastTimestamps::new_for_realm(ops.realm_id, last_realm_certificate_timestamp);
                    ops.certificates_ops
                        .poll_server_for_new_certificates(Some(&latest_known_timestamps))
                        .await
                        .map_err(|err| match err {
                            CertifPollServerError::Stopped => WorkspaceSyncError::Stopped,
                            CertifPollServerError::Offline => WorkspaceSyncError::Offline,
                            CertifPollServerError::InvalidCertificate(err) => WorkspaceSyncError::InvalidCertificate(err),
                            CertifPollServerError::Internal(err) => err.context("Cannot poll server for new certificates").into(),
                        })?;
                    continue;
                }
                // Sequester services has changed concurrently, should poll for new certificates and retry
                Rep::SequesterInconsistency { last_common_certificate_timestamp } => {
                    let latest_known_timestamps = PerTopicLastTimestamps::new_for_common(last_common_certificate_timestamp);
                    ops.certificates_ops
                        .poll_server_for_new_certificates(Some(&latest_known_timestamps))
                        .await
                        .map_err(|err| match err {
                            CertifPollServerError::Stopped => WorkspaceSyncError::Stopped,
                            CertifPollServerError::Offline => WorkspaceSyncError::Offline,
                            CertifPollServerError::InvalidCertificate(err) => WorkspaceSyncError::InvalidCertificate(err),
                            CertifPollServerError::Internal(err) => err.context("Cannot poll server for new certificates").into(),
                        })?;
                    continue;
                }

                // Unexpected errors :(
                bad_rep @ (
                    // Got sequester info from certificates
                    Rep::OrganizationNotSequestered
                    // Already checked the realm exists when we called `CertifOps::encrypt_for_realm`
                    | Rep::RealmNotFound
                    // Don't know what to do with this status :/
                    | Rep::UnknownStatus { .. }
                ) => {
                    Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }
        } else {
            use authenticated_cmds::latest::vlob_update::{Rep, Req};
            let req = Req {
                key_index,
                vlob_id: to_sync.id(),
                version: to_sync.version(),
                timestamp,
                blob: encrypted,
                sequester_blob,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_sync)),
                Rep::BadVlobVersion => Ok(UploadManifestOutcome::VersionConflict),
                // Rep::VlobAlreadyExists => Ok(UploadManifestOutcome::VersionConflict),
                Rep::RequireGreaterTimestamp { strictly_greater_than } => {
                    timestamp =
                        std::cmp::max(strictly_greater_than, ops.device.time_provider.now());
                    continue;
                }
                Rep::AuthorNotAllowed => return Err(WorkspaceSyncError::NotAllowed),
                Rep::TimestampOutOfBallpark { ballpark_client_early_offset, ballpark_client_late_offset, client_timestamp, server_timestamp } => {
                    return Err(WorkspaceSyncError::TimestampOutOfBallpark {
                        server_timestamp,
                        client_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                    })
                },

                // TODO: provide a dedicated error for this exotic behavior ?
                Rep::SequesterServiceUnavailable => Err(WorkspaceSyncError::Offline),
                // TODO: we should send a dedicated event for this, and return an according error
                Rep::RejectedBySequesterService { .. } => todo!(),
                // A key rotation occured concurrently, should poll for new certificates and retry
                Rep::BadKeyIndex { last_realm_certificate_timestamp } => {
                    let latest_known_timestamps = PerTopicLastTimestamps::new_for_realm(ops.realm_id, last_realm_certificate_timestamp);
                    ops.certificates_ops
                        .poll_server_for_new_certificates(Some(&latest_known_timestamps))
                        .await
                        .map_err(|err| match err {
                            CertifPollServerError::Stopped => WorkspaceSyncError::Stopped,
                            CertifPollServerError::Offline => WorkspaceSyncError::Offline,
                            CertifPollServerError::InvalidCertificate(err) => WorkspaceSyncError::InvalidCertificate(err),
                            CertifPollServerError::Internal(err) => err.context("Cannot poll server for new certificates").into(),
                        })?;
                    continue;
                }
                // Sequester services has changed concurrently, should poll for new certificates and retry
                Rep::SequesterInconsistency { last_common_certificate_timestamp }=> {
                    let latest_known_timestamps = PerTopicLastTimestamps::new_for_common(last_common_certificate_timestamp);
                    ops.certificates_ops
                        .poll_server_for_new_certificates(Some(&latest_known_timestamps))
                        .await
                        .map_err(|err| match err {
                            CertifPollServerError::Stopped => WorkspaceSyncError::Stopped,
                            CertifPollServerError::Offline => WorkspaceSyncError::Offline,
                            CertifPollServerError::InvalidCertificate(err) => WorkspaceSyncError::InvalidCertificate(err),
                            CertifPollServerError::Internal(err) => err.context("Cannot poll server for new certificates").into(),
                        })?;
                    continue;
                }

                // Unexpected errors :(
                bad_rep @ (
                    // Got sequester info from certificates
                    Rep::OrganizationNotSequestered
                    // Already checked the vlob exists since the manifest has version > 0
                    | Rep::VlobNotFound
                    // Don't know what to do with this status :/
                    | Rep::UnknownStatus { .. }
                ) => {
                    Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }
        };
    }
}
