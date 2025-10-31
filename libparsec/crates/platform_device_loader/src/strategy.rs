// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec_client_connection::ConnectionError;
use libparsec_crypto::{Password, SecretKey};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AccountVaultOperationsFetchOpaqueKeyError {
    #[error("Cannot decrypt the vault key access returned by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("No opaque key with this ID among the vault items")]
    UnknownOpaqueKey,
    #[error("The vault item containing this opaque key is corrupted")]
    CorruptedOpaqueKey,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AccountVaultOperationsUploadOpaqueKeyError {
    #[error("Cannot decrypt the vault key access returned by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

// Note we cannot use `async fn` in the trait since it is not compatible
// with dyn object (and we need to store the object implementing the trait
// as `Arc<dyn AccountVaultOperations>`).
pub type AccountVaultOperationsFutureResult<O, E> =
    std::pin::Pin<Box<dyn std::future::Future<Output = Result<O, E>> + Send>>;

/// This trait is needed to break a recursive dependency:
/// - `libparsec_platform_device_loader` depends on `libparsec_account` to
///   upload/fetch opaque keys in order to save/load a device file.
/// - `libparsec_account` depends on `libparsec_platform_device_loader` to
///   create new device from a recovery device.
///
/// So we introduce the `libparsec` main crate that will bind together the two
/// other crates by implementing this trait (see `libparsec/src/device.rs`).
pub trait AccountVaultOperations: std::fmt::Debug + Send + Sync {
    fn account_email(&self) -> &EmailAddress;
    fn fetch_opaque_key(
        &self,
        ciphertext_key_id: AccountVaultItemOpaqueKeyID,
    ) -> AccountVaultOperationsFutureResult<SecretKey, AccountVaultOperationsFetchOpaqueKeyError>;
    fn upload_opaque_key(
        &self,
    ) -> AccountVaultOperationsFutureResult<
        (AccountVaultItemOpaqueKeyID, SecretKey),
        AccountVaultOperationsUploadOpaqueKeyError,
    >;
}

/// Represent how (not where) to save a device file
#[derive(Debug, Clone)]
pub enum DeviceSaveStrategy {
    Keyring,
    Password {
        password: Password,
    },
    Smartcard {
        certificate_reference: X509CertificateReference,
    },
    AccountVault {
        operations: Arc<dyn AccountVaultOperations>,
    },
}

impl DeviceSaveStrategy {
    pub fn into_access(self, key_file: PathBuf) -> DeviceAccessStrategy {
        match self {
            DeviceSaveStrategy::Keyring => DeviceAccessStrategy::Keyring { key_file },
            DeviceSaveStrategy::Password { password } => {
                DeviceAccessStrategy::Password { key_file, password }
            }
            DeviceSaveStrategy::Smartcard { .. } => DeviceAccessStrategy::Smartcard { key_file },
            DeviceSaveStrategy::AccountVault { operations } => DeviceAccessStrategy::AccountVault {
                key_file,
                operations,
            },
        }
    }

    pub fn ty(&self) -> AvailableDeviceType {
        match self {
            Self::Keyring { .. } => AvailableDeviceType::Keyring,
            Self::Password { .. } => AvailableDeviceType::Password,
            Self::Smartcard { .. } => AvailableDeviceType::Smartcard,
            Self::AccountVault { .. } => AvailableDeviceType::AccountVault,
        }
    }
}

/// Represent how to load a device file
#[derive(Debug, Clone)]
pub enum DeviceAccessStrategy {
    Keyring {
        key_file: PathBuf,
    },
    Password {
        key_file: PathBuf,
        password: Password,
    },
    Smartcard {
        key_file: PathBuf,
    },
    AccountVault {
        key_file: PathBuf,
        operations: Arc<dyn AccountVaultOperations>,
    },
}

impl DeviceAccessStrategy {
    pub fn key_file(&self) -> &Path {
        match self {
            Self::Keyring { key_file } => key_file,
            Self::Password { key_file, .. } => key_file,
            Self::Smartcard { key_file, .. } => key_file,
            Self::AccountVault { key_file, .. } => key_file,
        }
    }

    /// Returns `None` if `extra_info`'s variant type doesn't match our strategy
    pub fn into_save_strategy(self, extra_info: AvailableDeviceType) -> Option<DeviceSaveStrategy> {
        // Don't do `match (self, extra_info)` since we would end up with a catch-all
        // `(_, _) => return None` condition that would prevent this code from breaking
        // whenever a new variant is introduced (hence hiding the fact this code has
        // to be updated).
        match self {
            DeviceAccessStrategy::Keyring { .. } => {
                if matches!(extra_info, AvailableDeviceType::Keyring) {
                    Some(DeviceSaveStrategy::Keyring)
                } else {
                    None
                }
            }
            DeviceAccessStrategy::Password { password, .. } => {
                if matches!(extra_info, AvailableDeviceType::Password) {
                    Some(DeviceSaveStrategy::Password { password })
                } else {
                    None
                }
            }
            DeviceAccessStrategy::Smartcard { .. } => todo!(),
            DeviceAccessStrategy::AccountVault { operations, .. } => {
                if matches!(extra_info, AvailableDeviceType::AccountVault) {
                    Some(DeviceSaveStrategy::AccountVault { operations })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AvailableDeviceType {
    Keyring,
    Password,
    Recovery,
    Smartcard,
    AccountVault,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AvailableDevice {
    pub key_file_path: PathBuf,
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: String,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub ty: AvailableDeviceType,
}
