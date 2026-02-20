// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_filesystem::{load_file, LoadFileError};

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec_types::prelude::*;

#[cfg(feature = "test-with-testbed")]
use crate::testbed;
use crate::{
    decrypt_device_file, device::load_ciphertext_key::load_ciphertext_key, AvailableDevice,
    AvailableDeviceType, DecryptDeviceFileError, DeviceAccessStrategy, LoadCiphertextKeyError,
    RemoteOperationServer,
};

#[derive(Debug, thiserror::Error)]
pub enum LoadAvailableDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error("Invalid path: {}", .0)]
    InvalidPath(anyhow::Error),
    #[error("Invalid data")]
    InvalidData,
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<LoadFileError> for LoadAvailableDeviceError {
    fn from(value: LoadFileError) -> Self {
        match value {
            LoadFileError::StorageNotAvailable => LoadAvailableDeviceError::StorageNotAvailable,
            LoadFileError::NotAFile
            | LoadFileError::InvalidParent
            | LoadFileError::InvalidPath
            | LoadFileError::NotFound => LoadAvailableDeviceError::InvalidPath(value.into()),
            LoadFileError::Internal(error) => LoadAvailableDeviceError::Internal(error),
        }
    }
}

/// Similar than `load_device`, but without the decryption part.
///
/// This is only needed for device file vault access that needs its
/// organization & device IDs to determine which account vault item
/// contains its decryption key.
///
/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn load_available_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    device_file: PathBuf,
) -> Result<AvailableDevice, LoadAvailableDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(all_available_devices) = testbed::maybe_list_available_devices(config_dir) {
        if let Some(result) = all_available_devices
            .into_iter()
            .find(|c_access| c_access.key_file_path == device_file)
        {
            return Ok(result);
        }
    }

    let file_content = load_file(&device_file).await?;

    load_available_device_from_blob(device_file, &file_content)
        .map_err(|_| LoadAvailableDeviceError::InvalidData)
}

#[derive(Debug, thiserror::Error)]
pub enum LoadDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error("Invalid path: {}", .0)]
    InvalidPath(anyhow::Error),
    #[error("Invalid data")]
    InvalidData,
    #[error("Decryption failed with the key obtained from TOTP challenge")]
    TOTPDecryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    /// Note only a subset of load strategies requires server access to
    /// fetch an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("No response from {server} server: {error}")]
    // We don't use `ConnectionError` here since this type only corresponds to
    // an answer from the Parsec server and here any arbitrary server may have
    // been (unsuccessfully) requested (e.g. OpenBao server).
    RemoteOpaqueKeyFetchOffline {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    /// Note only a subset of load strategies requires server access to
    /// fetch an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("{server} server opaque key fetch failed: {error}")]
    RemoteOpaqueKeyFetchFailed {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<LoadFileError> for LoadDeviceError {
    fn from(value: LoadFileError) -> Self {
        match value {
            LoadFileError::StorageNotAvailable => LoadDeviceError::StorageNotAvailable,
            LoadFileError::NotAFile
            | LoadFileError::InvalidParent
            | LoadFileError::InvalidPath
            | LoadFileError::NotFound => LoadDeviceError::InvalidPath(value.into()),
            LoadFileError::Internal(error) => LoadDeviceError::Internal(error),
        }
    }
}

/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn load_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    access: &DeviceAccessStrategy,
) -> Result<Arc<LocalDevice>, LoadDeviceError> {
    log::debug!("Loading device at {}", access.key_file.display());
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_load_device(config_dir, access) {
        return result;
    }

    let file_content = load_file(&access.key_file).await?;
    let device_file = DeviceFile::load(&file_content).map_err(|_| LoadDeviceError::InvalidData)?;
    let ciphertext_key =
        load_ciphertext_key(access, &device_file)
            .await
            .map_err(|err| match err {
                LoadCiphertextKeyError::InvalidData => LoadDeviceError::InvalidData,
                LoadCiphertextKeyError::DecryptionFailed => LoadDeviceError::DecryptionFailed,
                LoadCiphertextKeyError::Internal(err) => LoadDeviceError::Internal(err),
                LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline { server, error } => {
                    LoadDeviceError::RemoteOpaqueKeyFetchOffline { server, error }
                }
                LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed { server, error } => {
                    LoadDeviceError::RemoteOpaqueKeyFetchFailed { server, error }
                }
            })?;
    let totp_opaque_key = access.totp_protection.as_ref().map(|(_, key)| key);
    let device = decrypt_device_file(&device_file, &ciphertext_key, totp_opaque_key).map_err(
        |err| match err {
            DecryptDeviceFileError::TOTPDecrypt(_) => LoadDeviceError::TOTPDecryptionFailed,
            DecryptDeviceFileError::Decrypt(_) => LoadDeviceError::DecryptionFailed,
            DecryptDeviceFileError::Load(_) => LoadDeviceError::InvalidData,
        },
    )?;

    Ok(Arc::new(device))
}

fn load_available_device_from_blob(
    path: PathBuf,
    blob: &[u8],
) -> Result<AvailableDevice, libparsec_types::RmpDecodeError> {
    let device_file = DeviceFile::load(blob)?;

    let (
        ty,
        created_on,
        protected_on,
        server_addr,
        organization_id,
        user_id,
        device_id,
        human_handle,
        device_label,
        totp_opaque_key_id,
    ) = match device_file {
        DeviceFile::Keyring(device) => (
            AvailableDeviceType::Keyring,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
            device.totp_opaque_key_id,
        ),
        DeviceFile::Password(device) => (
            AvailableDeviceType::Password,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
            device.totp_opaque_key_id,
        ),
        DeviceFile::Recovery(device) => (
            AvailableDeviceType::Recovery,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
            None, // Recovery is never protected by TOTP
        ),
        DeviceFile::PKI(device) => (
            AvailableDeviceType::PKI {
                certificate_ref: device.certificate_ref,
            },
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
            device.totp_opaque_key_id,
        ),
        DeviceFile::AccountVault(device) => (
            AvailableDeviceType::AccountVault,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
            device.totp_opaque_key_id,
        ),
        DeviceFile::OpenBao(device) => (
            AvailableDeviceType::OpenBao {
                openbao_entity_id: device.openbao_entity_id,
                openbao_preferred_auth_id: device.openbao_preferred_auth_id,
            },
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
            device.totp_opaque_key_id,
        ),
    };

    Ok(AvailableDevice {
        key_file_path: path,
        created_on,
        protected_on,
        server_addr,
        organization_id,
        user_id,
        device_id,
        human_handle,
        device_label,
        totp_opaque_key_id,
        ty,
    })
}
