// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_protocol::authenticated_cmds::latest::list_frozen_users::{Rep, Req};
use libparsec_types::prelude::*;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum ClientListFrozenUsersError {
    #[error("Cannot reach the server")]
    Offline,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("author not allowed: must be admin to retrieve frozen users")]
    AuthorNotAllowed,
}

impl From<ConnectionError> for ClientListFrozenUsersError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn list_frozen_users(
    cmds: &Arc<AuthenticatedCmds>,
) -> Result<Vec<UserID>, ClientListFrozenUsersError> {
    match cmds.send(Req {}).await? {
        Rep::Ok { frozen_users } => Ok(frozen_users),
        Rep::AuthorNotAllowed => Err(ClientListFrozenUsersError::AuthorNotAllowed),
        bad_rep @ (Rep::UnknownStatus { .. } | Rep::AuthorNotFound) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
