// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use itertools::Itertools as _;
use keyring::Entry as KeyringEntry;
use libparsec_platform_async::future::FutureExt as _;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use uuid::Uuid;

use libparsec_types::prelude::*;

use crate::{ChangeAuthentificationError, LoadDeviceError, SaveDeviceError, DEVICE_FILE_EXT};

const KEYRING_SERVICE: &str = "parsec";

impl From<keyring::Error> for LoadDeviceError {
    fn from(value: keyring::Error) -> Self {
        Self::Internal(anyhow::anyhow!(value))
    }
}

impl From<keyring::Error> for SaveDeviceError {
    fn from(value: keyring::Error) -> Self {
        Self::Internal(anyhow::anyhow!(value))
    }
}

/*
 * List available devices
 */

fn find_device_files(path: PathBuf) -> Vec<PathBuf> {
    // TODO: make file access on a worker thread !

    let mut key_file_paths = vec![];

    // `fs::read_dir` fails if path doesn't exists, is not a folder or is not
    // accessible... In any case, there is not much we can do but to ignore it.
    if let Ok(children) = std::fs::read_dir(path) {
        for path in children.filter_map(|entry| entry.as_ref().map(std::fs::DirEntry::path).ok()) {
            if path.extension() == Some(DEVICE_FILE_EXT.as_ref()) {
                key_file_paths.push(path)
            } else if path.is_dir() {
                key_file_paths.append(&mut find_device_files(path))
            }
        }
    }

    key_file_paths
}

pub async fn list_available_devices(config_dir: &Path) -> Vec<AvailableDevice> {
    let key_file_paths = crate::get_devices_dir(config_dir);

    // Consider `.keys` files in devices directory
    let mut key_file_paths = find_device_files(key_file_paths);

    // Sort paths so the discovery order is deterministic
    // In the case of duplicate files, that means only the first discovered device is considered
    key_file_paths.sort();
    log::trace!("Found the following device files: {key_file_paths:?}");

    key_file_paths
        .into_iter()
        // List only valid devices.
        .filter_map(|path| {
            load_available_device(path.clone())
                .inspect_err(|e| {
                    log::debug!(
                        "Failed to load device at {path} with {e}",
                        path = path.display()
                    )
                })
                .ok()
        })
        // Ignore duplicate devices
        .unique_by(|v| v.device_id)
        .collect()
}

#[derive(Debug, thiserror::Error)]
enum LoadAvailableDeviceFileError {
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error("Cannot deserialize file content")]
    InvalidData,
}

fn load_available_device(
    key_file_path: PathBuf,
) -> Result<AvailableDevice, LoadAvailableDeviceFileError> {
    // TODO: make file access on a worker thread !
    let content = std::fs::read(&key_file_path)
        .map_err(|e| LoadAvailableDeviceFileError::InvalidPath(e.into()))?;

    super::load_available_device_from_blob(key_file_path, &content)
        .map_err(|_| LoadAvailableDeviceFileError::InvalidData)
}

/*
 * Save & load
 */

pub async fn load_device(
    access: &DeviceAccessStrategy,
) -> Result<(Arc<LocalDevice>, DateTime), LoadDeviceError> {
    let key_file = access.key_file();
    let content = tokio::fs::read(key_file)
        .await
        .map_err(|e| LoadDeviceError::InvalidPath(e.into()))?;

    // Regular load
    let device_file = DeviceFile::load(&content).map_err(|_| LoadDeviceError::InvalidData)?;

    let (key, created_on) = match (access, &device_file) {
        (DeviceAccessStrategy::Keyring { .. }, DeviceFile::Keyring(device)) => {
            let entry = KeyringEntry::new(&device.keyring_service, &device.keyring_user)?;

            let passphrase = entry.get_password()?.into();

            let key = SecretKey::from_recovery_passphrase(passphrase)
                .map_err(|_| LoadDeviceError::DecryptionFailed)?;

            Ok((key, device.created_on))
        }

        (DeviceAccessStrategy::Password { password, .. }, DeviceFile::Password(device)) => {
            let key = super::secret_key_from_password(password, &device.algorithm)
                .map_err(|_| LoadDeviceError::InvalidData)?;

            Ok((key, device.created_on))
        }

        (DeviceAccessStrategy::Smartcard { .. }, DeviceFile::Smartcard(_device)) => {
            todo!("Load smartcard device")
        }
        _ => Err(LoadDeviceError::InvalidData),
    }?;

    let device = super::decrypt_device_file(&device_file, &key)?;

    Ok((Arc::new(device), created_on))
}

