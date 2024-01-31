// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_platform_storage::workspace::GetChildManifestError as StorageGetChildManifestError;
use libparsec_types::prelude::*;

use super::{
    super::{
        fetch::{fetch_remote_child_manifest, FetchRemoteManifestError},
        WorkspaceOps,
    },
    FsOperationError,
};

pub(super) fn check_write_access(ops: &WorkspaceOps) -> Result<(), FsOperationError> {
    let config = ops.workspace_entry.lock().expect("Mutex is poisoned");
    if config.role.can_write() {
        Ok(())
    } else {
        Err(FsOperationError::ReadOnlyRealm)
    }
}

pub(super) async fn get_child_manifest(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<ArcLocalChildManifest, FsOperationError> {
    match ops.data_storage.get_child_manifest(entry_id).await {
        Ok(manifest) => Ok(manifest),
        Err(StorageGetChildManifestError::Internal(err)) => Err(err.into()),
        Err(StorageGetChildManifestError::NotFound) => {
            let remote_manifest =
                fetch_remote_child_manifest(ops, entry_id)
                    .await
                    .map_err(|error| match error {
                        FetchRemoteManifestError::Stopped => FsOperationError::Stopped,
                        FetchRemoteManifestError::Offline => FsOperationError::Offline,
                        FetchRemoteManifestError::VlobNotFound => {
                            // This is unexpected: we got an entry ID from a parent folder/workspace
                            // manifest, but this ID points to nothing according to the server :/
                            //
                            // That could means two things:
                            // - the server is lying to us
                            // - the client that have uploaded the parent folder/workspace manifest
                            //   was buggy and include the ID of a not-yet-synchronized entry
                            //
                            // In theory it would be good to do a self-healing here (e.g. remove
                            // the entry from the parent), but this is cumbersome and only possible
                            // if the user has write access.
                            // So instead we just pretend the entry doesn't exist.

                            // TODO: add warning log !
                            FsOperationError::EntryNotFound
                        }
                        // The realm doesn't exist on server side, hence we are it creator and
                        // it data only live on our local storage, which we have already checked.
                        FetchRemoteManifestError::RealmNotFound => FsOperationError::EntryNotFound,
                        FetchRemoteManifestError::NotAllowed => FsOperationError::NoRealmAccess,
                        FetchRemoteManifestError::InvalidKeysBundle(err) => {
                            FsOperationError::InvalidKeysBundle(err)
                        }
                        FetchRemoteManifestError::InvalidCertificate(err) => {
                            FsOperationError::InvalidCertificate(err)
                        }
                        FetchRemoteManifestError::InvalidManifest(err) => {
                            FsOperationError::InvalidManifest(err)
                        }
                        FetchRemoteManifestError::Internal(err) => err.into(),
                    })?;

            // Must save our manifest in the storage
            let (updater, expect_missing_manifest) =
                ops.data_storage.for_update_child_manifest(entry_id).await?;
            match expect_missing_manifest {
                // Plot twist: a concurrent operation has inserted the manifest in the storage !
                // TODO: we could be trying to update the existing data with the brand new one
                // however this would most likely do nothing (as the concurrent version must based
                // on very recent data)
                Some(local_manifest) => Ok(local_manifest),

                // As expected the storage didn't contain the manifest, it's up to us to store it then !
                None => {
                    let local_manifest = match remote_manifest {
                        ChildManifest::File(remote_manifest) => {
                            let manifest =
                                Arc::new(LocalFileManifest::from_remote(remote_manifest));
                            updater
                                .update_as_file_manifest(manifest.clone(), false, [].into_iter())
                                .await?;
                            ArcLocalChildManifest::File(manifest)
                        }
                        ChildManifest::Folder(remote_manifest) => {
                            let manifest =
                                Arc::new(LocalFolderManifest::from_remote(remote_manifest, None));
                            updater.update_as_folder_manifest(manifest.clone()).await?;
                            ArcLocalChildManifest::Folder(manifest)
                        }
                    };
                    Ok(local_manifest)
                }
            }
        }
    }
}

pub(super) struct FsPathResolution {
    pub entry_id: VlobID,
    /// The confinement point corresponds to the entry id of the folderish manifest
    /// (i.e. file or workspace manifest) that contains a child with a confined name
    /// in the corresponding path.
    ///
    /// If the entry is not confined, the confinement point is `None`.
    pub confinement_point: Option<VlobID>,
}

pub(super) async fn resolve_path(
    ops: &WorkspaceOps,
    path: &FsPath,
) -> Result<FsPathResolution, FsOperationError> {
    enum Parent {
        Root,
        /// The parent is itself the child of someone else
        Child(FsPathResolution),
    }
    let mut parent = Parent::Root;

    for child_name in path.parts() {
        let resolution = match parent {
            Parent::Root => {
                let manifest = ops.data_storage.get_workspace_manifest();

                let child_entry_id = manifest
                    .children
                    .get(child_name)
                    .ok_or(FsOperationError::EntryNotFound)?;

                let confinement_point = manifest
                    .local_confinement_points
                    .contains(child_entry_id)
                    .then_some(ops.realm_id);

                FsPathResolution {
                    entry_id: *child_entry_id,
                    confinement_point,
                }
            }

            Parent::Child(parent) => {
                let manifest = get_child_manifest(ops, parent.entry_id).await?;

                let (children, local_confinement_points) = match &manifest {
                    ArcLocalChildManifest::File(_) => {
                        // Cannot continue to resolve the path !
                        return Err(FsOperationError::EntryNotFound);
                    }
                    ArcLocalChildManifest::Folder(manifest) => {
                        (&manifest.children, &manifest.local_confinement_points)
                    }
                };

                let child_entry_id = children
                    .get(child_name)
                    .ok_or(FsOperationError::EntryNotFound)?;

                // Top-most confinement point shadows child ones if any
                let confinement_point = match parent.confinement_point {
                    confinement_point @ Some(_) => confinement_point,
                    None => local_confinement_points
                        .contains(child_entry_id)
                        .then_some(parent.entry_id),
                };

                FsPathResolution {
                    entry_id: *child_entry_id,
                    confinement_point,
                }
            }
        };

        parent = Parent::Child(resolution);
    }

    Ok(match parent {
        Parent::Root => FsPathResolution {
            entry_id: ops.realm_id,
            confinement_point: None,
        },
        Parent::Child(resolution) => resolution,
    })
}
