// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{
            EnsureManifestExistsWithParentError, ForUpdateFolderError, UpdateFolderManifestError,
        },
        WorkspaceOps,
    },
    EventWorkspaceOpsOutboundSyncNeeded,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceCreateFileError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Only have read access on this workspace")]
    ReadOnlyRealm,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Path doesn't point to an existing parent")]
    ParentNotFound,
    #[error("Path points to parent that is not a folder")]
    ParentNotAFolder,
    #[error("Target entry already exists")]
    EntryExists { entry_id: VlobID },
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for WorkspaceCreateFileError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub(crate) async fn create_file(
    ops: &WorkspaceOps,
    path: FsPath,
) -> Result<VlobID, WorkspaceCreateFileError> {
    if !ops
        .workspace_external_info
        .lock()
        .expect("Mutex is poisoned")
        .entry
        .role
        .can_write()
    {
        return Err(WorkspaceCreateFileError::ReadOnlyRealm);
    }

    let (parent_path, child_name) = path.into_parent();
    // Root already exists, cannot re-create it !
    let child_name = match child_name {
        None => {
            return Err(WorkspaceCreateFileError::EntryExists {
                entry_id: ops.realm_id,
            })
        }
        Some(name) => name,
    };

    let (mut parent_manifest, _, parent_updater) = ops
        .store
        .resolve_path_for_update_folder_manifest(&parent_path)
        .await
        .map_err(|err| match err {
            ForUpdateFolderError::Offline => WorkspaceCreateFileError::Offline,
            ForUpdateFolderError::Stopped => WorkspaceCreateFileError::Stopped,
            ForUpdateFolderError::EntryNotFound => WorkspaceCreateFileError::ParentNotFound,
            ForUpdateFolderError::EntryNotAFolder => WorkspaceCreateFileError::ParentNotAFolder,
            ForUpdateFolderError::NoRealmAccess => WorkspaceCreateFileError::NoRealmAccess,
            ForUpdateFolderError::InvalidKeysBundle(err) => {
                WorkspaceCreateFileError::InvalidKeysBundle(err)
            }
            ForUpdateFolderError::InvalidCertificate(err) => {
                WorkspaceCreateFileError::InvalidCertificate(err)
            }
            ForUpdateFolderError::InvalidManifest(err) => {
                WorkspaceCreateFileError::InvalidManifest(err)
            }
            ForUpdateFolderError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

    if let Some(entry_id) = parent_manifest.children.get(&child_name) {
        let entry_id = *entry_id;
        // The parent's `children` filed may contain invalid data (i.e. referencing
        // a non existing child ID, or a child which `parent` field doesn't correspond
        // to us). In this case we just pretend the entry doesn't exist.
        let is_child = ops
            .store
            .ensure_manifest_exists_with_parent(entry_id, parent_manifest.base.id)
            .await
            .map_err(|err| match err {
                EnsureManifestExistsWithParentError::Offline => WorkspaceCreateFileError::Offline,
                EnsureManifestExistsWithParentError::Stopped => WorkspaceCreateFileError::Stopped,
                EnsureManifestExistsWithParentError::NoRealmAccess => {
                    WorkspaceCreateFileError::NoRealmAccess
                }
                EnsureManifestExistsWithParentError::InvalidKeysBundle(err) => {
                    WorkspaceCreateFileError::InvalidKeysBundle(err)
                }
                EnsureManifestExistsWithParentError::InvalidCertificate(err) => {
                    WorkspaceCreateFileError::InvalidCertificate(err)
                }
                EnsureManifestExistsWithParentError::InvalidManifest(err) => {
                    WorkspaceCreateFileError::InvalidManifest(err)
                }
                EnsureManifestExistsWithParentError::Internal(err) => {
                    err.context("cannot ensure child/parent coherence").into()
                }
            })?;
        if is_child {
            return Err(WorkspaceCreateFileError::EntryExists { entry_id });
        }
    }

    let parent_id = parent_manifest.base.id;

    let now = ops.device.time_provider.now();
    let new_child = Arc::new(LocalFileManifest::new(
        ops.device.device_id.clone(),
        parent_id,
        now,
    ));
    let child_id = new_child.base.id;
    let mut_parent_manifest = Arc::make_mut(&mut parent_manifest);
    mut_parent_manifest
        .children
        .insert(child_name.to_owned(), child_id);
    // TODO: sync pattern
    mut_parent_manifest.updated = now;
    mut_parent_manifest.need_sync = true;

    parent_updater
        .update_folder_manifest(
            parent_manifest,
            Some(ArcLocalChildManifest::File(new_child)),
        )
        .await
        .map_err(|err| match err {
            UpdateFolderManifestError::Stopped => WorkspaceCreateFileError::Stopped,
            UpdateFolderManifestError::Internal(err) => {
                err.context("cannot update manifest").into()
            }
        })?;

    let event = EventWorkspaceOpsOutboundSyncNeeded {
        realm_id: ops.realm_id,
        entry_id: child_id,
    };
    ops.event_bus.send(&event);
    let event = EventWorkspaceOpsOutboundSyncNeeded {
        realm_id: ops.realm_id,
        entry_id: parent_id,
    };
    ops.event_bus.send(&event);

    Ok(child_id)
}