async fn save_content(key_file: &PathBuf, file_content: &[u8]) -> Result<(), SaveDeviceError> {
    if let Some(parent) = key_file.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| SaveDeviceError::InvalidPath(e.into()))?;
    }
    let tmp_path = match key_file.file_name() {
        Some(file_name) => {
            let mut tmp_path = key_file.clone();
            {
                let mut tmp_file_name = file_name.to_owned();
                tmp_file_name.push(".tmp");
                tmp_path.set_file_name(tmp_file_name);
            }
            tmp_path
        }
        None => {
            return Err(SaveDeviceError::InvalidPath(anyhow::anyhow!(
                "Path is missing a file name"
            )))
        }
    };

    // Classic pattern for atomic file creation:
    // - First write the file in a temporary location
    // - Then move the file to it final location
    // This way a crash during file write won't end up with a corrupted
    // file in the final location.
    tokio::fs::write(&tmp_path, file_content)
        .await
        .map_err(|e| SaveDeviceError::InvalidPath(e.into()))?;
    tokio::fs::rename(&tmp_path, key_file)
        .await
        .map_err(|e| SaveDeviceError::InvalidPath(e.into()))?;

    Ok(())
}

async fn generate_keyring_user(
    keyring_user_path: &PathBuf,
) -> Result<(SecretKey, String), SaveDeviceError> {
    // Generate a keyring user
    let keyring_user = Uuid::new_v4().to_string();

    // Generate a key
    let (passphrase, key) = SecretKey::generate_recovery_passphrase();

    let entry = KeyringEntry::new(KEYRING_SERVICE, &keyring_user)?;

    // Add the key to the keyring
    entry.set_password(&passphrase)?;

    // Save the keyring user to the config file
    save_content(keyring_user_path, keyring_user.as_bytes()).await?;

    Ok((key, keyring_user))
}

