// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec_client_connection::{
    AccountAuthMethod, AnonymousServerCmds, AuthenticatedAccountCmds, ConnectionError, ProxyConfig,
};
use libparsec_types::prelude::*;

use crate::{
    derive_auth_method_keys, retrieve_auth_method_master_secret_from_password, Account,
    AccountLoginStrategy, RetrieveAuthMethodMasterSecretFromPasswordError,
};

#[derive(Debug, thiserror::Error)]
pub enum AccountLoginError {
    /// Obviously only occur if `authentication_strategy` is passed as `Password`
    #[error("Server provided an invalid password algorithm config: {0}")]
    BadPasswordAlgorithm(CryptoError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn account_login(
    config_dir: PathBuf,
    proxy: ProxyConfig,
    addr: ParsecAddr,
    login_strategy: AccountLoginStrategy<'_>,
    // Since human handle is supposed to be fetched from the server, this parameter is
    // always set to `None`... Unless for `Account::test_new` where we precisely want
    // to avoid this initial server request!
    human_handle: Option<HumanHandle>,
) -> Result<Account, AccountLoginError> {
    let (maybe_anonymous_cmds, auth_method_keys) = match login_strategy {
        AccountLoginStrategy::Password { password, email } => {
            // The password algorithm configuration is obtained from the server
            // to know how to turn the password into `auth_method_master_secret`.

            let anonymous_cmds = AnonymousServerCmds::new(&config_dir, addr.clone(), proxy.clone())
                .context("Cannot configure server connection")?;

            let auth_method_master_secret =
                retrieve_auth_method_master_secret_from_password(&anonymous_cmds, email, password)
                    .await
                    .map_err(|e| match e {
                        RetrieveAuthMethodMasterSecretFromPasswordError::BadPasswordAlgorithm(
                            err,
                        ) => AccountLoginError::BadPasswordAlgorithm(err),
                        RetrieveAuthMethodMasterSecretFromPasswordError::Offline(err) => {
                            AccountLoginError::Offline(err)
                        }
                        RetrieveAuthMethodMasterSecretFromPasswordError::Internal(err) => {
                            AccountLoginError::Internal(err)
                        }
                    })?;

            let auth_method_keys = derive_auth_method_keys(&auth_method_master_secret);

            (Some(anonymous_cmds), auth_method_keys)
        }
        AccountLoginStrategy::MasterSecret(auth_method_master_secret) => {
            let auth_method_keys = derive_auth_method_keys(auth_method_master_secret);
            (None, auth_method_keys)
        }
    };

    let auth_method = AccountAuthMethod {
        time_provider: TimeProvider::default(),
        id: auth_method_keys.id,
        mac_key: auth_method_keys.mac_key,
    };

    // We must recycle anonymous_cmds if it exists, otherwise its drop will complain
    // during tests if some request registered with `test_register_sequence_of_send_hooks`
    // has yet to occur.
    let cmds = match maybe_anonymous_cmds {
        None => AuthenticatedAccountCmds::new(&config_dir, addr, proxy, auth_method)
            .context("Cannot configure server connection")?,
        Some(anonymous_cmds) => {
            AuthenticatedAccountCmds::from_anonymous(anonymous_cmds, auth_method)
        }
    };

    let human_handle = match human_handle {
        None => {
            use libparsec_protocol::authenticated_account_cmds::latest::account_info::{Rep, Req};

            let req = Req {};
            let rep = cmds.send(req).await?;

            match rep {
                Rep::Ok { human_handle } => human_handle,
                bad_rep @ Rep::UnknownStatus { .. } => {
                    return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
                }
            }
        }
        Some(human_handle) => human_handle,
    };

    Ok(Account {
        config_dir,
        human_handle,
        cmds,
        auth_method_id: auth_method_keys.id,
        auth_method_secret_key: auth_method_keys.secret_key,
    })
}

#[cfg(test)]
#[path = "../tests/unit/login.rs"]
mod tests;
