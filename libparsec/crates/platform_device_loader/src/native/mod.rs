// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use keyring::Entry as KeyringEntry;
use libparsec_platform_async::future::FutureExt as _;
use libparsec_platform_filesystem::save_content;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::{
    encrypt_device, LoadCiphertextKeyError, LoadDeviceError, SaveDeviceError,
    PARSEC_BASE_CONFIG_DIR, PARSEC_BASE_DATA_DIR, PARSEC_BASE_HOME_DIR,
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

pub(crate) async fn save_device_keyring(
    device: &LocalDevice,
    created_on: &DateTime,
    key_file: &Path,
    server_addr: &ParsecAddr,
    protected_on: &DateTime,
) -> Result<(), SaveDeviceError> {
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
        created_on: *created_on,
        protected_on: *protected_on,
        server_url: server_addr.clone(),
        organization_id: device.organization_id().clone(),
        user_id: device.user_id,
        device_id: device.device_id,
        human_handle: device.human_handle.clone(),
        device_label: device.device_label.clone(),
        keyring_service: KEYRING_SERVICE.into(),
        keyring_user,
        ciphertext,
        totp_opaque_key_id: None,
    });

    let file_content = file_content.dump();

    save_content(key_file, &file_content).await?;
    Ok(())
}

pub(crate) async fn save_device_pki(
    device: &LocalDevice,
    created_on: &DateTime,
    key_file: &Path,
    server_addr: &ParsecAddr,
    protected_on: &DateTime,
    certificate_ref: &X509CertificateReference,
) -> Result<(), SaveDeviceError> {
    // Generate a random key
    let secret_key = SecretKey::generate();

    // Encrypt the key using the public key related to a certificate from the store
    let (algorithm, encrypted_key) = encrypt_message(secret_key.as_ref(), certificate_ref)
        .await
        .map_err(|e| SaveDeviceError::Internal(e.into()))?;

    // May check if we are able to decrypt the encrypted key from the previous step
    assert_eq!(
        decrypt_message(algorithm, &encrypted_key, certificate_ref)
            .await
            .map_err(|e| SaveDeviceError::Internal(e.into()))?
            .as_ref(),
        secret_key.as_ref()
    );

    // Use the generated key to encrypt the device content
    let ciphertext = encrypt_device(device, &secret_key);

    // Save
    let file_content = DeviceFile::PKI(DeviceFilePKI {
        created_on: *created_on,
        protected_on: *protected_on,
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
        totp_opaque_key_id: None,
    });

    let file_content = file_content.dump();

    save_content(key_file, &file_content).await?;
    Ok(())
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

pub(super) async fn load_ciphertext_key_keyring(
    device_file: &DeviceFile,
) -> Result<SecretKey, LoadCiphertextKeyError> {
    if let DeviceFile::Keyring(device) = device_file {
        log::trace!(
            "Creating keyring entry (service={service}, user={user})",
            service = device.keyring_service,
            user = device.keyring_user
        );
        let entry =
            KeyringEntry::new(&device.keyring_service, &device.keyring_user).map_err(|e| {
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

pub(super) async fn load_ciphertext_key_pki(
    device_file: &DeviceFile,
) -> Result<SecretKey, LoadCiphertextKeyError> {
    if let DeviceFile::PKI(device) = device_file {
        decrypt_message(
            device.algorithm,
            device.encrypted_key.as_ref(),
            &device.certificate_ref,
        )
        .await
        .map_err(|_| LoadCiphertextKeyError::InvalidData)?
        .as_ref()
        .try_into()
        .map_err(|_| LoadCiphertextKeyError::InvalidData)
    } else {
        Err(LoadCiphertextKeyError::InvalidData)
    }
}

pub(super) fn get_default_data_base_dir() -> PathBuf {
    let mut path = if let Ok(data_dir) = std::env::var(PARSEC_BASE_DATA_DIR) {
        PathBuf::from(data_dir)
    } else {
        dirs::data_dir().expect("Could not determine base data directory")
    };

    path.push("parsec3");
    path
}

pub(super) fn get_default_config_dir() -> PathBuf {
    let mut path = if let Ok(config_dir) = std::env::var(PARSEC_BASE_CONFIG_DIR) {
        PathBuf::from(config_dir)
    } else {
        dirs::config_dir().expect("Could not determine base config directory")
    };

    path.push("parsec3/libparsec");
    path
}

pub(super) fn get_default_mountpoint_base_dir() -> PathBuf {
    let mut path = if let Ok(home_dir) = std::env::var(PARSEC_BASE_HOME_DIR) {
        PathBuf::from(home_dir)
    } else {
        dirs::home_dir().expect("Could not determine home directory")
    };

    path.push("Parsec3");
    path
}
