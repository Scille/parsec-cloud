// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;
// Testbed integration is tested in the `libparsec_tests_fixture` crate.
#[cfg(feature = "test-with-testbed")]
mod testbed;

#[path = "../tests/units/mod.rs"]
#[cfg(test)]
mod tests;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use zeroize::Zeroizing;

use libparsec_types::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use native as platform;
#[cfg(target_arch = "wasm32")]
use web as platform;

pub(crate) const DEVICE_FILE_EXT: &str = "keys";
pub(crate) const ARCHIVE_DEVICE_EXT: &str = "archived";

pub const PARSEC_BASE_CONFIG_DIR: &str = "PARSEC_BASE_CONFIG_DIR";
pub const PARSEC_BASE_DATA_DIR: &str = "PARSEC_BASE_DATA_DIR";
pub const PARSEC_BASE_HOME_DIR: &str = "PARSEC_BASE_HOME_DIR";

#[derive(Debug, thiserror::Error)]
enum ReadFileError {
    #[cfg_attr(not(target_arch = "wasm32"), expect(dead_code))]
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
enum LoadCiphertextKeyError {
    #[error("Invalid data")]
    InvalidData,
    #[cfg_attr(target_arch = "wasm32", expect(dead_code))]
    #[error("Decryption failed")]
    DecryptionFailed,
    #[cfg_attr(target_arch = "wasm32", expect(dead_code))]
    #[error(transparent)]
    Internal(anyhow::Error),
}

pub fn get_default_data_base_dir() -> PathBuf {
    #[cfg(target_arch = "wasm32")]
    {
        PathBuf::from("/")
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
        PathBuf::from("/")
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
        PathBuf::from("/")
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

#[derive(Debug, thiserror::Error)]
pub enum ListAvailableDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    Internal(anyhow::Error),
}

/// On web `config_dir` is used as database discriminant when using IndexedDB API
pub async fn list_available_devices(
    config_dir: &Path,
) -> Result<Vec<AvailableDevice>, ListAvailableDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_list_available_devices(config_dir) {
        return Ok(result);
    }

    platform::list_available_devices(config_dir).await
}

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

    let file_content = platform::read_file(&device_file)
        .await
        .map_err(|err| match err {
            ReadFileError::StorageNotAvailable => LoadAvailableDeviceError::StorageNotAvailable,
            ReadFileError::Internal(err) => LoadAvailableDeviceError::InvalidPath(err),
        })?;

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
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error(transparent)]
    Internal(anyhow::Error),
}

/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn load_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    access: &DeviceAccessStrategy,
) -> Result<Arc<LocalDevice>, LoadDeviceError> {
    log::debug!("Loading device at {}", access.key_file().display());
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_load_device(config_dir, access) {
        return result;
    }

    let file_content = platform::read_file(access.key_file())
        .await
        .map_err(|err| match err {
            ReadFileError::StorageNotAvailable => LoadDeviceError::StorageNotAvailable,
            ReadFileError::Internal(err) => LoadDeviceError::InvalidPath(err),
        })?;
    let device_file = DeviceFile::load(&file_content).map_err(|_| LoadDeviceError::InvalidData)?;
    let ciphertext_key = platform::load_ciphertext_key(access, &device_file)
        .await
        .map_err(|err| match err {
            LoadCiphertextKeyError::InvalidData => LoadDeviceError::InvalidData,
            LoadCiphertextKeyError::DecryptionFailed => LoadDeviceError::DecryptionFailed,
            LoadCiphertextKeyError::Internal(err) => LoadDeviceError::Internal(err),
        })?;
    let device = decrypt_device_file(&device_file, &ciphertext_key).map_err(|err| match err {
        DecryptDeviceFileError::Decrypt(_) => LoadDeviceError::DecryptionFailed,
        DecryptDeviceFileError::Load(_) => LoadDeviceError::InvalidData,
    })?;

    Ok(Arc::new(device))
}

