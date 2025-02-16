// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/// Inbound sync is the operation of fetching remote changes for an entry and applying
/// them in local.
/// Most of the time everything goes smoothly:
/// - the local hasn't been modified so the remote changes are applied as-is.
/// - the local has some changes but they can be merged with the remote (folders can
///   always be merged, file that only its parent has been modified can be merged).
/// - the local is currently being modified (i.e. we cannot lock it for update without
///   waiting). In this case we just return a special status to signify the sync should
///   be retried later (it is better to sync an entry that have settled anyway).
///
/// However if the merge fails (most common case is because a file's content has been
/// concurrently modified), we end up in a sync conflict.
///
/// The idea behind sync conflict is to integrate the remote change, while trying as much
/// as possible to retain the local change that created the conflict in the first place.
///
/// On top of that, the sync conflict involves the parent which should also be locked
/// for update (as sync conflict often involve creating a new file containing the local
/// conflicting changes).
/// Note special care must be taken to avoid deadlocks (due to the dependency between
/// already locked child and to-be-locked parent), hence if the parent cannot be locked
/// without waiting we stop there and signify the caller the sync should be retried later.
///
/// Now let's list the the possible sync conflict situations:
///
/// Initial situation: we try to sync an entry, and get `local` and `remote` manifests in conflict.
/// Then:
/// - A1: `local` is a file, `local`'s parent exists, is a folder and refers to `local` as its child.
/// - A2: Same as A1, but `local` is a folder.
/// - B1: `local`'s update lock is already held by a concurrent operation.
/// - B2: `local`'s parent update lock is already held by a concurrent operation.
/// - C1: `local` has been concurrently removed.
/// - C2: `local` has been concurrently updated and is no longer in conflict.
/// - C3: `local` has been concurrently updated and is still in conflict.
/// - D1: `local`'s parent doesn't exist.
/// - D2: `local`'s parent is not a folder.
/// - D3: `local`'s parent exists (and is a folder) but doesn't refer to `local` as its child.
///
/// The strategy to apply for each situation is:
/// - A1: This is the by-the-book case of a sync conflict. A new file is created in
///       `local`'s parent to store the conflicting local data, and `local` is
///       overwritten by `remote`.
/// - A2: A folder cannot be copied without modifying its children (otherwise the
///       children's parent field would still refer to the original folder). On top
///       of that a folder is not supposed to end up in conflict unless it type has change
///       (i.e. `local` is a `folder`, but remote is a `file`), which is something the clients
///        are never supposed to do.
///        Hence we consider this as an unlikely corner-case and just overwrite `local` by
///        `remote` (losing in the process any newly created child that hasn't been synchronized).
/// - B1: The entry being locked for update is a indicator it might have subsequent changes,
///       hence now is not a good time to try to synchronize it. Instead we return a special
///       status to tell the caller to retry later.
/// - B2: In theory we could wait to acquire the parent update lock (folders are locked for a
///       short time, unlike files which are locked for the whole time the file is opened).
///       But may lead to unexpected behavior with the entry-change-type corner-case, so
///       instead we follow a "sync operations never wait for update lock" strategy and
///       just tell the caller to retry later.
/// - C1/C2/C3: In all those case we can pretend we tried to access a bit earlier and hit
///       the update lock being held, hence falling into the B1 case.
/// - D1: This is a corner case, as a child is not accessible unless its parent is (otherwise
///       how the child would have been modified in the first place ?).
///       So we just settle the conflict by overwriting `local` with `remote`.
/// - D2: This is also a corner case (as an entry is not supposed to have its type changed).
///       Again, we take the easy route and just overwrite `local` with `remote`.
/// - D3: This case is not possible (we are going to explain why), but seems plausible when
//        a file child is moved to another parent while also having its content changed o_O.
///       The trick is to consider how local changes are made:
///       - Removing a locally modified file won't lead to this: this is because the
///         remove operation also clears non-synced changes of the removed
///         item (i.e. it doesn't just update the parent manifest's children).
///       - Moving a file in local is an atomic operation (i.e. old parent, new parent and
///         child are all modified at once) so the file always has a valid parent.
///       From this we can deduce that we should always be able to get the name of the
///       file in conflict from its parent \o/
use std::sync::Arc;

