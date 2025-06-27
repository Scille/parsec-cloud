// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_client::ProxyConfig;
use libparsec_client_connection::AnonymousAccountCmds;
use libparsec_types::prelude::*;

pub use libparsec_account::{AccountCreateError, AccountCreateSendValidationEmailError};

pub async fn account_create_1_send_validation_email(
    config_dir: &Path,
    addr: ParsecAddr,
    email: EmailAddress,
) -> Result<(), AccountCreateSendValidationEmailError> {
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;
    libparsec_account::Account::create_1_send_validation_email(&cmds, email).await
}

pub async fn account_create_2_check_validation_code(
    config_dir: &Path,
    addr: ParsecAddr,
    validation_code: ValidationCode,
    email: EmailAddress,
) -> Result<(), AccountCreateError> {
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;
    libparsec_account::Account::create_2_check_validation_code(&cmds, validation_code, email).await
}

pub async fn account_create_3_proceed(
    config_dir: &Path,
    addr: ParsecAddr,
    validation_code: ValidationCode,
    human_handle: HumanHandle,
    password: Password,
) -> Result<(), AccountCreateError> {
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;
    libparsec_account::Account::create_3_proceed(&cmds, validation_code, human_handle, password)
        .await
}
