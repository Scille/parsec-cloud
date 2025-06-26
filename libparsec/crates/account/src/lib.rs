// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashSet, path::PathBuf, sync::Arc};
use std::{path::Path, str::FromStr};

use libparsec_client_connection::{
    AccountAuthMethod, AnonymousAccountCmds, AuthenticatedAccountCmds, ConnectionError, ProxyConfig,
};
use libparsec_protocol::anonymous_account_cmds::v5::{
    account_create_proceed, account_create_send_validation_email,
};
use libparsec_types::prelude::*;

mod fetch_list_registration_devices;
mod list_invitations;
mod login;
mod register_new_device;

pub use fetch_list_registration_devices::*;
pub use list_invitations::*;
pub use login::*;
pub use register_new_device::*;

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
    config_dir: PathBuf,
    time_provider: TimeProvider,
    human_handle: HumanHandle,
    cmds: AuthenticatedAccountCmds,
    auth_method_secret_key: SecretKey,
    registration_devices_cache: Vec<Arc<LocalDevice>>,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountSendEmailValidationTokenError {
    #[error("Email recipient refused")]
    EmailRecipientRefused,
    #[error("Email server unavailable")]
    EmailServerUnavailable,
    #[error("Unable to parse email")]
    EmailParseError(#[from] EmailAddressParseError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl Account {
    /// Convenient helper to avoid the first request (fetching the human handle)
    /// whenever we need an `Account` instance in the tests.
    #[cfg(test)]
    pub async fn test_new(
        config_dir: PathBuf,
        addr: ParsecAddr,
        auth_method_master_secret: KeyDerivation,
        human_handle: HumanHandle,
    ) -> Self {
        account_login_with_master_secret(
            config_dir,
            ProxyConfig::default(),
            addr,
            auth_method_master_secret,
            None,
            Some(human_handle),
        )
        .await
        .unwrap()
    }

    pub fn human_handle(&self) -> &HumanHandle {
        &self.human_handle
    }

    pub async fn login_with_master_secret(
        config_dir: PathBuf,
        proxy: ProxyConfig,
        addr: ParsecAddr,
        auth_method_master_secret: KeyDerivation,
    ) -> Result<Self, AccountLoginWithMasterSecretError> {
        account_login_with_master_secret(
            config_dir,
            proxy,
            addr,
            auth_method_master_secret,
            None,
            None,
        )
        .await
    }

    pub async fn login_with_password(
        config_dir: PathBuf,
        proxy: ProxyConfig,
        addr: ParsecAddr,
        email: EmailAddress,
        password: Password,
    ) -> Result<Self, AccountLoginWithPasswordError> {
        account_login_with_password(config_dir, proxy, addr, email, password).await
    }

    pub async fn list_invitations(
        &self,
    ) -> Result<Vec<(OrganizationID, InvitationToken, InvitationType)>, AccountListInvitationsError>
    {
        account_list_invitations(self).await
    }

    pub async fn fetch_registration_devices(
        &mut self,
    ) -> Result<(), AccountFetchRegistrationDevicesError> {
        account_fetch_registration_devices(self).await
    }

    pub fn list_registration_devices(&self) -> HashSet<(OrganizationID, UserID)> {
        account_list_registration_devices(self)
    }

    pub async fn register_new_device(
        &self,
        organization_id: OrganizationID,
        user_id: UserID,
        new_device_label: DeviceLabel,
        save_strategy: DeviceSaveStrategy,
    ) -> Result<AvailableDevice, AccountRegisterNewDeviceError> {
        account_register_new_device(
            self,
            organization_id,
            user_id,
            new_device_label,
            save_strategy,
        )
        .await
    }
}

pub async fn account_create_send_validation_email(
    email: &str,
    config_dir: &Path,
    addr: ParsecAddr,
) -> Result<(), AccountSendEmailValidationTokenError> {
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;
    let email = EmailAddress::from_str(email)?;

    match cmds
        .send(account_create_send_validation_email::Req { email })
        .await?
    {
        account_create_send_validation_email::Rep::Ok => Ok(()),

        account_create_send_validation_email::Rep::EmailRecipientRefused => {
            Err(AccountSendEmailValidationTokenError::EmailRecipientRefused)
        }
        account_create_send_validation_email::Rep::EmailServerUnavailable => {
            Err(AccountSendEmailValidationTokenError::EmailServerUnavailable)
        }
        bad_rep @ account_create_send_validation_email::Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AccountCreateProceedError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Invalid validation code")]
    InvalidValidationCode,
    #[error("Auth method id already exists")]
    AuthMethodIdAlreadyExists,
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
}

pub enum AccountCreateStep {
    CheckCode {
        validation_code: ValidationCode,
        email: EmailAddress,
    },
    Create {
        human_handle: HumanHandle,
        password: Password,
        validation_code: ValidationCode,
    },
}

pub async fn account_create_proceed(
    step: AccountCreateStep,
    config_dir: &Path,
    addr: ParsecAddr,
) -> Result<(), AccountCreateProceedError> {
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;

    let req_step = match step {
        AccountCreateStep::CheckCode {
            validation_code,
            email,
        } => account_create_proceed::AccountCreateStep::Number0CheckCode {
            validation_code,
            email,
        },
        AccountCreateStep::Create {
            human_handle,
            password,
            validation_code,
        } => {
            let memlimit_kb = 1024;
            let parallelism = 1;
            let opslimit = 1;

            let auth_method_password_algorithm = UntrustedPasswordAlgorithm::Argon2id {
                opslimit,
                memlimit_kb,
                parallelism,
            };

            let trusted_password_algorithm = auth_method_password_algorithm
                .clone()
                .validate(human_handle.email().as_ref())?;

            let auth_method_master_secret =
                trusted_password_algorithm.compute_key_derivation(&password)?;

            let auth_method_secret_key = auth_method_master_secret
                .derive_secret_key_from_uuid(AUTH_METHOD_SECRET_KEY_DERIVATION_UUID);

            let auth_method = AccountAuthMethod {
                time_provider: TimeProvider::default(),
                id: AccountAuthMethodID::from(
                    auth_method_master_secret.derive_uuid_from_uuid(AUTH_METHOD_ID_DERIVATION_UUID),
                ),
                mac_key: auth_method_master_secret
                    .derive_secret_key_from_uuid(AUTH_METHOD_MAC_KEY_DERIVATION_UUID),
            };
            let vault_key = SecretKey::generate();

            let vault_key_access = AccountVaultKeyAccess { vault_key };

            let vault_key_access_bytes = vault_key_access.dump_and_encrypt(&auth_method_secret_key);

            account_create_proceed::AccountCreateStep::Number1Create {
                validation_code,
                auth_method_hmac_key: auth_method.mac_key,
                auth_method_id: auth_method.id,
                auth_method_password_algorithm: Some(auth_method_password_algorithm),
                human_handle,
                vault_key_access: vault_key_access_bytes.into(),
            }
        }
    };

    match cmds
        .send(account_create_proceed::Req {
            account_create_step: req_step,
        })
        .await?
    {
        account_create_proceed::Rep::Ok => Ok(()),
        account_create_proceed::Rep::AuthMethodIdAlreadyExists => {
            Err(AccountCreateProceedError::AuthMethodIdAlreadyExists)
        }
        account_create_proceed::Rep::InvalidValidationCode => {
            Err(AccountCreateProceedError::InvalidValidationCode)
        }
        bad_rep @ account_create_proceed::Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
