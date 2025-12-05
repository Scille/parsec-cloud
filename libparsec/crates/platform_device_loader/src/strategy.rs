// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec_client_connection::ConnectionError;
use libparsec_crypto::{Password, SecretKey};
use libparsec_platform_async::PinBoxFutureResult;
use libparsec_types::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum RemoteOperationServer {
    ParsecAccount,
    OpenBao,
}

impl std::fmt::Display for RemoteOperationServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            RemoteOperationServer::ParsecAccount => "Parsec account",
            RemoteOperationServer::OpenBao => "OpenBao",
        };
        write!(f, "{}", name)
    }
}

/*
 * Account vault operations
 */

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
    ) -> PinBoxFutureResult<SecretKey, AccountVaultOperationsFetchOpaqueKeyError>;
    fn upload_opaque_key(
        &self,
    ) -> PinBoxFutureResult<
        (AccountVaultItemOpaqueKeyID, SecretKey),
        AccountVaultOperationsUploadOpaqueKeyError,
    >;
}

#[derive(Debug, thiserror::Error)]
pub enum AccountVaultOperationsFetchOpaqueKeyError {
    #[error("Cannot decrypt the vault key access returned by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("No opaque key with this ID among the vault items in the server")]
    UnknownOpaqueKey,
    #[error("The vault item returned by the server and containing this opaque key is corrupted")]
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
    #[error("The server returned an unexpected response: {0}")]
    BadServerResponse(anyhow::Error),
}

/*
 * OpenBao operations
 */

/// We need to split the OpenBao trait between save and access operations since
/// `openbao_entity_id` & `openbao_preferred_auth_id` are only needed for save.
pub trait OpenBaoDeviceSaveOperations: std::fmt::Debug + Send + Sync {
    fn openbao_entity_id(&self) -> &str;
    fn openbao_preferred_auth_id(&self) -> &str;
    /// Returns `(<openbao_ciphertext_key_path>, <opaque_key>)`
    fn upload_opaque_key(
        &self,
    ) -> PinBoxFutureResult<(String, SecretKey), OpenBaoOperationsUploadOpaqueKeyError>;
    fn to_access_operations(&self) -> Arc<dyn OpenBaoDeviceAccessOperations>;
}

pub trait OpenBaoDeviceAccessOperations: std::fmt::Debug + Send + Sync {
    fn openbao_entity_id(&self) -> &str;
    fn fetch_opaque_key(
        &self,
        openbao_ciphertext_key_path: String,
    ) -> PinBoxFutureResult<SecretKey, OpenBaoOperationsFetchOpaqueKeyError>;
    fn to_save_operations(
        &self,
        openbao_preferred_auth_id: String,
    ) -> Arc<dyn OpenBaoDeviceSaveOperations>;
}

#[derive(Debug, thiserror::Error)]
pub enum OpenBaoOperationsFetchOpaqueKeyError {
    #[error("Invalid server URL: {0}")]
    BadURL(anyhow::Error),
    #[error("No response from the server: {0}")]
    NoServerResponse(anyhow::Error),
    #[error("The server returned an unexpected response: {0}")]
    BadServerResponse(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum OpenBaoOperationsUploadOpaqueKeyError {
    #[error("Invalid server URL: {0}")]
    BadURL(anyhow::Error),
    #[error("No response from the server: {0}")]
    NoServerResponse(anyhow::Error),
    #[error("The server returned an unexpected response: {0}")]
    BadServerResponse(anyhow::Error),
}

/*
 * DeviceSaveStrategy
 */

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
    OpenBao {
        operations: Arc<dyn OpenBaoDeviceSaveOperations>,
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
            DeviceSaveStrategy::OpenBao { operations } => DeviceAccessStrategy::OpenBao {
                key_file,
                operations: operations.to_access_operations(),
            },
        }
    }

    pub fn ty(&self) -> AvailableDeviceType {
        match self {
            Self::Keyring { .. } => AvailableDeviceType::Keyring,
            Self::Password { .. } => AvailableDeviceType::Password,
            Self::Smartcard { .. } => AvailableDeviceType::Smartcard,
            Self::AccountVault { .. } => AvailableDeviceType::AccountVault,
            Self::OpenBao { operations } => AvailableDeviceType::OpenBao {
                openbao_entity_id: operations.openbao_entity_id().to_owned(),
                openbao_preferred_auth_id: operations.openbao_preferred_auth_id().to_owned(),
            },
        }
    }
}

/*
 * DeviceAccessStrategy
 */

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
    OpenBao {
        key_file: PathBuf,
        operations: Arc<dyn OpenBaoDeviceAccessOperations>,
    },
}

impl DeviceAccessStrategy {
    pub fn key_file(&self) -> &Path {
        match self {
            Self::Keyring { key_file } => key_file,
            Self::Password { key_file, .. } => key_file,
            Self::Smartcard { key_file, .. } => key_file,
            Self::AccountVault { key_file, .. } => key_file,
            Self::OpenBao { key_file, .. } => key_file,
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
            DeviceAccessStrategy::OpenBao { operations, .. } => match extra_info {
                AvailableDeviceType::OpenBao {
                    openbao_entity_id,
                    openbao_preferred_auth_id,
                } if openbao_entity_id == operations.openbao_entity_id() => {
                    Some(DeviceSaveStrategy::OpenBao {
                        operations: operations.to_save_operations(openbao_preferred_auth_id),
                    })
                }
                _ => None,
            },
        }
    }
}

/*
 * AvailableDevice
 */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AvailableDeviceType {
    Keyring,
    Password,
    Recovery,
    Smartcard,
    AccountVault,
    OpenBao {
        openbao_entity_id: String,
        openbao_preferred_auth_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AvailableDevice {
    pub key_file_path: PathBuf,
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_addr: ParsecAddr,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub ty: AvailableDeviceType,
}
