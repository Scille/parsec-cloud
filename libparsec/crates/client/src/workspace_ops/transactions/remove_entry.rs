// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use super::{super::WorkspaceOps, check_write_access, resolve_path, FsOperationError};

pub(crate) enum RemoveEntryExpect {
    Anything,
    File,
    Folder,
    EmptyFolder,
}

pub(crate) async fn remove_entry(
    ops: &WorkspaceOps,
    path: &FsPath,
    expect: RemoveEntryExpect,
) -> Result<(), FsOperationError> {
    check_write_access(ops)?;

    let parent_path = path.parent();
    // Root already exists and is a folder !
    let child_name = match path.name() {
        None => return Err(FsOperationError::IsAFolder),
        Some(name) => name,
    };

    // Special case for /
    if parent_path.is_root() {
        let (updater, mut parent) = ops.data_storage.for_update_workspace_manifest().await;
        let mut_parent = Arc::make_mut(&mut parent);

        mut_parent.updated = ops.device.time_provider.now();
        mut_parent.need_sync = true;
        let child_id = match mut_parent.children.remove(child_name) {
            None => return Err(FsOperationError::EntryNotFound),
            Some(child_id) => child_id,
        };

        // Ensure there is no concurrent write operations on the child by taking the write
        // lock (even if we are not actually going to modify the child manifest !)
        let (_child_updater, child) = ops.data_storage.for_update_child_manifest(child_id).await?;
        match (expect, child) {
            (_, None) => return Err(FsOperationError::EntryNotFound),

            (RemoveEntryExpect::Anything, Some(_)) => (),

            (RemoveEntryExpect::File, Some(ArcLocalChildManifest::File(_))) => (),
            (RemoveEntryExpect::File, Some(ArcLocalChildManifest::Folder(_))) => {
                return Err(FsOperationError::IsAFolder)
            }

            (RemoveEntryExpect::Folder, Some(ArcLocalChildManifest::Folder(_))) => (),
            (RemoveEntryExpect::EmptyFolder, Some(ArcLocalChildManifest::Folder(child))) => {
                if !child.children.is_empty() {
                    return Err(FsOperationError::FolderNotEmpty);
                }
            }
            (
                RemoveEntryExpect::Folder | RemoveEntryExpect::EmptyFolder,
                Some(ArcLocalChildManifest::File(_)),
            ) => return Err(FsOperationError::NotAFolder),
        }

        updater.update_workspace_manifest(parent).await?;

        Ok(())
    } else {
        let resolution = resolve_path(ops, &parent_path).await?;

        let (updater, parent) = ops
            .data_storage
            .for_update_child_manifest(resolution.entry_id)
            .await?;
        let mut parent = match parent {
            Some(ArcLocalChildManifest::Folder(parent)) => parent,
            None | Some(ArcLocalChildManifest::File(_)) => {
                return Err(FsOperationError::EntryNotFound)
            }
        };
        let mut_parent = Arc::make_mut(&mut parent);

        mut_parent.updated = ops.device.time_provider.now();
        mut_parent.need_sync = true;
        let child_id = match mut_parent.children.remove(child_name) {
            None => return Err(FsOperationError::EntryNotFound),
            Some(child_id) => child_id,
        };

        // Ensure there is no concurrent write operations on the child by taking the write
        // lock (even if we are not actually going to modify the child manifest !)
        let (_child_updater, child) = ops.data_storage.for_update_child_manifest(child_id).await?;
        match (expect, child) {
            (_, None) => return Err(FsOperationError::EntryNotFound),

            (RemoveEntryExpect::Anything, Some(_)) => (),

            (RemoveEntryExpect::File, Some(ArcLocalChildManifest::File(_))) => (),
            (RemoveEntryExpect::File, Some(ArcLocalChildManifest::Folder(_))) => {
                return Err(FsOperationError::IsAFolder)
            }

            (RemoveEntryExpect::Folder, Some(ArcLocalChildManifest::Folder(_))) => (),
            (RemoveEntryExpect::EmptyFolder, Some(ArcLocalChildManifest::Folder(child))) => {
                if !child.children.is_empty() {
                    return Err(FsOperationError::FolderNotEmpty);
                }
            }
            (
                RemoveEntryExpect::Folder | RemoveEntryExpect::EmptyFolder,
                Some(ArcLocalChildManifest::File(_)),
            ) => return Err(FsOperationError::NotAFolder),
        }

        updater.update_as_folder_manifest(parent).await?;

        Ok(())
    }
}
