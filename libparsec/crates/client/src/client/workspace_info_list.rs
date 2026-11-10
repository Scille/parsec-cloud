// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_protocol::authenticated_cmds::latest::realm_list_info::{Rep, Req};
use libparsec_types::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct WorkspaceInfoItem {
    pub realm_id: VlobID,
    pub metadata_size: u64,
    pub data_size: u64,
    pub users_per_role: HashMap<RealmRole, u64>,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientListWorkspacesInfoError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("author not allowed: must be admin to retrieve the workspaces info")]
    AuthorNotAllowed,
}

pub async fn list_workspaces_info(
    cmds: &Arc<AuthenticatedCmds>,
) -> Result<Vec<WorkspaceInfoItem>, ClientListWorkspacesInfoError> {
    match cmds.send(Req {}).await? {
        Rep::Ok { realms } => Ok(realms
            .into_iter()
            .map(|realm| WorkspaceInfoItem {
                realm_id: realm.realm_id,
                metadata_size: realm.metadata_size,
                data_size: realm.data_size,
                users_per_role: realm.users_per_role,
            })
            .collect()),
        Rep::AuthorNotAllowed => Err(ClientListWorkspacesInfoError::AuthorNotAllowed),
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {bad_rep:?}").into())
        }
    }
}
