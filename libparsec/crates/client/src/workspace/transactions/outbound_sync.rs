// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::protocol::authenticated_cmds;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use super::super::WorkspaceOps;
use super::WorkspaceSyncError;
use crate::certif::{
    CertifBootstrapWorkspaceError, CertifEncryptForRealmError, CertifPollServerError,
};
use crate::workspace::store::{
    ForUpdateSyncError, ForUpdateSyncLocalOnlyError, PathConfinementPoint, ReadChunkOrBlockError,
    RetrievePathFromIDEntry, WorkspaceStoreOperationError,
};
use crate::{
    greater_timestamp, EncrytionUsage, EventWorkspaceOpsOutboundSyncAborted,
    EventWorkspaceOpsOutboundSyncDone, EventWorkspaceOpsOutboundSyncProgress,
    EventWorkspaceOpsOutboundSyncStarted, GreaterTimestampOffset,
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
    outbound_sync_child(ops, entry_id).await
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

#[derive(Debug)]
pub enum OutboundSyncOutcome {
    Done,
    /// A more recent version of the entry exists on the server, we must first integrate
    /// it before re-trying to sync our own changes.
    InboundSyncNeeded,
    /// The entry is already locked, this is typically because it is being modified.
    /// Hence now is not the right time to sync it given we will need to re-sync
    /// it after the modification is done. Instead we should just retry later.
    EntryIsBusy,
    /// The entry is unreachable, this is typically because the entry has been deleted.
    EntryIsUnreachable,
    /// The entry is confined
    EntryIsConfined {
        confinement_point: VlobID,
        // The chain of entry IDs from the root to the provided entry
        // (both the root and the provided entry are included in the chain).
        entry_chain: Vec<VlobID>,
    },
}

async fn outbound_sync_child(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<OutboundSyncOutcome, WorkspaceSyncError> {
    // 1) Ensure the sync is needed

    let local = {
        let outcome = ops.store.for_update_sync(entry_id, false).await;
        match outcome {
            Ok((_, RetrievePathFromIDEntry::Missing)) => return Ok(OutboundSyncOutcome::Done),
            Ok((
                _,
                RetrievePathFromIDEntry::Reachable {
                    manifest,
                    confinement_point: PathConfinementPoint::NotConfined,
                    ..
                },
            )) => manifest,
            Ok((
                _,
                RetrievePathFromIDEntry::Reachable {
                    confinement_point: PathConfinementPoint::Confined(confinement_point),
                    entry_chain,
                    ..
                },
            )) => {
                return Ok(OutboundSyncOutcome::EntryIsConfined {
                    confinement_point,
                    entry_chain,
                })
            }
            Ok((_, RetrievePathFromIDEntry::Unreachable { .. })) => {
                return Ok(OutboundSyncOutcome::EntryIsUnreachable)
            }
            Err(ForUpdateSyncError::WouldBlock) => return Ok(OutboundSyncOutcome::EntryIsBusy),
            Err(ForUpdateSyncError::Offline(e)) => return Err(WorkspaceSyncError::Offline(e)),
            Err(ForUpdateSyncError::InvalidKeysBundle(e)) => {
                return Err(WorkspaceSyncError::InvalidKeysBundle(e))
            }
            Err(ForUpdateSyncError::InvalidCertificate(e)) => {
                return Err(WorkspaceSyncError::InvalidCertificate(e))
            }
            Err(ForUpdateSyncError::InvalidManifest(e)) => {
                return Err(WorkspaceSyncError::InvalidManifest(e))
            }
            Err(ForUpdateSyncError::NoRealmAccess) => return Err(WorkspaceSyncError::NotAllowed),

            Err(ForUpdateSyncError::Stopped) => return Err(WorkspaceSyncError::Stopped),
            Err(ForUpdateSyncError::Internal(err)) => {
                return Err(err.context("cannot access entry in store").into())
            }
        }
    };

    // Note we have dropped the lock on the entry to sync, this is because we don't want
    // our sync to block local operations (e.g. file opening, creating file etc.)
    // Instead we will re-take the lock at the end of the sync to merge the remote manifest
    // with the local changes (and possibly fail to do so if the entry is locked at that point).

    #[cfg(test)]
    libparsec_tests_fixtures::moment_define_inject_point(
        libparsec_tests_fixtures::Moment::OutboundSyncLocalRetrieved,
    )
    .await;

    let (need_sync, base_version) = match &local {
        ArcLocalChildManifest::File(m) => (m.need_sync, m.base.version),
        ArcLocalChildManifest::Folder(m) => (m.need_sync, m.base.version),
    };
    if !need_sync {
        return Ok(OutboundSyncOutcome::Done);
    }

    // 2) Bootstrap workspace if needed

    // The only way to be sure the bootstrap occurred is to ask the certificate ops, however
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
                CertifBootstrapWorkspaceError::Offline(e) => WorkspaceSyncError::Offline(e),
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

    // Synchro start/done/aborted events are handled here, the last one is the progress
    // event that is only triggered when uploading blocks and hence is handled in
    // `reshape_and_upload_blocks`.

    let event = EventWorkspaceOpsOutboundSyncStarted {
        realm_id: ops.realm_id,
        entry_id,
    };
    ops.event_bus.send(&event);

    let outcome = match local {
        ArcLocalChildManifest::File(local) => outbound_sync_file(ops, local).await,
        ArcLocalChildManifest::Folder(local) => outbound_sync_folder(ops, local).await,
    };

    if matches!(outcome, Ok(OutboundSyncOutcome::Done)) {
        let event = EventWorkspaceOpsOutboundSyncDone {
            realm_id: ops.realm_id,
            entry_id,
        };
        ops.event_bus.send(&event);
    } else {
        let event = EventWorkspaceOpsOutboundSyncAborted {
            realm_id: ops.realm_id,
            entry_id,
        };
        ops.event_bus.send(&event);
    }

    outcome
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

    #[cfg(test)]
    libparsec_tests_fixtures::moment_define_inject_point(
        libparsec_tests_fixtures::Moment::OutboundSyncFileReshapedAndBlockUploaded,
    )
    .await;

    // 2) Upload the manifest on the server

    let timestamp = ops.device.now();
    let to_upload = local
        .to_remote(ops.device.device_id.to_owned(), timestamp)
        // An error occurs if the manifest is not reshaped
        // but we've just reshaped it, so this unwrap is safe
        .expect("Already reshaped");

    let remote = match upload_manifest(ops, to_upload).await? {
        UploadManifestOutcome::VersionConflict => {
            return Ok(OutboundSyncOutcome::InboundSyncNeeded)
        }
        UploadManifestOutcome::Success(remote) => remote,
    };

    // 3) Update the local storage with the new remote manifest

    // Lock back the entry or abort if it has changed in the meantime

    let updater = {
        let outcome = ops.store.for_update_sync_local_only(local.base.id).await;
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
            Err(ForUpdateSyncLocalOnlyError::WouldBlock) => {
                return Ok(OutboundSyncOutcome::EntryIsBusy)
            }
            Err(ForUpdateSyncLocalOnlyError::Stopped) => return Err(WorkspaceSyncError::Stopped),
            Err(ForUpdateSyncLocalOnlyError::Internal(err)) => {
                return Err(err.context("cannot access entry in store").into())
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
        let outcome = ops.store.for_update_sync_local_only(local.base.id).await;
        match outcome {
            Ok((updater, Some(ArcLocalChildManifest::Folder(refreshed_local))))
                if !folder_has_changed(&local, &refreshed_local) =>
            {
                updater
            }
            Ok(_) => return Ok(OutboundSyncOutcome::EntryIsBusy),
            Err(ForUpdateSyncLocalOnlyError::WouldBlock) => {
                return Ok(OutboundSyncOutcome::EntryIsBusy)
            }
            Err(ForUpdateSyncLocalOnlyError::Stopped) => return Err(WorkspaceSyncError::Stopped),
            Err(ForUpdateSyncLocalOnlyError::Internal(err)) => {
                return Err(err.context("cannot access entry in store").into())
            }
        }
    };

    // Do the actual storage update

    let local_from_remote = Arc::new(
        LocalFolderManifest::from_remote_with_restored_local_confinement_points(
            remote,
            &ops.config.prevent_sync_pattern,
            local.as_ref(),
            timestamp,
        ),
    );
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
    let vlob_id = to_upload.id();

    loop {
        // Build vlob

        let signed = to_upload.dump_and_sign(&ops.device.signing_key);
        let (encrypted, key_index) = ops
            .certificates_ops
            .encrypt_for_realm(EncrytionUsage::Vlob(vlob_id), ops.realm_id(), &signed)
            .await
            .map_err(|e| match e {
                CertifEncryptForRealmError::Stopped => WorkspaceSyncError::Stopped,
                CertifEncryptForRealmError::Offline(e) => WorkspaceSyncError::Offline(e),
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

        // Sync the vlob with server

        return if to_upload.version() == 1 {
            use authenticated_cmds::latest::vlob_create::{Rep, Req};
            let req = Req {
                realm_id: ops.realm_id,
                key_index,
                vlob_id,
                timestamp: to_upload.timestamp(),
                blob: encrypted,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_upload)),
                Rep::VlobAlreadyExists => Ok(UploadManifestOutcome::VersionConflict),
                Rep::RequireGreaterTimestamp { strictly_greater_than } => {
                    let  timestamp = greater_timestamp(
                        &ops.device.time_provider,
                        GreaterTimestampOffset::Manifest,
                        strictly_greater_than,
                    );
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
                Rep::SequesterServiceUnavailable { service_id } => Err(anyhow::anyhow!("Sequester service {service_id} unavailable").into()),
                // TODO: we should send a dedicated event for this, and return an according error
                Rep::RejectedBySequesterService { service_id, reason } => Err(anyhow::anyhow!("Rejected by sequester service {service_id} ({reason:?})").into()),
                // A key rotation occurred concurrently, should poll for new certificates and retry
                Rep::BadKeyIndex { last_realm_certificate_timestamp } => {
                    let latest_known_timestamps = PerTopicLastTimestamps::new_for_realm(ops.realm_id, last_realm_certificate_timestamp);
                    ops.certificates_ops
                        .poll_server_for_new_certificates(Some(&latest_known_timestamps))
                        .await
                        .map_err(|err| match err {
                            CertifPollServerError::Stopped => WorkspaceSyncError::Stopped,
                            CertifPollServerError::Offline(e) => WorkspaceSyncError::Offline(e),
                            CertifPollServerError::InvalidCertificate(err) => WorkspaceSyncError::InvalidCertificate(err),
                            CertifPollServerError::Internal(err) => err.context("Cannot poll server for new certificates").into(),
                        })?;
                    continue;
                }

                // Unexpected errors :(
                bad_rep @ (
                    // Already checked the realm exists when we called `CertificateOps::encrypt_for_realm`
                    Rep::RealmNotFound
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
                realm_id: ops.realm_id,
                vlob_id: to_upload.id(),
                version: to_upload.version(),
                timestamp: to_upload.timestamp(),
                blob: encrypted,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_upload)),
                Rep::BadVlobVersion => Ok(UploadManifestOutcome::VersionConflict),
                Rep::RequireGreaterTimestamp { strictly_greater_than } => {
                    let timestamp = greater_timestamp(
                        &ops.device.time_provider,
                        GreaterTimestampOffset::Manifest,
                        strictly_greater_than,
                    );
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
                Rep::SequesterServiceUnavailable { service_id } => Err(anyhow::anyhow!("Sequester service {service_id} unavailable").into()),
                // TODO: we should send a dedicated event for this, and return an according error
                Rep::RejectedBySequesterService { service_id, reason } => Err(anyhow::anyhow!("Rejected by sequester service {service_id} ({reason:?})").into()),
                // A key rotation occurred concurrently, should poll for new certificates and retry
                Rep::BadKeyIndex { last_realm_certificate_timestamp } => {
                    let latest_known_timestamps = PerTopicLastTimestamps::new_for_realm(ops.realm_id, last_realm_certificate_timestamp);
                    ops.certificates_ops
                        .poll_server_for_new_certificates(Some(&latest_known_timestamps))
                        .await
                        .map_err(|err| match err {
                            CertifPollServerError::Stopped => WorkspaceSyncError::Stopped,
                            CertifPollServerError::Offline(e) => WorkspaceSyncError::Offline(e),
                            CertifPollServerError::InvalidCertificate(err) => WorkspaceSyncError::InvalidCertificate(err),
                            CertifPollServerError::Internal(err) => err.context("Cannot poll server for new certificates").into(),
                        })?;
                    continue;
                }

                // Unexpected errors :(
                bad_rep @ (
                    // Already checked the realm exists when we called `CertificateOps::encrypt_for_realm`
                    Rep::RealmNotFound
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

    // 2) Now upload the blocks that are missing on the server

    upload_blocks(ops, &manifest)
        .await
        .map(move |_| ReshapeAndUploadBlocksOutcome::Done(manifest))
}

enum DoNextReshapeOperationOutcome {
    Done(Arc<LocalFileManifest>),
    AlreadyReshaped,
    EntryIsBusy,
}

/// A reshape is done every time the file is closed, hence this function should be a noop
/// in most case.
/// However the reshape on file close doesn't download blocks that are missing in local,
/// hence this function has to handle this corner case.
async fn do_next_reshape_operation(
    ops: &WorkspaceOps,
    mut manifest: Arc<LocalFileManifest>,
) -> Result<DoNextReshapeOperationOutcome, WorkspaceSyncError> {
    let original_manifest = manifest.clone();
    let manifest_base = &original_manifest.base;
    let manifest_mut: &mut LocalFileManifest = Arc::make_mut(&mut manifest);

    let reshape = match super::file_operations::prepare_reshape(manifest_mut).next() {
        Some(reshape) => reshape,
        // Reshape is all finished \o/
        None => return Ok(DoNextReshapeOperationOutcome::AlreadyReshaped),
    };

    // 1) Build the chunk of data resulting of the reshape...

    let mut buf = Vec::with_capacity(reshape.destination().size() as usize);
    let mut buf_size = 0;
    let start = reshape.destination().start;
    for chunk_view in reshape.source().iter() {
        let outcome = ops
            .store
            .get_chunk_or_block(chunk_view, manifest_base)
            .await;
        match outcome {
            Ok(chunk_data) => {
                chunk_view
                    .copy_between_start_and_stop(&chunk_data, start, &mut buf, &mut buf_size)
                    .expect("write on vec cannot fail");
            }
            Err(err) => {
                return Err(match err {
                    ReadChunkOrBlockError::Offline(e) => WorkspaceSyncError::Offline(e),
                    ReadChunkOrBlockError::ServerBlockstoreUnavailable => {
                        WorkspaceSyncError::ServerBlockstoreUnavailable
                    }
                    ReadChunkOrBlockError::Stopped => WorkspaceSyncError::Stopped,
                    // TODO: manifest seems to contain invalid data (or the server is lying to us)
                    ReadChunkOrBlockError::ChunkNotFound => todo!(
                        "manifest seems to contain invalid data (or the server is lying to us)"
                    ),
                    ReadChunkOrBlockError::NoRealmAccess => WorkspaceSyncError::NotAllowed,
                    ReadChunkOrBlockError::InvalidBlockAccess(err) => {
                        WorkspaceSyncError::InvalidBlockAccess(err)
                    }
                    ReadChunkOrBlockError::InvalidKeysBundle(err) => {
                        WorkspaceSyncError::InvalidKeysBundle(err)
                    }
                    ReadChunkOrBlockError::InvalidCertificate(err) => {
                        WorkspaceSyncError::InvalidCertificate(err)
                    }
                    ReadChunkOrBlockError::Internal(err) => err.context("cannot get chunk").into(),
                });
            }
        }
    }

    // Sanity check: make sure that the buffer is fully filled
    assert!(buf.len() == reshape.destination().size() as usize);

    // 2) Save the manifest with the new chunk view in the storage

    // Lock back the entry or abort if it has changed in the meantime

    let updater = {
        let outcome = ops.store.for_update_sync_local_only(manifest_base.id).await;
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
            Err(ForUpdateSyncLocalOnlyError::WouldBlock) => {
                return Ok(DoNextReshapeOperationOutcome::EntryIsBusy)
            }
            Err(ForUpdateSyncLocalOnlyError::Stopped) => return Err(WorkspaceSyncError::Stopped),
            Err(ForUpdateSyncLocalOnlyError::Internal(err)) => {
                return Err(err.context("cannot access entry in store").into())
            }
        }
    };

    // Do the actual storage update

    let to_remove_chunk_ids = reshape.cleanup_ids();
    let new_chunk_id = reshape.destination().id;
    reshape.commit(&buf);
    updater
        .update_file_manifest_and_chunks(
            manifest.clone(),
            [(new_chunk_id, buf.as_ref())].into_iter(),
            to_remove_chunk_ids.into_iter(),
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

async fn upload_blocks(
    ops: &WorkspaceOps,
    manifest: &LocalFileManifest,
) -> Result<(), WorkspaceSyncError> {
    for (block_index, block) in manifest.blocks.iter().enumerate() {
        assert!(block.len() == 1); // Sanity check: the manifest is guaranteed to be reshaped
        let chunk_view = &block[0];
        let block_access = chunk_view.access.as_ref().expect("already reshaped");
        let block_id = block_access.id;
        assert_eq!(block_id, chunk_view.id.into()); // Sanity check

        // 1) Get back the block's data

        let maybe_data = ops
            .store
            .get_not_uploaded_chunk(chunk_view.id)
            .await
            .map_err(|err| match err {
                WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                WorkspaceStoreOperationError::Internal(err) => {
                    err.context("cannot get chunk").into()
                }
            })?;

        let data = match maybe_data {
            // Already uploaded, nothing to do
            None => continue,
            Some(data) => data,
        };

        // 2) Upload the block

        let event = EventWorkspaceOpsOutboundSyncProgress {
            realm_id: ops.realm_id,
            entry_id: manifest.base.id,
            block_index: block_index as IndexInt,
            blocks: manifest.blocks.len() as IndexInt,
            blocksize: *manifest.blocksize,
        };
        ops.event_bus.send(&event);

        loop {
            let (encrypted, key_index) = ops
                .certificates_ops
                .encrypt_for_realm(EncrytionUsage::Block(block_id), ops.realm_id(), &data)
                .await
                .map_err(|e| match e {
                    CertifEncryptForRealmError::Stopped => WorkspaceSyncError::Stopped,
                    CertifEncryptForRealmError::Offline(e) => WorkspaceSyncError::Offline(e),
                    CertifEncryptForRealmError::NotAllowed => WorkspaceSyncError::NotAllowed,
                    CertifEncryptForRealmError::NoKey => WorkspaceSyncError::NoKey,
                    CertifEncryptForRealmError::InvalidKeysBundle(err) => {
                        WorkspaceSyncError::InvalidKeysBundle(err)
                    }
                    CertifEncryptForRealmError::Internal(err) => {
                        err.context("Cannot encrypt manifest for realm").into()
                    }
                })?;

            use authenticated_cmds::latest::block_create::{Rep, Req};
            let req = Req {
                realm_id: ops.realm_id,
                key_index,
                block_id,
                block: encrypted.into(),
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok
                // Go idempotent if the block has already been uploaded, as this might
                // happen when a failure occurs before the local storage is updated
                | Rep::BlockAlreadyExists => (),
                Rep::AuthorNotAllowed => return Err(WorkspaceSyncError::NotAllowed),
                // Nothing we can do if server is not ready to store our data, retry later
                Rep::StoreUnavailable => return Err(WorkspaceSyncError::ServerBlockstoreUnavailable),
                    // A key rotation occurred concurrently, should poll for new certificates and retry
                    Rep::BadKeyIndex { last_realm_certificate_timestamp } => {
                        let latest_known_timestamps = PerTopicLastTimestamps::new_for_realm(ops.realm_id, last_realm_certificate_timestamp);
                        ops.certificates_ops
                            .poll_server_for_new_certificates(Some(&latest_known_timestamps))
                            .await
                            .map_err(|err| match err {
                                CertifPollServerError::Stopped => WorkspaceSyncError::Stopped,
                                CertifPollServerError::Offline(e) => WorkspaceSyncError::Offline(e),
                                CertifPollServerError::InvalidCertificate(err) => WorkspaceSyncError::InvalidCertificate(err),
                                CertifPollServerError::Internal(err) => err.context("Cannot poll server for new certificates").into(),
                            })?;
                        continue;
                    }

                // Unexpected errors :(
                bad_rep @ (
                    // Already checked the realm exists when we called `CertificateOps::encrypt_for_realm`
                    | Rep::RealmNotFound
                    // Don't know what to do with this status :/
                    | Rep::UnknownStatus { .. }
                ) => {
                    return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }
            break;
        }

        // 3) Mark the block as uploaded on local storage

        ops.store
            .promote_local_only_chunk_to_uploaded_block(chunk_view.id)
            .await
            .map_err(|err| match err {
                WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                WorkspaceStoreOperationError::Internal(err) => {
                    err.context("cannot promote chunk to uploaded block").into()
                }
            })?;
    }

    Ok(())
}
