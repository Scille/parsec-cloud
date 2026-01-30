// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_filesystem::{load_file, remove_file, LoadFileError};
use std::path::Path;

use libparsec_types::prelude::*;

#[cfg(feature = "test-with-testbed")]
use crate::testbed;
use crate::{
    decrypt_device_file, load, load_available_device, AvailableDevice, DecryptDeviceFileError,
    DeviceAccessStrategy, DeviceSaveStrategy, LoadAvailableDeviceError, LoadCiphertextKeyError,
    RemoteOperationServer, SaveDeviceError,
};
#[derive(Debug, thiserror::Error)]
pub enum UpdateDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error("Path is invalid")]
    InvalidPath,
    #[error("Cannot deserialize file content")]
    InvalidData,
    #[error("Failed to decrypt file content")]
    DecryptionFailed,
    /// Note only a subset of load/save strategies requires server access to
    /// fetch/upload an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("No response from {server} server: {error}")]
    // We don't use `ConnectionError` here since this type only corresponds to
    // an answer from the Parsec server and here any arbitrary server may have
    // been (unsuccessfully) requested (e.g. OpenBao server).
    RemoteOpaqueKeyOperationOffline {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    /// Note only a subset of load/save strategies requires server access to
    /// fetch/upload an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("{server} server opaque key operation failed: {error}")]
    RemoteOpaqueKeyOperationFailed {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<SaveDeviceError> for UpdateDeviceError {
    fn from(value: SaveDeviceError) -> Self {
        match value {
            SaveDeviceError::RemoteOpaqueKeyUploadOffline { server, error } => {
                UpdateDeviceError::RemoteOpaqueKeyOperationOffline { server, error }
            }
            SaveDeviceError::RemoteOpaqueKeyUploadFailed { server, error } => {
                UpdateDeviceError::RemoteOpaqueKeyOperationFailed { server, error }
            }
            SaveDeviceError::Internal(error) => UpdateDeviceError::Internal(error),
            SaveDeviceError::StorageNotAvailable => UpdateDeviceError::StorageNotAvailable,
            SaveDeviceError::InvalidPath => UpdateDeviceError::InvalidPath,
        }
    }
}

impl From<LoadFileError> for UpdateDeviceError {
    fn from(value: LoadFileError) -> Self {
        match value {
            LoadFileError::StorageNotAvailable => UpdateDeviceError::StorageNotAvailable,
            LoadFileError::NotAFile
            | LoadFileError::InvalidParent
            | LoadFileError::InvalidPath
            | LoadFileError::NotFound => UpdateDeviceError::InvalidPath,
            LoadFileError::Internal(error) => UpdateDeviceError::Internal(error),
        }
    }
}
pub async fn update_device(
    device: &LocalDevice,
    created_on: DateTime,
    current_key_file: &Path,
    new_strategy: &DeviceSaveStrategy,
    new_key_file: &Path,
) -> Result<AvailableDevice, UpdateDeviceError> {
    let available_device =
        super::save_device(new_strategy, device, created_on, new_key_file.to_path_buf()).await?;

    if current_key_file != new_key_file {
        if let Err(err) = remove_file(current_key_file).await {
            log::warn!("Cannot remove old key file {current_key_file:?}: {err}");
        }
    }

    Ok(available_device)
}

/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn update_device_change_authentication(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    current_access: &DeviceAccessStrategy,
    new_strategy: &DeviceSaveStrategy,
    new_key_file: &Path,
) -> Result<AvailableDevice, UpdateDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) =
        testbed::maybe_update_device(config_dir, current_access, new_strategy, new_key_file, None)
    {
        return result.map(|(available_device, _)| available_device);
    }

    let current_key_file = current_access.key_file();

    // 1. Load the current device keys file...

    let file_content = load_file(current_key_file).await?;
    let device_file =
        DeviceFile::load(&file_content).map_err(|_| UpdateDeviceError::InvalidData)?;
    let ciphertext_key = load::load_ciphertext_key(current_access, &device_file)
        .await
        .map_err(|err| match err {
            LoadCiphertextKeyError::InvalidData => UpdateDeviceError::InvalidData,
            LoadCiphertextKeyError::DecryptionFailed => UpdateDeviceError::DecryptionFailed,
            LoadCiphertextKeyError::Internal(err) => UpdateDeviceError::Internal(err),
            LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline { server, error } => {
                UpdateDeviceError::RemoteOpaqueKeyOperationOffline { server, error }
            }
            LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed { server, error } => {
                UpdateDeviceError::RemoteOpaqueKeyOperationFailed { server, error }
            }
        })?;
    let device = decrypt_device_file(&device_file, &ciphertext_key).map_err(|err| match err {
        DecryptDeviceFileError::Decrypt(_) => UpdateDeviceError::DecryptionFailed,
        DecryptDeviceFileError::Load(_) => UpdateDeviceError::InvalidData,
    })?;

    // 2. ...and ask to overwrite it

    update_device(
        &device,
        device_file.created_on(),
        current_key_file,
        new_strategy,
        new_key_file,
    )
    .await
}

