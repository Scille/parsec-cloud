// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub(crate) mod error;
pub(crate) mod internal;
pub(crate) mod wrapper;
use anyhow::anyhow;
use libparsec_types::prelude::*;
use std::path::{Path, PathBuf};

use crate::{
    AccountVaultOperationsFetchOpaqueKeyError, ArchiveDeviceError, AvailableDevice,
    AvailablePendingAsyncEnrollment, DeviceAccessStrategy, DeviceSaveStrategy,
    ListAvailableDeviceError, ListPendingAsyncEnrollmentsError, ListPkiLocalPendingError,
    LoadCiphertextKeyError, OpenBaoOperationsFetchOpaqueKeyError, RemoteOperationServer,
    RemoveDeviceError, SaveDeviceError, UpdateDeviceError,
};
use error::ListFileEntriesError;
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
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed {
                                server: RemoteOperationServer::ParsecAccount,
                                error: err.into(),
                            }
                        }
                        AccountVaultOperationsFetchOpaqueKeyError::Offline(_) => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline {
                                server: RemoteOperationServer::ParsecAccount,
                                error: err.into(),
                            }
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

        DeviceAccessStrategy::OpenBao { operations, .. } => {
            if let DeviceFile::OpenBao(device) = device_file {
                let ciphertext_key = operations
                    .fetch_opaque_key(device.openbao_ciphertext_key_path.clone())
                    .await
                    .map_err(|err| match err {
                        OpenBaoOperationsFetchOpaqueKeyError::BadURL(_)
                        | OpenBaoOperationsFetchOpaqueKeyError::BadServerResponse(_) => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed {
                                server: RemoteOperationServer::OpenBao,
                                error: err.into(),
                            }
                        }
                        OpenBaoOperationsFetchOpaqueKeyError::NoServerResponse(_) => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline {
                                server: RemoteOperationServer::OpenBao,
                                error: err.into(),
                            }
                        }
                    })?;
                Ok(ciphertext_key)
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }

        DeviceAccessStrategy::Keyring { .. } => panic!("Keyring not supported on Web"),
        DeviceAccessStrategy::PKI { .. } => panic!("PKI not supported on Web"),
    }
}

pub(super) async fn save_device(
    strategy: &DeviceSaveStrategy,
    device: &LocalDevice,
    created_on: DateTime,
    key_file: PathBuf,
) -> Result<AvailableDevice, SaveDeviceError> {
    Storage::save_device(strategy, key_file, device, created_on)
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
        return Err(UpdateDeviceError::Internal(anyhow!(
            "storage not available"
        ))); // TODO # 11995
    };

    let available_device = Storage::save_device(
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

pub(super) async fn list_pending_async_enrollments(
    pending_async_enrollments_dir: &Path,
) -> Result<Vec<AvailablePendingAsyncEnrollment>, ListPendingAsyncEnrollmentsError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(ListPendingAsyncEnrollmentsError::StorageNotAvailable);
    };

    let files = storage
        .list_file_entries(
            pending_async_enrollments_dir,
            crate::PENDING_ASYNC_ENROLLMENT_EXT,
        )
        .await
        .or_else(|err| match err {
            ListFileEntriesError::NotFound { .. } => Ok(vec![]),
            _ => {
                log::error!(
                    "Cannot list pending request files in {}: {err}",
                    pending_async_enrollments_dir.display()
                );
                Err(ListPendingAsyncEnrollmentsError::StorageNotAvailable)
            }
        })?;

    let mut items = Vec::with_capacity(files.len());
    for file in files {
        let outcome = file.read_to_end().await.ok().and_then(|raw| {
            super::load_pending_async_enrollment_as_available_frow_raw(file.path, &raw).ok()
        });
        if let Some(available) = outcome {
            items.push(available);
        }
    }

    Ok(items)
}
