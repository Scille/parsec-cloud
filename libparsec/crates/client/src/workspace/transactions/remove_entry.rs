// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{
            EnsureManifestExistsWithParentError, ForUpdateFolderError, GetManifestError,
            UpdateFolderManifestError,
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

    let (mut parent_manifest, _, parent_updater) = ops
        .store
        .resolve_path_for_update_folder(&parent_path)
        .await
        .map_err(|err| match err {
            ForUpdateFolderError::Offline => WorkspaceRemoveEntryError::Offline,
            ForUpdateFolderError::Stopped => WorkspaceRemoveEntryError::Stopped,
            ForUpdateFolderError::EntryNotFound => WorkspaceRemoveEntryError::EntryNotFound,
            ForUpdateFolderError::EntryNotAFolder => WorkspaceRemoveEntryError::EntryNotFound,
            ForUpdateFolderError::NoRealmAccess => WorkspaceRemoveEntryError::NoRealmAccess,
            ForUpdateFolderError::InvalidKeysBundle(err) => {
                WorkspaceRemoveEntryError::InvalidKeysBundle(err)
            }
            ForUpdateFolderError::InvalidCertificate(err) => {
                WorkspaceRemoveEntryError::InvalidCertificate(err)
            }
            ForUpdateFolderError::InvalidManifest(err) => {
                WorkspaceRemoveEntryError::InvalidManifest(err)
            }
            ForUpdateFolderError::Internal(err) => err.context("cannot resolve parent path").into(),
        })?;

    let parent_id = parent_manifest.base.id;
    let mut_parent_manifest = Arc::make_mut(&mut parent_manifest);

    let child_id = match mut_parent_manifest.children.get(&child_name) {
        None => return Err(WorkspaceRemoveEntryError::EntryNotFound),
        Some(&child_id) => child_id,
    };

    let child = ops
        .store
        .get_manifest(child_id)
        .await
        .map_err(|err| match err {
            GetManifestError::Offline => WorkspaceRemoveEntryError::Offline,
            GetManifestError::Stopped => WorkspaceRemoveEntryError::Stopped,
            GetManifestError::EntryNotFound => WorkspaceRemoveEntryError::EntryNotFound,
            GetManifestError::NoRealmAccess => WorkspaceRemoveEntryError::NoRealmAccess,
            GetManifestError::InvalidKeysBundle(err) => {
                WorkspaceRemoveEntryError::InvalidKeysBundle(err)
            }
            GetManifestError::InvalidCertificate(err) => {
                WorkspaceRemoveEntryError::InvalidCertificate(err)
            }
            GetManifestError::InvalidManifest(err) => {
                WorkspaceRemoveEntryError::InvalidManifest(err)
            }
            GetManifestError::Internal(err) => err.context("cannot get entry manifest").into(),
        })?;

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
            for (_, maybe_grandchild_id) in child.children.iter() {
                // The parent's `children` filed may contain invalid data (i.e. referencing
                // a non existing child ID, or a child which `parent` field doesn't correspond
                // to us). In this case we just pretend the entry doesn't exist.
                let maybe_grandchild = ops
                    .store
                    .ensure_manifest_exists_with_parent(*maybe_grandchild_id, child_id)
                    .await
                    .map_err(|err| match err {
                        EnsureManifestExistsWithParentError::Offline => {
                            WorkspaceRemoveEntryError::Offline
                        }
                        EnsureManifestExistsWithParentError::Stopped => {
                            WorkspaceRemoveEntryError::Stopped
                        }
                        EnsureManifestExistsWithParentError::NoRealmAccess => {
                            WorkspaceRemoveEntryError::NoRealmAccess
                        }
                        EnsureManifestExistsWithParentError::InvalidKeysBundle(err) => {
                            WorkspaceRemoveEntryError::InvalidKeysBundle(err)
                        }
                        EnsureManifestExistsWithParentError::InvalidCertificate(err) => {
                            WorkspaceRemoveEntryError::InvalidCertificate(err)
                        }
                        EnsureManifestExistsWithParentError::InvalidManifest(err) => {
                            WorkspaceRemoveEntryError::InvalidManifest(err)
                        }
                        EnsureManifestExistsWithParentError::Internal(err) => {
                            err.context("cannot ensure child/parent coherence").into()
                        }
                    })?;

                if maybe_grandchild.is_some() {
                    return Err(WorkspaceRemoveEntryError::EntryIsNonEmptyFolder);
                }
            }
        }
        (
            RemoveEntryExpect::Folder | RemoveEntryExpect::EmptyFolder,
            ArcLocalChildManifest::File(_),
        ) => return Err(WorkspaceRemoveEntryError::EntryIsFile),
    }

    mut_parent_manifest.evolve_children_and_mark_updated(
        HashMap::from([(child_name, None)]),
        &ops.config.prevent_sync_pattern,
        ops.device.time_provider.now(),
    );

    parent_updater
        .update_folder_manifest(parent_manifest, None)
        .await
        .map_err(|err| match err {
            UpdateFolderManifestError::Stopped => WorkspaceRemoveEntryError::Stopped,
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
