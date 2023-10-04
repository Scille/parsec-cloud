// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use super::{super::WorkspaceOps, check_write_access, resolve_path, FsOperationError};

pub(crate) async fn create_folder(
    ops: &WorkspaceOps,
    path: &FsPath,
) -> Result<VlobID, FsOperationError> {
    check_write_access(ops)?;

    let parent_path = path.parent();
    // Root already exists, cannot re-create it !
    let child_name = match path.name() {
        None => return Err(FsOperationError::EntryExists),
        Some(name) => name,
    };

    // Special case for /
    if parent_path.is_root() {
        let (updater, mut parent) = ops.data_storage.for_update_workspace_manifest().await;

        let now = ops.device.time_provider.now();
        let new_child = Arc::new(LocalFolderManifest::new(
            ops.device.device_id.clone(),
            parent.base.id,
            now,
        ));
        let child_id = new_child.base.id;
        let mut_parent = Arc::make_mut(&mut parent);
        mut_parent.children.insert(child_name.to_owned(), child_id);
        // TODO: sync pattern
        mut_parent.updated = now;

        updater.new_child_folder_manifest(new_child).await?;
        updater.update_workspace_manifest(parent).await?;

        Ok(child_id)
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

        let now = ops.device.time_provider.now();
        let new_child = Arc::new(LocalFolderManifest::new(
            ops.device.device_id.clone(),
            parent.base.id,
            now,
        ));
        let child_id = new_child.base.id;
        let mut_parent = Arc::make_mut(&mut parent);
        mut_parent.children.insert(child_name.to_owned(), child_id);
        // TODO: sync pattern
        mut_parent.updated = now;

        updater.new_child_folder_manifest(new_child).await?;
        updater.update_as_folder_manifest(parent).await?;

        Ok(child_id)
    }
}
