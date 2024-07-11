// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;
// Testbed integration is tested in the `libparsec_tests_fixture` crate.
#[cfg(feature = "test-with-testbed")]
mod testbed;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec_types::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use native as platform;
#[cfg(target_arch = "wasm32")]
use web as platform;

pub const ARGON2ID_DEFAULT_MEMLIMIT_KB: u32 = 128 * 1024; // 128 Mo
pub const ARGON2ID_DEFAULT_OPSLIMIT: u32 = 3;
// Be careful when changing parallelism: libsodium only supports 1 thread !
pub const ARGON2ID_DEFAULT_PARALLELISM: u32 = 1;

pub(crate) const DEVICE_FILE_EXT: &str = "keys";

pub const PARSEC_CONFIG_DIR: &str = "PARSEC_CONFIG_DIR";
pub const PARSEC_BASE_DATA_DIR: &str = "PARSEC_BASE_DATA_DIR";
pub const PARSEC_HOME_DIR: &str = "PARSEC_HOME_DIR";

pub fn get_default_data_base_dir() -> PathBuf {
    #[cfg(target_arch = "wasm32")]
    {
        PathBuf::from("")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut path = if let Ok(data_dir) = std::env::var(PARSEC_BASE_DATA_DIR) {
            PathBuf::from(data_dir)
        } else {
            dirs::data_dir().expect("Could not determine base data directory")
        };

        path.push("parsec3");
        path
    }
}

pub fn get_default_config_dir() -> PathBuf {
    #[cfg(target_arch = "wasm32")]
    {
        PathBuf::from("")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut path = if let Ok(config_dir) = std::env::var(PARSEC_CONFIG_DIR) {
            PathBuf::from(config_dir)
        } else {
            dirs::config_dir().expect("Could not determine base config directory")
        };

        path.push("parsec3/libparsec");
        path
    }
}

pub fn get_default_mountpoint_base_dir() -> PathBuf {
    #[cfg(target_arch = "wasm32")]
    {
        PathBuf::from("")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut path = if let Ok(home_dir) = std::env::var(PARSEC_HOME_DIR) {
            PathBuf::from(home_dir)
        } else {
            dirs::home_dir().expect("Could not determine home directory")
        };

        path.push("Parsec3");
        path
    }
}

/// Return the default keyfile path for a given device.
///
/// Note that the filename does not carry any intrinsic meaning.
/// Here, we simply use the device ID (as it is a UUID) to avoid name collision.
pub fn get_default_key_file(config_dir: &Path, device: &LocalDevice) -> PathBuf {
    let mut device_path = config_dir.to_path_buf();

    device_path.push("devices");

    device_path.push(format!("{}.{DEVICE_FILE_EXT}", device.device_id.hex()));

    device_path
}

/// On web `config_dir` is used as database discriminant when using IndexedDB API
pub async fn list_available_devices(config_dir: &Path) -> Vec<AvailableDevice> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_list_available_devices(config_dir) {
        return result;
    }

    platform::list_available_devices(config_dir).await
}

#[derive(Debug, thiserror::Error)]
pub enum LoadDeviceError {
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error("Cannot deserialize file content")]
    InvalidData,
    #[error("Failed to decrypt file content")]
    DecryptionFailed,
    #[error(transparent)]
    Internal(anyhow::Error),
}

/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn load_device(
    // TODO: Should we set under testbed feature ?
    #[allow(unused)] config_dir: &Path,
    access: &DeviceAccessStrategy,
) -> Result<Arc<LocalDevice>, LoadDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_load_device(config_dir, access) {
        return result;
    }

    platform::load_device(access)
        .await
        .map(|(device, _)| device)
}

#[derive(Debug, thiserror::Error)]
pub enum SaveDeviceError {
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error(transparent)]
    Internal(anyhow::Error),
}

/// Note `config_dir` is only used as discriminant for the testbed here
#[allow(unused)]
pub async fn save_device(
    config_dir: &Path,
    access: &DeviceAccessStrategy,
    device: &LocalDevice,
) -> Result<AvailableDevice, SaveDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_save_device(config_dir, access, device) {
        return result;
    }

    platform::save_device(access, device, device.now()).await
}

#[derive(Debug, thiserror::Error)]
pub enum ChangeAuthentificationError {
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error("Cannot deserialize file content")]
    InvalidData,
    #[error("Failed to decrypt file content")]
    DecryptionFailed,
    #[error("Cannot remove the old device")]
    CannotRemoveOldDevice,
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<LoadDeviceError> for ChangeAuthentificationError {
    fn from(value: LoadDeviceError) -> Self {
        match value {
            LoadDeviceError::DecryptionFailed => Self::DecryptionFailed,
            LoadDeviceError::InvalidData => Self::InvalidData,
            LoadDeviceError::InvalidPath(e) => Self::InvalidPath(e),
            LoadDeviceError::Internal(e) => Self::Internal(e),
        }
    }
}

impl From<SaveDeviceError> for ChangeAuthentificationError {
    fn from(value: SaveDeviceError) -> Self {
        match value {
            SaveDeviceError::InvalidPath(e) => Self::InvalidPath(e),
            SaveDeviceError::Internal(e) => Self::Internal(e),
        }
    }
}

/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn change_authentication(
    #[allow(unused)] config_dir: &Path,
    current_access: &DeviceAccessStrategy,
    new_access: &DeviceAccessStrategy,
) -> Result<AvailableDevice, ChangeAuthentificationError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) =
        testbed::maybe_change_authentication(config_dir, current_access, new_access)
    {
        return result;
    }

    platform::change_authentication(current_access, new_access).await
}

#[derive(Debug, thiserror::Error)]
pub enum SaveRecoveryDeviceError {
    #[error(transparent)]
    InvalidPath(anyhow::Error),
}

pub async fn save_recovery_device(
    key_file: &Path,
    device: &LocalDevice,
) -> Result<SecretKeyPassphrase, SaveRecoveryDeviceError> {
    platform::save_recovery_device(key_file, device).await
}

#[derive(Debug, thiserror::Error)]
pub enum LoadRecoveryDeviceError {
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error("Cannot deserialize file content")]
    InvalidData,
    #[error("Passphrase format is invalid")]
    InvalidPassphrase,
    #[error("Failed to decrypt file content")]
    DecryptionFailed,
}

pub async fn load_recovery_device(
    key_file: &Path,
    passphrase: SecretKeyPassphrase,
) -> Result<LocalDevice, LoadRecoveryDeviceError> {
    platform::load_recovery_device(key_file, passphrase).await
}

pub fn is_keyring_available() -> bool {
    #[cfg(target_arch = "wasm32")]
    return false;

    #[cfg(not(target_arch = "wasm32"))]
    native::is_keyring_available()
}
