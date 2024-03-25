// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{protocol::authenticated_cmds, ConnectionError};
use libparsec_types::prelude::*;

use super::super::WorkspaceOps;
use crate::{
    certif::{
        CertifOps, CertifValidateManifestError, InvalidCertificateError, InvalidKeysBundleError,
        InvalidManifestError,
    },
    workspace::store::{
        ChildUpdater, ForUpdateChildLocalOnlyError, ForUpdateFolderError, WorkspaceStore,
        WorkspaceStoreOperationError,
    },
    InvalidBlockAccessError,
};

pub type WorkspaceGetNeedInboundSyncEntriesError = WorkspaceStoreOperationError;

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

impl From<ConnectionError> for WorkspaceSyncError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
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
    update_store_with_remote_root(ops, remote)
        .await
        .map_err(|err| err.into())

    // TODO: we used to send a SHARING_UPDATED event here, however it would simpler
    // to send such event from the CertifOps (i.e. when a realm role certificate
    // is added). The downside of this approach is we don't have the guarantee that
    // WorkspaceOps have processed the changes (e.g. GUI receive an event about an new
    // workspace shared with us, but got an error when trying to get the workspace
    // from WorkspaceOps...)
}

async fn update_store_with_remote_root(
    ops: &WorkspaceOps,
    remote: WorkspaceManifest,
) -> anyhow::Result<InboundSyncOutcome> {
    let (updater, local) = ops.store.for_update_root().await;
    // Note merge may end up with nothing new, typically if the remote version is
    // already the one local is based on
    if let Some(merged) = super::super::merge::merge_local_workspace_manifests(
        &ops.device.device_id,
        ops.device.now(),
        &local,
        remote,
    ) {
        updater
            .update_workspace_manifest(Arc::new(merged), None)
            .await?;
        Ok(InboundSyncOutcome::Updated)
    } else {
        Ok(InboundSyncOutcome::NoChange)
    }
}

