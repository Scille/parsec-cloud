// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{ForUpdateFolderError, UpdateFolderManifestError},
        WorkspaceOps,
    },
    EventWorkspaceOpsOutboundSyncNeeded,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceRenameEntryError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Root path cannot be renamed")]
    CannotRenameRoot,
    #[error("Only have read access on this workspace")]
    ReadOnlyRealm,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Destination already exists")]
    DestinationExists { entry_id: VlobID },
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for WorkspaceRenameEntryError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub(crate) async fn rename_entry(
    ops: &WorkspaceOps,
    path: FsPath,
    new_name: EntryName,
    overwrite: bool,
) -> Result<(), WorkspaceRenameEntryError> {
    // TODO: Renaming a placeholder into a non-placeholder entry of similar type
    //       should copy the content of the placeholder into the non-placeholder
    //       in order to keep the entry ID.
    //       This is because rename is often used as a way to update an entry
    //       in an atomic way.

    if !ops
        .workspace_external_info
        .lock()
        .expect("Mutex is poisoned")
        .entry
        .role
        .can_write()
    {
        return Err(WorkspaceRenameEntryError::ReadOnlyRealm);
    }

    let (parent_path, old_name) = path.into_parent();
    // Cannot rename root !
    let old_name = match old_name {
        None => return Err(WorkspaceRenameEntryError::CannotRenameRoot),
        Some(name) => name,
    };

    // Do nothing if source and destination are the same !
    if old_name == new_name {
        return Ok(());
    }

    let (mut parent_manifest, _, parent_updater) = ops
        .store
        .resolve_path_for_update_folder_manifest(&parent_path)
        .await
        .map_err(|err| match err {
            ForUpdateFolderError::Offline => WorkspaceRenameEntryError::Offline,
            ForUpdateFolderError::Stopped => WorkspaceRenameEntryError::Stopped,
            ForUpdateFolderError::EntryNotFound => WorkspaceRenameEntryError::EntryNotFound,
            ForUpdateFolderError::EntryNotAFolder => WorkspaceRenameEntryError::EntryNotFound,
            ForUpdateFolderError::NoRealmAccess => WorkspaceRenameEntryError::NoRealmAccess,
            ForUpdateFolderError::InvalidKeysBundle(err) => {
                WorkspaceRenameEntryError::InvalidKeysBundle(err)
            }
            ForUpdateFolderError::InvalidCertificate(err) => {
                WorkspaceRenameEntryError::InvalidCertificate(err)
            }
            ForUpdateFolderError::InvalidManifest(err) => {
                WorkspaceRenameEntryError::InvalidManifest(err)
            }
            ForUpdateFolderError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

    let mut_parent_manifest = Arc::make_mut(&mut parent_manifest);

    let child_id = match mut_parent_manifest.children.remove(&old_name) {
        None => return Err(WorkspaceRenameEntryError::EntryNotFound),
        Some(child_id) => child_id,
    };

    match mut_parent_manifest.children.entry(new_name) {
        std::collections::hash_map::Entry::Occupied(mut entry) => {
            if !overwrite {
                return Err(WorkspaceRenameEntryError::DestinationExists {
                    entry_id: *entry.get(),
                });
            }
            entry.insert(child_id);
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            entry.insert(child_id);
        }
    }

    mut_parent_manifest.updated = ops.device.time_provider.now();
    mut_parent_manifest.need_sync = true;

    let parent_id = parent_manifest.base.id;
    parent_updater
        .update_folder_manifest(parent_manifest, None)
        .await
        .map_err(|err| match err {
            UpdateFolderManifestError::Stopped => WorkspaceRenameEntryError::Stopped,
            UpdateFolderManifestError::Internal(err) => {
                err.context("cannot update manifest").into()
            }
        })?;

    let event = EventWorkspaceOpsOutboundSyncNeeded {
        realm_id: ops.realm_id,
        entry_id: parent_id,
    };
    ops.event_bus.send(&event);

    Ok(())
}
