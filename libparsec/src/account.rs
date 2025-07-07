// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

pub use libparsec_account::{
    AccountCreateError, AccountCreateSendValidationEmailError, AccountDeleteProceedError,
    AccountDeleteSendValidationEmailError, AccountFetchDeviceFileAccountVaultKeyError,
    AccountListInvitationsError, AccountListRegistrationDevicesError,
    AccountLoginWithMasterSecretError, AccountLoginWithPasswordError, AccountRecoverProceedError,
    AccountRecoverSendValidationEmailError, AccountRegisterNewDeviceError,
    AccountUploadDeviceFileAccountVaultKeyError,
};
use libparsec_client_connection::{AnonymousAccountCmds, ConnectionError, ProxyConfig};
use libparsec_types::prelude::*;

use crate::handle::{
    borrow_from_handle, register_handle, take_and_close_handle, Handle, HandleItem,
};

fn borrow_account(account: Handle) -> anyhow::Result<Arc<libparsec_account::Account>> {
    borrow_from_handle(account, |x| match x {
        HandleItem::Account(account) => Some(account.clone()),
        _ => None,
    })
}

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
    password: &Password,
) -> Result<(), AccountCreateError> {
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;
    libparsec_account::Account::create_3_proceed(&cmds, validation_code, human_handle, password)
        .await
}

pub async fn account_login_with_master_secret(
    config_dir: PathBuf,
    addr: ParsecAddr,
    auth_method_master_secret: KeyDerivation,
) -> Result<Handle, AccountLoginWithMasterSecretError> {
    let proxy = ProxyConfig::default();
    let account = libparsec_account::Account::login_with_master_secret(
        config_dir,
        proxy,
        addr,
        auth_method_master_secret,
    )
    .await?;

    let handle = register_handle(HandleItem::Account(account.into()));

    Ok(handle)
}

pub async fn account_login_with_password(
    config_dir: PathBuf,
    addr: ParsecAddr,
    email: EmailAddress,
    password: &Password,
) -> Result<Handle, AccountLoginWithPasswordError> {
    let proxy = ProxyConfig::default();
    let account =
        libparsec_account::Account::login_with_password(config_dir, proxy, addr, email, password)
            .await?;

    let handle = register_handle(HandleItem::Account(account.into()));

    Ok(handle)
}

#[derive(Debug, thiserror::Error)]
pub enum AccountLogoutError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub fn account_logout(account: Handle) -> Result<(), AccountLogoutError> {
    let account_handle = account;

    let account = take_and_close_handle(account_handle, |x| match x {
        HandleItem::Account(account) => Ok(account),
        invalid => Err(invalid),
    })?;

    // Note that account is ref counted, so its destructor is not guaranteed to
    // be executed before this function returns (typically if some concurrent
    // operation is still in progress).
    drop(account);

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum AccountGetHumanHandleError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub fn account_get_human_handle(
    account: Handle,
) -> Result<HumanHandle, AccountGetHumanHandleError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    Ok(account.human_handle().to_owned())
}

pub async fn account_list_invitations(
    account: Handle,
) -> Result<Vec<(OrganizationID, InvitationToken, InvitationType)>, AccountListInvitationsError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;
    account.list_invitations().await
}

pub async fn account_fetch_device_file_account_vault_key(
    account: Handle,
    organization_id: &OrganizationID,
    device_id: DeviceID,
) -> Result<SecretKey, AccountFetchDeviceFileAccountVaultKeyError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    account
        .fetch_device_file_account_vault_key(organization_id, device_id)
        .await
}

pub async fn account_upload_device_file_account_vault_key(
    account: Handle,
    organization_id: OrganizationID,
    device_id: DeviceID,
) -> Result<SecretKey, AccountUploadDeviceFileAccountVaultKeyError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    let ciphertext_key = SecretKey::generate();

    account
        .upload_device_file_account_vault_key(organization_id, device_id, ciphertext_key.clone())
        .await?;

    Ok(ciphertext_key)
}

