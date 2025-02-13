// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_protocol::authenticated_cmds::latest::list_frozen_users::{Rep, Req};
use libparsec_types::prelude::*;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum ClientListFrozenUsersError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("author not allowed: must be admin to retrieve frozen users")]
    AuthorNotAllowed,
}

pub async fn list_frozen_users(
    cmds: &Arc<AuthenticatedCmds>,
) -> Result<Vec<UserID>, ClientListFrozenUsersError> {
    match cmds.send(Req {}).await? {
        Rep::Ok { frozen_users } => Ok(frozen_users),
        Rep::AuthorNotAllowed => Err(ClientListFrozenUsersError::AuthorNotAllowed),
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
