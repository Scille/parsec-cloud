// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;
use std::sync::Arc;

use libparsec_client_connection::protocol::authenticated_cmds;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use super::super::WorkspaceOps;
use super::WorkspaceSyncError;
use crate::certif::{
    CertifBootstrapWorkspaceError, CertifEncryptForRealmError,
    CertifEncryptForSequesterServicesError, CertifPollServerError,
};
use crate::workspace::store::{
    ForUpdateChildLocalOnlyError, ReadChunkError, ReadChunkLocalOnlyError,
    WorkspaceStoreOperationError,
};

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

/// Helper to check if the manifest has been updated concurrently in the storage.
///
/// Note we could also just do `original != new` instead of using this function,
/// however we want to make sure the comparison is done by pointer value before
/// falling back to value comparison (as our most likely case is the manifest
/// hasn't changed and the arc object is kept in the storage cache).
///
/// However `Arc`'s equality uses this optimisation only if the wrapped structure
/// implements `Eq`, so removing this from `LocalFileManifest` would silently
/// break this optimisation.
///
/// see https://github.com/rust-lang/rust/blob/c3b05c6e5b5b59613350b8c2875b0add67ed74df/library/alloc/src/sync.rs#L3074-L3092
fn file_has_changed(original: &Arc<LocalFileManifest>, new: &Arc<LocalFileManifest>) -> bool {
    !Arc::ptr_eq(original, new) || **original != **new
}

fn folder_has_changed(original: &Arc<LocalFolderManifest>, new: &Arc<LocalFolderManifest>) -> bool {
    !Arc::ptr_eq(original, new) || **original != **new
}

