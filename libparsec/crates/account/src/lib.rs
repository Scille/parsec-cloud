// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AccountAuthMethod, AuthenticatedAccountCmds, ProxyConfig};
use libparsec_client_connection::{AnonymousAccountCmds, ConnectionError};
use libparsec_protocol::anonymous_account_cmds::v5::account_create_with_password_proceed::PasswordAlgorithm;
use libparsec_protocol::anonymous_account_cmds::v5::{
    account_create_send_validation_email, account_create_with_password_proceed,
};
use libparsec_types::{
    anyhow, AccountAuthMethodID, Bytes, EmailAddress, EmailAddressParseError, EmailValidationToken,
    ParsecAnonymousAccountAddr, ParsecAuthenticatedAccountAddr, SecretKey,
};
use std::path::PathBuf;
use std::{path::Path, str::FromStr};

/*
Anonymous api
*/

#[derive(Debug, thiserror::Error)]
pub enum AccountSendEmailValidationTokenError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Email recipient refused")]
    EmailRecipientRefused,
    #[error("Email server unavailable")]
    EmailServerUnavailable,
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Unable to parse email")]
    EmailParseError(#[from] EmailAddressParseError),
}

pub async fn account_send_email_validation_token(
    email: &str,
    config_dir: &Path,
    addr: ParsecAnonymousAccountAddr,
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
pub enum AccountCreateWithPasswordProceedError {
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
}

#[allow(clippy::too_many_arguments)]
pub async fn account_create_with_password_proceed(
    auth_method_hmac_key: SecretKey,
    human_label: String,
    password_algorithm: PasswordAlgorithm,
    validation_token: EmailValidationToken,
    vault_key_access: Bytes,
    auth_method_id: AccountAuthMethodID,
    config_dir: &Path,
    addr: ParsecAnonymousAccountAddr,
) -> Result<(), AccountCreateWithPasswordProceedError> {
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;

    match cmds
        .send(account_create_with_password_proceed::Req {
            auth_method_hmac_key,
            human_label,
            password_algorithm,
            validation_token,
            vault_key_access,
            auth_method_id,
        })
        .await?
    {
        account_create_with_password_proceed::Rep::Ok => Ok(()),
        account_create_with_password_proceed::Rep::AuthMethodIdAlreadyExists => {
            Err(AccountCreateWithPasswordProceedError::AuthMethodIdAlreadyExists)
        }
        account_create_with_password_proceed::Rep::InvalidValidationToken => {
            Err(AccountCreateWithPasswordProceedError::InvalidValidationToken)
        }
        bad_rep @ account_create_with_password_proceed::Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

struct Account {
    cmds: AuthenticatedAccountCmds,
    auth_method: AccountAuthMethod,
    config: AccountConfig,
}

struct AccountConfig {
    config_dir: PathBuf,
    addr: ParsecAuthenticatedAccountAddr,
    proxy: ProxyConfig,
}
impl Account {
    fn start(
        config: AccountConfig,
        auth_method: AccountAuthMethod,
    ) -> Result<Account, anyhow::Error> {
        let cmds = AuthenticatedAccountCmds::new(
            &config.config_dir,
            config.addr.clone(),
            config.proxy.clone(),
            auth_method.clone(),
        )?;
        Ok(Account {
            cmds,
            config,
            auth_method,
        })
    }

    /*
    Authenticated api
     */
}
