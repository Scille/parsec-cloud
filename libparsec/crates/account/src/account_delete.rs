// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::Account;

#[derive(Debug, thiserror::Error)]
pub enum AccountDeleteSendValidationEmailError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Email recipient refused")]
    EmailRecipientRefused,
    #[error("Email server unavailable")]
    EmailServerUnavailable,
}

pub(super) async fn account_delete_send_validation_email(
    account: &Account,
) -> Result<(), AccountDeleteSendValidationEmailError> {
    use libparsec_protocol::authenticated_account_cmds::v5::account_delete_send_validation_email::{Req, Rep};

    match account.cmds.send(Req {}).await? {
        Rep::Ok => Ok(()),

        Rep::EmailRecipientRefused => {
            Err(AccountDeleteSendValidationEmailError::EmailRecipientRefused)
        }
        Rep::EmailServerUnavailable => {
            Err(AccountDeleteSendValidationEmailError::EmailServerUnavailable)
        }
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AccountDeleteProceedError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Invalid validation code")]
    InvalidValidationCode,
    #[error("Send validation code required")]
    SendValidationEmailRequired,
}

pub(super) async fn account_delete_proceed(
    account: &Account,
    validation_code: ValidationCode,
) -> Result<(), AccountDeleteProceedError> {
    use libparsec_protocol::authenticated_account_cmds::v5::account_delete_proceed::{Rep, Req};

    match account.cmds.send(Req { validation_code }).await? {
        Rep::Ok => Ok(()),
        Rep::InvalidValidationCode => Err(AccountDeleteProceedError::InvalidValidationCode),
        Rep::SendValidationEmailRequired => {
            Err(AccountDeleteProceedError::SendValidationEmailRequired)
        }
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/account_delete/mod.rs"]
mod tests;
