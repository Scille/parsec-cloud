// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_types::ParsecAddr;

pub use libparsec_account::{
    AccountCreateProceedError, AccountCreateStep, AccountSendEmailValidationTokenError,
};

pub async fn account_create_send_validation_email(
    email: &str,
    config_dir: &Path,
    addr: ParsecAddr,
) -> Result<(), AccountSendEmailValidationTokenError> {
    libparsec_account::account_create_send_validation_email(email, config_dir, addr).await
}

pub async fn account_create_proceed(
    step: AccountCreateStep,
    config_dir: &Path,
    addr: ParsecAddr,
) -> Result<(), AccountCreateProceedError> {
    libparsec_account::account_create_proceed(step, config_dir, addr).await
}
