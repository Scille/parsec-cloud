// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use keyring::Entry as KeyringEntry;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};
use uuid::Uuid;
use zeroize::Zeroize;

use libparsec_types::prelude::*;

use crate::{
    ChangeAuthentificationError, LoadDeviceError, LoadRecoveryDeviceError, SaveDeviceError,
    SaveRecoveryDeviceError, DEVICE_FILE_EXT,
};

const KEYRING_SERVICE: &str = "parsec";

fn get_default_data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("Could not determine base data directory")
        .join("parsec-v3-alpha")
}

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
        for child_dir in children.filter_map(|entry| entry.ok()) {
            let path = child_dir.path();
            if path.extension() == Some(OsStr::new(DEVICE_FILE_EXT)) {
                key_file_paths.push(path)
            } else if path.is_dir() {
                key_file_paths.append(&mut find_device_files(path))
            }
        }
    }

    key_file_paths
}

pub async fn list_available_devices(config_dir: &Path) -> Vec<AvailableDevice> {
    let mut devices: Vec<AvailableDevice> = vec![];

    let key_file_paths = config_dir.join("devices");

    // Consider `.keys` files in devices directory
    let mut key_file_paths = find_device_files(key_file_paths);

    // Sort paths so the discovery order is deterministic
    // In the case of duplicate files, that means only the first discovered device is considered
    key_file_paths.sort();

    for key_file_path in key_file_paths {
        let device = match load_available_device(key_file_path) {
            // Load the device file
            Ok(device) => device,
            // Ignore invalid files
            Err(_) => continue,
        };

        // Ignore duplicate files
        for existing in &devices {
            if existing.slug == device.slug {
                continue;
            }
        }

        devices.push(device);
    }

    devices
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

    // Regular load
    let device_file = DeviceFile::load(&content)
        .or_else(|_| {
            // In case of failure, try to use the legacy device format
            load_legacy_device_file_from_content(&key_file_path, &content)
        })
        .map_err(|_| LoadAvailableDeviceFileError::InvalidData)?;

    let (ty, organization_id, device_id, human_handle, device_label, slug) = match device_file {
        DeviceFile::Keyring(device) => (
            DeviceFileType::Keyring,
            device.organization_id,
            device.device_id,
            device.human_handle,
            device.device_label,
            device.slug,
        ),
        DeviceFile::Password(device) => (
            DeviceFileType::Password,
            device.organization_id,
            device.device_id,
            device.human_handle,
            device.device_label,
            device.slug,
        ),
        DeviceFile::Recovery(device) => (
            DeviceFileType::Recovery,
            device.organization_id,
            device.device_id,
            device.human_handle,
            device.device_label,
            device.slug,
        ),
        DeviceFile::Smartcard(device) => (
            DeviceFileType::Smartcard,
            device.organization_id,
            device.device_id,
            device.human_handle,
            device.device_label,
            device.slug,
        ),
    };

    Ok(AvailableDevice {
        key_file_path,
        organization_id,
        device_id,
        human_handle,
        device_label,
        slug,
        ty,
    })
}

fn load_legacy_device_file_from_content(
    key_file: &Path,
    content: &[u8],
) -> Result<DeviceFile, LoadAvailableDeviceFileError> {
    let legacy_device = LegacyDeviceFilePassword::load(content)
        .map_err(|_| LoadAvailableDeviceFileError::InvalidData)?;

    // Legacy device slug is their stem, ex: `9d84fbd57a#Org#Zack@PC1.keys`
    let (slug, organization_id, device_id) = key_file
        .file_stem()
        .and_then(|x| x.to_str())
        .and_then(|x| {
            let (organization_id, device_id) = LocalDevice::load_slug(x).ok()?;
            Some((x.to_owned(), organization_id, device_id))
        })
        .ok_or_else(|| {
            LoadAvailableDeviceFileError::InvalidPath(anyhow::anyhow!(
                "Cannot extract device slug from file name"
            ))
        })?;

    // `device_label` & `human_handle` fields has been introduced in Parsec v1.14, hence
    // they may not be present.
    //
    // If that's the case, we are in an exotic case (very old device), so we don't
    // bother much an use the redacted system to obtain device label & human handle.
    // Of course redacted certificate has nothing to do with this, but it's just
    // convenient and "good enough" to go this way ;-)
    //
    // Note we no longer save in this legacy format: if save is required (e.g. the user
    // is changing password) it will be done with the newer format. Hence there is no
    // risk of serializing the human handle email which uses the redacted domain name
    // (which is reserved and would cause error on deserialization !)
    let human_handle = match legacy_device.human_handle {
        Some(human_handle) => human_handle,
        None => HumanHandle::new_redacted(device_id.user_id()),
    };
    let device_label = match legacy_device.device_label {
        Some(device_label) => device_label,
        None => DeviceLabel::new_redacted(device_id.device_name()),
    };

    Ok(DeviceFile::Password(DeviceFilePassword {
        salt: legacy_device.salt,
        ciphertext: legacy_device.ciphertext,
        human_handle,
        device_label,
        device_id,
        organization_id,
        slug,
    }))
}