async fn inbound_sync_child(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<InboundSyncOutcome, WorkspaceSyncError> {
    // Start by a cheap optional check: the sync is going to end up with `EntryInBusy`
    // if the entry is locked at the time we want to merge the remote manifest with
    // the local one. In such case, the caller is expected to retry later.
    // But what if the entry stays locked for a long time ? Hence this check that
    // will fail early instead of fetching the remote manifest.
    if ops.store.is_child_entry_locked(entry_id).await {
        return Ok(InboundSyncOutcome::EntryIsBusy);
    }

    // Retrieve remote
    let remote = match fetch_remote_manifest_with_self_heal(ops, entry_id).await? {
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

    let outcome = ops.store.for_update_child_local_only(entry_id).await;
    let (updater, local) = match outcome {
        Ok((updater, manifest)) => (updater, manifest),
        Err(ForUpdateChildLocalOnlyError::WouldBlock) => {
            return Ok(InboundSyncOutcome::EntryIsBusy)
        }
        Err(ForUpdateChildLocalOnlyError::Stopped) => return Err(WorkspaceSyncError::Stopped),
        Err(ForUpdateChildLocalOnlyError::Internal(err)) => {
            return Err(err.context("cannot lock for update").into())
        }
    };

    update_store_with_remote_child(ops, updater, local, remote).await
}

async fn update_store_with_remote_child(
    ops: &WorkspaceOps,
    updater: ChildUpdater<'_>,
    local: Option<ArcLocalChildManifest>,
    remote: ChildManifest,
) -> Result<InboundSyncOutcome, WorkspaceSyncError> {
    // Note merge may end up with nothing new, typically if the remote version is
    // already the one local is based on
    match (local, remote) {
        // File added remotely
        (None, ChildManifest::File(remote)) => {
            let local_manifest = Arc::new(LocalFileManifest::from_remote(remote));
            updater
                .update_manifest(ArcLocalChildManifest::File(local_manifest))
                .await
                .map_err(|err| match err {
                    WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                    WorkspaceStoreOperationError::Internal(err) => {
                        err.context("cannot update manifest").into()
                    }
                })?;
            Ok(InboundSyncOutcome::Updated)
        }

        // Folder added remotely
        (None, ChildManifest::Folder(remote)) => {
            let local_manifest = Arc::new(LocalFolderManifest::from_remote(remote, None));
            updater
                .update_manifest(ArcLocalChildManifest::Folder(local_manifest))
                .await
                .map_err(|err| match err {
                    WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                    WorkspaceStoreOperationError::Internal(err) => {
                        err.context("cannot update manifest").into()
                    }
                })?;
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
                    .update_manifest(ArcLocalChildManifest::Folder(Arc::new(merged_manifest)))
                    .await
                    .map_err(|err| match err {
                        WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                        WorkspaceStoreOperationError::Internal(err) => {
                            err.context("cannot update manifest").into()
                        }
                    })?;
                Ok(InboundSyncOutcome::Updated)
            } else {
                Ok(InboundSyncOutcome::NoChange)
            }
        }

        // TODO: finish this !

        // File present in both remote and local, either they are the same or we have a file conflict !
        (Some(ArcLocalChildManifest::File(local)), ChildManifest::File(remote)) => {
            // Ignore outdated remote
            if local.base.version >= remote.version {
                return Ok(InboundSyncOutcome::NoChange);
            }

            // Just use the new remote if there is no local changes
            if !local.need_sync {
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
                return Ok(InboundSyncOutcome::Updated);
            }

            // Both remote and local have changed, we have a conflict !

            // Retrieve the parent

            // TODO:
            // - handle the case the parent has changed between local and remote

            let conflicted = {
                let mut conflicted = LocalFileManifest::new(
                    ops.device.device_id.clone(),
                    local.base.parent,
                    ops.device.now(),
                );
                let LocalFileManifest {
                    base: _,
                    need_sync,
                    updated,
                    size,
                    blocksize,
                    blocks,
                } = local.as_ref();
                conflicted.need_sync = *need_sync;
                conflicted.updated = *updated;
                conflicted.size = *size;
                conflicted.blocksize = *blocksize;
                conflicted.blocks = blocks.to_owned();
                Arc::new(conflicted)
            };

            if local.base.parent == ops.realm_id {
                let (parent_updater, mut parent_manifest) = ops.store.for_update_root().await;
                let parent_manifest_mut = Arc::make_mut(&mut parent_manifest);

                parent_manifest_mut.need_sync = true;
                let child_name = match parent_manifest_mut
                    .children
                    .iter()
                    .find(|(_, id)| **id == local.base.id)
                {
                    // The conflicted file doesn't exist anymore
                    None => {
                        let local_from_remote = Arc::new(LocalFileManifest::from_remote(remote));
                        updater
                            .update_manifest(ArcLocalChildManifest::File(local_from_remote))
                            .await
                            .map_err(|err| match err {
                                WorkspaceStoreOperationError::Stopped => {
                                    WorkspaceSyncError::Stopped
                                }
                                WorkspaceStoreOperationError::Internal(err) => {
                                    err.context("cannot update manifest").into()
                                }
                            })?;
                        return Ok(InboundSyncOutcome::Updated);
                    }
                    Some((child_name, _)) => child_name.to_owned(),
                };

                let (child_base_name, child_extension) = child_name.base_and_extension();
                let mut attempt = 2;
                macro_rules! build_next_conflicted_name {
                    () => {{
                        let mut base_name = child_base_name;
                        let mut extension = child_extension;
                        loop {
                            let name = match extension {
                                None => format!("{} ({})", base_name, attempt),
                                Some(extension) => {
                                    format!("{} ({}).{}", base_name, attempt, extension)
                                }
                            };
                            match name.parse::<EntryName>() {
                                Ok(name) => break name,
                                // Entry name too long
                                Err(EntryNameError::NameTooLong) => {
                                    // Simply strip 10 characters from the first name then try again
                                    if base_name.len() > 10 {
                                        base_name = &base_name[..base_name.len() - 10];
                                    } else {
                                        // Very rare case where the extensions are very long,
                                        // we have no choice but to strip it...
                                        let extension_str = extension.expect("must be present");
                                        extension =
                                            Some(&extension_str[..extension_str.len() - 10]);
                                    }
                                }
                                // Not possible given name is only composed of valid characters
                                Err(EntryNameError::InvalidName) => unreachable!(),
                            }
                        }
                    }};
                }
                let mut conflicted_name = build_next_conflicted_name!();
                loop {
                    match parent_manifest_mut.children.entry(conflicted_name) {
                        std::collections::hash_map::Entry::Occupied(_) => {
                            attempt += 1;
                            conflicted_name = build_next_conflicted_name!();
                        }
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(conflicted.base.id);
                            break;
                        }
                    }
                }

                // TODO: should have a dedicated method in the storage to be able to atomically
                // update the parent and the child in case of conflict

                parent_updater
                    .update_workspace_manifest(
                        parent_manifest,
                        Some(ArcLocalChildManifest::File(conflicted)),
                    )
                    .await
                    .map_err(|err| match err {
                        WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                        WorkspaceStoreOperationError::Internal(err) => {
                            err.context("cannot update parent manifest").into()
                        }
                    })?;

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

                Ok(InboundSyncOutcome::Updated)
            } else {
                let outcome = ops.store.for_update_folder(local.base.parent).await;
                let (parent_updater, mut parent_manifest) = match outcome {
                    Ok((parent_updater, parent_manifest)) => (parent_updater, parent_manifest),
                    Err(err) => match err {
                        // The conflicted file doesn't exist anymore
                        ForUpdateFolderError::EntryNotFound
                        | ForUpdateFolderError::EntryNotAFolder => {
                            let local_from_remote =
                                Arc::new(LocalFileManifest::from_remote(remote));
                            updater
                                .update_manifest(ArcLocalChildManifest::File(local_from_remote))
                                .await
                                .map_err(|err| match err {
                                    WorkspaceStoreOperationError::Stopped => {
                                        WorkspaceSyncError::Stopped
                                    }
                                    WorkspaceStoreOperationError::Internal(err) => {
                                        err.context("cannot update manifest").into()
                                    }
                                })?;
                            return Ok(InboundSyncOutcome::Updated);
                        }
                        ForUpdateFolderError::Offline => return Err(WorkspaceSyncError::Offline),
                        ForUpdateFolderError::Stopped => return Err(WorkspaceSyncError::Stopped),
                        ForUpdateFolderError::NoRealmAccess => {
                            return Err(WorkspaceSyncError::NotAllowed)
                        }
                        ForUpdateFolderError::InvalidKeysBundle(err) => {
                            return Err(WorkspaceSyncError::InvalidKeysBundle(err))
                        }
                        ForUpdateFolderError::InvalidCertificate(err) => {
                            return Err(WorkspaceSyncError::InvalidCertificate(err))
                        }
                        ForUpdateFolderError::InvalidManifest(err) => {
                            return Err(WorkspaceSyncError::InvalidManifest(err))
                        }
                        ForUpdateFolderError::Internal(err) => {
                            return Err(err.context("cannot retrieve parent manifest").into())
                        }
                    },
                };
                let parent_manifest_mut = Arc::make_mut(&mut parent_manifest);

                parent_manifest_mut.need_sync = true;
                let child_name = match parent_manifest_mut
                    .children
                    .iter()
                    .find(|(_, id)| **id == local.base.id)
                {
                    // The conflicted file doesn't exist anymore
                    None => {
                        let local_from_remote = Arc::new(LocalFileManifest::from_remote(remote));
                        updater
                            .update_manifest(ArcLocalChildManifest::File(local_from_remote))
                            .await
                            .map_err(|err| match err {
                                WorkspaceStoreOperationError::Stopped => {
                                    WorkspaceSyncError::Stopped
                                }
                                WorkspaceStoreOperationError::Internal(err) => {
                                    err.context("cannot update manifest").into()
                                }
                            })?;
                        return Ok(InboundSyncOutcome::Updated);
                    }
                    Some((child_name, _)) => child_name.to_owned(),
                };

                let (child_base_name, child_extension) = child_name.base_and_extension();
                let mut attempt = 2;
                macro_rules! build_next_conflicted_name {
                    () => {{
                        let mut base_name = child_base_name;
                        let mut extension = child_extension;
                        loop {
                            let name = match extension {
                                None => format!("{} ({})", base_name, attempt),
                                Some(extension) => {
                                    format!("{} ({}).{}", base_name, attempt, extension)
                                }
                            };
                            match name.parse::<EntryName>() {
                                Ok(name) => break name,
                                // Entry name too long
                                Err(EntryNameError::NameTooLong) => {
                                    // Simply strip 10 characters from the first name then try again
                                    if base_name.len() > 10 {
                                        base_name = &base_name[..base_name.len() - 10];
                                    } else {
                                        // Very rare case where the extensions are very long,
                                        // we have no choice but to strip it...
                                        let extension_str = extension.expect("must be present");
                                        extension =
                                            Some(&extension_str[..extension_str.len() - 10]);
                                    }
                                }
                                // Not possible given name is only composed of valid characters
                                Err(EntryNameError::InvalidName) => unreachable!(),
                            }
                        }
                    }};
                }
                let mut conflicted_name = build_next_conflicted_name!();
                loop {
                    match parent_manifest_mut.children.entry(conflicted_name) {
                        std::collections::hash_map::Entry::Occupied(_) => {
                            attempt += 1;
                            conflicted_name = build_next_conflicted_name!();
                        }
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(conflicted.base.id);
                            break;
                        }
                    }
                }

                // TODO: should have a dedicated method in the storage to be able to atomically
                // update the parent and the child in case of conflict

                parent_updater
                    .update_folder_manifest(
                        parent_manifest,
                        Some(ArcLocalChildManifest::File(conflicted)),
                    )
                    .await
                    .map_err(|err| match err {
                        WorkspaceStoreOperationError::Stopped => WorkspaceSyncError::Stopped,
                        WorkspaceStoreOperationError::Internal(err) => {
                            err.context("cannot update parent manifest").into()
                        }
                    })?;

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

                Ok(InboundSyncOutcome::Updated)
            }
        }

        // The entry has changed it type, this is not expected :/
        // Solve this by considering this is a file conflict
        (Some(ArcLocalChildManifest::Folder(_)), ChildManifest::File(_)) => todo!(),
        (Some(ArcLocalChildManifest::File(_)), ChildManifest::Folder(_)) => todo!(),
    }
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

impl RemoteManifest for WorkspaceManifest {
    type LocalManifest = Arc<LocalWorkspaceManifest>;

    fn extract_base_from_local(local: Self::LocalManifest) -> Self {
        local.base.clone()
    }

    // TODO: handle entry not found
    async fn get_from_storage(
        store: &WorkspaceStore,
        _entry_id: VlobID,
    ) -> Result<(VersionInt, Self::LocalManifest), anyhow::Error> {
        let manifest = store.get_workspace_manifest();
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
        let manifest = store.get_child_manifest(entry_id).await?;
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