#[derive(Debug, thiserror::Error)]
pub enum SaveDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error(transparent)]
    Internal(anyhow::Error),
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

    platform::save_device(strategy, device, device.now(), key_file).await
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
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

    let file_content = platform::read_file(current_key_file)
        .await
        .map_err(|err| match err {
            ReadFileError::StorageNotAvailable => UpdateDeviceError::StorageNotAvailable,
            ReadFileError::Internal(err) => UpdateDeviceError::InvalidPath(err),
        })?;
    let device_file =
        DeviceFile::load(&file_content).map_err(|_| UpdateDeviceError::InvalidData)?;
    let ciphertext_key = platform::load_ciphertext_key(current_access, &device_file)
        .await
        .map_err(|err| match err {
            LoadCiphertextKeyError::InvalidData => UpdateDeviceError::InvalidData,
            LoadCiphertextKeyError::DecryptionFailed => UpdateDeviceError::DecryptionFailed,
            LoadCiphertextKeyError::Internal(err) => UpdateDeviceError::Internal(err),
        })?;
    let device = decrypt_device_file(&device_file, &ciphertext_key).map_err(|err| match err {
        DecryptDeviceFileError::Decrypt(_) => UpdateDeviceError::DecryptionFailed,
        DecryptDeviceFileError::Load(_) => UpdateDeviceError::InvalidData,
    })?;

    // 2. ...and ask to overwrite it

    platform::update_device(
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
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    strategy: &DeviceAccessStrategy,
    new_server_addr: ParsecAddr,
) -> Result<ParsecAddr, UpdateDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_update_device(
        config_dir,
        strategy,
        &strategy.clone().into_save_strategy(),
        strategy.key_file(),
        Some(new_server_addr.clone()),
    ) {
        return result.map(|(_, old_server_addr)| old_server_addr);
    }

    // 1. Load the current device keys file...

    let file_content = platform::read_file(strategy.key_file())
        .await
        .map_err(|err| match err {
            ReadFileError::StorageNotAvailable => UpdateDeviceError::StorageNotAvailable,
            ReadFileError::Internal(err) => UpdateDeviceError::InvalidPath(err),
        })?;
    let device_file =
        DeviceFile::load(&file_content).map_err(|_| UpdateDeviceError::InvalidData)?;
    let ciphertext_key = platform::load_ciphertext_key(strategy, &device_file)
        .await
        .map_err(|err| match err {
            LoadCiphertextKeyError::InvalidData => UpdateDeviceError::InvalidData,
            LoadCiphertextKeyError::DecryptionFailed => UpdateDeviceError::DecryptionFailed,
            LoadCiphertextKeyError::Internal(err) => UpdateDeviceError::Internal(err),
        })?;
    let mut device =
        decrypt_device_file(&device_file, &ciphertext_key).map_err(|err| match err {
            DecryptDeviceFileError::Decrypt(_) => UpdateDeviceError::DecryptionFailed,
            DecryptDeviceFileError::Load(_) => UpdateDeviceError::InvalidData,
        })?;

    let old_server_addr = ParsecAddr::new(
        device.organization_addr.hostname().to_owned(),
        Some(device.organization_addr.port()),
        device.organization_addr.use_ssl(),
    );
    device.organization_addr = ParsecOrganizationAddr::new(
        new_server_addr,
        device.organization_addr.organization_id().to_owned(),
        device.organization_addr.root_verify_key().to_owned(),
    );

    // 2. ...and ask to overwrite it

    platform::update_device(
        &device,
        device_file.created_on(),
        strategy.key_file(),
        &strategy.clone().into_save_strategy(),
        strategy.key_file(),
    )
    .await?;

    Ok(old_server_addr)
}

pub fn is_keyring_available() -> bool {
    #[cfg(target_arch = "wasm32")]
    return false;

    #[cfg(not(target_arch = "wasm32"))]
    native::is_keyring_available()
}

#[derive(Debug, thiserror::Error)]
pub enum ArchiveDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(crate) fn get_device_archive_path(path: &Path) -> PathBuf {
    if let Some(current_file_extension) = path.extension() {
        // Add ARCHIVE_DEVICE_EXT to the current file extension resulting in extension `.{current}.{ARCHIVE_DEVICE_EXT}`.
        let mut ext = current_file_extension.to_owned();
        ext.extend([".".as_ref(), ARCHIVE_DEVICE_EXT.as_ref()]);
        path.with_extension(ext)
    } else {
        path.with_extension(ARCHIVE_DEVICE_EXT)
    }
}

/// Archive a device identified by its path.
pub async fn archive_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    device_path: &Path,
) -> Result<(), ArchiveDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_archive_device(config_dir, device_path) {
        return result;
    }

    platform::archive_device(device_path).await
}

