// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_platform_storage2::workspace::workspace_storage_non_speculative_init;
use libparsec_types::prelude::*;

use super::{UserOps, UserOpsError};

pub(super) async fn workspace_create(
    ops: &UserOps,
    name: EntryName,
) -> Result<EntryID, UserOpsError> {
    let guard = ops.update_user_manifest_lock.lock().await;

    let timestamp = ops.device.time_provider.now();
    let workspace_entry = WorkspaceEntry::generate(name, timestamp);
    let workspace_id = workspace_entry.id;
    let mut user_manifest = ops.storage.get_user_manifest();

    // `Arc::make_mut` clones user manifest before we modify it
    Arc::make_mut(&mut user_manifest)
        .evolve_workspaces_and_mark_updated(workspace_entry, timestamp);

    // Given *we* are the creator of the workspace, our placeholder is
    // the only non-speculative one.
    //
    // Note the save order is important given there is no atomicity
    // between saving the non-speculative workspace manifest placeholder
    // and the save of the user manifest containing the workspace entry.
    // Indeed, if we would save the user manifest first and a crash
    // occurred before saving the placeholder, we would end-up in the same
    // situation as if the workspace has been created by someone else
    // (i.e. a workspace entry but no local data about this workspace)
    // so we would fallback to a local speculative workspace manifest.
    // However a speculative manifest means the workspace have been
    // created by somebody else, and hence we shouldn't try to create
    // it corresponding realm in the backend !
    workspace_storage_non_speculative_init(&ops.data_base_dir, &ops.device, workspace_id)
        .await
        .map_err(UserOpsError::Internal)?;
    ops.storage
        .set_user_manifest(user_manifest)
        .await
        .map_err(UserOpsError::Internal)?;
    // TODO: handle events
    // ops.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=ops.user_manifest_id)
    // ops.event_bus.send(CoreEvent.FS_WORKSPACE_CREATED, new_entry=workspace_entry)

    drop(guard);
    Ok(workspace_id)
}

pub(super) async fn workspace_rename(
    ops: &UserOps,
    workspace_id: &EntryID,
    new_name: EntryName,
) -> Result<(), UserOpsError> {
    let guard = ops.update_user_manifest_lock.lock().await;

    let mut user_manifest = ops.storage.get_user_manifest();

    if let Some(workspace_entry) = user_manifest.get_workspace_entry(workspace_id) {
        let mut updated_workspace_entry = workspace_entry.to_owned();
        updated_workspace_entry.name = new_name;
        let timestamp = ops.device.time_provider.now();

        // `Arc::make_mut` clones user manifest before we modify it
        Arc::make_mut(&mut user_manifest)
            .evolve_workspaces_and_mark_updated(updated_workspace_entry, timestamp);

        ops.storage
            .set_user_manifest(user_manifest)
            .await
            .map_err(UserOpsError::Internal)?;
        // TODO: handle events
        // ops.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=ops.user_manifest_id)
    } else {
        return Err(UserOpsError::UnknownWorkspace(workspace_id.to_owned()));
    }

    drop(guard);
    Ok(())
}
