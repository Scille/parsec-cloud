// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use itertools::Itertools as _;
use keyring::Entry as KeyringEntry;
use libparsec_platform_async::future::FutureExt as _;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::{
    encrypt_device, get_device_archive_path, AccountVaultOperationsFetchOpaqueKeyError,
    AccountVaultOperationsUploadOpaqueKeyError, ArchiveDeviceError, AvailableDevice,
    DeviceAccessStrategy, DeviceSaveStrategy, ListAvailableDeviceError, LoadCiphertextKeyError,
    LoadDeviceError, ReadFileError, RemoveDeviceError, SaveDeviceError, UpdateDeviceError,
    DEVICE_FILE_EXT,
};
use libparsec_platform_pki::{decrypt_message, encrypt_message};
use libparsec_types::prelude::*;

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

fn find_device_files(path: &Path) -> Result<Vec<PathBuf>, ListAvailableDeviceError> {
    // TODO: make file access on a worker thread !

    match std::fs::read_dir(path) {
        Ok(children) => {
            let mut key_file_paths = vec![];
            for path in
                children.filter_map(|entry| entry.as_ref().map(std::fs::DirEntry::path).ok())
            {
                if path.extension() == Some(DEVICE_FILE_EXT.as_ref()) {
                    key_file_paths.push(path)
                } else if path.is_dir() {
                    key_file_paths.append(&mut find_device_files(&path)?)
                }
            }
            Ok(key_file_paths)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => {
            log::error!("Cannot list available devices at {}: {e}", path.display());
            Err(ListAvailableDeviceError::StorageNotAvailable)
        }
    }
}

pub(super) async fn list_available_devices(
    config_dir: &Path,
) -> Result<Vec<AvailableDevice>, ListAvailableDeviceError> {
    let devices_dir = crate::get_devices_dir(config_dir);

    // Consider `.keys` files in devices directory
    let mut key_file_paths = find_device_files(&devices_dir)?;

    // Sort paths so the discovery order is deterministic
    // In the case of duplicate files, that means only the first discovered device is considered
    key_file_paths.sort();
    log::trace!("Found the following device files: {key_file_paths:?}");

    Ok(key_file_paths
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
        .collect())
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

pub(super) async fn read_file(file: &Path) -> Result<Vec<u8>, ReadFileError> {
    tokio::fs::read(file)
        .await
        .map_err(|e| ReadFileError::Internal(e.into()))
}

pub(super) async fn load_ciphertext_key(
    access: &DeviceAccessStrategy,
    device_file: &DeviceFile,
) -> Result<SecretKey, LoadCiphertextKeyError> {
    // Don't do `match (access, device_file)` since we would end up with a catch-all
    // `(_, _) => return <error>` condition that would prevent this code from breaking
    // whenever a new variant is introduced (hence hiding the fact this code has
    // to be updated).
    match access {
        DeviceAccessStrategy::Keyring { .. } => {
            if let DeviceFile::Keyring(device) = device_file {
                let entry = KeyringEntry::new(&device.keyring_service, &device.keyring_user)
                    .map_err(|e| {
                        LoadCiphertextKeyError::Internal(anyhow::anyhow!("OS Keyring error: {e}"))
                    })?;

                let passphrase = entry
                    .get_password()
                    .map_err(|e| {
                        LoadCiphertextKeyError::Internal(anyhow::anyhow!(
                            "Cannot retrieve password from OS Keyring: {e}"
                        ))
                    })?
                    .into();

                let key = SecretKey::from_recovery_passphrase(passphrase)
                    .map_err(|_| LoadCiphertextKeyError::DecryptionFailed)?;

                Ok(key)
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }

        DeviceAccessStrategy::Password { password, .. } => {
            if let DeviceFile::Password(device) = device_file {
                let key = device
                    .algorithm
                    .compute_secret_key(password)
                    .map_err(|_| LoadCiphertextKeyError::InvalidData)?;

                Ok(key)
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }

        DeviceAccessStrategy::Smartcard { .. } => {
            if let DeviceFile::Smartcard(device) = device_file {
                Ok(decrypt_message(
                    device.algorithm_for_encrypted_key,
                    device.encrypted_key.as_ref(),
                    &device.certificate_ref,
                )
                .map_err(|_| LoadCiphertextKeyError::InvalidData)?
                .data
                .as_ref()
                .try_into()
                .map_err(|_| LoadCiphertextKeyError::InvalidData)?)
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }

        DeviceAccessStrategy::AccountVault { operations, .. } => {
            if let DeviceFile::AccountVault(device) = device_file {
                let ciphertext_key = operations
                    .fetch_opaque_key(device.ciphertext_key_id)
                    .await
                    .map_err(|err| match err {
                        AccountVaultOperationsFetchOpaqueKeyError::BadVaultKeyAccess(_)
                        | AccountVaultOperationsFetchOpaqueKeyError::UnknownOpaqueKey
                        | AccountVaultOperationsFetchOpaqueKeyError::CorruptedOpaqueKey => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed(err.into())
                        }
                        AccountVaultOperationsFetchOpaqueKeyError::Offline(err) => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline(err)
                        }
                        AccountVaultOperationsFetchOpaqueKeyError::Internal(err) => {
                            LoadCiphertextKeyError::Internal(err)
                        }
                    })?;
                Ok(ciphertext_key)
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }
    }
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

pub(super) async fn save_device(
    strategy: &DeviceSaveStrategy,
    device: &LocalDevice,
    created_on: DateTime,
    key_file: PathBuf,
) -> Result<AvailableDevice, SaveDeviceError> {
    let protected_on = device.now();
    let server_url = super::server_url_from_device(device);

    match strategy {
        DeviceSaveStrategy::Keyring => {
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

            save_content(&key_file, &file_content).await?;
        }

        DeviceSaveStrategy::Password { password } => {
            let key_algo =
                PasswordAlgorithm::generate_argon2id(PasswordAlgorithmSaltStrategy::Random);
            let key = key_algo
                .compute_secret_key(password)
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

            save_content(&key_file, &file_content).await?;
        }

        DeviceSaveStrategy::Smartcard {
            certificate_reference,
        } => {
            // Generate a random key
            let secret_key = SecretKey::generate();

            // Encrypt the key using the public key related to a certificate from the store
            let encrypted_message = encrypt_message(secret_key.as_ref(), certificate_reference)
                .map_err(|e| SaveDeviceError::Internal(e.into()))?;

            // May check if we are able to decrypt the encrypted key from the previous step
            assert_eq!(
                decrypt_message(
                    encrypted_message.algo,
                    &encrypted_message.ciphered,
                    certificate_reference,
                )
                .map_err(|e| SaveDeviceError::Internal(e.into()))?
                .data
                .as_ref(),
                secret_key.as_ref()
            );

            // Use the generated key to encrypt the device content
            let ciphertext = encrypt_device(device, &secret_key);

            // Save
            let file_content = DeviceFile::Smartcard(DeviceFileSmartcard {
                created_on,
                protected_on,
                server_url: server_url.clone(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.to_owned(),
                device_label: device.device_label.to_owned(),
                certificate_ref: encrypted_message.cert_ref,
                encrypted_key: encrypted_message.ciphered,
                ciphertext,
                algorithm_for_encrypted_key: encrypted_message.algo,
            });

            let file_content = file_content.dump();

            save_content(&key_file, &file_content).await?;
        }

        DeviceSaveStrategy::AccountVault { operations } => {
            let (ciphertext_key_id, ciphertext_key) = operations
                .upload_opaque_key(device.organization_id().to_owned())
                .await
                .map_err(|err| match err {
                    err @ (
                        AccountVaultOperationsUploadOpaqueKeyError::NotAllowedByOrganizationVaultStrategy
                        | AccountVaultOperationsUploadOpaqueKeyError::CannotObtainOrganizationVaultStrategy
                        | AccountVaultOperationsUploadOpaqueKeyError::BadVaultKeyAccess(_)
                    ) => {
                        SaveDeviceError::RemoteOpaqueKeyUploadFailed(err.into())
                    }
                    AccountVaultOperationsUploadOpaqueKeyError::Offline(err) => {
                        SaveDeviceError::RemoteOpaqueKeyUploadOffline(err)
                    }
                    AccountVaultOperationsUploadOpaqueKeyError::Internal(err) => {
                        SaveDeviceError::Internal(err)
                    }
                })?;

            let ciphertext = super::encrypt_device(device, &ciphertext_key);

            let file_content = DeviceFile::AccountVault(DeviceFileAccountVault {
                created_on,
                protected_on,
                server_url: server_url.clone(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.to_owned(),
                device_label: device.device_label.to_owned(),
                ciphertext_key_id,
                ciphertext,
            });

            let file_content = file_content.dump();

            save_content(&key_file, &file_content).await?;
        }
    }

    Ok(AvailableDevice {
        key_file_path: key_file,
        server_url,
        created_on,
        protected_on,
        organization_id: device.organization_id().to_owned(),
        user_id: device.user_id,
        device_id: device.device_id,
        device_label: device.device_label.clone(),
        human_handle: device.human_handle.clone(),
        ty: strategy.ty(),
    })
}

pub(super) async fn update_device(
    device: &LocalDevice,
    created_on: DateTime,
    current_key_file: &Path,
    new_strategy: &DeviceSaveStrategy,
    new_key_file: &Path,
) -> Result<AvailableDevice, UpdateDeviceError> {
    let available_device =
        save_device(new_strategy, device, created_on, new_key_file.to_path_buf())
            .await
            .map_err(|err| match err {
                SaveDeviceError::StorageNotAvailable => UpdateDeviceError::StorageNotAvailable,
                SaveDeviceError::InvalidPath(err) => UpdateDeviceError::InvalidPath(err),
                SaveDeviceError::Internal(err) => UpdateDeviceError::Internal(err),
                SaveDeviceError::RemoteOpaqueKeyUploadOffline(err) => {
                    UpdateDeviceError::RemoteOpaqueKeyOperationOffline(err)
                }
                SaveDeviceError::RemoteOpaqueKeyUploadFailed(err) => {
                    UpdateDeviceError::RemoteOpaqueKeyOperationFailed(err)
                }
            })?;

    if current_key_file != new_key_file {
        if let Err(err) = tokio::fs::remove_file(current_key_file).await {
            log::warn!("Cannot remove old key file {current_key_file:?}: {err}");
        }
    }

    Ok(available_device)
}

pub(super) async fn archive_device(device_path: &Path) -> Result<(), ArchiveDeviceError> {
    let archive_device_path = get_device_archive_path(device_path);

    log::debug!(
        "Archiving device {} to {}",
        device_path.display(),
        archive_device_path.display()
    );

    tokio::fs::rename(device_path, archive_device_path)
        .await
        .map_err(|e| ArchiveDeviceError::Internal(e.into()))
}

pub(super) async fn remove_device(device_path: &Path) -> Result<(), RemoveDeviceError> {
    log::debug!("Removing device {}", device_path.display());

    tokio::fs::remove_file(device_path)
        .await
        .map_err(|e| RemoveDeviceError::Internal(e.into()))
}

pub(super) fn is_keyring_available() -> bool {
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
