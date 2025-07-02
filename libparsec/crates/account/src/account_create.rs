// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AccountAuthMethod, AnonymousAccountCmds, ConnectionError};
use libparsec_types::prelude::*;

use super::{
    AUTH_METHOD_ID_DERIVATION_UUID, AUTH_METHOD_MAC_KEY_DERIVATION_UUID,
    AUTH_METHOD_SECRET_KEY_DERIVATION_UUID,
};

#[derive(Debug, thiserror::Error)]
pub enum AccountCreateSendValidationEmailError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Email recipient refused")]
    EmailRecipientRefused,
    #[error("Email server unavailable")]
    EmailServerUnavailable,
}

pub(super) async fn account_create_send_validation_email(
    cmds: &AnonymousAccountCmds,
    email: EmailAddress,
) -> Result<(), AccountCreateSendValidationEmailError> {
    use libparsec_protocol::anonymous_account_cmds::v5::account_create_send_validation_email::{
        Rep, Req,
    };

    match cmds.send(Req { email }).await? {
        Rep::Ok => Ok(()),

        Rep::EmailRecipientRefused => {
            Err(AccountCreateSendValidationEmailError::EmailRecipientRefused)
        }
        Rep::EmailServerUnavailable => {
            Err(AccountCreateSendValidationEmailError::EmailServerUnavailable)
        }
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AccountCreateError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Invalid validation code")]
    InvalidValidationCode,
    #[error("Send validation code required")]
    SendValidationEmailRequired,
    #[error("Auth method id already exists")]
    AuthMethodIdAlreadyExists,
}

pub(super) enum AccountCreateStep {
    CheckCode {
        validation_code: ValidationCode,
        email: EmailAddress,
    },
    Proceed {
        human_handle: HumanHandle,
        password: Password,
        validation_code: ValidationCode,
    },
}

pub(super) async fn account_create(
    cmds: &AnonymousAccountCmds,
    step: AccountCreateStep,
) -> Result<(), AccountCreateError> {
    use libparsec_protocol::anonymous_account_cmds::v5::account_create_proceed::{
        AccountCreateStep as ReqAccountCreateStep, Rep, Req,
    };

    let req_step = match step {
        AccountCreateStep::CheckCode {
            validation_code,
            email,
        } => ReqAccountCreateStep::Number0CheckCode {
            validation_code,
            email,
        },
        AccountCreateStep::Proceed {
            human_handle,
            password,
            validation_code,
        } => {
            let auth_method_password_algorithm = PasswordAlgorithm::generate_argon2id(
                PasswordAlgorithmSaltStrategy::DerivedFromEmail {
                    email: human_handle.email().as_ref(),
                },
            );
            let auth_method_master_secret = auth_method_password_algorithm
                .compute_key_derivation(&password)
                .expect("algorithm config is valid");

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

            ReqAccountCreateStep::Number1Create {
                validation_code,
                auth_method_hmac_key: auth_method.mac_key,
                auth_method_id: auth_method.id,
                auth_method_password_algorithm: Some(auth_method_password_algorithm.into()),
                human_handle,
                vault_key_access: vault_key_access_bytes.into(),
            }
        }
    };

    match cmds
        .send(Req {
            account_create_step: req_step,
        })
        .await?
    {
        Rep::Ok => Ok(()),
        Rep::AuthMethodIdAlreadyExists => Err(AccountCreateError::AuthMethodIdAlreadyExists),
        Rep::InvalidValidationCode => Err(AccountCreateError::InvalidValidationCode),
        Rep::SendValidationEmailRequired => Err(AccountCreateError::SendValidationEmailRequired),
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/account_create/mod.rs"]
mod tests;
