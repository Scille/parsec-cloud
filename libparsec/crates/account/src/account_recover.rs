// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AccountAuthMethod, AnonymousAccountCmds, ConnectionError};
use libparsec_types::prelude::*;

use super::{
    AUTH_METHOD_ID_DERIVATION_UUID, AUTH_METHOD_MAC_KEY_DERIVATION_UUID,
    AUTH_METHOD_SECRET_KEY_DERIVATION_UUID,
};

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
}

pub async fn account_recover_send_validation_email(
    cmds: &AnonymousAccountCmds,
    email: EmailAddress,
) -> Result<(), AccountRecoverSendValidationEmailError> {
    use libparsec_protocol::anonymous_account_cmds::v5::account_recover_send_validation_email::{
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
    #[error("Auth method id already exists")]
    AuthMethodIdAlreadyExists,
}

pub async fn account_recover_proceed(
    cmds: &AnonymousAccountCmds,
    validation_code: ValidationCode,
    email: EmailAddress,
    new_password: &Password,
) -> Result<(), AccountRecoverProceedError> {
    use libparsec_protocol::anonymous_account_cmds::v5::account_recover_proceed::{Rep, Req};

    let new_auth_method_password_algorithm =
        PasswordAlgorithm::generate_argon2id(PasswordAlgorithmSaltStrategy::DerivedFromEmail {
            email: email.as_ref(),
        });
    let new_auth_method_master_secret = new_auth_method_password_algorithm
        .compute_key_derivation(new_password)
        .expect("algorithm config is valid");

    let auth_method_secret_key = new_auth_method_master_secret
        .derive_secret_key_from_uuid(AUTH_METHOD_SECRET_KEY_DERIVATION_UUID);

    let new_auth_method = AccountAuthMethod {
        time_provider: TimeProvider::default(),
        id: AccountAuthMethodID::from(
            new_auth_method_master_secret.derive_uuid_from_uuid(AUTH_METHOD_ID_DERIVATION_UUID),
        ),
        mac_key: new_auth_method_master_secret
            .derive_secret_key_from_uuid(AUTH_METHOD_MAC_KEY_DERIVATION_UUID),
    };
    let vault_key = SecretKey::generate();

    let vault_key_access = AccountVaultKeyAccess { vault_key };

    match cmds
        .send(Req {
            email,
            validation_code,
            new_vault_key_access: vault_key_access
                .dump_and_encrypt(&auth_method_secret_key)
                .into(),
            new_auth_method_id: new_auth_method.id,
            new_auth_method_mac_key: new_auth_method.mac_key,
            new_auth_method_password_algorithm: Some(new_auth_method_password_algorithm.into()),
        })
        .await?
    {
        Rep::Ok => Ok(()),
        Rep::AuthMethodIdAlreadyExists => {
            Err(AccountRecoverProceedError::AuthMethodIdAlreadyExists)
        }
        Rep::InvalidValidationCode => Err(AccountRecoverProceedError::InvalidValidationCode),
        Rep::SendValidationEmailRequired => {
            Err(AccountRecoverProceedError::SendValidationEmailRequired)
        }
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/account_recover/mod.rs"]
mod tests;
