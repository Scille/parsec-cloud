// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub(crate) mod error;
pub(crate) mod internal;
pub(crate) mod wrapper;

use std::path::Path;

use libparsec_types::prelude::*;

use crate::{
    ArchiveDeviceError, ListAvailableDeviceError, LoadCiphertextKeyError, ReadFileError,
    RemoveDeviceError, SaveDeviceError, UpdateDeviceError,
};
use internal::Storage;

/*
 * List available devices
 */

pub async fn list_available_devices(
    config_dir: &Path,
) -> Result<Vec<AvailableDevice>, ListAvailableDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(ListAvailableDeviceError::StorageNotAvailable);
    };
    storage
        .list_available_devices(config_dir)
        .await
        .inspect(|v| {
            log::trace!("Found the following devices: {v:?}");
        })
        .inspect_err(|e| {
            log::error!("Failed to list available devices: {e}");
        })
        .map_err(|e| ListAvailableDeviceError::Internal(anyhow::anyhow!("{e}")))
}

/*
 * Save & load
 */

pub async fn read_file(file: &Path) -> Result<Vec<u8>, ReadFileError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(ReadFileError::StorageNotAvailable);
    };
    storage
        .read_file(file)
        .await
        .map_err(|err| ReadFileError::Internal(anyhow::anyhow!("{err}")))
}

pub async fn load_ciphertext_key(
    access: &DeviceAccessStrategy,
    device_file: &DeviceFile,
) -> Result<SecretKey, LoadCiphertextKeyError> {
    match (access, &device_file) {
        (DeviceAccessStrategy::Password { password, .. }, DeviceFile::Password(device)) => {
            let key = device
                .algorithm
                .compute_secret_key(password)
                .map_err(|_| LoadCiphertextKeyError::InvalidData)?;

            Ok(key)
        }

        (
            DeviceAccessStrategy::AccountVault { ciphertext_key, .. },
            DeviceFile::AccountVault(_),
        ) => Ok(ciphertext_key.clone()),

        (DeviceAccessStrategy::Keyring { .. }, _) => panic!("Keyring not supported on Web"),
        (DeviceAccessStrategy::Smartcard { .. }, _) => panic!("Smartcard not supported on Web"),

        _ => Err(LoadCiphertextKeyError::InvalidData),
    }
}

pub async fn save_device(
    access: &DeviceAccessStrategy,
    device: &LocalDevice,
    created_on: DateTime,
) -> Result<AvailableDevice, SaveDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(SaveDeviceError::StorageNotAvailable);
    };
    storage
        .save_device(access, device, created_on)
        .await
        .map_err(Into::into)
}

pub async fn update_device(
    device: &LocalDevice,
    created_on: DateTime,
    current_key_file: &Path,
    new_access: &DeviceAccessStrategy,
) -> Result<AvailableDevice, UpdateDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(UpdateDeviceError::StorageNotAvailable);
    };

    let available_device = storage.save_device(new_access, &device, created_on).await?;

    let new_key_file = new_access.key_file();

    if current_key_file != new_key_file {
        if let Err(err) = storage.remove_device(current_key_file).await {
            log::warn!("Cannot remove old key file {current_key_file:?}: {err}");
        }
    }

    Ok(available_device)
}

pub async fn archive_device(device_path: &Path) -> Result<(), ArchiveDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(ArchiveDeviceError::StorageNotAvailable);
    };
    storage
        .archive_device(device_path)
        .await
        .map_err(Into::into)
}

pub async fn remove_device(device_path: &Path) -> Result<(), RemoveDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(RemoveDeviceError::StorageNotAvailable);
    };
    storage.remove_device(device_path).await.map_err(Into::into)
}
