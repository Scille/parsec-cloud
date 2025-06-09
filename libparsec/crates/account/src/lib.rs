// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_client_connection::{
    AccountAuthMethod, AnonymousAccountCmds, AuthenticatedAccountCmds, ConnectionError, ProxyConfig,
};
use libparsec_types::prelude::*;

pub const AUTH_METHOD_ID_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("00000000-0000-0000-0000-000000000000");
pub const AUTH_METHOD_MAC_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("11111111-1111-1111-1111-111111111111");
pub const AUTH_METHOD_SECRET_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("22222222-2222-2222-2222-222222222222");

#[derive(Debug)]
pub struct Account {
    #[expect(unused)]
    cmds: AuthenticatedAccountCmds,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountFromPasswordError {
    #[error("Server provided an invalid password algorithm config: {0}")]
    BadPasswordAlgorithm(CryptoError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl Account {
    pub fn new(
        config_dir: &Path,
        proxy: ProxyConfig,
        addr: ParsecAddr,
        auth_method_master_secret: KeyDerivation,
    ) -> anyhow::Result<Self> {
        let auth_method = AccountAuthMethod {
            time_provider: TimeProvider::default(),
            id: AccountAuthMethodID::from(
                auth_method_master_secret.derive_uuid_from_uuid(AUTH_METHOD_ID_DERIVATION_UUID),
            ),
            mac_key: auth_method_master_secret
                .derive_secret_key_from_uuid(AUTH_METHOD_MAC_KEY_DERIVATION_UUID),
        };
        let cmds = AuthenticatedAccountCmds::new(config_dir, addr, proxy, auth_method)
            .context("Cannot configure server connection")?;

        Ok(Self { cmds })
    }

    pub async fn from_password(
        config_dir: &Path,
        proxy: ProxyConfig,
        addr: ParsecAddr,
        email: EmailAddress,
        password: Password,
    ) -> Result<Self, AccountFromPasswordError> {
        let password_algorithm = {
            let cmds = AnonymousAccountCmds::new(config_dir, addr.clone(), proxy.clone())
                .context("Cannot configure server connection")?;

            use libparsec_protocol::anonymous_account_cmds::latest::auth_method_password_get_algorithm::{Rep, Req};

            let req = Req { email };
            let rep = cmds.send(req).await?;

            match rep {
                Rep::Ok { password_algorithm } => password_algorithm,
                bad_rep @ Rep::UnknownStatus { .. } => {
                    return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
                }
            }
        };

        let auth_method_master_secret = password_algorithm
            .compute_key_derivation(&password)
            .map_err(AccountFromPasswordError::BadPasswordAlgorithm)?;

        Self::new(config_dir, proxy, addr, auth_method_master_secret).map_err(|err| err.into())
    }
}

#[cfg(test)]
#[path = "../tests/unit/account.rs"]
mod tests;