#[derive(Debug, thiserror::Error)]
pub enum RemoveDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn remove_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    device_path: &Path,
) -> Result<(), RemoveDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_remove_device(config_dir, device_path) {
        return result;
    }

    platform::remove_device(device_path).await
}

#[derive(Debug, thiserror::Error)]
pub enum LoadRecoveryDeviceError {
    #[error("Cannot deserialize file content")]
    InvalidData,
    #[error("Passphrase format is invalid")]
    InvalidPassphrase,
    #[error("Failed to decrypt file content")]
    DecryptionFailed,
}

/// Decrypt and deserialize the local device to use as recovery device (i.e. the device
/// creating a new device) during a recovery device import operation.
pub fn load_recovery_device(
    recovery_device: &[u8],
    passphrase: SecretKeyPassphrase,
) -> Result<Arc<LocalDevice>, LoadRecoveryDeviceError> {
    let key = SecretKey::from_recovery_passphrase(passphrase)
        .map_err(|_| LoadRecoveryDeviceError::InvalidPassphrase)?;

    // Regular load
    let device_file =
        DeviceFile::load(recovery_device).map_err(|_| LoadRecoveryDeviceError::InvalidData)?;

    let recovery_device = match device_file {
        DeviceFile::Recovery(x) => {
            let cleartext = key
                .decrypt(&x.ciphertext)
                .map(Zeroizing::new)
                .map_err(|_| LoadRecoveryDeviceError::DecryptionFailed)?;

            LocalDevice::load(&cleartext).map_err(|_| LoadRecoveryDeviceError::InvalidData)?
        }
        // We are not expecting other type of device file
        _ => return Err(LoadRecoveryDeviceError::InvalidData),
    };

    Ok(Arc::new(recovery_device))
}

/// Serialize the provided local device into a package that can be exported as
/// recovery device (i.e. a buffer containing the encrypted local device and
/// its corresponding passphrase to be used for decryption).
pub fn dump_recovery_device(recovery_device: &LocalDevice) -> (SecretKeyPassphrase, Vec<u8>) {
    let created_on = recovery_device.now();
    let server_url = {
        ParsecAddr::new(
            recovery_device.organization_addr.hostname().to_owned(),
            Some(recovery_device.organization_addr.port()),
            recovery_device.organization_addr.use_ssl(),
        )
        .to_http_url(None)
        .to_string()
    };

    let (passphrase, key) = SecretKey::generate_recovery_passphrase();

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

    (passphrase, file_content)
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
        server_url,
        organization_id,
        user_id,
        device_id,
        human_handle,
        device_label,
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
        ),
        DeviceFile::Smartcard(device) => (
            AvailableDeviceType::Smartcard,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
        DeviceFile::AccountVault(device) => (
            AvailableDeviceType::AccountVault {
                ciphertext_key_id: device.ciphertext_key_id,
            },
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
        DeviceFile::OpenBao(device) => (
            AvailableDeviceType::OpenBao {
                openbao_url: device.openbao_url.clone(),
                openbao_ciphertext_key_path: device.openbao_ciphertext_key_path.clone(),
                openbao_auth_path: device.openbao_auth_path.clone(),
                openbao_auth_type: device.openbao_auth_type,
            },
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

fn encrypt_device(device: &LocalDevice, key: &SecretKey) -> Bytes {
    let cleartext = zeroize::Zeroizing::new(device.dump());
    key.encrypt(&cleartext).into()
}

#[derive(Debug, thiserror::Error)]
pub enum DecryptDeviceFileError {
    #[error("Failed to decrypt device file: {0}")]
    Decrypt(CryptoError),
    #[error("Failed to load device: {0}")]
    Load(&'static str),
}

fn decrypt_device_file(
    device_file: &DeviceFile,
    ciphertext_key: &SecretKey,
) -> Result<LocalDevice, DecryptDeviceFileError> {
    let cleartext = ciphertext_key
        .decrypt(device_file.ciphertext())
        .map_err(DecryptDeviceFileError::Decrypt)
        .map(zeroize::Zeroizing::new)?;
    LocalDevice::load(&cleartext).map_err(DecryptDeviceFileError::Load)
}

fn server_url_from_device(device: &LocalDevice) -> String {
    ParsecAddr::new(
        device.organization_addr.hostname().to_owned(),
        Some(device.organization_addr.port()),
        device.organization_addr.use_ssl(),
    )
    .to_http_url(None)
    .to_string()
}
