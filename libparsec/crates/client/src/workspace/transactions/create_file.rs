// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{
            FolderishManifestAndUpdater, GetFolderishEntryError, UpdateFolderManifestError,
            UpdateWorkspaceManifestError,
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
    #[error("Path points to a file as parent")]
    ParentIsFile,
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

    let resolution = ops
        .store
        .resolve_path_for_update_folderish_manifest(&parent_path)
        .await
        .map_err(|err| match err {
            GetFolderishEntryError::Offline => WorkspaceCreateFileError::Offline,
            GetFolderishEntryError::Stopped => WorkspaceCreateFileError::Stopped,
            GetFolderishEntryError::EntryNotFound => WorkspaceCreateFileError::ParentNotFound,
            GetFolderishEntryError::EntryIsFile => WorkspaceCreateFileError::ParentIsFile,
            GetFolderishEntryError::NoRealmAccess => WorkspaceCreateFileError::NoRealmAccess,
            GetFolderishEntryError::InvalidKeysBundle(err) => {
                WorkspaceCreateFileError::InvalidKeysBundle(err)
            }
            GetFolderishEntryError::InvalidCertificate(err) => {
                WorkspaceCreateFileError::InvalidCertificate(err)
            }
            GetFolderishEntryError::InvalidManifest(err) => {
                WorkspaceCreateFileError::InvalidManifest(err)
            }
            GetFolderishEntryError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

    let (parent_id, child_id) = match resolution {
        FolderishManifestAndUpdater::Folder {
            manifest: mut parent,
            updater,
            ..
        } => {
            if let Some(entry) = parent.children.get(&child_name) {
                return Err(WorkspaceCreateFileError::EntryExists { entry_id: *entry });
            }
            let parent_id = parent.base.id;

            let now = ops.device.time_provider.now();
            let new_child = Arc::new(LocalFileManifest::new(
                ops.device.device_id.clone(),
                parent_id,
                now,
            ));
            let child_id = new_child.base.id;
            let mut_parent = Arc::make_mut(&mut parent);
            mut_parent.children.insert(child_name.to_owned(), child_id);
            // TODO: sync pattern
            mut_parent.updated = now;
            mut_parent.need_sync = true;

            updater
                .update_folder_manifest(parent, Some(ArcLocalChildManifest::File(new_child)))
                .await
                .map_err(|err| match err {
                    UpdateFolderManifestError::Stopped => WorkspaceCreateFileError::Stopped,
                    UpdateFolderManifestError::Internal(err) => {
                        err.context("cannot update manifest").into()
                    }
                })?;

            (parent_id, child_id)
        }

        FolderishManifestAndUpdater::Root {
            manifest: mut parent,
            updater,
        } => {
            if let Some(entry) = parent.children.get(&child_name) {
                return Err(WorkspaceCreateFileError::EntryExists { entry_id: *entry });
            }
            let parent_id = parent.base.id;

            let now = ops.device.time_provider.now();
            let new_child = Arc::new(LocalFileManifest::new(
                ops.device.device_id.clone(),
                parent_id,
                now,
            ));
            let child_id = new_child.base.id;
            let mut_parent = Arc::make_mut(&mut parent);
            mut_parent.children.insert(child_name.to_owned(), child_id);
            // TODO: sync pattern
            mut_parent.updated = now;
            mut_parent.need_sync = true;

            updater
                .update_workspace_manifest(parent, Some(ArcLocalChildManifest::File(new_child)))
                .await
                .map_err(|err| match err {
                    UpdateWorkspaceManifestError::Stopped => WorkspaceCreateFileError::Stopped,
                    UpdateWorkspaceManifestError::Internal(err) => {
                        err.context("cannot update manifest").into()
                    }
                })?;

            (parent_id, child_id)
        }
    };

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
