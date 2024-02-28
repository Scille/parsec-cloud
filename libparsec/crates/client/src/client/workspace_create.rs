// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_storage::workspace::workspace_storage_non_speculative_init;
use libparsec_types::prelude::*;

use super::Client;
use crate::{event_bus::EventWorkspaceLocallyCreated, user::UserStoreUpdateError};

#[derive(Debug, thiserror::Error)]
pub enum ClientCreateWorkspaceError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn create_workspace(
    client_ops: &Client,
    name: EntryName,
) -> Result<VlobID, ClientCreateWorkspaceError> {
    let realm_id = VlobID::default();

    let updater = client_ops.user_ops.for_update_local_workspaces().await;
    let old_local_workspaces = updater.workspaces();

    let local_workspaces = {
        let mut local_workspaces = Vec::with_capacity(old_local_workspaces.len() + 1);
        local_workspaces.extend(old_local_workspaces.iter().cloned());
        local_workspaces.push(LocalUserManifestWorkspaceEntry {
            id: realm_id,
            name: name.clone(),
            name_origin: CertificateBasedInfoOrigin::Placeholder,
            role: RealmRole::Owner,
            role_origin: CertificateBasedInfoOrigin::Placeholder,
        });
        local_workspaces.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        local_workspaces
    };

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
    // However a speculative manifest means the workspace has been
    // created by somebody else, and hence we shouldn't try to create
    // its corresponding realm in the server !
    workspace_storage_non_speculative_init(
        &client_ops.config.data_base_dir,
        &client_ops.device,
        realm_id,
    )
    .await?;

    updater
        .set_workspaces(local_workspaces)
        .await
        .map_err(|err| match err {
            UserStoreUpdateError::Stopped => ClientCreateWorkspaceError::Stopped,
            UserStoreUpdateError::Internal(err) => ClientCreateWorkspaceError::Internal(err),
        })?;
    client_ops
        .event_bus
        .send(&EventWorkspaceLocallyCreated { realm_id, name });
    // Don't need to send an `EventUserOpsNeedSync` event given local user manifest's
    // `local_workspaces` field does not need to be synchronized.

    Ok(realm_id)
}