use libparsec_client_connection::{protocol::authenticated_cmds, ConnectionError};
use libparsec_types::prelude::*;

use super::super::WorkspaceOps;
use crate::{
    certif::{
        CertifValidateManifestError, CertificateOps, InvalidCertificateError,
        InvalidKeysBundleError, InvalidManifestError,
    },
    workspace::{
        merge::{MergeLocalFileManifestOutcome, MergeLocalFolderManifestOutcome},
        store::{
            ForUpdateSyncLocalOnlyError, IntoSyncConflictUpdaterError, SyncUpdater, WorkspaceStore,
            WorkspaceStoreOperationError,
        },
    },
    EventWorkspaceOpsInboundSyncDone, InvalidBlockAccessError,
};

pub type WorkspaceGetNeedInboundSyncEntriesError = WorkspaceStoreOperationError;

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceSyncError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Block access is temporary unavailable on the server")]
    ServerBlockstoreUnavailable,
    #[error("Client has stopped")]
    Stopped,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("The realm doesn't have any key yet")]
    NoKey,
    #[error("The workspace's realm hasn't been created yet on server")]
    NoRealm,
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    InvalidBlockAccess(#[from] Box<InvalidBlockAccessError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    // No `InvalidManifest` here, this is because we self-repair in case of invalid
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

pub async fn refresh_realm_checkpoint(ops: &WorkspaceOps) -> Result<(), WorkspaceSyncError> {
    let last_checkpoint = ops
        .store
        .get_realm_checkpoint()
        .await
        .map_err(|err| match err {
            WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
            WorkspaceStoreOperationError::Internal(err) => {
                err.context("cannot get local realm checkpoint").into()
            }
        })?;

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
            } => (changes, current_checkpoint),
            // TODO: error handling !
            Rep::AuthorNotAllowed => return Err(WorkspaceSyncError::NotAllowed),
            Rep::RealmNotFound { .. } => return Err(WorkspaceSyncError::NoRealm),
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }
    };

    if last_checkpoint != current_checkpoint {
        ops.store
            .update_realm_checkpoint(current_checkpoint, &changes)
            .await
            .map_err(|err| match err {
                WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                WorkspaceStoreOperationError::Internal(err) => {
                    err.context("cannot update realm checkpoint").into()
                }
            })?;
    }

    Ok(())
}

pub async fn get_need_inbound_sync(
    ops: &WorkspaceOps,
    limit: u32,
) -> Result<Vec<VlobID>, WorkspaceGetNeedInboundSyncEntriesError> {
    ops.store.get_inbound_need_sync_entries(limit).await
}

#[derive(Debug)]
pub enum InboundSyncOutcome {
    Updated,
    NoChange,
    /// The entry is already locked, this is typically because it is being modified.
    /// Hence now is not the right time to sync it given our incoming changes will
    /// be overwritten by the ongoing modification. Instead we should just retry later.
    EntryIsBusy,
}

