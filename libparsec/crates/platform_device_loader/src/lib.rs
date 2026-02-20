// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod async_enrollment;
mod device;
mod strategy;
pub use device::*;

pub use async_enrollment::*;
pub use strategy::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native/mod.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
mod platform;
// Testbed integration is tested in the `libparsec_tests_fixture` crate.
#[cfg(feature = "test-with-testbed")]
mod testbed;

#[path = "../tests/units/mod.rs"]
#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use libparsec_types::prelude::*;

const LOCAL_PENDING_EXT: &str = "pending";

pub(crate) const DEVICE_FILE_EXT: &str = "keys";
pub(crate) const ARCHIVE_DEVICE_EXT: &str = "archived";
pub(crate) const PENDING_ASYNC_ENROLLMENT_EXT: &str = "pending";

pub const PARSEC_BASE_CONFIG_DIR: &str = "PARSEC_BASE_CONFIG_DIR";
pub const PARSEC_BASE_DATA_DIR: &str = "PARSEC_BASE_DATA_DIR";
pub const PARSEC_BASE_HOME_DIR: &str = "PARSEC_BASE_HOME_DIR";

pub fn get_default_data_base_dir() -> PathBuf {
    platform::get_default_data_base_dir()
}

pub fn get_default_config_dir() -> PathBuf {
    platform::get_default_config_dir()
}

pub fn get_default_mountpoint_base_dir() -> PathBuf {
    platform::get_default_mountpoint_base_dir()
}

fn get_devices_dir(config_dir: &Path) -> PathBuf {
    config_dir.join("devices")
}

/// Return the default keyfile path for a given device.
///
/// Note that the filename does not carry any intrinsic meaning.
/// Here, we simply use the device ID (as it is a UUID) to avoid name collision.
pub fn get_default_key_file(config_dir: &Path, device_id: DeviceID) -> PathBuf {
    let mut device_path = get_devices_dir(config_dir);
    device_path.push(format!("{}.{DEVICE_FILE_EXT}", device_id.hex()));
    device_path
}

pub fn get_default_local_pending_file(
    config_dir: &Path,
    enrollment_id: PKIEnrollmentID,
) -> PathBuf {
    let mut local_pending_path = get_local_pending_dir(config_dir);
    local_pending_path.push(format!("{}.{LOCAL_PENDING_EXT}", enrollment_id.hex()));
    local_pending_path
}

fn get_local_pending_dir(config_dir: &Path) -> PathBuf {
    config_dir.join("pending_requests")
}

pub fn get_default_pending_async_enrollment_file(
    config_dir: &Path,
    enrollment_id: AsyncEnrollmentID,
) -> PathBuf {
    let mut enrollments_dir = get_pending_async_enrollment_dir(config_dir);

    enrollments_dir.push(format!(
        "{}.{PENDING_ASYNC_ENROLLMENT_EXT}",
        enrollment_id.hex()
    ));

    enrollments_dir
}

fn get_pending_async_enrollment_dir(config_dir: &Path) -> PathBuf {
    config_dir.join("async_enrollments")
}

/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn save_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    strategy: &DeviceSaveStrategy,
    device: &LocalDevice,
    key_file: PathBuf,
) -> Result<AvailableDevice, SaveDeviceError> {
    log::debug!("Saving device at {}", key_file.display());
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_save_device(config_dir, strategy, device, key_file.clone())
    {
        return result;
    }

    device::save_device(strategy, device, device.now(), key_file).await
}

pub fn is_keyring_available() -> bool {
    platform::is_keyring_available()
}

fn encrypt_device(
    device: &LocalDevice,
    ciphertext_key: &SecretKey,
    totp_opaque_key: Option<&SecretKey>,
) -> Bytes {
    let cleartext = zeroize::Zeroizing::new(device.dump());
    let ciphertext = ciphertext_key.encrypt(&cleartext);
    if let Some(totp_opaque_key) = totp_opaque_key {
        totp_opaque_key.encrypt(&ciphertext).into()
    } else {
        ciphertext.into()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecryptDeviceFileError {
    #[error("Failed to decrypt device file with the key obtained from TOTP challenge: {0}")]
    TOTPDecrypt(CryptoError),
    #[error("Failed to decrypt device file: {0}")]
    Decrypt(CryptoError),
    #[error("Failed to load device: {0}")]
    Load(&'static str),
}

fn decrypt_device_file(
    device_file: &DeviceFile,
    ciphertext_key: &SecretKey,
    // This key has been obtained from the server after a TOTP challenge,
    // hence can never be used alone (otherwise the server would be able to
    // decrypt the device keys file !).
    totp_opaque_key: Option<&SecretKey>,
) -> Result<LocalDevice, DecryptDeviceFileError> {
    let ciphertext = device_file.ciphertext();
    let cleartext = if let Some(totp_opaque_key) = totp_opaque_key {
        let intermediate_ciphertext = totp_opaque_key
            .decrypt(ciphertext)
            .map_err(DecryptDeviceFileError::TOTPDecrypt)?;

        ciphertext_key
            .decrypt(&intermediate_ciphertext)
            .map_err(DecryptDeviceFileError::Decrypt)
            .map(zeroize::Zeroizing::new)?
    } else {
        ciphertext_key
            .decrypt(ciphertext)
            .map_err(DecryptDeviceFileError::Decrypt)
            .map(zeroize::Zeroizing::new)?
    };

    LocalDevice::load(&cleartext).map_err(DecryptDeviceFileError::Load)
}
