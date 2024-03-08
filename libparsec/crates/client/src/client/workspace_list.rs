// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::Client;

#[derive(Debug)]
pub struct WorkspaceInfo {
    pub id: VlobID,
    pub current_name: EntryName,
    pub current_self_role: RealmRole,
    pub is_started: bool,
    /// `true` once the bootstrap has been done (i.e. initial user realm role certificate,
    /// key rotation & realm name certificate uploaded on server-side).
    ///
    /// If `false`, the workspace should be considered only available from local (as it is
    /// when it has just been created). Though things are more complex in practice as the
    /// bootstrap is composed of multiple operations that are not atomic (basically a workspace
    /// which hasn't finished bootstrap might be visible from the author's other devices).
    ///
    /// Note that this is unrelated with the synchronization of the workspace's data (i.e. vlob/blob).
    pub is_bootstrapped: bool,
}

pub async fn list_workspaces(client_ops: &Client) -> Vec<WorkspaceInfo> {
    // We base ourself on the the local user manifest's `local_workspaces` field (instead
    // of directly querying the certificates).
    //
    // The idea is that the local user manifest gets updated (typically by a monitor)
    // any time new certificates are received from the server.

    let started_workspaces = client_ops.workspaces.lock().await;
    let user_manifest = client_ops.user_ops.get_user_manifest();

    let mut infos: Vec<_> = user_manifest
        .local_workspaces
        .iter()
        .map(|entry| {
            // Having a name certificate is a proof that the workspace has been bootstrapped
            // (as it is the last step during bootstrap).
            let is_bootstrapped = matches!(
                entry.name_origin,
                CertificateBasedInfoOrigin::Certificate { .. }
            );
            let is_started = started_workspaces
                .iter()
                .any(|workspace_ops| workspace_ops.realm_id() == entry.id);
            WorkspaceInfo {
                id: entry.id,
                current_name: entry.name.clone(),
                current_self_role: entry.role,
                is_started,
                is_bootstrapped,
            }
        })
        .collect();

    // Workspace list is sorted during it refresh, but there is no strong guarantee
    // that's the case if the data comes from a serialized local user manifest.
    infos.sort_unstable_by(|a, b| a.current_name.cmp(&b.current_name));

    infos
}
