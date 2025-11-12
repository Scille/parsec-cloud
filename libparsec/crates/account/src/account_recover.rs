// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AnonymousServerCmds, ConnectionError};
use libparsec_types::prelude::*;

use crate::AccountAuthMethodStrategy;

#[derive(Debug, thiserror::Error)]
pub enum AccountRecoverSendValidationEmailError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Email recipient refused")]
    EmailRecipientRefused,
    #[error("Email server unavailable")]
    EmailServerUnavailable,
    #[error("Too many email sent, must wait until {}", &wait_until.to_rfc3339())]
    EmailSendingRateLimited { wait_until: DateTime },
}

pub async fn account_recover_send_validation_email(
    cmds: &AnonymousServerCmds,
    email: EmailAddress,
) -> Result<(), AccountRecoverSendValidationEmailError> {
    use libparsec_protocol::anonymous_server_cmds::v5::account_recover_send_validation_email::{
        Rep, Req,
    };

    match cmds.send(Req { email }).await? {
        Rep::Ok => Ok(()),

        Rep::EmailRecipientRefused => {
            Err(AccountRecoverSendValidationEmailError::EmailRecipientRefused)
        }
        Rep::EmailServerUnavailable => {
            Err(AccountRecoverSendValidationEmailError::EmailServerUnavailable)
        }
        Rep::EmailSendingRateLimited { wait_until } => {
            Err(AccountRecoverSendValidationEmailError::EmailSendingRateLimited { wait_until })
        }
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AccountRecoverProceedError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Invalid validation code")]
    InvalidValidationCode,
    #[error("Send validation email required")]
    SendValidationEmailRequired,
}

pub async fn account_recover_proceed(
    cmds: &AnonymousServerCmds,
    validation_code: ValidationCode,
    email: EmailAddress,
    auth_method_strategy: AccountAuthMethodStrategy<'_>,
) -> Result<(), AccountRecoverProceedError> {
    use libparsec_protocol::anonymous_server_cmds::v5::account_recover_proceed::{Rep, Req};

    let (new_auth_method_password_algorithm, new_auth_method_keys) =
        auth_method_strategy.derive_keys(&email);

    let vault_key = SecretKey::generate();
    let vault_key_access = AccountVaultKeyAccess { vault_key };
    let vault_key_access_bytes =
        vault_key_access.dump_and_encrypt(&new_auth_method_keys.secret_key);

    match cmds
        .send(Req {
            email,
            validation_code,
            new_vault_key_access: vault_key_access_bytes.into(),
            new_auth_method_id: new_auth_method_keys.id,
            new_auth_method_mac_key: new_auth_method_keys.mac_key,
            new_auth_method_password_algorithm,
        })
        .await?
    {
        Rep::Ok => Ok(()),
        Rep::InvalidValidationCode => Err(AccountRecoverProceedError::InvalidValidationCode),
        Rep::SendValidationEmailRequired => {
            Err(AccountRecoverProceedError::SendValidationEmailRequired)
        }
        bad_rep @ (
            // Auth method ID is an UUID derived from the master secret, so it should
            // statically never collide with an existing one.
            Rep::AuthMethodIdAlreadyExists
            | Rep::UnknownStatus { .. }
        ) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/account_recover/mod.rs"]
mod tests;
