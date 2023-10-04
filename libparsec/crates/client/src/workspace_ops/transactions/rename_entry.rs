// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use super::{super::WorkspaceOps, check_write_access, resolve_path, FsOperationError};

pub(crate) async fn rename_entry(
    ops: &WorkspaceOps,
    path: &FsPath,
    new_name: EntryName,
    overwrite: bool,
) -> Result<(), FsOperationError> {
    check_write_access(ops)?;

    let parent_path = path.parent();
    // Cannot rename root !
    let old_name = match path.name() {
        None => return Err(FsOperationError::CannotRenameRoot),
        Some(name) => name,
    };

    // Do nothing if source and destination are the same !
    if *old_name == new_name {
        return Ok(());
    }

    // Special case for /
    if parent_path.is_root() {
        let (updater, mut parent) = ops.data_storage.for_update_workspace_manifest().await;
        let mut_parent = Arc::make_mut(&mut parent);

        mut_parent.updated = ops.device.time_provider.now();
        mut_parent.need_sync = true;

        let child_id = match mut_parent.children.remove(old_name) {
            None => return Err(FsOperationError::EntryNotFound),
            Some(child_id) => child_id,
        };

        match mut_parent.children.entry(new_name) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                if !overwrite {
                    return Err(FsOperationError::EntryExists);
                }
                entry.insert(child_id);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(child_id);
            }
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

        let child_id = match mut_parent.children.remove(old_name) {
            None => return Err(FsOperationError::EntryNotFound),
            Some(child_id) => child_id,
        };

        match mut_parent.children.entry(new_name) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                if !overwrite {
                    return Err(FsOperationError::EntryExists);
                }
                entry.insert(child_id);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(child_id);
            }
        }

        updater.update_as_folder_manifest(parent).await?;

        Ok(())
    }
}
