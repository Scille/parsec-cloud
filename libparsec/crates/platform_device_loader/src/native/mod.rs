// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use keyring::Entry as KeyringEntry;
use libparsec_platform_async::future::FutureExt as _;
use libparsec_platform_filesystem::save_content;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::{
    encrypt_device, get_device_archive_path, AccountVaultOperationsFetchOpaqueKeyError,
    AccountVaultOperationsUploadOpaqueKeyError, ArchiveDeviceError, AvailableDevice,
    DeviceAccessStrategy, DeviceSaveStrategy, LoadCiphertextKeyError, LoadDeviceError,
    OpenBaoOperationsFetchOpaqueKeyError, OpenBaoOperationsUploadOpaqueKeyError,
    RemoteOperationServer, RemoveDeviceError, SaveDeviceError, UpdateDeviceError,
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
 * Save & load
 */

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
                log::trace!(
                    "Creating keyring entry (service={service}, user={user})",
                    service = device.keyring_service,
                    user = device.keyring_user
                );
                let entry = KeyringEntry::new(&device.keyring_service, &device.keyring_user)
                    .map_err(|e| {
                        LoadCiphertextKeyError::Internal(anyhow::anyhow!("OS Keyring error: {e}"))
                    })?;

                log::trace!("Retrieving passphrase from keyring entry");
                let passphrase = entry
                    .get_password()
                    .map_err(|e| {
                        LoadCiphertextKeyError::Internal(anyhow::anyhow!(
                            "Cannot retrieve password from OS Keyring: {e}"
                        ))
                    })?
                    .into();

                log::trace!("Obtained passphrase from keyring entry");
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

        DeviceAccessStrategy::PKI { .. } => {
            if let DeviceFile::PKI(device) = device_file {
                Ok(decrypt_message(
                    device.algorithm,
                    device.encrypted_key.as_ref(),
                    &device.certificate_ref,
                )
                .map_err(|_| LoadCiphertextKeyError::InvalidData)?
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
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed {
                                server: RemoteOperationServer::ParsecAccount,
                                error: err.into(),
                            }
                        }
                        AccountVaultOperationsFetchOpaqueKeyError::Offline(_) => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline {
                                server: RemoteOperationServer::ParsecAccount,
                                error: err.into(),
                            }
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

        DeviceAccessStrategy::OpenBao { operations, .. } => {
            if let DeviceFile::OpenBao(device) = device_file {
                let ciphertext_key = operations
                    .fetch_opaque_key(device.openbao_ciphertext_key_path.clone())
                    .await
                    .map_err(|err| match err {
                        err @ (OpenBaoOperationsFetchOpaqueKeyError::BadURL(_)
                        | OpenBaoOperationsFetchOpaqueKeyError::BadServerResponse(_)) => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed {
                                server: RemoteOperationServer::OpenBao,
                                error: err.into(),
                            }
                        }
                        OpenBaoOperationsFetchOpaqueKeyError::NoServerResponse(_) => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline {
                                server: RemoteOperationServer::OpenBao,
                                error: err.into(),
                            }
                        }
                    })?;
                Ok(ciphertext_key)
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }
    }
}

async fn generate_keyring_user(
    keyring_user_path: &Path,
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
    let server_addr: ParsecAddr = device.organization_addr.clone().into();

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
                server_url: server_addr.clone(),
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
                server_url: server_addr.clone(),
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

        DeviceSaveStrategy::PKI { certificate_ref } => {
            // Generate a random key
            let secret_key = SecretKey::generate();

            // Encrypt the key using the public key related to a certificate from the store
            let (algorithm, encrypted_key) = encrypt_message(secret_key.as_ref(), certificate_ref)
                .map_err(|e| SaveDeviceError::Internal(e.into()))?;

            // May check if we are able to decrypt the encrypted key from the previous step
            assert_eq!(
                decrypt_message(algorithm, &encrypted_key, certificate_ref,)
                    .map_err(|e| SaveDeviceError::Internal(e.into()))?
                    .as_ref(),
                secret_key.as_ref()
            );

            // Use the generated key to encrypt the device content
            let ciphertext = encrypt_device(device, &secret_key);

            // Save
            let file_content = DeviceFile::PKI(DeviceFilePKI {
                created_on,
                protected_on,
                server_url: server_addr.clone(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.to_owned(),
                device_label: device.device_label.to_owned(),
                certificate_ref: certificate_ref.to_owned(),
                encrypted_key,
                ciphertext,
                algorithm,
            });

            let file_content = file_content.dump();

            save_content(&key_file, &file_content).await?;
        }

        DeviceSaveStrategy::AccountVault { operations } => {
            let (ciphertext_key_id, ciphertext_key) = operations
                .upload_opaque_key()
                .await
                .map_err(|err| match err {
                    AccountVaultOperationsUploadOpaqueKeyError::BadVaultKeyAccess(_)
                    | AccountVaultOperationsUploadOpaqueKeyError::BadServerResponse(_) => {
                        SaveDeviceError::RemoteOpaqueKeyUploadFailed {
                            server: RemoteOperationServer::ParsecAccount,
                            error: err.into(),
                        }
                    }
                    AccountVaultOperationsUploadOpaqueKeyError::Offline(_) => {
                        SaveDeviceError::RemoteOpaqueKeyUploadOffline {
                            server: RemoteOperationServer::ParsecAccount,
                            error: err.into(),
                        }
                    }
                })?;

            let ciphertext = super::encrypt_device(device, &ciphertext_key);

            let file_content = DeviceFile::AccountVault(DeviceFileAccountVault {
                created_on,
                protected_on,
                server_url: server_addr.clone(),
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

        DeviceSaveStrategy::OpenBao { operations } => {
            let (openbao_ciphertext_key_path, ciphertext_key) = operations
                .upload_opaque_key()
                .await
                .map_err(|err| match err {
                    OpenBaoOperationsUploadOpaqueKeyError::NoServerResponse(_) => {
                        SaveDeviceError::RemoteOpaqueKeyUploadOffline {
                            server: RemoteOperationServer::OpenBao,
                            error: err.into(),
                        }
                    }
                    OpenBaoOperationsUploadOpaqueKeyError::BadURL(_)
                    | OpenBaoOperationsUploadOpaqueKeyError::BadServerResponse(_) => {
                        SaveDeviceError::RemoteOpaqueKeyUploadFailed {
                            server: RemoteOperationServer::OpenBao,
                            error: err.into(),
                        }
                    }
                })?;

            let ciphertext = super::encrypt_device(device, &ciphertext_key);

            let file_content = DeviceFile::OpenBao(DeviceFileOpenBao {
                created_on,
                protected_on,
                server_url: server_addr.clone(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.to_owned(),
                device_label: device.device_label.to_owned(),
                openbao_preferred_auth_id: operations.openbao_preferred_auth_id().to_owned(),
                openbao_entity_id: operations.openbao_entity_id().to_owned(),
                openbao_ciphertext_key_path,
                ciphertext,
            });

            let file_content = file_content.dump();

            save_content(&key_file, &file_content).await?;
        }
    }

    Ok(AvailableDevice {
        key_file_path: key_file,
        server_addr,
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
        save_device(new_strategy, device, created_on, new_key_file.to_path_buf()).await?;

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
        .map_err(Into::into)
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
