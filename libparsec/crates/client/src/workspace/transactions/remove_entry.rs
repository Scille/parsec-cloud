// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{
            FolderishManifestAndUpdater, GetEntryError, GetFolderishEntryError,
            UpdateFolderManifestError, UpdateWorkspaceManifestError,
        },
        WorkspaceOps,
    },
    EventWorkspaceOpsOutboundSyncNeeded,
};

pub(crate) enum RemoveEntryExpect {
    Anything,
    File,
    Folder,
    EmptyFolder,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceRemoveEntryError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Only have read access on this workspace")]
    ReadOnlyRealm,
    #[error("Root path cannot be removed")]
    CannotRemoveRoot,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Path points to a file")]
    EntryIsFile,
    #[error("Path points to a folder")]
    EntryIsFolder,
    #[error("Path points to a non-empty folder")]
    EntryIsNonEmptyFolder,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for WorkspaceRemoveEntryError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub(crate) async fn remove_entry(
    ops: &WorkspaceOps,
    path: FsPath,
    expect: RemoveEntryExpect,
) -> Result<(), WorkspaceRemoveEntryError> {
    if !ops
        .workspace_external_info
        .lock()
        .expect("Mutex is poisoned")
        .entry
        .role
        .can_write()
    {
        return Err(WorkspaceRemoveEntryError::ReadOnlyRealm);
    }

    let (parent_path, child_name) = path.into_parent();
    let child_name = match child_name {
        None => {
            return Err(WorkspaceRemoveEntryError::CannotRemoveRoot);
        }
        Some(name) => name,
    };

    let resolution = ops
        .store
        .resolve_path_for_update_folderish_manifest(&parent_path)
        .await
        .map_err(|err| match err {
            GetFolderishEntryError::Offline => WorkspaceRemoveEntryError::Offline,
            GetFolderishEntryError::Stopped => WorkspaceRemoveEntryError::Stopped,
            GetFolderishEntryError::EntryNotFound => WorkspaceRemoveEntryError::EntryNotFound,
            GetFolderishEntryError::EntryIsFile => WorkspaceRemoveEntryError::EntryNotFound,
            GetFolderishEntryError::NoRealmAccess => WorkspaceRemoveEntryError::NoRealmAccess,
            GetFolderishEntryError::InvalidKeysBundle(err) => {
                WorkspaceRemoveEntryError::InvalidKeysBundle(err)
            }
            GetFolderishEntryError::InvalidCertificate(err) => {
                WorkspaceRemoveEntryError::InvalidCertificate(err)
            }
            GetFolderishEntryError::InvalidManifest(err) => {
                WorkspaceRemoveEntryError::InvalidManifest(err)
            }
            GetFolderishEntryError::Internal(err) => {
                err.context("cannot resolve parent path").into()
            }
        })?;

    let check_child_expect = |child| {
        match (expect, child) {
            (RemoveEntryExpect::Anything, _) => (),

            (RemoveEntryExpect::File, ArcLocalChildManifest::File(_)) => (),
            (RemoveEntryExpect::File, ArcLocalChildManifest::Folder(_)) => {
                return Err(WorkspaceRemoveEntryError::EntryIsFolder)
            }

            // A word about removing non-empty folder:
            //
            // Here we only remove the target directory and call it a day. However on
            // most of other programs (e.g. Rust's `std::fs::delete_folder_all`,
            // Python's `shutil.rmtree`) this is implemented as a recursive operation
            // deleting all children first then parent until the actual directory to delete
            // is reached.
            //
            // This is because the other programs at application are talking to a
            // filesystem through the OS while we are directly talking to our filesystem
            // here. The OS doesn't (cannot ?) provide a "remove dir even if it not empty"
            // syscall given it cannot be an atomic operation on most filesystem (e.g. on
            // ext3 the filenames are hashed and put into hash tree leading to the inode,
            // hence it is not possible to find back the children of a given path).
            //
            // On the other hand, Parsec's filesystem architecture makes trivial to
            // remove a non-empty folder, hence here we are ;-)
            (RemoveEntryExpect::Folder, ArcLocalChildManifest::Folder(_)) => (),
            (RemoveEntryExpect::EmptyFolder, ArcLocalChildManifest::Folder(child)) => {
                if !child.children.is_empty() {
                    return Err(WorkspaceRemoveEntryError::EntryIsNonEmptyFolder);
                }
            }
            (
                RemoveEntryExpect::Folder | RemoveEntryExpect::EmptyFolder,
                ArcLocalChildManifest::File(_),
            ) => return Err(WorkspaceRemoveEntryError::EntryIsFile),
        }
        Ok(())
    };

    let parent_id = match resolution {
        FolderishManifestAndUpdater::Folder {
            manifest: mut parent,
            updater,
            ..
        } => {
            let parent_id = parent.base.id;
            let mut_parent = Arc::make_mut(&mut parent);

            let child_id = match mut_parent.children.remove(&child_name) {
                None => return Err(WorkspaceRemoveEntryError::EntryNotFound),
                Some(child_id) => child_id,
            };

            let child = ops
                .store
                .get_child_manifest(child_id)
                .await
                .map_err(|err| match err {
                    GetEntryError::Offline => WorkspaceRemoveEntryError::Offline,
                    GetEntryError::Stopped => WorkspaceRemoveEntryError::Stopped,
                    GetEntryError::EntryNotFound => WorkspaceRemoveEntryError::EntryNotFound,
                    GetEntryError::NoRealmAccess => WorkspaceRemoveEntryError::NoRealmAccess,
                    GetEntryError::InvalidKeysBundle(err) => {
                        WorkspaceRemoveEntryError::InvalidKeysBundle(err)
                    }
                    GetEntryError::InvalidCertificate(err) => {
                        WorkspaceRemoveEntryError::InvalidCertificate(err)
                    }
                    GetEntryError::InvalidManifest(err) => {
                        WorkspaceRemoveEntryError::InvalidManifest(err)
                    }
                    GetEntryError::Internal(err) => err.context("cannot get entry manifest").into(),
                })?;

            check_child_expect(child)?;

            mut_parent.updated = ops.device.time_provider.now();
            mut_parent.need_sync = true;

            updater
                .update_folder_manifest(parent, None)
                .await
                .map_err(|err| match err {
                    UpdateFolderManifestError::Stopped => WorkspaceRemoveEntryError::Stopped,
                    UpdateFolderManifestError::Internal(err) => {
                        err.context("cannot update manifest").into()
                    }
                })?;

            parent_id
        }

        FolderishManifestAndUpdater::Root {
            manifest: mut parent,
            updater,
        } => {
            let parent_id = parent.base.id;
            let mut_parent = Arc::make_mut(&mut parent);

            let child_id = match mut_parent.children.remove(&child_name) {
                None => return Err(WorkspaceRemoveEntryError::EntryNotFound),
                Some(child_id) => child_id,
            };

            let child = ops
                .store
                .get_child_manifest(child_id)
                .await
                .map_err(|err| match err {
                    GetEntryError::Offline => WorkspaceRemoveEntryError::Offline,
                    GetEntryError::Stopped => WorkspaceRemoveEntryError::Stopped,
                    GetEntryError::EntryNotFound => WorkspaceRemoveEntryError::EntryNotFound,
                    GetEntryError::NoRealmAccess => WorkspaceRemoveEntryError::NoRealmAccess,
                    GetEntryError::InvalidKeysBundle(err) => {
                        WorkspaceRemoveEntryError::InvalidKeysBundle(err)
                    }
                    GetEntryError::InvalidCertificate(err) => {
                        WorkspaceRemoveEntryError::InvalidCertificate(err)
                    }
                    GetEntryError::InvalidManifest(err) => {
                        WorkspaceRemoveEntryError::InvalidManifest(err)
                    }
                    GetEntryError::Internal(err) => err.context("cannot get entry manifest").into(),
                })?;

            check_child_expect(child)?;

            mut_parent.updated = ops.device.time_provider.now();
            mut_parent.need_sync = true;

            updater
                .update_workspace_manifest(parent, None)
                .await
                .map_err(|err| match err {
                    UpdateWorkspaceManifestError::Stopped => WorkspaceRemoveEntryError::Stopped,
                    UpdateWorkspaceManifestError::Internal(err) => {
                        err.context("cannot update manifest").into()
                    }
                })?;

            parent_id
        }
    };

    let event = EventWorkspaceOpsOutboundSyncNeeded {
        realm_id: ops.realm_id,
        entry_id: parent_id,
    };
    ops.event_bus.send(&event);

    Ok(())
}
