// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_protocol::authenticated_cmds::latest::realm_list_stats::{Rep, Req};
use libparsec_types::prelude::*;

/// Stats about a workspace, as seen by the server.
///
/// Note the workspace name is not provided: it is encrypted with the
/// workspace's keys, which the author may not have access to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceStats {
    pub workspace_id: VlobID,
    /// Total size of the vlobs (all versions) stored in the workspace.
    pub metadata_size: u64,
    /// Total size of the blocks stored in the workspace.
    pub data_size: u64,
    // Number of (non-revoked) users currently having the given role in the workspace.
    pub owners: u64,
    pub managers: u64,
    pub contributors: u64,
    pub readers: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientListWorkspacesStatsError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Author not allowed: must be admin to retrieve workspaces stats")]
    AuthorNotAllowed,
}

/// List stats for all the workspaces of the organization (including the ones
/// the current user is not part of).
///
/// Only an ADMIN is allowed to do this.
pub(super) async fn list_workspaces_stats(
    cmds: &Arc<AuthenticatedCmds>,
) -> Result<Vec<WorkspaceStats>, ClientListWorkspacesStatsError> {
    match cmds.send(Req).await? {
        Rep::Ok { realms } => Ok(realms
            .into_iter()
            .map(|realm| WorkspaceStats {
                workspace_id: realm.realm_id,
                metadata_size: realm.metadata_size,
                data_size: realm.data_size,
                owners: realm.owners,
                managers: realm.managers,
                contributors: realm.contributors,
                readers: realm.readers,
            })
            .collect()),
        Rep::AuthorNotAllowed => Err(ClientListWorkspacesStatsError::AuthorNotAllowed),
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
