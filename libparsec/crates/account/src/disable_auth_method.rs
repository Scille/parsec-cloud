// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::Account;

#[derive(Debug, thiserror::Error)]
pub enum AccountDisableAuthMethodError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Authentication method not found")]
    AuthMethodNotFound,
    #[error("Authentication method already disabled")]
    AuthMethodAlreadyDisabled,
    #[error("Cannot disable the authentication method used to do the request")]
    SelfDisableNotAllowed,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn account_disable_auth_method(
    account: &Account,
    auth_method_id: AccountAuthMethodID,
) -> Result<(), AccountDisableAuthMethodError> {
    use libparsec_protocol::authenticated_account_cmds::latest::auth_method_disable::{Rep, Req};

    let req = Req { auth_method_id };
    let rep = account.cmds.send(req).await?;

    match rep {
        Rep::Ok => Ok(()),
        Rep::AuthMethodNotFound => Err(AccountDisableAuthMethodError::AuthMethodNotFound),
        Rep::AuthMethodAlreadyDisabled => {
            Err(AccountDisableAuthMethodError::AuthMethodAlreadyDisabled)
        }
        Rep::SelfDisableNotAllowed => Err(AccountDisableAuthMethodError::SelfDisableNotAllowed),
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/disable_auth_method.rs"]
mod tests;
