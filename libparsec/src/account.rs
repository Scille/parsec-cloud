// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_account::{
    AccountCreateWithPasswordProceedError, AccountSendEmailValidationTokenError,
};
use libparsec_protocol::anonymous_account_cmds::v5::account_create_with_password_proceed::PasswordAlgorithm;
use libparsec_types::{
    AccountAuthMethodID, Bytes, EmailValidationToken, ParsecAnonymousAccountAddr, SecretKey,
};

pub async fn account_send_email_validation_token(
    email: &str,
    config_dir: &Path,
    addr: ParsecAnonymousAccountAddr,
) -> Result<(), AccountSendEmailValidationTokenError> {
    libparsec_account::account_send_email_validation_token(email, config_dir, addr).await
}

#[allow(clippy::too_many_arguments)]
pub async fn account_create_with_password_proceed(
    config_dir: &Path,
    addr: ParsecAnonymousAccountAddr,
    auth_method_hmac_key: SecretKey,
    human_label: String,
    password_algorithm: PasswordAlgorithm,
    validation_token: EmailValidationToken,
    vault_key_access: Bytes,
    auth_method_id: AccountAuthMethodID,
) -> Result<(), AccountCreateWithPasswordProceedError> {
    libparsec_account::account_create_with_password_proceed(
        auth_method_hmac_key,
        human_label,
        password_algorithm,
        validation_token,
        vault_key_access,
        auth_method_id,
        config_dir,
        addr,
    )
    .await
}
