// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

pub use libparsec_account::{
    AccountCreateAuthMethodError, AccountCreateError, AccountCreateSendValidationEmailError,
    AccountDeleteProceedError, AccountDeleteSendValidationEmailError,
    AccountDisableAuthMethodError, AccountFetchOpaqueKeyFromVaultError,
    AccountListAuthMethodsError, AccountListInvitationsError, AccountListRegistrationDevicesError,
    AccountLoginError, AccountRecoverProceedError, AccountRecoverSendValidationEmailError,
    AccountRegisterNewDeviceError, AccountUploadOpaqueKeyInVaultError, AuthMethodInfo,
};
use libparsec_client_connection::{AnonymousAccountCmds, ConnectionError, ProxyConfig};
use libparsec_types::prelude::*;

use crate::handle::{
    borrow_from_handle, register_handle, take_and_close_handle, Handle, HandleItem,
};

// TODO: must reimplement this structure since the bindings doesn't support
//       structs with lifetime.
pub enum AccountAuthMethodStrategy {
    // Use struct style instead of tuple to declare the single params, since
    // it is more convenient to use in the bindings.
    MasterSecret { master_secret: KeyDerivation },
    Password { password: Password },
}

impl AccountAuthMethodStrategy {
    pub fn as_real(&self) -> libparsec_account::AccountAuthMethodStrategy<'_> {
        match self {
            AccountAuthMethodStrategy::MasterSecret { master_secret } => {
                libparsec_account::AccountAuthMethodStrategy::MasterSecret(master_secret)
            }
            AccountAuthMethodStrategy::Password { password } => {
                libparsec_account::AccountAuthMethodStrategy::Password(password)
            }
        }
    }
}

// TODO: must reimplement this structure since the bindings doesn't support
//       structs with lifetime.
pub enum AccountLoginStrategy {
    // Use struct style instead of tuple to declare the single params, since
    // it is more convenient to use in the bindings.
    MasterSecret {
        master_secret: KeyDerivation,
    },
    Password {
        email: EmailAddress,
        password: Password,
    },
}

impl AccountLoginStrategy {
    pub fn as_real(&self) -> libparsec_account::AccountLoginStrategy<'_> {
        match self {
            AccountLoginStrategy::MasterSecret { master_secret } => {
                libparsec_account::AccountLoginStrategy::MasterSecret(master_secret)
            }
            AccountLoginStrategy::Password { email, password } => {
                libparsec_account::AccountLoginStrategy::Password { email, password }
            }
        }
    }
}

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
    // Note we do the validation code parsing directly within the function. This is because:
    // - The validation code is expected to be provided by the user, so having an incorrect
    //  validation code is a to-be-expected error.
    // - Using a `ValidationCode` as parameter would mean the parsing is done in the bindings,
    //  where a Javascript type error is thrown, however this behavior is only supposed to
    //  occur for unexpected errors (e.g. passing a dummy value as DeviceID, since the GUI
    //  is only supposed to pass a DevicesID that have previously been obtained from libparsec).
    validation_code: &str,
    email: EmailAddress,
) -> Result<(), AccountCreateError> {
    let validation_code: ValidationCode = validation_code
        .parse()
        .map_err(|_| AccountCreateError::InvalidValidationCode)?;
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;
    libparsec_account::Account::create_2_check_validation_code(&cmds, validation_code, email).await
}

pub async fn account_create_3_proceed(
    config_dir: &Path,
    addr: ParsecAddr,
    // Note we do the validation code parsing directly within the function. This is because:
    // - The validation code is expected to be provided by the user, so having an incorrect
    //  validation code is a to-be-expected error.
    // - Using a `ValidationCode` as parameter would mean the parsing is done in the bindings,
    //  where a Javascript type error is thrown, however this behavior is only supposed to
    //  occur for unexpected errors (e.g. passing a dummy value as DeviceID, since the GUI
    //  is only supposed to pass a DevicesID that have previously been obtained from libparsec).
    validation_code: &str,
    human_handle: HumanHandle,
    auth_method_strategy: AccountAuthMethodStrategy,
) -> Result<(), AccountCreateError> {
    let validation_code: ValidationCode = validation_code
        .parse()
        .map_err(|_| AccountCreateError::InvalidValidationCode)?;
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;
    libparsec_account::Account::create_3_proceed(
        &cmds,
        validation_code,
        human_handle,
        auth_method_strategy.as_real(),
    )
    .await
}

