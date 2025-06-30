// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

pub use libparsec_account::{
    AccountCreateError, AccountCreateSendValidationEmailError,
    AccountFetchRegistrationDevicesError, AccountListInvitationsError,
    AccountLoginWithMasterSecretError, AccountLoginWithPasswordError,
    AccountRegisterNewDeviceError,
};
use libparsec_client_connection::{AnonymousAccountCmds, ProxyConfig};
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
    password: Password,
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
    password: Password,
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

pub async fn account_fetch_registration_devices(
    account: Handle,
) -> Result<(), AccountFetchRegistrationDevicesError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;
    account.fetch_registration_devices().await
}

#[derive(Debug, thiserror::Error)]
pub enum AccountListRegistrationDevicesError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub fn account_list_registration_devices(
    account: Handle,
) -> Result<HashSet<(OrganizationID, UserID)>, AccountListRegistrationDevicesError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;
    Ok(account.list_registration_devices())
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