/// Note `config_dir` is only used as discriminant for the testbed here
///
/// Returns the old server address
pub async fn update_device_overwrite_server_addr(
    config_dir: &Path,
    strategy: &DeviceAccessStrategy,
    new_server_addr: ParsecAddr,
) -> Result<ParsecAddr, UpdateDeviceError> {
    let key_file = strategy.key_file();
    let available_device = load_available_device(config_dir, key_file.to_owned())
        .await
        .map_err(|err| match err {
            LoadAvailableDeviceError::StorageNotAvailable => UpdateDeviceError::StorageNotAvailable,
            LoadAvailableDeviceError::InvalidPath(_) => UpdateDeviceError::InvalidPath,
            LoadAvailableDeviceError::InvalidData => UpdateDeviceError::InvalidData,
            LoadAvailableDeviceError::Internal(err) => UpdateDeviceError::Internal(err),
        })?;
    let save_strategy = strategy
        .clone()
        .into_save_strategy(available_device.ty)
        // Return error if the file exists but doesn't correspond to the save strategy
        .ok_or(UpdateDeviceError::InvalidData)?;

    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_update_device(
        config_dir,
        strategy,
        &save_strategy,
        key_file,
        Some(new_server_addr.clone()),
    ) {
        return result.map(|(_, old_server_addr)| old_server_addr);
    }

    // 1. Load the current device keys file...

    let file_content = load_file(key_file).await?;
    let device_file =
        DeviceFile::load(&file_content).map_err(|_| UpdateDeviceError::InvalidData)?;
    let ciphertext_key = load::load_ciphertext_key(strategy, &device_file)
        .await
        .map_err(|err| match err {
            LoadCiphertextKeyError::InvalidData => UpdateDeviceError::InvalidData,
            LoadCiphertextKeyError::DecryptionFailed => UpdateDeviceError::DecryptionFailed,
            LoadCiphertextKeyError::Internal(err) => UpdateDeviceError::Internal(err),
            LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline { server, error } => {
                UpdateDeviceError::RemoteOpaqueKeyOperationOffline { server, error }
            }
            LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed { server, error } => {
                UpdateDeviceError::RemoteOpaqueKeyOperationFailed { server, error }
            }
        })?;
    let mut device =
        decrypt_device_file(&device_file, &ciphertext_key).map_err(|err| match err {
            DecryptDeviceFileError::Decrypt(_) => UpdateDeviceError::DecryptionFailed,
            DecryptDeviceFileError::Load(_) => UpdateDeviceError::InvalidData,
        })?;

    let old_server_addr: ParsecAddr = device.organization_addr.clone().into();
    device.organization_addr = ParsecOrganizationAddr::new(
        new_server_addr,
        device.organization_addr.organization_id().to_owned(),
        device.organization_addr.root_verify_key().to_owned(),
    );

    // 2. ...and ask to overwrite it

    update_device(
        &device,
        device_file.created_on(),
        key_file,
        &save_strategy,
        key_file,
    )
    .await?;

    Ok(old_server_addr)
}
