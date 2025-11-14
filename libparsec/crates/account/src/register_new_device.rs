// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client::RegisterNewDeviceError;
use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_platform_device_loader::{
    get_default_key_file, save_device, AvailableDevice, DeviceSaveStrategy, SaveDeviceError,
};
use libparsec_types::prelude::*;

use super::{fetch_vault_items, Account, FetchVaultItemsError};

#[derive(Debug, thiserror::Error)]
pub enum AccountRegisterNewDeviceError {
    #[error("Cannot decrypt the vault key access returned by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("No registration device exists for this organization/user IDs")]
    UnknownRegistrationDevice,
    #[error("The registration device appears to be corrupted")]
    CorruptedRegistrationDevice,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    /// Note only a subset of save strategies requires server access to
    /// upload an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("Remote opaque key upload failed from server rejection: {0}")]
    // We don't use `ConnectionError` here since this type only corresponds to
    // an answer from the Parsec server and here any arbitrary server may have
    // been (unsuccessfully) requested (e.g. OpenBao server).
    RemoteOpaqueKeyUploadOffline(anyhow::Error),
    /// Note only a subset of save strategies requires server access to
    /// upload an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("Remote opaque key upload failed: {0}")]
    RemoteOpaqueKeyUploadFailed(anyhow::Error),
}

pub(super) async fn account_register_new_device(
    account: &Account,
    organization_id: OrganizationID,
    user_id: UserID,
    new_device_label: DeviceLabel,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, AccountRegisterNewDeviceError> {
    // 1. Retrieve from the server the vault item containing the registration device

    let (vault_key, vault_items) = fetch_vault_items(account).await.map_err(|err| match err {
        FetchVaultItemsError::BadVaultKeyAccess(err) => {
            AccountRegisterNewDeviceError::BadVaultKeyAccess(err)
        }
        FetchVaultItemsError::Offline(err) => AccountRegisterNewDeviceError::Offline(err),
        FetchVaultItemsError::Internal(err) => AccountRegisterNewDeviceError::Internal(err),
    })?;

    let encrypted_data = vault_items
        .into_iter()
        .find_map(|item| match item {
            AccountVaultItem::RegistrationDevice(item)
                if item.organization_id == organization_id && item.user_id == user_id =>
            {
                Some(item.encrypted_data)
            }
            _ => None,
        })
        .ok_or(AccountRegisterNewDeviceError::UnknownRegistrationDevice)?;

    // 2. Decrypt the encrypted data and verify it actually corresponds to our registration device

    let unsecure_registration_device = LocalDevice::decrypt_and_load(&encrypted_data, &vault_key)
        .map(Arc::new)
        .map_err(|_| AccountRegisterNewDeviceError::CorruptedRegistrationDevice)?;

    if *unsecure_registration_device.organization_id() != organization_id
        || unsecure_registration_device.user_id != user_id
    {
        return Err(AccountRegisterNewDeviceError::CorruptedRegistrationDevice);
    }

    let registration_device = unsecure_registration_device;

    // 3. Generate the new device (device ID, signing key etc.)

    let new_device =
        LocalDevice::from_existing_device_for_user(&registration_device, new_device_label);

    // We first register the new device to the server, and only then save it on disk.
    // This is to ensure the device found on disk are always valid, however the drawback
    // is it can lead to losing the device if something goes wrong between the two steps.
    //
    // This is considered acceptable given 1) the error window is small and
    // 2) if this occurs, the user can always re-import the recovery device.

    // 3. Upload the new device on the server

    let cmds = AuthenticatedCmds::from_client(
        account.cmds.client().to_owned(),
        &account.config_dir,
        registration_device.clone(),
    );
    libparsec_client::register_new_device(
        &cmds,
        &new_device,
        DevicePurpose::Standard,
        &registration_device,
    )
    .await
    .map_err(|err| match err {
        RegisterNewDeviceError::Offline(err) => AccountRegisterNewDeviceError::Offline(err),
        RegisterNewDeviceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        } => AccountRegisterNewDeviceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        },
        RegisterNewDeviceError::Internal(err) => AccountRegisterNewDeviceError::Internal(err),
    })?;

    // From now on, our new device has an actual existence !

    // 4. Save the new device on disk

    let key_file = get_default_key_file(&account.config_dir, new_device.device_id);

    let new_available_device =
        save_device(&account.config_dir, &save_strategy, &new_device, key_file)
            .await
            .map_err(|err| match err {
                SaveDeviceError::StorageNotAvailable => {
                    AccountRegisterNewDeviceError::StorageNotAvailable
                }
                SaveDeviceError::InvalidPath(error) => {
                    AccountRegisterNewDeviceError::InvalidPath(error)
                }
                SaveDeviceError::Internal(error) => AccountRegisterNewDeviceError::Internal(error),
                SaveDeviceError::RemoteOpaqueKeyUploadOffline(error) => {
                    AccountRegisterNewDeviceError::RemoteOpaqueKeyUploadOffline(error)
                }
                SaveDeviceError::RemoteOpaqueKeyUploadFailed(error) => {
                    AccountRegisterNewDeviceError::RemoteOpaqueKeyUploadFailed(error)
                }
            })?;

    Ok(new_available_device)
}

#[cfg(test)]
#[path = "../tests/unit/register_new_device.rs"]
mod tests;