/*
 * Save & load
 */

pub async fn load_device(
    access: &DeviceAccessStrategy,
) -> Result<Arc<LocalDevice>, LoadDeviceError> {
    let device = match access {
        DeviceAccessStrategy::Keyring { key_file } => {
            // TODO: make file access on a worker thread !
            let content =
                std::fs::read(key_file).map_err(|e| LoadDeviceError::InvalidPath(e.into()))?;

            // Regular load
            let device_file =
                DeviceFile::load(&content).map_err(|_| LoadDeviceError::InvalidData)?;

            if let DeviceFile::Keyring(x) = device_file {
                let entry = KeyringEntry::new(&x.keyring_service, &x.keyring_user)?;

                let passphrase = entry.get_password()?.into();

                let key = SecretKey::from_recovery_passphrase(passphrase)
                    .map_err(|_| LoadDeviceError::DecryptionFailed)?;

                let mut cleartext = key
                    .decrypt(&x.ciphertext)
                    .map_err(|_| LoadDeviceError::DecryptionFailed)?;

                let device =
                    LocalDevice::load(&cleartext).map_err(|_| LoadDeviceError::InvalidData)?;

                cleartext.zeroize();

                device
            } else {
                return Err(LoadDeviceError::InvalidData);
            }
        }

        DeviceAccessStrategy::Password { key_file, password } => {
            // TODO: make file access on a worker thread !
            let content =
                std::fs::read(key_file).map_err(|e| LoadDeviceError::InvalidPath(e.into()))?;

            // Regular load
            let device_file = DeviceFile::load(&content)
                .or_else(|_| {
                    // In case of failure, try to use the legacy device format
                    load_legacy_device_file_from_content(key_file, &content)
                })
                .map_err(|_| LoadDeviceError::InvalidData)?;

            match device_file {
                DeviceFile::Password(x) => {
                    let key = SecretKey::from_password(password, &x.salt)
                        .map_err(|_| LoadDeviceError::InvalidData)?;
                    let mut cleartext = key
                        .decrypt(&x.ciphertext)
                        .map_err(|_| LoadDeviceError::DecryptionFailed)?;
                    let device =
                        LocalDevice::load(&cleartext).map_err(|_| LoadDeviceError::InvalidData)?;
                    cleartext.zeroize(); // Scrub the buffer given it contains keys in clear
                    device
                }
                // We are not expecting other type of device file
                _ => return Err(LoadDeviceError::InvalidData),
            }
        }

        DeviceAccessStrategy::Smartcard { .. } => {
            // TODO !
            todo!()
        }
    };

    Ok(Arc::new(device))
}

