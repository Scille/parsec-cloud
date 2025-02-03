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

pub const PARSEC_BASE_CONFIG_DIR: &str = "PARSEC_BASE_CONFIG_DIR";
pub const PARSEC_BASE_DATA_DIR: &str = "PARSEC_BASE_DATA_DIR";
pub const PARSEC_BASE_HOME_DIR: &str = "PARSEC_BASE_HOME_DIR";

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
        let mut path = if let Ok(config_dir) = std::env::var(PARSEC_BASE_CONFIG_DIR) {
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
        let mut path = if let Ok(home_dir) = std::env::var(PARSEC_BASE_HOME_DIR) {
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
pub fn get_default_key_file(config_dir: &Path, device_id: &DeviceID) -> PathBuf {
    let mut device_path = config_dir.to_path_buf();

    device_path.push("devices");

    device_path.push(format!("{}.{DEVICE_FILE_EXT}", device_id.hex()));

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

pub fn is_keyring_available() -> bool {
    #[cfg(target_arch = "wasm32")]
    return false;

    #[cfg(not(target_arch = "wasm32"))]
    native::is_keyring_available()
}

#[derive(Debug, thiserror::Error)]
pub enum ArchiveDeviceError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub use platform::archive_device;

#[derive(Debug, thiserror::Error)]
pub enum RemoveDeviceError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub use platform::remove_device;
use zeroize::Zeroizing;

#[derive(Debug, thiserror::Error)]
pub enum PlatformImportRecoveryDeviceError {
    #[error("Cannot deserialize file content")]
    InvalidData,
    #[error("Passphrase format is invalid")]
    InvalidPassphrase,
    #[error("Failed to decrypt file content")]
    DecryptionFailed,
}

/// Returns recovery device
pub async fn import_recovery_device(
    recovery_device: &[u8],
    passphrase: SecretKeyPassphrase,
) -> Result<LocalDevice, PlatformImportRecoveryDeviceError> {
    let key = SecretKey::from_recovery_passphrase(passphrase)
        .map_err(|_| PlatformImportRecoveryDeviceError::InvalidPassphrase)?;

    // Regular load
    let device_file = DeviceFile::load(recovery_device)
        .map_err(|_| PlatformImportRecoveryDeviceError::InvalidData)?;

    let recovery_device = match device_file {
        DeviceFile::Recovery(x) => {
            let cleartext = key
                .decrypt(&x.ciphertext)
                .map(Zeroizing::new)
                .map_err(|_| PlatformImportRecoveryDeviceError::DecryptionFailed)?;

            LocalDevice::load(&cleartext)
                .map_err(|_| PlatformImportRecoveryDeviceError::InvalidData)?
        }
        // We are not expecting other type of device file
        _ => return Err(PlatformImportRecoveryDeviceError::InvalidData),
    };

    Ok(recovery_device)
}

/// create a new recovery device from provided local device
/// returns the passphrase associated with the key that
/// encrypted device data, the dumped recovery device data
/// and the recovery device itself
pub async fn export_recovery_device(
    device: &LocalDevice,
    device_label: DeviceLabel,
) -> (SecretKeyPassphrase, Vec<u8>, LocalDevice) {
    let created_on = device.now();
    let server_url = {
        ParsecAddr::new(
            device.organization_addr.hostname().to_owned(),
            Some(device.organization_addr.port()),
            device.organization_addr.use_ssl(),
        )
        .to_http_url(None)
        .to_string()
    };

    let (passphrase, key) = SecretKey::generate_recovery_passphrase();

    let recovery_device = LocalDevice::from_existing_device_for_user(device, device_label);
    let ciphertext = {
        let cleartext = Zeroizing::new(recovery_device.dump());
        let ciphertext = key.encrypt(&cleartext);
        ciphertext.into()
    };

    let file_content = DeviceFile::Recovery(DeviceFileRecovery {
        created_on,
        // Note recovery device is not supposed to change its protection
        protected_on: created_on,
        server_url,
        organization_id: recovery_device.organization_id().to_owned(),
        user_id: recovery_device.user_id,
        device_id: recovery_device.device_id,
        human_handle: recovery_device.human_handle.to_owned(),
        device_label: recovery_device.device_label.to_owned(),
        ciphertext,
    })
    .dump();

    (passphrase, file_content, recovery_device)
}

#[cfg_attr(target_arch = "wasm32", expect(dead_code))]
fn load_available_device_from_blob(
    path: PathBuf,
    blob: &[u8],
) -> Result<AvailableDevice, libparsec_types::RmpDecodeError> {
    let device_file = DeviceFile::load(blob)?;

    let (
        ty,
        created_on,
        protected_on,
        server_url,
        organization_id,
        user_id,
        device_id,
        human_handle,
        device_label,
    ) = match device_file {
        DeviceFile::Keyring(device) => (
            DeviceFileType::Keyring,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
        DeviceFile::Password(device) => (
            DeviceFileType::Password,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
        DeviceFile::Recovery(device) => (
            DeviceFileType::Recovery,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
        DeviceFile::Smartcard(device) => (
            DeviceFileType::Smartcard,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
    };

    Ok(AvailableDevice {
        key_file_path: path,
        created_on,
        protected_on,
        server_url,
        organization_id,
        user_id,
        device_id,
        human_handle,
        device_label,
        ty,
    })
}

#[cfg_attr(target_arch = "wasm32", expect(dead_code))]
fn secret_key_from_password(
    password: &Password,
    algorithm: &DeviceFilePasswordAlgorithm,
) -> Result<SecretKey, CryptoError> {
    match algorithm {
        DeviceFilePasswordAlgorithm::Argon2id {
            memlimit_kb,
            opslimit,
            parallelism,
            salt,
        } => SecretKey::from_argon2id_password(
            password,
            salt,
            (*opslimit).try_into().or(Err(CryptoError::DataSize))?,
            (*memlimit_kb).try_into().or(Err(CryptoError::DataSize))?,
            (*parallelism).try_into().or(Err(CryptoError::DataSize))?,
        ),
    }
}
