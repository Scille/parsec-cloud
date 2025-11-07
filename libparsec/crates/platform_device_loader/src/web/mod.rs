// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub(crate) mod error;
pub(crate) mod internal;
pub(crate) mod wrapper;

use std::path::{Path, PathBuf};

use libparsec_types::prelude::*;

use crate::{
    AccountVaultOperationsFetchOpaqueKeyError, ArchiveDeviceError, AvailableDevice,
    DeviceAccessStrategy, DeviceSaveStrategy, ListAvailableDeviceError, ListPkiLocalPendingError,
    LoadCiphertextKeyError, ReadFileError, RemoveDeviceError, SaveDeviceError,
    SavePkiLocalPendingError, UpdateDeviceError,
};
use internal::Storage;

/*
 * List available devices
 */

pub(super) async fn list_available_devices(
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

pub(super) async fn read_file(file: &Path) -> Result<Vec<u8>, ReadFileError> {
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

pub(super) async fn load_ciphertext_key(
    access: &DeviceAccessStrategy,
    device_file: &DeviceFile,
) -> Result<SecretKey, LoadCiphertextKeyError> {
    // Don't do `match (access, device_file)` since we would end up with a catch-all
    // `(_, _) => return <error>` condition that would prevent this code from breaking
    // whenever a new variant is introduced (hence hiding the fact this code has
    // to be updated).
    match access {
        DeviceAccessStrategy::Password { password, .. } => {
            if let DeviceFile::Password(device) = device_file {
                let key = device
                    .algorithm
                    .compute_secret_key(password)
                    .map_err(|_| LoadCiphertextKeyError::InvalidData)?;

                Ok(key)
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }

        DeviceAccessStrategy::AccountVault { operations, .. } => {
            if let DeviceFile::AccountVault(device) = device_file {
                let ciphertext_key = operations
                    .fetch_opaque_key(device.ciphertext_key_id)
                    .await
                    .map_err(|err| match err {
                        AccountVaultOperationsFetchOpaqueKeyError::BadVaultKeyAccess(_)
                        | AccountVaultOperationsFetchOpaqueKeyError::UnknownOpaqueKey
                        | AccountVaultOperationsFetchOpaqueKeyError::CorruptedOpaqueKey => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed(err.into())
                        }
                        AccountVaultOperationsFetchOpaqueKeyError::Offline(err) => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline(err)
                        }
                        AccountVaultOperationsFetchOpaqueKeyError::Internal(err) => {
                            LoadCiphertextKeyError::Internal(err)
                        }
                    })?;
                Ok(ciphertext_key)
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }

        DeviceAccessStrategy::Keyring { .. } => panic!("Keyring not supported on Web"),
        DeviceAccessStrategy::Smartcard { .. } => panic!("Smartcard not supported on Web"),
    }
}

pub(super) async fn save_device(
    strategy: &DeviceSaveStrategy,
    device: &LocalDevice,
    created_on: DateTime,
    key_file: PathBuf,
) -> Result<AvailableDevice, SaveDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(SaveDeviceError::StorageNotAvailable);
    };
    storage
        .save_device(strategy, key_file, device, created_on)
        .await
        .map_err(Into::into)
}

pub(super) async fn update_device(
    device: &LocalDevice,
    created_on: DateTime,
    current_key_file: &Path,
    new_strategy: &DeviceSaveStrategy,
    new_key_file: &Path,
) -> Result<AvailableDevice, UpdateDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(UpdateDeviceError::StorageNotAvailable);
    };

    let available_device = storage
        .save_device(
            new_strategy,
            new_key_file.to_path_buf(),
            &device,
            created_on,
        )
        .await?;

    if current_key_file != new_key_file {
        if let Err(err) = storage.remove_device(current_key_file).await {
            log::warn!("Cannot remove old key file {current_key_file:?}: {err}");
        }
    }

    Ok(available_device)
}

pub(super) async fn archive_device(device_path: &Path) -> Result<(), ArchiveDeviceError> {
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

pub(super) async fn remove_device(device_path: &Path) -> Result<(), RemoveDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(RemoveDeviceError::StorageNotAvailable);
    };
    storage.remove_device(device_path).await.map_err(Into::into)
}

pub(super) async fn save_pki_local_pending(
    local_pending: PKILocalPendingEnrollment,
    local_file: PathBuf,
) -> Result<(), SavePkiLocalPendingError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(SavePkiLocalPendingError::StorageNotAvailable);
    };
    storage
        .save_pki_local_pending(local_file, local_pending)
        .await
        .map_err(Into::into)
}

pub(super) async fn list_pki_local_pending(
    config_dir: &Path,
) -> Result<Vec<PKILocalPendingEnrollment>, ListPkiLocalPendingError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(ListPkiLocalPendingError::StorageNotAvailable);
    };
    storage
        .list_pki_local_pending(config_dir)
        .await
        .inspect(|v| {
            log::trace!("Found the following devices: {v:?}");
        })
        .inspect_err(|e| {
            log::error!("Failed to list available devices: {e}");
        })
        .map_err(|e| ListPkiLocalPendingError::Internal(anyhow::anyhow!("{e}")))
}
