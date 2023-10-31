// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use super::{super::WorkspaceOps, check_write_access, resolve_path, FsOperationError};
use crate::EventWorkspaceOpsOutboundSyncNeeded;

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
    let parent_id = if parent_path.is_root() {
        let (updater, mut parent) = ops.data_storage.for_update_workspace_manifest().await;
        let parent_id = parent.base.id;
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

        parent_id
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
        let parent_id = parent.base.id;
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

        parent_id
    };

    let event = EventWorkspaceOpsOutboundSyncNeeded {
        realm_id: ops.realm_id,
        entry_id: parent_id,
    };
    ops.event_bus.send(&event);

    Ok(())
}
