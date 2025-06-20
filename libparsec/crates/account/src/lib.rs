// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, str::FromStr};

use libparsec_client_connection::{
    AccountAuthMethod, AnonymousAccountCmds, AuthenticatedAccountCmds, ConnectionError, ProxyConfig,
};
use libparsec_protocol::anonymous_account_cmds::v5::{
    account_create_proceed, account_create_send_validation_email,
};
use libparsec_types::prelude::*;

// The auth method master secret is the root secret from which are derived
// all other data used for authentication and end-2-end encryption:
// - ID & MAC key: used for server authentication.
// - Secret key: used for end-2-end encryption.
pub const AUTH_METHOD_ID_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("00000000-0000-0000-0000-000000000000");
pub const AUTH_METHOD_MAC_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("11111111-1111-1111-1111-111111111111");
pub const AUTH_METHOD_SECRET_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("22222222-2222-2222-2222-222222222222");
pub const AUTH_METHOD_VAULT_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("33333333-3333-3333-3333-333333333333");

#[derive(Debug)]
pub struct Account {
    #[expect(unused)]
    cmds: AuthenticatedAccountCmds,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountFromPasswordError {
    #[error("Server provided an invalid password algorithm config: {0}")]
    BadPasswordAlgorithm(CryptoError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AccountSendEmailValidationTokenError {
    #[error("Email recipient refused")]
    EmailRecipientRefused,
    #[error("Email server unavailable")]
    EmailServerUnavailable,
    #[error("Invalid email")]
    InvalidEmail,
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
    pub fn new(
        config_dir: &Path,
        proxy: ProxyConfig,
        addr: ParsecAddr,
        auth_method_master_secret: KeyDerivation,
    ) -> anyhow::Result<Self> {
        // Derive from the master secret the data used to authenticate with the server.
        let auth_method_id = AccountAuthMethodID::from(
            auth_method_master_secret.derive_uuid_from_uuid(AUTH_METHOD_ID_DERIVATION_UUID),
        );
        let auth_method_mac_key = auth_method_master_secret
            .derive_secret_key_from_uuid(AUTH_METHOD_MAC_KEY_DERIVATION_UUID);

        let auth_method = AccountAuthMethod {
            time_provider: TimeProvider::default(),
            id: auth_method_id,
            mac_key: auth_method_mac_key,
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
        // The password algorithm configuration is obtained from the server
        // to know how to turn the password into `auth_method_master_secret`.

        let untrusted_password_algorithm = {
            let cmds = AnonymousAccountCmds::new(config_dir, addr.clone(), proxy.clone())
                .context("Cannot configure server connection")?;

            use libparsec_protocol::anonymous_account_cmds::latest::auth_method_password_get_algorithm::{Rep, Req};

            let req = Req {
                email: email.clone(),
            };
            let rep = cmds.send(req).await?;

            match rep {
                Rep::Ok { password_algorithm } => password_algorithm,
                bad_rep @ Rep::UnknownStatus { .. } => {
                    return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
                }
            }
        };

        let password_algorithm = untrusted_password_algorithm
            .validate(email.as_ref())
            .map_err(AccountFromPasswordError::BadPasswordAlgorithm)?;

        let auth_method_master_secret = password_algorithm
            .compute_key_derivation(&password)
            .map_err(AccountFromPasswordError::BadPasswordAlgorithm)?;

        Self::new(config_dir, proxy, addr, auth_method_master_secret).map_err(|err| err.into())
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
        account_create_send_validation_email::Rep::InvalidEmail => {
            Err(AccountSendEmailValidationTokenError::InvalidEmail)
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
    #[error("Invalid validation Token")]
    InvalidValidationToken,
    #[error("Auth method id already exists")]
    AuthMethodIdAlreadyExists,
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
}

pub enum AccountCreateStep {
    CheckCode {
        code: String,
    },
    Create {
        human_label: String,
        password: Password,
    },
}

pub async fn account_create_proceed(
    step: AccountCreateStep,
    email: EmailAddress,
    config_dir: &Path,
    addr: ParsecAddr,
) -> Result<(), AccountCreateProceedError> {
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;

    let req_step = match step {
        AccountCreateStep::CheckCode { code } => {
            account_create_proceed::AccountCreateStep::Number0CheckCode { code }
        }
        AccountCreateStep::Create {
            human_label,
            password,
        } => {
            let memlimit_kb = 1024;
            let parallelism = 1;
            let opslimit = 1;

            let auth_method_password_algorithm = UntrustedPasswordAlgorithm::Argon2id {
                opslimit,
                memlimit_kb,
                parallelism,
            };

            let trusted_password_algorithm = auth_method_password_algorithm.clone().validate(email.as_ref())?;

            let auth_method_master_secret_key =
                trusted_password_algorithm.compute_secret_key(&password)?;

            let auth_method_master_secret =
                trusted_password_algorithm.compute_key_derivation(&password)?;

            let auth_method = AccountAuthMethod {
                time_provider: TimeProvider::default(),
                id: AccountAuthMethodID::from(
                    auth_method_master_secret.derive_uuid_from_uuid(AUTH_METHOD_ID_DERIVATION_UUID),
                ),
                mac_key: auth_method_master_secret
                    .derive_secret_key_from_uuid(AUTH_METHOD_MAC_KEY_DERIVATION_UUID),
            };
            let vault_key = auth_method_master_secret
                .derive_secret_key_from_uuid(AUTH_METHOD_VAULT_KEY_DERIVATION_UUID);

            let vault_key_access = auth_method_master_secret_key
                .encrypt(vault_key.as_ref())
                .into();

            account_create_proceed::AccountCreateStep::Number1Create {
                auth_method_hmac_key: auth_method.mac_key,
                auth_method_id: auth_method.id,
                auth_method_password_algorithm: Some(auth_method_password_algorithm),
                human_label,
                vault_key_access,
            }
        }
    };

    match cmds
        .send(account_create_proceed::Req {
            account_create_step: req_step,
            email,
        })
        .await?
    {
        account_create_proceed::Rep::Ok => Ok(()),
        account_create_proceed::Rep::AuthMethodIdAlreadyExists => {
            Err(AccountCreateProceedError::AuthMethodIdAlreadyExists)
        }
        account_create_proceed::Rep::InvalidValidationToken => {
            Err(AccountCreateProceedError::InvalidValidationToken)
        }
        bad_rep @ account_create_proceed::Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/account.rs"]
mod tests;