pub async fn save_device(
    access: &DeviceAccessStrategy,
    device: &LocalDevice,
    created_on: DateTime,
) -> Result<AvailableDevice, SaveDeviceError> {
    let protected_on = device.now();
    let server_url = super::server_url_from_device(device);

    match access {
        DeviceAccessStrategy::Keyring { key_file } => {
            let keyring_user_path = crate::get_default_data_base_dir().join("keyring_user.txt");

            let keyring_info = tokio::fs::read_to_string(&keyring_user_path)
                .map(|keyring_user| {
                    keyring_user.ok().and_then(|keyring_user| {
                        KeyringEntry::new(KEYRING_SERVICE, &keyring_user)
                            .and_then(|entry| {
                                entry
                                    .get_password()
                                    .map(libparsec_types::SecretKeyPassphrase::from)
                            })
                            .ok()
                            .and_then(|secret| SecretKey::from_recovery_passphrase(secret).ok())
                            .map(|key| (key, keyring_user))
                    })
                })
                .await;
            let (key, keyring_user) = match keyring_info {
                Some(v) => v,
                None => generate_keyring_user(&keyring_user_path).await?,
            };

            let ciphertext = super::encrypt_device(device, &key);

            let file_content = DeviceFile::Keyring(DeviceFileKeyring {
                created_on,
                protected_on,
                server_url: server_url.clone(),
                organization_id: device.organization_id().clone(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.clone(),
                device_label: device.device_label.clone(),
                keyring_service: KEYRING_SERVICE.into(),
                keyring_user,
                ciphertext,
            });

            let file_content = file_content.dump();

            save_content(key_file, &file_content).await?;
        }

        DeviceAccessStrategy::Password { key_file, password } => {
            let key_algo = super::generate_default_password_algorithm_parameters();
            let key = super::secret_key_from_password(password, &key_algo)
                .expect("Failed to derive key from password");

            let ciphertext = super::encrypt_device(device, &key);

            let file_content = DeviceFile::Password(DeviceFilePassword {
                created_on,
                protected_on,
                server_url: server_url.clone(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.to_owned(),
                device_label: device.device_label.to_owned(),
                algorithm: key_algo,
                ciphertext,
            });

            let file_content = file_content.dump();

            save_content(key_file, &file_content).await?;
        }

        DeviceAccessStrategy::Smartcard { .. } => {
            todo!("Save smartcard device")
        }
    }

    Ok(AvailableDevice {
        key_file_path: access.key_file().to_owned(),
        server_url,
        created_on,
        protected_on,
        organization_id: device.organization_id().to_owned(),
        user_id: device.user_id,
        device_id: device.device_id,
        device_label: device.device_label.clone(),
        human_handle: device.human_handle.clone(),
        ty: access.ty(),
    })
}

pub async fn change_authentication(
    current_access: &DeviceAccessStrategy,
    new_access: &DeviceAccessStrategy,
) -> Result<AvailableDevice, ChangeAuthentificationError> {
    let (device, created_on) = load_device(current_access).await?;

    let available_device = save_device(new_access, &device, created_on).await?;

    let key_file = current_access.key_file();
    let new_key_file = new_access.key_file();

    if key_file != new_key_file {
        tokio::fs::remove_file(key_file)
            .await
            .map_err(|_| ChangeAuthentificationError::CannotRemoveOldDevice)?;
    }

    Ok(available_device)
}

pub const ARCHIVE_DEVICE_EXT: &str = "archived";

/// Archive a device identified by its path.
pub async fn archive_device(device_path: &Path) -> Result<(), crate::ArchiveDeviceError> {
    let archive_device_path = if let Some(current_file_extension) = device_path.extension() {
        // Add ARCHIVE_DEVICE_EXT to the current file extension resulting in extension `.{current}.{ARCHIVE_DEVICE_EXT}`.
        let mut ext = current_file_extension.to_owned();
        ext.extend([".".as_ref(), ARCHIVE_DEVICE_EXT.as_ref()]);
        device_path.with_extension(ext)
    } else {
        device_path.with_extension(ARCHIVE_DEVICE_EXT)
    };

    log::debug!(
        "Archiving device {} to {}",
        device_path.display(),
        archive_device_path.display()
    );

    tokio::fs::rename(device_path, archive_device_path)
        .await
        .map_err(|e| crate::ArchiveDeviceError::Internal(e.into()))
}

pub async fn remove_device(device_path: &Path) -> Result<(), crate::RemoveDeviceError> {
    log::debug!("Removing device {}", device_path.display());

    tokio::fs::remove_file(device_path)
        .await
        .map_err(|e| crate::RemoveDeviceError::Internal(e.into()))
}

pub fn is_keyring_available() -> bool {
    // Using "tmp" as user, because keyring-rs forbids the use of empty string
    // due to an issue in macOS. See: https://github.com/hwchen/keyring-rs/pull/87
    let result = KeyringEntry::new(KEYRING_SERVICE, "tmp");
    let error = if cfg!(target_os = "macos") {
        // On macOS, trying to access the entry password prompts the user for their session password.
        // We don't want that here, so we simply check that the keyring is available.
        result.err()
    } else {
        // On other platforms, accessing the entry password is a good way to avoid false positives.
        // For instance, an isolated snap package may have access to the keyring API but would fail
        // when trying to access the password.
        result.and_then(|x| x.get_password()).err()
    };
    match error {
        None => true,
        Some(keyring::error::Error::NoEntry) => true,
        Some(err) => {
            log::warn!("Keyring is not available: {err:?}");
            false
        }
    }
}
