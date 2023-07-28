// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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

pub(crate) const DEVICE_FILE_EXT: &str = "keys";

/// Return the default keyfile path for a given device.
///
/// Note that the filename does not carry any intrinsic meaning.
/// Here, we simply use the slughash to avoid name collision.
pub fn get_default_key_file(config_dir: &Path, device: &LocalDevice) -> PathBuf {
    let mut device_path = config_dir.to_path_buf();

    device_path.push("devices");

    device_path.push(format!("{}.{DEVICE_FILE_EXT}", device.slughash()));

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
}

/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn load_device(
    config_dir: &Path,
    access: &DeviceAccessStrategy,
) -> Result<Arc<LocalDevice>, LoadDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_load_device(config_dir, access) {
        return result;
    }

    platform::load_device(access).await
}

#[derive(Debug, thiserror::Error)]
pub enum SaveDeviceError {
    #[error(transparent)]
    InvalidPath(anyhow::Error),
}

/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn save_device(
    config_dir: &Path,
    access: &DeviceAccessStrategy,
    device: &LocalDevice,
) -> Result<(), SaveDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_save_device(config_dir, access, device) {
        return result;
    }

    platform::save_device(access, device).await
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
