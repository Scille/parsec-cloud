// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AnonymousServerCmds, ConnectionError};
use libparsec_types::prelude::*;

use crate::AccountAuthMethodStrategy;

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
    #[error("Too many email sent, must wait until {}", &wait_until.to_rfc3339())]
    EmailSendingRateLimited { wait_until: DateTime },
}

pub(super) async fn account_create_send_validation_email(
    cmds: &AnonymousServerCmds,
    email: EmailAddress,
) -> Result<(), AccountCreateSendValidationEmailError> {
    use libparsec_protocol::anonymous_server_cmds::v5::account_create_send_validation_email::{
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
        Rep::EmailSendingRateLimited { wait_until } => {
            Err(AccountCreateSendValidationEmailError::EmailSendingRateLimited { wait_until })
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
    #[error("Send validation email required")]
    SendValidationEmailRequired,
}

pub(super) enum AccountCreateStep<'a> {
    CheckCode {
        validation_code: ValidationCode,
        email: EmailAddress,
    },
    Proceed {
        validation_code: ValidationCode,
        human_handle: HumanHandle,
        auth_method_strategy: AccountAuthMethodStrategy<'a>,
    },
}

pub(super) async fn account_create(
    cmds: &AnonymousServerCmds,
    step: AccountCreateStep<'_>,
) -> Result<(), AccountCreateError> {
    use libparsec_protocol::anonymous_server_cmds::v5::account_create_proceed::{
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
            validation_code,
            human_handle,
            auth_method_strategy,
        } => {
            let (auth_method_password_algorithm, auth_method_keys) =
                auth_method_strategy.derive_keys(human_handle.email());

            let vault_key = SecretKey::generate();
            let vault_key_access = AccountVaultKeyAccess { vault_key };
            let vault_key_access_bytes =
                vault_key_access.dump_and_encrypt(&auth_method_keys.secret_key);

            ReqAccountCreateStep::Number1Create {
                validation_code,
                auth_method_mac_key: auth_method_keys.mac_key,
                auth_method_id: auth_method_keys.id,
                auth_method_password_algorithm,
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
        Rep::InvalidValidationCode => Err(AccountCreateError::InvalidValidationCode),
        Rep::SendValidationEmailRequired => Err(AccountCreateError::SendValidationEmailRequired),
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
#[path = "../tests/unit/account_create/mod.rs"]
mod tests;
