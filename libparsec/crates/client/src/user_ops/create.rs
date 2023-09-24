// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_platform_storage::workspace::workspace_storage_non_speculative_init;
use libparsec_types::prelude::*;

use super::UserOps;

pub(super) async fn create_workspace(
    ops: &UserOps,
    name: EntryName,
) -> Result<VlobID, anyhow::Error> {
    let (updater, mut user_manifest) = ops.storage.for_update().await;

    let timestamp = ops.device.time_provider.now();
    let workspace_entry = WorkspaceEntry::generate(name, timestamp);
    let workspace_id = workspace_entry.id;

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
    workspace_storage_non_speculative_init(&ops.config.data_base_dir, &ops.device, workspace_id)
        .await?;
    updater.set_user_manifest(user_manifest).await?;
    // TODO: handle events
    // ops.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=ops.user_realm_id)
    // ops.event_bus.send(CoreEvent.FS_WORKSPACE_CREATED, new_entry=workspace_entry)

    Ok(workspace_id)
}

#[derive(Debug, thiserror::Error)]
pub enum RenameWorkspaceError {
    #[error("Unknown workspace `{0}`")]
    UnknownWorkspace(VlobID),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn rename_workspace(
    ops: &UserOps,
    realm_id: VlobID,
    new_name: EntryName,
) -> Result<(), RenameWorkspaceError> {
    let (updater, mut user_manifest) = ops.storage.for_update().await;

    if let Some(workspace_entry) = user_manifest.get_workspace_entry(realm_id) {
        let mut updated_workspace_entry = workspace_entry.to_owned();
        updated_workspace_entry.name = new_name;
        let timestamp = ops.device.time_provider.now();

        // `Arc::make_mut` clones user manifest before we modify it
        Arc::make_mut(&mut user_manifest)
            .evolve_workspaces_and_mark_updated(updated_workspace_entry, timestamp);

        updater
            .set_user_manifest(user_manifest)
            .await
            .map_err(RenameWorkspaceError::Internal)?;
        // TODO: handle events
        // ops.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=ops.user_realm_id)
    } else {
        return Err(RenameWorkspaceError::UnknownWorkspace(realm_id.to_owned()));
    }

    Ok(())
}