pub async fn account_create_auth_method(
    account: Handle,
    auth_method_strategy: AccountAuthMethodStrategy,
) -> Result<(), AccountCreateAuthMethodError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    account
        .create_auth_method(auth_method_strategy.as_real())
        .await
}

#[derive(Debug, thiserror::Error)]
pub enum AccountGetInUseAuthMethodError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub fn account_get_in_use_auth_method(
    account: Handle,
) -> Result<AccountAuthMethodID, AccountGetInUseAuthMethodError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    Ok(account.auth_method_id())
}

pub async fn account_list_auth_methods(
    account: Handle,
) -> Result<Vec<AuthMethodInfo>, AccountListAuthMethodsError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    account.list_auth_methods().await
}

pub async fn account_disable_auth_method(
    account: Handle,
    auth_method_id: AccountAuthMethodID,
) -> Result<(), AccountDisableAuthMethodError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    account.disable_auth_method(auth_method_id).await
}

pub async fn account_login(
    config_dir: PathBuf,
    addr: ParsecAddr,
    login_strategy: AccountLoginStrategy,
) -> Result<Handle, AccountLoginError> {
    let proxy = ProxyConfig::default();
    let account =
        libparsec_account::Account::login(config_dir, proxy, addr, login_strategy.as_real())
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
) -> Result<
    Vec<(
        ParsecInvitationAddr,
        OrganizationID,
        InvitationToken,
        InvitationType,
    )>,
    AccountListInvitationsError,
> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;
    let invitations = account.list_invitations().await?;
    Ok(invitations
        .into_iter()
        .map(|addr| {
            let organization_id = addr.organization_id().to_owned();
            let token = addr.token();
            let invitation_type = addr.invitation_type();
            (addr, organization_id, token, invitation_type)
        })
        .collect())
}

pub async fn account_fetch_opaque_key_from_vault(
    account: Handle,
    key_id: AccountVaultItemOpaqueKeyID,
) -> Result<SecretKey, AccountFetchOpaqueKeyFromVaultError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    account.fetch_opaque_key_from_vault(key_id).await
}

pub async fn account_upload_opaque_key_in_vault(
    account: Handle,
) -> Result<(AccountVaultItemOpaqueKeyID, SecretKey), AccountUploadOpaqueKeyInVaultError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    account.upload_opaque_key_in_vault().await
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
    // Note we do the validation code parsing directly within the function. This is because:
    // - The validation code is expected to be provided by the user, so having an incorrect
    //  validation code is a to-be-expected error.
    // - Using a `ValidationCode` as parameter would mean the parsing is done in the bindings,
    //  where a Javascript type error is thrown, however this behavior is only supposed to
    //  occur for unexpected errors (e.g. passing a dummy value as DeviceID, since the GUI
    //  is only supposed to pass a DevicesID that have previously been obtained from libparsec).
    validation_code: &str,
) -> Result<(), AccountDeleteProceedError> {
    let account_handle = account;
    let account = borrow_account(account_handle)?;

    let validation_code: ValidationCode = validation_code
        .parse()
        .map_err(|_| AccountDeleteProceedError::InvalidValidationCode)?;
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
    // Note we do the validation code parsing directly within the function. This is because:
    // - The validation code is expected to be provided by the user, so having an incorrect
    //  validation code is a to-be-expected error.
    // - Using a `ValidationCode` as parameter would mean the parsing is done in the bindings,
    //  where a Javascript type error is thrown, however this behavior is only supposed to
    //  occur for unexpected errors (e.g. passing a dummy value as DeviceID, since the GUI
    //  is only supposed to pass a DevicesID that have previously been obtained from libparsec).
    validation_code: &str,
    email: EmailAddress,
    auth_method_strategy: AccountAuthMethodStrategy,
) -> Result<(), AccountRecoverProceedError> {
    let validation_code: ValidationCode = validation_code
        .parse()
        .map_err(|_| AccountRecoverProceedError::InvalidValidationCode)?;
    let cmds = AnonymousAccountCmds::new(config_dir, addr, ProxyConfig::default())?;
    libparsec_account::Account::recover_2_proceed(
        &cmds,
        validation_code,
        email,
        auth_method_strategy.as_real(),
    )
    .await
}
