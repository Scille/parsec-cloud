// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_client_connection::{
    AccountAuthMethod, AnonymousAccountCmds, AuthenticatedAccountCmds, ConnectionError, ProxyConfig,
};
use libparsec_types::prelude::*;

// The auth method master secret is the root secret from which are derived
// all other data used for authentication and end-2-end encryption:
// - ID & MAC key: used for server authentication.
// - Secret key: used for end-2-end encryption.
const AUTH_METHOD_ID_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("00000000-0000-0000-0000-000000000000");
const AUTH_METHOD_MAC_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("11111111-1111-1111-1111-111111111111");
const AUTH_METHOD_SECRET_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("22222222-2222-2222-2222-222222222222");

#[derive(Debug)]
pub struct Account {
    human_handle: HumanHandle,
    cmds: AuthenticatedAccountCmds,
    auth_method_secret_key: SecretKey,
}

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

pub struct AccountListRegistrationDeviceItem {
    pub organization_id: OrganizationID,
    pub user_id: UserID,
}

pub struct AccountListRegistrationDeviceItems {
    pub items: Vec<AccountListRegistrationDeviceItem>,
    pub bad_entries: Vec<()>,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountListRegistrationDeviceError {
    #[error("Cannot decrypt the vault key access return by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl Account {
    pub fn human_handle(&self) -> &HumanHandle {
        &self.human_handle
    }

    pub async fn login_with_master_secret(
        config_dir: &Path,
        proxy: ProxyConfig,
        addr: ParsecAddr,
        auth_method_master_secret: KeyDerivation,
    ) -> Result<Self, AccountLoginWithMasterSecretError> {
        Self::_login_with_master_secret(config_dir, proxy, addr, auth_method_master_secret, None)
            .await
    }

    async fn _login_with_master_secret(
        config_dir: &Path,
        proxy: ProxyConfig,
        addr: ParsecAddr,
        auth_method_master_secret: KeyDerivation,
        // We must recycle anonymous_cmds if it exists, otherwise its drop will complain
        // during tests if some request registered with `test_register_sequence_of_send_hooks`
        // has yet to occur.
        anonymous_cmds: Option<AnonymousAccountCmds>,
    ) -> Result<Self, AccountLoginWithMasterSecretError> {
        // Derive from the master secret the data used to authenticate with the server.
        let auth_method_id = AccountAuthMethodID::from(
            auth_method_master_secret.derive_uuid_from_uuid(AUTH_METHOD_ID_DERIVATION_UUID),
        );
        let auth_method_mac_key = auth_method_master_secret
            .derive_secret_key_from_uuid(AUTH_METHOD_MAC_KEY_DERIVATION_UUID);
        let auth_method_secret_key = auth_method_master_secret
            .derive_secret_key_from_uuid(AUTH_METHOD_SECRET_KEY_DERIVATION_UUID);

        let auth_method = AccountAuthMethod {
            time_provider: TimeProvider::default(),
            id: auth_method_id,
            mac_key: auth_method_mac_key,
        };

        let cmds = match anonymous_cmds {
            None => AuthenticatedAccountCmds::new(config_dir, addr, proxy, auth_method)
                .context("Cannot configure server connection")?,
            Some(anonymous_cmds) => {
                AuthenticatedAccountCmds::from_anonymous(anonymous_cmds, auth_method)
            }
        };

        let human_handle = {
            use libparsec_protocol::authenticated_account_cmds::latest::account_info::{Rep, Req};

            let req = Req {};
            let rep = cmds.send(req).await?;

            match rep {
                Rep::Ok { human_handle } => human_handle,
                bad_rep @ Rep::UnknownStatus { .. } => {
                    return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
                }
            }
        };

        Ok(Self {
            human_handle,
            cmds,
            auth_method_secret_key,
        })
    }

    pub async fn login_with_password(
        config_dir: &Path,
        proxy: ProxyConfig,
        addr: ParsecAddr,
        email: EmailAddress,
        password: Password,
    ) -> Result<Self, AccountLoginWithPasswordError> {
        // The password algorithm configuration is obtained from the server
        // to know how to turn the password into `auth_method_master_secret`.

        let anonymous_cmds = AnonymousAccountCmds::new(config_dir, addr.clone(), proxy.clone())
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
            .compute_key_derivation(&password)
            .map_err(AccountLoginWithPasswordError::BadPasswordAlgorithm)?;

        Self::_login_with_master_secret(
            config_dir,
            proxy,
            addr,
            auth_method_master_secret,
            Some(anonymous_cmds),
        )
        .await
        .map_err(|err| err.into())
    }

    pub async fn list_registration_device(
        &self,
    ) -> Result<AccountListRegistrationDeviceItems, AccountListRegistrationDeviceError> {
        let (ciphered_key_access, items) = {
            use libparsec_protocol::authenticated_account_cmds::latest::vault_item_list::{
                Rep, Req,
            };

            let req = Req {};
            let rep = self.cmds.send(req).await?;

            match rep {
                Rep::Ok { key_access, items } => (key_access, items),
                bad_rep @ Rep::UnknownStatus { .. } => {
                    return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
                }
            }
        };

        let _vault_key = {
            let access = AccountVaultKeyAccess::decrypt_and_load(
                &ciphered_key_access,
                &self.auth_method_secret_key,
            )
            .map_err(AccountListRegistrationDeviceError::BadVaultKeyAccess)?;
            access.vault_key
        };

        let mut res = AccountListRegistrationDeviceItems {
            items: Vec::new(),
            bad_entries: Vec::new(),
        };

        for (_, item_raw) in items {
            let item = match AccountVaultItem::load(&item_raw) {
                Ok(item) => item,
                Err(err) => {
                    log::warn!("Cannot deserialize account vault item: {}", err);
                    continue;
                }
            };
            match item {
                AccountVaultItem::RegistrationDevice(registration_device) => {
                    res.items.push(AccountListRegistrationDeviceItem {
                        organization_id: registration_device.organization_id,
                        user_id: registration_device.user_id,
                    });
                }
                AccountVaultItem::WebLocalDeviceKey(_) => (),
            }
        }

        Ok(res)
    }
}

#[cfg(test)]
#[path = "../tests/unit/account.rs"]
mod tests;
