// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_types::{EmailValidationToken, ParsecAnonymousAccountAddr, Password};

pub use libparsec_account::{AccountCreateProceedError, AccountSendEmailValidationTokenError};

pub async fn account_send_email_validation_token(
    email: &str,
    config_dir: &Path,
    addr: ParsecAnonymousAccountAddr,
) -> Result<(), AccountSendEmailValidationTokenError> {
    libparsec_account::account_send_email_validation_token(email, config_dir, addr).await
}

#[allow(clippy::too_many_arguments)]
pub async fn account_create_proceed(
    human_label: String,
    validation_token: EmailValidationToken,
    config_dir: &Path,
    addr: ParsecAnonymousAccountAddr,
    password: Password,
) -> Result<(), AccountCreateProceedError> {
    libparsec_account::account_create_proceed(
        human_label,
        validation_token,
        config_dir,
        addr,
        password,
    )
    .await
}
