// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use keyring::Entry as KeyringEntry;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};
use uuid::Uuid;
use zeroize::{Zeroize, Zeroizing};

use libparsec_types::prelude::*;

use crate::{
    ChangeAuthentificationError, LoadDeviceError, SaveDeviceError, ARGON2ID_DEFAULT_MEMLIMIT_KB,
    ARGON2ID_DEFAULT_OPSLIMIT, ARGON2ID_DEFAULT_PARALLELISM, DEVICE_FILE_EXT,
};

#[cfg(target_os = "windows")]
mod biometrics;

const KEYRING_SERVICE: &str = "parsec";

#[cfg(target_os = "windows")]
const BIOMETRICS_SERVICE: &str = "parsec";

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
            if existing.device_id == device.device_id {
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

    let device_file =
        DeviceFile::load(&content).map_err(|_| LoadAvailableDeviceFileError::InvalidData)?;

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
        DeviceFile::Biometrics(device) => (
            DeviceFileType::Biometrics,
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
        key_file_path,
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

/*
 * Save & load
 */

pub async fn load_device(
    access: &DeviceAccessStrategy,
) -> Result<(Arc<LocalDevice>, DateTime), LoadDeviceError> {
    let (device, created_on) = match access {
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

                (device, x.created_on)
            } else {
                return Err(LoadDeviceError::InvalidData);
            }
        }

        DeviceAccessStrategy::Password { key_file, password } => {
            // TODO: make file access on a worker thread !
            let content =
                std::fs::read(key_file).map_err(|e| LoadDeviceError::InvalidPath(e.into()))?;

            let device_file =
                DeviceFile::load(&content).map_err(|_| LoadDeviceError::InvalidData)?;

            match device_file {
                DeviceFile::Password(x) => {
                    let (salt, opslimit, memlimit_kb, parallelism) = match x.algorithm {
                        DeviceFilePasswordAlgorithm::Argon2id {
                            salt,
                            opslimit,
                            memlimit_kb,
                            parallelism,
                        } => {
                            let opslimit: u32 = opslimit
                                .try_into()
                                .map_err(|_| LoadDeviceError::InvalidData)?;
                            let memlimit_kb: u32 = memlimit_kb
                                .try_into()
                                .map_err(|_| LoadDeviceError::InvalidData)?;
                            let parallelism: u32 = parallelism
                                .try_into()
                                .map_err(|_| LoadDeviceError::InvalidData)?;
                            (salt, opslimit, memlimit_kb, parallelism)
                        }
                    };
                    let key = SecretKey::from_argon2id_password(
                        password,
                        &salt,
                        opslimit,
                        memlimit_kb,
                        parallelism,
                    )
                    .map_err(|_| LoadDeviceError::InvalidData)?;
                    let cleartext = key
                        .decrypt(&x.ciphertext)
                        .map(Zeroizing::new)
                        .map_err(|_| LoadDeviceError::DecryptionFailed)?;
                    let device =
                        LocalDevice::load(&cleartext).map_err(|_| LoadDeviceError::InvalidData)?;

                    (device, x.created_on)
                }
                // We are not expecting other type of device file
                _ => return Err(LoadDeviceError::InvalidData),
            }
        }

        DeviceAccessStrategy::Biometrics { key_file } => {
            #[cfg(not(target_os = "windows"))]
            {
                return Err(LoadDeviceError::Internal(anyhow::anyhow!(
                    "Biometrics are not available on this platform, cannot load path {key_file:?}"
                )));
            }
            #[cfg(target_os = "windows")]
            {
                use biometrics::derive_key_from_biometrics;

                // TODO: make file access on a worker thread !
                let content =
                    std::fs::read(key_file).map_err(|e| LoadDeviceError::InvalidPath(e.into()))?;

                // Regular load
                let device_file =
                    DeviceFile::load(&content).map_err(|_| LoadDeviceError::InvalidData)?;

                if let DeviceFile::Biometrics(x) = device_file {
                    let key = derive_key_from_biometrics(&x.biometrics_service, x.device_id)
                        .map_err(LoadDeviceError::Internal)?;
                    let mut cleartext = key
                        .decrypt(&x.ciphertext)
                        .map_err(|_| LoadDeviceError::DecryptionFailed)?;

                    let device =
                        LocalDevice::load(&cleartext).map_err(|_| LoadDeviceError::InvalidData)?;

                    cleartext.zeroize();

                    (device, x.created_on)
                } else {
                    return Err(LoadDeviceError::InvalidData);
                }
            }
        }

        DeviceAccessStrategy::Smartcard { .. } => {
            // TODO !
            todo!()
        }
    };

    Ok((Arc::new(device), created_on))
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
    created_on: DateTime,
) -> Result<AvailableDevice, SaveDeviceError> {
    let protected_on = device.now();
    let server_url = {
        ParsecAddr::new(
            device.organization_addr.hostname().to_owned(),
            Some(device.organization_addr.port()),
            device.organization_addr.use_ssl(),
        )
        .to_http_url(None)
        .to_string()
    };

    match access {
        DeviceAccessStrategy::Keyring { key_file } => {
            let keyring_user_path = crate::get_default_data_base_dir().join("keyring_user.txt");

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
                ciphertext: ciphertext.into(),
            });

            let file_content = file_content.dump();

            save_content(key_file, &file_content)?;
        }

        DeviceAccessStrategy::Password { key_file, password } => {
            let salt = SecretKey::generate_salt();
            let opslimit = ARGON2ID_DEFAULT_OPSLIMIT;
            let memlimit_kb = ARGON2ID_DEFAULT_MEMLIMIT_KB;
            let parallelism = ARGON2ID_DEFAULT_PARALLELISM;

            let key = SecretKey::from_argon2id_password(
                password,
                &salt,
                opslimit,
                memlimit_kb,
                parallelism,
            )
            .expect("Salt has the correct length");

            let ciphertext = {
                let cleartext = Zeroizing::new(device.dump());
                let ciphertext = key.encrypt(&cleartext);
                ciphertext.into()
            };

            let file_content = DeviceFile::Password(DeviceFilePassword {
                created_on,
                protected_on,
                server_url: server_url.clone(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.to_owned(),
                device_label: device.device_label.to_owned(),
                algorithm: DeviceFilePasswordAlgorithm::Argon2id {
                    salt: salt.into(),
                    opslimit: opslimit.into(),
                    memlimit_kb: memlimit_kb.into(),
                    parallelism: parallelism.into(),
                },
                ciphertext,
            });

            let file_content = file_content.dump();

            save_content(key_file, &file_content)?;
        }

        DeviceAccessStrategy::Biometrics { key_file } => {
            #[cfg(not(target_os = "windows"))]
            {
                return Err(SaveDeviceError::Internal(anyhow::anyhow!(
                    "Biometrics are not available on this platform, cannot save to path {key_file:?}"
                )));
            }
            #[cfg(target_os = "windows")]
            {
                use biometrics::derive_key_from_biometrics;
                let key = derive_key_from_biometrics(BIOMETRICS_SERVICE, device.device_id)
                    .map_err(SaveDeviceError::Internal)?;
                let cleartext = device.dump();
                let ciphertext = key.encrypt(&cleartext);

                let file_content = DeviceFile::Biometrics(DeviceFileBiometrics {
                    created_on,
                    protected_on,
                    server_url: server_url.clone(),
                    organization_id: device.organization_id().clone(),
                    user_id: device.user_id,
                    device_id: device.device_id,
                    human_handle: device.human_handle.clone(),
                    device_label: device.device_label.clone(),
                    biometrics_service: BIOMETRICS_SERVICE.into(),
                    ciphertext: ciphertext.into(),
                });

                let file_content = file_content.dump();

                save_content(key_file, &file_content)?;
            }
        }

        DeviceAccessStrategy::Smartcard { .. } => {
            // TODO
            todo!()
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
        std::fs::remove_file(key_file)
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

#[cfg(not(target_os = "windows"))]
#[allow(unused)]
pub fn is_biometrics_available() -> bool {
    false
}

#[cfg(target_os = "windows")]
pub use biometrics::is_biometrics_available;