fn save_content(key_file: &PathBuf, file_content: &[u8]) -> Result<(), SaveDeviceError> {
    if let Some(parent) = key_file.parent() {
        std::fs::create_dir_all(parent).map_err(|e| SaveDeviceError::InvalidPath(e.into()))?;
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
    std::fs::write(&tmp_path, file_content).map_err(|e| SaveDeviceError::InvalidPath(e.into()))?;
    std::fs::rename(&tmp_path, key_file).map_err(|e| SaveDeviceError::InvalidPath(e.into()))?;

    Ok(())
}

fn generate_keyring_user(
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
    save_content(keyring_user_path, keyring_user.as_bytes())?;

    Ok((key, keyring_user))
}

pub async fn save_device(
    access: &DeviceAccessStrategy,
    device: &LocalDevice,
) -> Result<(), SaveDeviceError> {
    match access {
        DeviceAccessStrategy::Keyring { key_file } => {
            let keyring_user_path = get_default_data_dir().join("keyring_user.txt");

            let (key, keyring_user) = std::fs::read_to_string(&keyring_user_path)
                .ok()
                .and_then(|keyring_user| {
                    KeyringEntry::new(KEYRING_SERVICE, &keyring_user)
                        .map(|x| (x, keyring_user))
                        .ok()
                })
                .and_then(|(entry, keyring_user)| {
                    entry.get_password().map(|x| (x, keyring_user)).ok()
                })
                .and_then(|(passphrase, keyring_user)| {
                    SecretKey::from_recovery_passphrase(passphrase.into())
                        .map(|x| (x, keyring_user))
                        .ok()
                })
                .unwrap_or(generate_keyring_user(&keyring_user_path)?);

            let cleartext = device.dump();
            let ciphertext = key.encrypt(&cleartext);
            let file_content = DeviceFile::Keyring(DeviceFileKeyring {
                ciphertext: ciphertext.into(),
                human_handle: device.human_handle.clone(),
                device_label: device.device_label.clone(),
                device_id: device.device_id.clone(),
                organization_id: device.organization_id().clone(),
                slug: device.slug(),
                keyring_service: KEYRING_SERVICE.into(),
                keyring_user,
            });

            let file_content = file_content.dump();

            save_content(key_file, &file_content)
        }

        DeviceAccessStrategy::Password { key_file, password } => {
            let salt = SecretKey::generate_salt();
            let key =
                SecretKey::from_password(password, &salt).expect("Salt has the correct length");

            let ciphertext = {
                let mut cleartext = device.dump();
                let ciphertext = key.encrypt(&cleartext);
                cleartext.zeroize(); // Scrub the buffer given it contains keys in clear
                ciphertext.into()
            };

            let file_content = DeviceFile::Password(DeviceFilePassword {
                ciphertext,
                human_handle: device.human_handle.to_owned(),
                device_label: device.device_label.to_owned(),
                device_id: device.device_id.to_owned(),
                organization_id: device.organization_id().to_owned(),
                slug: device.slug(),
                salt: salt.into(),
            });

            let file_content = file_content.dump();

            save_content(key_file, &file_content)
        }

        DeviceAccessStrategy::Smartcard { .. } => {
            // TODO
            todo!()
        }
    }
}

pub async fn change_authentication(
    current_access: &DeviceAccessStrategy,
    new_access: &DeviceAccessStrategy,
) -> Result<(), ChangeAuthentificationError> {
    let device = load_device(current_access).await?;

    save_device(new_access, &device).await?;

    let key_file = current_access.key_file();
    let new_key_file = new_access.key_file();

    if key_file != new_key_file {
        std::fs::remove_file(key_file)
            .map_err(|_| ChangeAuthentificationError::CannotRemoveOldDevice)?;
    }

    Ok(())
}

/*
 * Recovery
 */

pub async fn load_recovery_device(
    key_file: &Path,
    passphrase: SecretKeyPassphrase,
) -> Result<LocalDevice, LoadRecoveryDeviceError> {
    let key = SecretKey::from_recovery_passphrase(passphrase)
        .map_err(|_| LoadRecoveryDeviceError::InvalidPassphrase)?;

    // TODO: make file access on a worker thread !
    let content =
        std::fs::read(key_file).map_err(|e| LoadRecoveryDeviceError::InvalidPath(e.into()))?;

    // Regular load
    let device_file =
        DeviceFile::load(&content).map_err(|_| LoadRecoveryDeviceError::InvalidData)?;

    let device = match device_file {
        DeviceFile::Recovery(x) => {
            let mut cleartext = key
                .decrypt(&x.ciphertext)
                .map_err(|_| LoadRecoveryDeviceError::DecryptionFailed)?;
            let device =
                LocalDevice::load(&cleartext).map_err(|_| LoadRecoveryDeviceError::InvalidData)?;
            cleartext.zeroize(); // Scrub the buffer given it contains keys in clear
            device
        }
        // We are not expecting other type of device file
        _ => return Err(LoadRecoveryDeviceError::InvalidData),
    };

    Ok(device)
}

pub async fn save_recovery_device(
    key_file: &Path,
    device: &LocalDevice,
) -> Result<SecretKeyPassphrase, SaveRecoveryDeviceError> {
    let (passphrase, key) = SecretKey::generate_recovery_passphrase();

    let ciphertext = {
        let mut cleartext = device.dump();
        let ciphertext = key.encrypt(&cleartext);
        cleartext.zeroize(); // Scrub the buffer given it contains keys in clear
        ciphertext.into()
    };

    let file_content = DeviceFile::Recovery(DeviceFileRecovery {
        ciphertext,
        human_handle: device.human_handle.to_owned(),
        device_label: device.device_label.to_owned(),
        device_id: device.device_id.to_owned(),
        organization_id: device.organization_id().to_owned(),
        slug: device.slug(),
    })
    .dump();

    if let Some(parent) = key_file.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| SaveRecoveryDeviceError::InvalidPath(e.into()))?;
    }
    let tmp_path = match key_file.file_name() {
        Some(file_name) => {
            let mut tmp_path = key_file.to_owned();
            {
                let mut tmp_file_name = file_name.to_owned();
                tmp_file_name.push(".tmp");
                tmp_path.set_file_name(tmp_file_name);
            }
            tmp_path
        }
        None => {
            return Err(SaveRecoveryDeviceError::InvalidPath(anyhow::anyhow!(
                "Path is missing a file name"
            )))
        }
    };

    // Classic pattern for atomic file creation:
    // - First write the file in a temporary location
    // - Then move the file to it final location
    // This way a crash during file write won't end up with a corrupted
    // file in the final location.
    std::fs::write(&tmp_path, file_content)
        .map_err(|e| SaveRecoveryDeviceError::InvalidPath(e.into()))?;
    std::fs::rename(&tmp_path, key_file)
        .map_err(|e| SaveRecoveryDeviceError::InvalidPath(e.into()))?;

    Ok(passphrase)
}

pub fn is_keyring_available() -> bool {
    // Using "tmp" as user, because keyring-rs forbids the use of empty string
    // due to an issue in macOS. See: https://github.com/hwchen/keyring-rs/pull/87
    match KeyringEntry::new(KEYRING_SERVICE, "tmp").and_then(|x| x.get_password()) {
        Ok(_) => true,
        Err(keyring::error::Error::NoEntry) => true,
        Err(err) => {
            log::warn!("Keyring is not available: {err:?}");
            false
        }
    }
}