pub async fn account_list_registration_devices(
    account: Handle,
) -> Result<HashSet<(OrganizationID, UserID)>, AccountListRegistrationDevicesError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    account.list_registration_devices().await
}

pub async fn account_create_registration_device(
    account: Handle,
    existing_local_device_access: &DeviceAccessStrategy,
) -> Result<(), AccountCreateRegistrationDeviceError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    let existing_local_device = libparsec_platform_device_loader::load_device(
        account.config_dir(),
        existing_local_device_access,
    )
    .await?;

    account
        .create_registration_device(existing_local_device)
        .await?;

    Ok(())
}

pub async fn account_register_new_device(
    account: Handle,
    organization_id: OrganizationID,
    user_id: UserID,
    new_device_label: DeviceLabel,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, AccountRegisterNewDeviceError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    account
        .register_new_device(organization_id, user_id, new_device_label, save_strategy)
        .await
}

pub async fn account_delete_1_send_validation_email(
    account: Handle,
) -> Result<(), AccountDeleteSendValidationEmailError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    account.delete_1_send_validation_email().await
}

pub async fn account_delete_2_proceed(
    account: Handle,
    validation_code: ValidationCode,
) -> Result<(), AccountDeleteProceedError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    account.delete_2_proceed(validation_code).await
}

#[derive(Debug, thiserror::Error)]
pub enum AccountCreateRegistrationDeviceError {
    #[error("Cannot load device file: invalid path ({})", .0)]
    LoadDeviceInvalidPath(anyhow::Error),
    #[error("Cannot load device file: invalid data")]
    LoadDeviceInvalidData,
    #[error("Cannot load device file: decryption failed")]
    LoadDeviceDecryptionFailed,
    #[error("Cannot decrypt the vault key access return by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
}

impl From<libparsec_platform_device_loader::LoadDeviceError>
    for AccountCreateRegistrationDeviceError
{
    fn from(value: libparsec_platform_device_loader::LoadDeviceError) -> Self {
        use libparsec_platform_device_loader::LoadDeviceError;
        match value {
            e @ LoadDeviceError::StorageNotAvailable => Self::LoadDeviceInvalidPath(e.into()),
            LoadDeviceError::InvalidPath(err) => Self::LoadDeviceInvalidPath(err),
            LoadDeviceError::InvalidData => Self::LoadDeviceInvalidData,
            LoadDeviceError::DecryptionFailed => Self::LoadDeviceDecryptionFailed,
            LoadDeviceError::Internal(e) => Self::Internal(e),
        }
    }
}

impl From<libparsec_account::AccountCreateRegistrationDeviceError>
    for AccountCreateRegistrationDeviceError
{
    fn from(value: libparsec_account::AccountCreateRegistrationDeviceError) -> Self {
        match value {
            libparsec_account::AccountCreateRegistrationDeviceError::BadVaultKeyAccess(err) => {
                AccountCreateRegistrationDeviceError::BadVaultKeyAccess(err)
            }
            libparsec_account::AccountCreateRegistrationDeviceError::Offline(err) => {
                AccountCreateRegistrationDeviceError::Offline(err)
            }
            libparsec_account::AccountCreateRegistrationDeviceError::Internal(err) => {
                AccountCreateRegistrationDeviceError::Internal(err)
            }
            libparsec_account::AccountCreateRegistrationDeviceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => AccountCreateRegistrationDeviceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
        }
    }
}

pub async fn account_recover_1_send_validation_email(
    config_dir: &Path,
    addr: ParsecAddr,
    email: EmailAddress,
) -> Result<(), AccountRecoverSendValidationEmailError> {
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;
    libparsec_account::Account::recover_1_send_validation_email(&cmds, email).await
}

pub async fn account_recover_2_proceed(
    config_dir: &Path,
    addr: ParsecAddr,
    validation_code: ValidationCode,
    email: EmailAddress,
    new_password: &Password,
) -> Result<(), AccountRecoverProceedError> {
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;
    libparsec_account::Account::recover_2_proceed(&cmds, validation_code, email, new_password).await
}