fn root_has_changed(
    original: &Arc<LocalWorkspaceManifest>,
    new: &Arc<LocalWorkspaceManifest>,
) -> bool {
    !Arc::ptr_eq(original, new) || **original != **new
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

    // 1) If the manifest's vlob has already been synced we know the realm exists,
    // otherwise we have to make sure it is the case !
    if local.base.version == 0 {
        let name = {
            let guard = ops
                .workspace_external_info
                .lock()
                .expect("Mutex is poisoned");
            guard.entry.name.clone()
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

    // 2) Upload the manifest on the server

    let to_upload = local.to_remote(ops.device.device_id.to_owned(), ops.device.now());

    let remote = match upload_manifest(ops, to_upload).await? {
        UploadManifestOutcome::VersionConflict => {
            return Ok(OutboundSyncOutcome::InboundSyncNeeded)
        }
        UploadManifestOutcome::Success(remote) => remote,
    };

    // 3) Update the local storage with the new remote manifest

    // Lock back the entry or abort if it has changed in the meantime

    let (updater, refreshed_local) = ops.store.for_update_root().await;
    if root_has_changed(&local, &refreshed_local) {
        return Ok(OutboundSyncOutcome::EntryIsBusy);
    }

    // Do the actual storage update

    let local_from_remote = Arc::new(LocalWorkspaceManifest::from_remote(remote, None));
    updater
        .update_workspace_manifest(local_from_remote, None)
        .await
        .map_err(|err| match err {
            WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
            WorkspaceStoreOperationError::Internal(err) => {
                err.context("cannot update manifest").into()
            }
        })?;

    Ok(OutboundSyncOutcome::Done)
}

async fn outbound_sync_child(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<OutboundSyncOutcome, WorkspaceSyncError> {
    // 1) Ensure the sync is needed

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

    // 2) Bootstrap workspace if needed

    // The only way to be sure the bootstrap occured is to ask the certificate ops, however
    // before that there is two simple tests we can do to filter out most false positive:
    // - If the manifest's vlob has already been synced we know we can do it again !
    // - If name origin is not a placeholder, then the workspace has been bootstrapped
    //   (given initial rename is the last step).
    if base_version == 0
        && matches!(
            ops.workspace_external_info
                .lock()
                .expect("Mutex is poisoned")
                .entry
                .name_origin,
            CertificateBasedInfoOrigin::Placeholder
        )
    {
        let name = {
            let guard = ops
                .workspace_external_info
                .lock()
                .expect("Mutex is poisoned");
            guard.entry.name.clone()
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

    match local {
        ArcLocalChildManifest::File(local) => outbound_sync_file(ops, local).await,
        ArcLocalChildManifest::Folder(local) => outbound_sync_folder(ops, local).await,
    }
}

async fn outbound_sync_file(
    ops: &WorkspaceOps,
    local: Arc<LocalFileManifest>,
) -> Result<OutboundSyncOutcome, WorkspaceSyncError> {
    // 1) The uploaded manifest should only refers to data that exist on the server.
    // Hence we must first upload the pseudo-blocks that are not already on the server.

    let outcome = reshape_and_upload_blocks(ops, local).await?;
    let local = match outcome {
        ReshapeAndUploadBlocksOutcome::Done(local_reshaped) => local_reshaped,
        ReshapeAndUploadBlocksOutcome::EntryIsBusy => return Ok(OutboundSyncOutcome::EntryIsBusy),
    };

    // 2) Upload the manifest on the server

    let timestamp = ops.device.now();
    let to_upload = local
        .to_remote(ops.device.device_id.to_owned(), timestamp)
        .expect("already reshaped");

    let remote = match upload_manifest(ops, to_upload).await? {
        UploadManifestOutcome::VersionConflict => {
            return Ok(OutboundSyncOutcome::InboundSyncNeeded)
        }
        UploadManifestOutcome::Success(remote) => remote,
    };

    // 3) Update the local storage with the new remote manifest

    // Lock back the entry or abort if it has changed in the meantime

    let updater = {
        let outcome = ops.store.for_update_child_local_only(local.base.id).await;
        match outcome {
            Ok((updater, Some(ArcLocalChildManifest::File(refreshed_local))))
                if !file_has_changed(&local, &refreshed_local) =>
            {
                updater
            }
            // TODO: There is a issue here if a inbound sync is triggered before we retry
            // the outbound sync. In this case the inbound sync will detect a false conflict
            // there is a new remote (which we've just uploaded !) that doesn't correspond
            // to the local's base.
            // A possible solution for this would be to acknowledge the new remote by
            // updating the local's base before returning `EntryIsBusy`. However we should
            // be careful about concurrency implications :/
            Ok(_) => return Ok(OutboundSyncOutcome::EntryIsBusy),
            Err(ForUpdateChildLocalOnlyError::WouldBlock) => {
                return Ok(OutboundSyncOutcome::EntryIsBusy)
            }
            Err(ForUpdateChildLocalOnlyError::Stopped) => return Err(WorkspaceSyncError::Stopped),
            Err(ForUpdateChildLocalOnlyError::Internal(err)) => {
                return Err(err.context("cannot acces entry in store").into())
            }
        }
    };

    // Do the actual storage update

    let local_from_remote = Arc::new(LocalFileManifest::from_remote(remote));
    updater
        .update_manifest(ArcLocalChildManifest::File(local_from_remote))
        .await
        .map_err(|err| match err {
            WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
            WorkspaceStoreOperationError::Internal(err) => {
                err.context("cannot update manifest").into()
            }
        })?;

    Ok(OutboundSyncOutcome::Done)
}

async fn outbound_sync_folder(
    ops: &WorkspaceOps,
    local: Arc<LocalFolderManifest>,
) -> Result<OutboundSyncOutcome, WorkspaceSyncError> {
    // 1) Upload the manifest on the server

    let timestamp = ops.device.now();
    let to_upload = local.to_remote(ops.device.device_id.to_owned(), timestamp);

    let remote = match upload_manifest(ops, to_upload).await? {
        UploadManifestOutcome::VersionConflict => {
            return Ok(OutboundSyncOutcome::InboundSyncNeeded)
        }
        UploadManifestOutcome::Success(remote) => remote,
    };

    // 2) Update the local storage with the new remote manifest

    // Lock back the entry or abort if it has changed in the meantime

    let updater = {
        let outcome = ops.store.for_update_child_local_only(local.base.id).await;
        match outcome {
            Ok((updater, Some(ArcLocalChildManifest::Folder(refreshed_local))))
                if !folder_has_changed(&local, &refreshed_local) =>
            {
                updater
            }
            Ok(_) => return Ok(OutboundSyncOutcome::EntryIsBusy),
            Err(ForUpdateChildLocalOnlyError::WouldBlock) => {
                return Ok(OutboundSyncOutcome::EntryIsBusy)
            }
            Err(ForUpdateChildLocalOnlyError::Stopped) => return Err(WorkspaceSyncError::Stopped),
            Err(ForUpdateChildLocalOnlyError::Internal(err)) => {
                return Err(err.context("cannot acces entry in store").into())
            }
        }
    };

    // Do the actual storage update

    let local_from_remote = Arc::new(LocalFolderManifest::from_remote(remote, None));
    updater
        .update_manifest(ArcLocalChildManifest::Folder(local_from_remote))
        .await
        .map_err(|err| match err {
            WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
            WorkspaceStoreOperationError::Internal(err) => {
                err.context("cannot update manifest").into()
            }
        })?;

    Ok(OutboundSyncOutcome::Done)
}

trait RemoteManifest: Sized {
    type LocalManifest: Sized;

    fn timestamp(&self) -> DateTime;

    fn update_timestamp(&mut self, timestamp: DateTime);

    fn version(&self) -> VersionInt;

    fn id(&self) -> VlobID;

    fn dump_and_sign(&self, key: &SigningKey) -> Vec<u8>;
}

impl RemoteManifest for WorkspaceManifest {
    type LocalManifest = Arc<LocalWorkspaceManifest>;

    fn timestamp(&self) -> DateTime {
        self.timestamp
    }

    fn update_timestamp(&mut self, timestamp: DateTime) {
        self.timestamp = timestamp;
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

    fn timestamp(&self) -> DateTime {
        match self {
            ChildManifest::File(m) => m.timestamp,
            ChildManifest::Folder(m) => m.timestamp,
        }
    }

    fn update_timestamp(&mut self, timestamp: DateTime) {
        match self {
            ChildManifest::File(m) => m.timestamp = timestamp,
            ChildManifest::Folder(m) => m.timestamp = timestamp,
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

impl RemoteManifest for FolderManifest {
    type LocalManifest = Arc<LocalFolderManifest>;

    fn timestamp(&self) -> DateTime {
        self.timestamp
    }

    fn update_timestamp(&mut self, timestamp: DateTime) {
        self.timestamp = timestamp;
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

impl RemoteManifest for FileManifest {
    type LocalManifest = Arc<LocalFileManifest>;

    fn timestamp(&self) -> DateTime {
        self.timestamp
    }

    fn update_timestamp(&mut self, timestamp: DateTime) {
        self.timestamp = timestamp;
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

enum UploadManifestOutcome<M> {
    Success(M),
    VersionConflict,
}

async fn upload_manifest<M: RemoteManifest>(
    ops: &WorkspaceOps,
    mut to_upload: M,
) -> Result<UploadManifestOutcome<M>, WorkspaceSyncError> {
    loop {
        // Build vlob

        let signed = to_upload.dump_and_sign(&ops.device.signing_key);
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

        return if to_upload.version() == 1 {
            use authenticated_cmds::latest::vlob_create::{Rep, Req};
            let req = Req {
                realm_id: ops.realm_id,
                key_index,
                vlob_id: to_upload.id(),
                timestamp: to_upload.timestamp(),
                blob: encrypted,
                sequester_blob,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_upload)),
                Rep::VlobAlreadyExists => Ok(UploadManifestOutcome::VersionConflict),
                Rep::RequireGreaterTimestamp { strictly_greater_than } => {
                    let timestamp =
                        std::cmp::max(strictly_greater_than, ops.device.time_provider.now());
                    to_upload.update_timestamp(timestamp);
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
                vlob_id: to_upload.id(),
                version: to_upload.version(),
                timestamp: to_upload.timestamp(),
                blob: encrypted,
                sequester_blob,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_upload)),
                Rep::BadVlobVersion => Ok(UploadManifestOutcome::VersionConflict),
                // Rep::VlobAlreadyExists => Ok(UploadManifestOutcome::VersionConflict),
                Rep::RequireGreaterTimestamp { strictly_greater_than } => {
                    let timestamp =
                        std::cmp::max(strictly_greater_than, ops.device.time_provider.now());
                    to_upload.update_timestamp(timestamp);
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

enum ReshapeAndUploadBlocksOutcome {
    Done(Arc<LocalFileManifest>),
    EntryIsBusy,
}

async fn reshape_and_upload_blocks(
    ops: &WorkspaceOps,
    mut manifest: Arc<LocalFileManifest>,
) -> Result<ReshapeAndUploadBlocksOutcome, WorkspaceSyncError> {
    // We don't lock the entry given sync operation shouldn't take priority over
    // local operations (e.g. file opening, creating file etc.).
    //
    // Instead we re-take the lock anytime we need to update the manifest, and
    // ensure it hasn't changed in the meantime (in which case we just abort the
    // sync, it will be retried later when things are more quiet).
    //
    // So we use the pointer on the manifest to check if it has changed, this a bit of
    // a hack as it relies on the fact the store keep the manifest in cache (and never
    // clean them).
    //
    // That's why `manifest` variable gets modified each time the storage is updated
    // as a way of keeping track of the current state of the manifest we know about.

    // 1) Ensure the file is reshaped

    loop {
        match do_next_reshape_operation(ops, manifest.clone()).await? {
            DoNextReshapeOperationOutcome::Done(refreshed_manifest) => {
                manifest = refreshed_manifest
            }
            DoNextReshapeOperationOutcome::AlreadyReshaped => break,
            DoNextReshapeOperationOutcome::EntryIsBusy => {
                return Ok(ReshapeAndUploadBlocksOutcome::EntryIsBusy)
            }
        };
    }

    // 2) Now turn the pseudo-blocks into actual blocks by uploading them

    loop {
        match do_next_pseudo_block_upload(ops, manifest.clone()).await? {
            DoNextPseudoBlockUploadOutcome::Done(refreshed_manifest) => {
                manifest = refreshed_manifest;
            }
            DoNextPseudoBlockUploadOutcome::NoMorePseudoBlock => break,
            DoNextPseudoBlockUploadOutcome::EntryIsBusy => {
                return Ok(ReshapeAndUploadBlocksOutcome::EntryIsBusy)
            }
        };
    }

    Ok(ReshapeAndUploadBlocksOutcome::Done(manifest))
}

enum DoNextReshapeOperationOutcome {
    Done(Arc<LocalFileManifest>),
    AlreadyReshaped,
    EntryIsBusy,
}

async fn do_next_reshape_operation(
    ops: &WorkspaceOps,
    mut manifest: Arc<LocalFileManifest>,
) -> Result<DoNextReshapeOperationOutcome, WorkspaceSyncError> {
    let original_manifest = manifest.clone();
    let manifest_base = &original_manifest.base;
    let manifest_mut: &mut LocalFileManifest = Arc::make_mut(&mut manifest);

    let reshape = match super::file_operations::prepare_reshape(manifest_mut)
        .find(|reshape| !reshape.is_pseudo_block())
    {
        Some(reshape) => reshape,
        // Reshape is all finished \o/
        None => return Ok(DoNextReshapeOperationOutcome::AlreadyReshaped),
    };

    // 1) Build the chunk of data resulting of the reshape...

    let mut buf = Vec::with_capacity(reshape.destination().size() as usize);
    let mut buf_size = 0;
    let start = reshape.destination().start;
    for chunk in reshape.source().iter() {
        let outcome = ops.store.get_chunk(chunk, manifest_base).await;
        match outcome {
            Ok(chunk_data) => {
                chunk
                    .copy_between_start_and_stop(&chunk_data, start, &mut buf, &mut buf_size)
                    .expect("write on vec cannot fail");
            }
            Err(err) => {
                return Err(match err {
                    ReadChunkError::Offline => WorkspaceSyncError::Offline,
                    ReadChunkError::Stopped => WorkspaceSyncError::Stopped,
                    // TODO: manifest seems to contain invalid data (or the server is lying to us)
                    ReadChunkError::ChunkNotFound => todo!(),
                    ReadChunkError::NoRealmAccess => WorkspaceSyncError::NotAllowed,
                    ReadChunkError::InvalidBlockAccess(err) => {
                        WorkspaceSyncError::InvalidBlockAccess(err)
                    }
                    ReadChunkError::InvalidKeysBundle(err) => {
                        WorkspaceSyncError::InvalidKeysBundle(err)
                    }
                    ReadChunkError::InvalidCertificate(err) => {
                        WorkspaceSyncError::InvalidCertificate(err)
                    }
                    ReadChunkError::Internal(err) => err.context("cannot get chunk").into(),
                });
            }
        }
    }

    if buf_size < buf.len() {
        buf.extend_from_slice(&vec![0; buf.len() - buf_size]);
    }

    // 2) Save the manifest with the new chunk in the storage

    // Lock back the entry or abort if it has changed in the meantime

    let updater = {
        let outcome = ops
            .store
            .for_update_child_local_only(manifest_base.id)
            .await;
        match outcome {
            Ok((updater, Some(ArcLocalChildManifest::File(refreshed_manifest)))) => {
                if file_has_changed(&original_manifest, &refreshed_manifest) {
                    return Ok(DoNextReshapeOperationOutcome::EntryIsBusy);
                }
                updater
            }
            // Entry has changed type, hence it has been modified and we should retry later
            Ok((_, None | Some(ArcLocalChildManifest::Folder(_)))) => {
                return Ok(DoNextReshapeOperationOutcome::EntryIsBusy)
            }
            Err(ForUpdateChildLocalOnlyError::WouldBlock) => {
                return Ok(DoNextReshapeOperationOutcome::EntryIsBusy)
            }
            Err(ForUpdateChildLocalOnlyError::Stopped) => return Err(WorkspaceSyncError::Stopped),
            Err(ForUpdateChildLocalOnlyError::Internal(err)) => {
                return Err(err.context("cannot acces entry in store").into())
            }
        }
    };

    // Do the actual storage update

    let to_remove_chunk_ids = reshape.cleanup_ids();
    let new_chunk_id = reshape.destination().id;
    reshape.commit(&buf);
    updater
        .update_manifest_and_chunks(
            ArcLocalChildManifest::File(manifest.clone()),
            [(new_chunk_id, buf)].into_iter(),
            to_remove_chunk_ids.into_iter(),
            [].into_iter(),
        )
        .await
        .map_err(|err| match err {
            WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
            WorkspaceStoreOperationError::Internal(err) => {
                err.context("cannot update file manifest&chunks").into()
            }
        })?;

    Ok(DoNextReshapeOperationOutcome::Done(manifest))
}

enum DoNextPseudoBlockUploadOutcome {
    Done(Arc<LocalFileManifest>),
    NoMorePseudoBlock,
    EntryIsBusy,
}

async fn do_next_pseudo_block_upload(
    ops: &WorkspaceOps,
    mut manifest: Arc<LocalFileManifest>,
) -> Result<DoNextPseudoBlockUploadOutcome, WorkspaceSyncError> {
    let original_manifest = manifest.clone();
    let manifest_base = &original_manifest.base;
    let manifest_mut: &mut LocalFileManifest = Arc::make_mut(&mut manifest);

    // 1) Find the next pseudo-block to upload

    let pseudo_block = manifest_mut.blocks.iter_mut().find(|block| {
        assert!(block.len() == 1); // Already reshaped
        !block[0].is_block()
    });

    let chunk = match pseudo_block {
        None => return Ok(DoNextPseudoBlockUploadOutcome::NoMorePseudoBlock),
        Some(pseudo_block) => &mut pseudo_block[0],
    };

    // 2) Get back data and encrypt with the last realm key

    let data = ops
        .store
        .get_chunk_local_only(chunk)
        .await
        .map_err(|err| match err {
            ReadChunkLocalOnlyError::Stopped => WorkspaceSyncError::Stopped,
            ReadChunkLocalOnlyError::ChunkNotFound => anyhow::anyhow!(
                "Local manifest {} references chunk {} that doesn' exist in the local storage !",
                manifest_base.id,
                chunk.id
            )
            .into(),
            ReadChunkLocalOnlyError::Internal(err) => err.context("cannot get chunk").into(),
        })?;

    let (encrypted, key_index) = ops
        .certificates_ops
        .encrypt_for_realm(ops.realm_id, &data)
        .await
        .map_err(|err| match err {
            CertifEncryptForRealmError::Stopped => WorkspaceSyncError::Stopped,
            CertifEncryptForRealmError::Offline => WorkspaceSyncError::Offline,
            CertifEncryptForRealmError::NotAllowed => WorkspaceSyncError::NotAllowed,
            CertifEncryptForRealmError::NoKey => WorkspaceSyncError::NoKey,
            CertifEncryptForRealmError::InvalidKeysBundle(err) => {
                WorkspaceSyncError::InvalidKeysBundle(err)
            }
            CertifEncryptForRealmError::Internal(err) => {
                err.context("cannot encrypt for realm").into()
            }
        })?;

    // 3) Upload the block

    // It's is important to create a new block ID here instead of use the chunk ID !
    //
    // This is because the block upload operation can't be handled in an idempotent
    // way as the data are encrypted with the last realm key index and we store the
    // key index in the manifest only after the block is uploaded.
    let block_id = BlockID::default();

    {
        use authenticated_cmds::latest::block_create::{Rep, Req};
        let req = Req {
            realm_id: ops.realm_id,
            block_id,
            block: encrypted.into(),
        };
        let rep = ops.cmds.send(req).await?;
        match rep {
            Rep::Ok => (),
            Rep::AuthorNotAllowed => return Err(WorkspaceSyncError::NotAllowed),
            // Nothing we can do if server is not ready to store our data, retry later
            Rep::StoreUnavailable => return Ok(DoNextPseudoBlockUploadOutcome::EntryIsBusy),

            // Unexpected errors :(
            bad_rep @ (
                // Already checked the realm exists when we called `CertifOps::encrypt_for_realm`
                | Rep::RealmNotFound
                // We have just generated this block ID, it can't already exist
                | Rep::BlockAlreadyExists
                // Don't know what to do with this status :/
                | Rep::UnknownStatus { .. }
            ) => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }
    }

    // 4) Update the manifest now that the block has become an actual block

    // Update the chunk entry with it new ID and key index

    {
        // TODO: This block promotion doesn't fit our current design given it considers
        // reshape and block upload as a single operation.
        // In fact we first do the reshape (i.e. tweak the existing chunks resulting from
        // the local file write operations into a single chunk with a correct size and
        // offset ready to be uploaded as block, aka a pseudo block).
        // Then we do the block upload (i.e. what we are doing right now), at wich point
        // the chunk have an access field that:
        // - Corresponds to a real block
        // - Corresponds to nothing, in which case the `key_index` field has 0 value. This
        //   is if this chunk has been reshaped.
        // - Is `None` if the chunk hasn't been reshaped.
        let _ = chunk.promote_as_block(&data);

        let access = chunk.access.as_mut().expect("pseudo block has access");
        access.key_index = key_index;
        access.key = None;
        access.id = block_id;
    }
    let to_promote_chunk_id = chunk.id;
    chunk.id = block_id.into();

    // Lock back the entry or abort if it has changed in the meantime

    let updater = {
        let outcome = ops
            .store
            .for_update_child_local_only(manifest_base.id)
            .await;
        match outcome {
            Ok((updater, Some(ArcLocalChildManifest::File(refreshed_manifest)))) => {
                if file_has_changed(&original_manifest, &refreshed_manifest) {
                    return Ok(DoNextPseudoBlockUploadOutcome::EntryIsBusy);
                }
                updater
            }
            // Entry has changed type, hence it has been modified and we should retry later
            Ok((_, None | Some(ArcLocalChildManifest::Folder(_)))) => {
                return Ok(DoNextPseudoBlockUploadOutcome::EntryIsBusy)
            }
            Err(ForUpdateChildLocalOnlyError::WouldBlock) => {
                return Ok(DoNextPseudoBlockUploadOutcome::EntryIsBusy)
            }
            Err(ForUpdateChildLocalOnlyError::Stopped) => return Err(WorkspaceSyncError::Stopped),
            Err(ForUpdateChildLocalOnlyError::Internal(err)) => {
                return Err(err.context("cannot acces entry in store").into())
            }
        }
    };

    // Do the actual storage update

    updater
        .update_manifest_and_chunks(
            ArcLocalChildManifest::File(manifest.clone()),
            [].into_iter(),
            [].into_iter(),
            [(to_promote_chunk_id, block_id, ops.device.now())].into_iter(),
        )
        .await
        .map_err(|err| match err {
            WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
            WorkspaceStoreOperationError::Internal(err) => {
                err.context("cannot update file manifest&chunks").into()
            }
        })?;

    Ok(DoNextPseudoBlockUploadOutcome::Done(manifest))
}
