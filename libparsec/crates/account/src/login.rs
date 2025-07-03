// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec_client_connection::{
    AccountAuthMethod, AnonymousAccountCmds, AuthenticatedAccountCmds, ConnectionError, ProxyConfig,
};
use libparsec_types::prelude::*;

use super::{
    Account, AUTH_METHOD_ID_DERIVATION_UUID, AUTH_METHOD_MAC_KEY_DERIVATION_UUID,
    AUTH_METHOD_SECRET_KEY_DERIVATION_UUID,
};

#[derive(Debug, thiserror::Error)]
pub enum AccountLoginWithPasswordError {
    #[error("Server provided an invalid password algorithm config: {0}")]
    BadPasswordAlgorithm(CryptoError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AccountLoginWithMasterSecretError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<AccountLoginWithMasterSecretError> for AccountLoginWithPasswordError {
    fn from(value: AccountLoginWithMasterSecretError) -> Self {
        match value {
            AccountLoginWithMasterSecretError::Offline(err) => {
                AccountLoginWithPasswordError::Offline(err)
            }
            AccountLoginWithMasterSecretError::Internal(err) => {
                AccountLoginWithPasswordError::Internal(err)
            }
        }
    }
}

pub(super) async fn account_login_with_master_secret(
    config_dir: PathBuf,
    proxy: ProxyConfig,
    addr: ParsecAddr,
    auth_method_master_secret: KeyDerivation,
    // We must recycle anonymous_cmds if it exists, otherwise its drop will complain
    // during tests if some request registered with `test_register_sequence_of_send_hooks`
    // has yet to occur.
    anonymous_cmds: Option<AnonymousAccountCmds>,
    // Since human handle is supposed to be fetched from the server, this parameter is
    // always set to `None`... Unless for `Account::test_new` where we precisely want
    // to avoid this initial server request!
    human_handle: Option<HumanHandle>,
) -> Result<Account, AccountLoginWithMasterSecretError> {
    let time_provider = TimeProvider::default();

    // Derive from the master secret the data used to authenticate with the server.
    let auth_method_id = AccountAuthMethodID::from(
        auth_method_master_secret.derive_uuid_from_uuid(AUTH_METHOD_ID_DERIVATION_UUID),
    );
    let auth_method_mac_key =
        auth_method_master_secret.derive_secret_key_from_uuid(AUTH_METHOD_MAC_KEY_DERIVATION_UUID);
    let auth_method_secret_key = auth_method_master_secret
        .derive_secret_key_from_uuid(AUTH_METHOD_SECRET_KEY_DERIVATION_UUID);

    let auth_method = AccountAuthMethod {
        time_provider: time_provider.clone(),
        id: auth_method_id,
        mac_key: auth_method_mac_key,
    };

    let cmds = match anonymous_cmds {
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
        time_provider,
        human_handle,
        cmds,
        auth_method_secret_key,
        registration_devices_cache: Default::default(),
    })
}

pub(super) async fn account_login_with_password(
    config_dir: PathBuf,
    proxy: ProxyConfig,
    addr: ParsecAddr,
    email: EmailAddress,
    password: &Password,
) -> Result<Account, AccountLoginWithPasswordError> {
    // The password algorithm configuration is obtained from the server
    // to know how to turn the password into `auth_method_master_secret`.

    let anonymous_cmds = AnonymousAccountCmds::new(&config_dir, addr.clone(), proxy.clone())
        .context("Cannot configure server connection")?;

    let untrusted_password_algorithm = {
        use libparsec_protocol::anonymous_account_cmds::latest::auth_method_password_get_algorithm::{Rep, Req};

        let req = Req {
            email: email.clone(),
        };
        let rep = anonymous_cmds.send(req).await?;

        match rep {
            Rep::Ok { password_algorithm } => password_algorithm,
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
            }
        }
    };

    let password_algorithm = untrusted_password_algorithm
        .validate(email.as_ref())
        .map_err(AccountLoginWithPasswordError::BadPasswordAlgorithm)?;

    let auth_method_master_secret = password_algorithm
        .compute_key_derivation(password)
        .map_err(AccountLoginWithPasswordError::BadPasswordAlgorithm)?;

    account_login_with_master_secret(
        config_dir,
        proxy,
        addr,
        auth_method_master_secret,
        Some(anonymous_cmds),
        None,
    )
    .await
    .map_err(|err| err.into())
}

#[cfg(test)]
#[path = "../tests/unit/login.rs"]
mod tests;
