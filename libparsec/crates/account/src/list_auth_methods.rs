// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::Account;

#[derive(Debug, thiserror::Error)]
pub enum AccountListAuthMethodsError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthMethodInfo {
    pub auth_method_id: AccountAuthMethodID,
    pub created_on: DateTime,
    pub created_by_ip: String,
    pub created_by_user_agent: String,
    pub use_password: bool,
}

pub(super) async fn account_list_auth_methods(
    account: &Account,
) -> Result<Vec<AuthMethodInfo>, AccountListAuthMethodsError> {
    use libparsec_protocol::authenticated_account_cmds::latest::auth_method_list::{Rep, Req};

    let req = Req {};
    let rep = account.cmds.send(req).await?;

    match rep {
        Rep::Ok { items } => {
            let auth_methods = items
                .into_iter()
                .map(|item| AuthMethodInfo {
                    auth_method_id: item.auth_method_id,
                    created_on: item.created_on,
                    created_by_ip: item.created_by_ip,
                    created_by_user_agent: item.created_by_user_agent,
                    use_password: item.password_algorithm.is_some(),
                })
                .collect();
            Ok(auth_methods)
        }
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/list_auth_methods.rs"]
mod tests;