/// Download and merge remote changes from the server.
///
/// If the client contains local changes, an outbound sync is still needed to
/// have the client fully synchronized with the server.
///
/// If `InboundSyncOutcome::Updated` is returned (the sync operation has led to local
/// changes), an `EventWorkspaceOpsInboundSyncDone` event is sent accordingly.
pub async fn inbound_sync(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<InboundSyncOutcome, WorkspaceSyncError> {
    // Start by a cheap optional check: the sync is going to end up with `EntryInBusy`
    // if the entry is locked at the time we want to merge the remote manifest with
    // the local one. In such case, the caller is expected to retry later.
    // But what if the entry is currently locked an plans to stay it this way for a long
    // time ? Hence this check that will fail early instead of fetching the remote manifest.
    if ops.store.is_entry_locked(entry_id).await {
        return Ok(InboundSyncOutcome::EntryIsBusy);
    }

    // Inbound sync on a opened file is a bad idea:
    // - It may change the content of the file, while keeping the opened file unaware.
    // - While never done in practice, it may also change manifest type into a folder...
    //
    // For this reason, the file used to be locked (with `WorkspaceStore::for_update_file`)
    // for as long as it was opened. However we had to change this in a stopgap solution
    // as it was causing deadlocks when re-parenting an opened file :'(
    //
    // As a result, we must now manually check if the file is opened here.
    macro_rules! abort_if_file_opened {
        () => {{
            let guard = ops.opened_files.lock().expect("Mutex is poisoned");
            if guard.opened_files.contains_key(&entry_id) {
                return Ok(InboundSyncOutcome::EntryIsBusy);
            }
        }};
    }
    abort_if_file_opened!();

    // Retrieve remote

    let remote_manifest = if entry_id == ops.realm_id {
        match fetch_remote_manifest_with_self_heal::<RootManifest>(ops, entry_id).await? {
            FetchWithSelfHealOutcome::LastVersion(RootManifest(manifest)) => {
                ChildManifest::Folder(manifest)
            }
            FetchWithSelfHealOutcome::SelfHeal {
                last_version,
                last_valid_manifest: RootManifest(mut last_valid_manifest),
            } => {
                // If we use the manifest as-is, we won't be able to do outbound sync
                // given the server will always complain a more recent manifest exists
                // (i.e. the one that is corrupted !).
                // So we tweak this old manifest into pretending it is the corrupted one.
                last_valid_manifest.version = last_version;
                ChildManifest::Folder(last_valid_manifest)
            }
        }
    } else {
        match fetch_remote_manifest_with_self_heal::<ChildManifest>(ops, entry_id).await? {
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
        }
    };

    // Now merge the remote with the current local manifest

    let (updater, local_manifest) = {
        let outcome = ops.store.for_update_sync_local_only(entry_id).await;
        match outcome {
            // Nothing to sync
            Ok((updater, local_manifest)) => {
                abort_if_file_opened!();
                (updater, local_manifest)
            }
            Err(ForUpdateSyncLocalOnlyError::WouldBlock) => {
                return Ok(InboundSyncOutcome::EntryIsBusy)
            }
            Err(ForUpdateSyncLocalOnlyError::Stopped) => return Err(WorkspaceSyncError::Stopped),
            Err(ForUpdateSyncLocalOnlyError::Internal(err)) => {
                return Err(err.context("cannot lock entry for update").into())
            }
        }
    };

    let outcome =
        merge_manifest_and_update_store(ops, updater, local_manifest, remote_manifest).await?;

    match outcome {
        InboundSyncOutcomeWithParentID::Updated { parent_id } => {
            let event = EventWorkspaceOpsInboundSyncDone {
                realm_id: ops.realm_id,
                entry_id,
                parent_id,
            };
            ops.event_bus.send(&event);
            Ok(InboundSyncOutcome::Updated)
        }
        InboundSyncOutcomeWithParentID::NoChange => Ok(InboundSyncOutcome::NoChange),
        InboundSyncOutcomeWithParentID::EntryIsBusy => Ok(InboundSyncOutcome::EntryIsBusy),
    }
}

#[derive(Debug)]
enum InboundSyncOutcomeWithParentID {
    Updated { parent_id: VlobID },
    NoChange,
    EntryIsBusy,
}

async fn merge_manifest_and_update_store(
    ops: &WorkspaceOps,
    updater: SyncUpdater<'_>,
    local_manifest: Option<ArcLocalChildManifest>,
    remote_manifest: ChildManifest,
) -> Result<InboundSyncOutcomeWithParentID, WorkspaceSyncError> {
    // Note merge may end up with nothing new, typically if the remote version is
    // already the one local is based on
    match (local_manifest, remote_manifest) {
        // File added remotely
        (None, ChildManifest::File(remote_manifest)) => {
            let local_manifest = Arc::new(LocalFileManifest::from_remote(remote_manifest));
            let parent_id = local_manifest.parent;
            updater
                .update_manifest(ArcLocalChildManifest::File(local_manifest))
                .await
                .map_err(|err| match err {
                    WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                    WorkspaceStoreOperationError::Internal(err) => {
                        err.context("cannot update manifest").into()
                    }
                })?;
            Ok(InboundSyncOutcomeWithParentID::Updated { parent_id })
        }

        // Folder added remotely
        (None, ChildManifest::Folder(remote_manifest)) => {
            let local_manifest = Arc::new(LocalFolderManifest::from_remote(
                remote_manifest,
                &ops.config.prevent_sync_pattern,
            ));
            let parent_id = local_manifest.parent;
            updater
                .update_manifest(ArcLocalChildManifest::Folder(local_manifest))
                .await
                .map_err(|err| match err {
                    WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                    WorkspaceStoreOperationError::Internal(err) => {
                        err.context("cannot update manifest").into()
                    }
                })?;
            Ok(InboundSyncOutcomeWithParentID::Updated { parent_id })
        }

        // Folder present in both remote and local, need to merge them
        (
            Some(ArcLocalChildManifest::Folder(local_manifest)),
            ChildManifest::Folder(remote_manifest),
        ) => {
            // Note merge may end up with nothing to sync, typically if the remote version is
            // already the one local is based on
            let merge_outcome = super::super::merge::merge_local_folder_manifest(
                ops.device.device_id,
                ops.device.now(),
                &ops.config.prevent_sync_pattern,
                &local_manifest,
                remote_manifest,
            );
            match merge_outcome {
                MergeLocalFolderManifestOutcome::NoChange => {
                    Ok(InboundSyncOutcomeWithParentID::NoChange)
                }
                MergeLocalFolderManifestOutcome::Merged(merged_manifest) => {
                    let parent_id = merged_manifest.parent;
                    updater
                        .update_manifest(ArcLocalChildManifest::Folder(merged_manifest))
                        .await
                        .map_err(|err| match err {
                            WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                            WorkspaceStoreOperationError::Internal(err) => {
                                err.context("cannot update manifest").into()
                            }
                        })?;
                    Ok(InboundSyncOutcomeWithParentID::Updated { parent_id })
                }
            }
        }

        // File present in both remote and local, need to merge them
        (
            Some(ArcLocalChildManifest::File(local_manifest)),
            ChildManifest::File(remote_manifest),
        ) => {
            // Note merge may end up with nothing to sync, typically if the remote version is
            // already the one local is based on
            let merge_outcome = super::super::merge::merge_local_file_manifest(
                ops.device.device_id,
                ops.device.now(),
                &local_manifest,
                remote_manifest,
            );
            match merge_outcome {
                MergeLocalFileManifestOutcome::NoChange => {
                    Ok(InboundSyncOutcomeWithParentID::NoChange)
                }
                MergeLocalFileManifestOutcome::Merged(merged_manifest) => {
                    let parent_id = merged_manifest.parent;
                    updater
                        .update_manifest(ArcLocalChildManifest::File(Arc::new(merged_manifest)))
                        .await
                        .map_err(|err| match err {
                            WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                            WorkspaceStoreOperationError::Internal(err) => {
                                err.context("cannot update manifest").into()
                            }
                        })?;
                    Ok(InboundSyncOutcomeWithParentID::Updated { parent_id })
                }
                MergeLocalFileManifestOutcome::Conflict(remote_manifest) => {
                    handle_conflict_and_update_store(
                        ops,
                        updater,
                        ArcLocalChildManifest::File(local_manifest),
                        ChildManifest::File(remote_manifest),
                    )
                    .await
                }
            }
        }

        // The entry has changed it type, this is not expected :/
        // Solve this by considering this is a file conflict
        (
            Some(local_manifest @ ArcLocalChildManifest::Folder(_)),
            remote_manifest @ ChildManifest::File(_),
        ) => handle_conflict_and_update_store(ops, updater, local_manifest, remote_manifest).await,

        (
            Some(local_manifest @ ArcLocalChildManifest::File(_)),
            remote_manifest @ ChildManifest::Folder(_),
        ) => handle_conflict_and_update_store(ops, updater, local_manifest, remote_manifest).await,
    }
}

async fn handle_conflict_and_update_store(
    ops: &WorkspaceOps,
    sync_updater: SyncUpdater<'_>,
    local_child_manifest: ArcLocalChildManifest,
    remote_child_manifest: ChildManifest,
) -> Result<InboundSyncOutcomeWithParentID, WorkspaceSyncError> {
    // 1) No mater what, remote changes will be used as new version for the child manifest

    let (parent_id, merged_child_manifest) = match remote_child_manifest {
        ChildManifest::File(remote_manifest) => (
            remote_manifest.parent,
            ArcLocalChildManifest::File(Arc::new(LocalFileManifest::from_remote(remote_manifest))),
        ),
        ChildManifest::Folder(remote_manifest) => (
            remote_manifest.parent,
            ArcLocalChildManifest::Folder(Arc::new(LocalFolderManifest::from_remote(
                remote_manifest,
                &ops.config.prevent_sync_pattern,
            ))),
        ),
    };

    // 2) Local changes causing the conflict are transfered to a new manifest

    let (original_child_id, conflicting_new_child_manifest) = match local_child_manifest {
        // Most likely case since file content cannot be merged
        ArcLocalChildManifest::File(child_manifest) => {
            let LocalFileManifest {
                base: _,
                parent,
                need_sync,
                updated,
                size,
                blocksize,
                blocks,
            } = child_manifest.as_ref();

            let mut conflicting =
                LocalFileManifest::new(ops.device.device_id, *parent, ops.device.now());
            conflicting.need_sync = *need_sync;
            conflicting.updated = *updated;
            conflicting.size = *size;
            conflicting.blocksize = *blocksize;
            blocks.clone_into(&mut conflicting.blocks);

            (child_manifest.base.id, Arc::new(conflicting))
        }

        // The conflicting child is a folder, this is unexpected given merging folders
        // should be always fine !
        //
        // This can still occur however if the type of the entry has changed between local
        // and remote (cannot merge a folder with a file !).
        //
        // On top of that, we cannot create a new folder manifest with the content of the
        // conflicting one (like we do with the file manifest) given the child of the
        // conflicting manifest won't refer this new folder as their parent.
        //
        // Finally, while possible, changing the type of an entry is something the client
        // is never supposed to do (there is no benefit in doing this anyway); so if we
        // are in this case it's likely a bug or the act of a malicious client :(
        //
        // For those reasons, we keep it simple here and just accept to lose the conflicting
        // folder manifest (hence losing any new file created in it).
        ArcLocalChildManifest::Folder(_) => {
            sync_updater
                .update_manifest(merged_child_manifest)
                .await
                .map_err(|err| match err {
                    WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                    WorkspaceStoreOperationError::Internal(err) => {
                        err.context("cannot update child manifest in store").into()
                    }
                })?;
            return Ok(InboundSyncOutcomeWithParentID::Updated { parent_id });
        }
    };

    // 3) Conflict involves the parent, hence the updater evolves to lock it

    let (sync_conflict_updater, mut parent_manifest) = match sync_updater
        .into_sync_conflict_updater()
        .await
    {
        Ok(ok) => ok,
        Err((sync_updater, err)) => {
            return match err {
                // Cannot lock the parent for the moment, tell to caller to retry later
                IntoSyncConflictUpdaterError::WouldBlock => {
                    Ok(InboundSyncOutcomeWithParentID::EntryIsBusy)
                }

                // The parent is not valid, this is not supposed to occur in theory given
                // how could we have locally modified the child in the first place then ?
                // A possible explanation is an inbound sync has changed the parent type,
                // but this is not something a non buggy/malicious client is supposed to do
                // (see comment above about handling folder as local conflict) !
                IntoSyncConflictUpdaterError::ParentNotFound
                | IntoSyncConflictUpdaterError::ParentIsNotAFolder => {
                    sync_updater
                        .update_manifest(merged_child_manifest)
                        .await
                        .map_err(|err| match err {
                            WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                            WorkspaceStoreOperationError::Internal(err) => {
                                err.context("cannot update child manifest in store").into()
                            }
                        })?;
                    Ok(InboundSyncOutcomeWithParentID::Updated { parent_id })
                }

                // Actual errors
                IntoSyncConflictUpdaterError::Offline(e) => Err(WorkspaceSyncError::Offline(e)),
                IntoSyncConflictUpdaterError::Stopped => Err(WorkspaceSyncError::Stopped),
                IntoSyncConflictUpdaterError::NoRealmAccess => Err(WorkspaceSyncError::NotAllowed),
                IntoSyncConflictUpdaterError::InvalidKeysBundle(err) => {
                    Err(WorkspaceSyncError::InvalidKeysBundle(err))
                }
                IntoSyncConflictUpdaterError::InvalidCertificate(err) => {
                    Err(WorkspaceSyncError::InvalidCertificate(err))
                }
                IntoSyncConflictUpdaterError::InvalidManifest(err) => {
                    Err(WorkspaceSyncError::InvalidManifest(err))
                }
                IntoSyncConflictUpdaterError::Internal(err) => {
                    Err(err.context("cannot lock parent for update").into())
                }
            };
        }
    };

    // 4) Add the conflicting new child in the parent

    let parent_manifest_mut = Arc::make_mut(&mut parent_manifest);
    insert_conflicting_new_child_in_parent(
        original_child_id,
        conflicting_new_child_manifest.base.id,
        parent_manifest_mut,
    );

    // 5) Finally update the store

    sync_conflict_updater
        .update_manifests(
            merged_child_manifest,
            parent_manifest,
            ArcLocalChildManifest::File(conflicting_new_child_manifest),
        )
        .await
        .map_err(|err| match err {
            WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
            WorkspaceStoreOperationError::Internal(err) => {
                err.context("cannot update manifests in store").into()
            }
        })?;

    Ok(InboundSyncOutcomeWithParentID::Updated { parent_id })
}

fn insert_conflicting_new_child_in_parent(
    original_child_id: VlobID,
    conflicting_new_child_id: VlobID,
    parent_manifest: &mut LocalFolderManifest,
) {
    let (child_prefix, child_suffix) = match parent_manifest
        .children
        .iter()
        .find(|(_, id)| **id == original_child_id)
    {
        Some((child_name, _)) => child_name.prefix_and_suffix(),
        // The parent no longer refers the child, we can still get the name if
        // this is a local-only change.
        None => match parent_manifest
            .base
            .children
            .iter()
            .find(|(_, id)| **id == original_child_id)
        {
            Some((child_name, _)) => child_name.prefix_and_suffix(),
            // No luck ! There is no way to get the name of the conflicting child
            // (we could in theory search into older version of the parent manifest,
            // but this too cumbersome). Just use a default name then.
            None => ("lost+found", None),
        },
    };

    let mut attempt = 2;
    macro_rules! build_next_conflict_name {
        () => {{
            let mut prefix = child_prefix;
            let mut suffix = child_suffix;
            loop {
                let name = match suffix {
                    None => format!("{} (Parsec sync conflict {})", prefix, attempt),
                    Some(suffix) => {
                        format!("{} (Parsec sync conflict {}).{}", prefix, attempt, suffix)
                    }
                };
                match name.parse::<EntryName>() {
                    Ok(name) => break name,
                    // Entry name too long
                    Err(EntryNameError::NameTooLong) => {
                        // Simply strip 10 characters from the first name then try again
                        if prefix.len() > 10 {
                            prefix = &prefix[..prefix.len() - 10];
                        } else {
                            // Very rare case where the suffixes are very long,
                            // we have no choice but to strip it...
                            let suffix_str = suffix.expect("must be present");
                            match suffix_str.split_once('.') {
                                // Remove one item among the suffix (e.g. `tar.gz` -> `gz`)
                                Some((_, kept_suffix)) => {
                                    suffix = Some(kept_suffix);
                                }
                                // Suffix is composed of a single very long item, so remove
                                // it entirely
                                None => {
                                    suffix = None;
                                }
                            }
                        }
                    }
                    // Not possible given name is only composed of valid characters
                    Err(EntryNameError::InvalidName) => unreachable!(),
                }
            }
        }};
    }

    let conflict_name = loop {
        let conflict_name = build_next_conflict_name!();
        if !parent_manifest.children.contains_key(&conflict_name) {
            break conflict_name;
        }
        attempt += 1;
    };

    parent_manifest
        .children
        .insert(conflict_name, conflicting_new_child_id);
}

pub trait RemoteManifest: Sized {
    type LocalManifest: Sized;

    fn extract_base_from_local(local: Self::LocalManifest) -> Self;

    async fn get_from_storage(
        store: &WorkspaceStore,
        entry_id: VlobID,
    ) -> Result<(VersionInt, Self::LocalManifest), anyhow::Error>;

    #[allow(clippy::too_many_arguments)]
    async fn validate(
        certif_ops: &CertificateOps,
        needed_realm_certificate_timestamp: DateTime,
        needed_common_certificate_timestamp: DateTime,
        realm_id: VlobID,
        key_index: IndexInt,
        vlob_id: VlobID,
        author: DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<Self, CertifValidateManifestError>;
}

/// Root and non root folder manifest have different validation rules, hence this
/// shim to handle the root one.
struct RootManifest(FolderManifest);

impl RemoteManifest for RootManifest {
    type LocalManifest = Arc<LocalFolderManifest>;

    fn extract_base_from_local(local: Self::LocalManifest) -> Self {
        Self(local.base.clone())
    }

    // TODO: handle entry not found
    async fn get_from_storage(
        store: &WorkspaceStore,
        _entry_id: VlobID,
    ) -> Result<(VersionInt, Self::LocalManifest), anyhow::Error> {
        let manifest = store.get_root_manifest();
        Ok((manifest.base.version, manifest))
    }

    async fn validate(
        certif_ops: &CertificateOps,
        needed_realm_certificate_timestamp: DateTime,
        needed_common_certificate_timestamp: DateTime,
        realm_id: VlobID,
        key_index: IndexInt,
        vlob_id: VlobID,
        author: DeviceID,
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
            .map(RootManifest)
    }
}

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
        store: &WorkspaceStore,
        entry_id: VlobID,
    ) -> Result<(VersionInt, Self::LocalManifest), anyhow::Error> {
        let manifest = store.get_manifest(entry_id).await?;
        let base_version = match &manifest {
            Self::LocalManifest::File(m) => m.base.version,
            Self::LocalManifest::Folder(m) => m.base.version,
        };
        Ok((base_version, manifest))
    }

    async fn validate(
        certif_ops: &CertificateOps,
        needed_realm_certificate_timestamp: DateTime,
        needed_common_certificate_timestamp: DateTime,
        realm_id: VlobID,
        key_index: IndexInt,
        vlob_id: VlobID,
        author: DeviceID,
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
                InvalidManifestError::CannotDecrypt { version, .. } => version,
                InvalidManifestError::CleartextCorrupted { version, .. } => version,
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
    let (local_base_version, local_manifest) = M::get_from_storage(&ops.store, entry_id).await?;

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
        author,
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
            CertifValidateManifestError::Offline(e) => Err(WorkspaceSyncError::Offline(e)),
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
        author,
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
            CertifValidateManifestError::Offline(e) => Err(WorkspaceSyncError::Offline(e)),
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
