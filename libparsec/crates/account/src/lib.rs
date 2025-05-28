// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AccountAuthMethod, ProxyConfig};
use libparsec_client_connection::{AnonymousAccountCmds, ConnectionError};
use libparsec_protocol::anonymous_account_cmds::v5::{
    account_create_proceed, account_create_send_validation_email,
};
use libparsec_types::uuid::Uuid;
use libparsec_types::{
    anyhow, uuid, AccountAuthMethodID, Bytes, CryptoError, EmailAddress, EmailAddressParseError,
    EmailValidationToken, KeyDerivation, ParsecAnonymousAccountAddr, Password, PasswordAlgorithm,
    SecretKey, TimeProvider,
};
use std::{path::Path, str::FromStr};

pub const AUTH_METHOD_ID_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("00000000-0000-0000-0000-000000000000");
pub const AUTH_METHOD_MAC_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("11111111-1111-1111-1111-111111111111");
pub const AUTH_METHOD_SECRET_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("22222222-2222-2222-2222-222222222222");
pub const AUTH_METHOD_VAULT_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("33333333-3333-3333-3333-333333333333");

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

#[allow(clippy::too_many_arguments)]
pub async fn account_create_proceed(
    human_label: String,
    validation_token: EmailValidationToken,
    config_dir: &Path,
    addr: ParsecAnonymousAccountAddr,
    password: Password,
) -> Result<(), AccountCreateProceedError> {
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;
    let salt = Bytes::from(Uuid::new_v4().to_string());
    let memlimit_kb = 1024;
    let parallelism = 1;
    let opslimit = 1;

    let auth_method_master_secret_key =
        SecretKey::from_argon2id_password(&password, &salt, opslimit, memlimit_kb, parallelism)?;

    let auth_method_master_secret = KeyDerivation::from(
        <[u8; KeyDerivation::SIZE]>::try_from(
            &auth_method_master_secret_key.as_ref()[..KeyDerivation::SIZE],
        )
        .expect("unexpected error"),
    );

    let auth_method = AccountAuthMethod {
        time_provider: TimeProvider::default(),
        id: AccountAuthMethodID::from(
            auth_method_master_secret.derive_uuid_from_uuid(AUTH_METHOD_ID_DERIVATION_UUID),
        ),
        hmac_key: auth_method_master_secret
            .derive_secret_key_from_uuid(AUTH_METHOD_MAC_KEY_DERIVATION_UUID),
    };
    let vault_key = auth_method_master_secret
        .derive_secret_key_from_uuid(AUTH_METHOD_VAULT_KEY_DERIVATION_UUID);

    let vault_key_access = auth_method_master_secret_key
        .encrypt(vault_key.as_ref())
        .into();

    let auth_method_password_algorithm = Some(PasswordAlgorithm::Argon2id {
        salt,
        opslimit,
        memlimit_kb,
        parallelism,
    });
    match cmds
        .send(account_create_proceed::Req {
            auth_method_hmac_key: auth_method.hmac_key,
            human_label,
            auth_method_password_algorithm,
            validation_token,
            vault_key_access,
            auth_method_id: auth_method.id,
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

/*
Authenticated api
 */
