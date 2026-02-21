// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

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

pub trait OpenBaoDeviceOperations: std::fmt::Debug + Send + Sync {
    fn openbao_entity_id(&self) -> &str;
    fn openbao_preferred_auth_id(&self) -> &str;
    /// Returns `(<openbao_ciphertext_key_path>, <opaque_key>)`
    fn upload_opaque_key(
        &self,
    ) -> PinBoxFutureResult<(String, SecretKey), OpenBaoOperationsUploadOpaqueKeyError>;
    fn fetch_opaque_key(
        &self,
        openbao_ciphertext_key_path: String,
    ) -> PinBoxFutureResult<SecretKey, OpenBaoOperationsFetchOpaqueKeyError>;
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

#[derive(Debug, Clone)]
pub enum DevicePrimaryProtectionStrategy {
    Keyring,
    Password {
        password: Password,
    },
    PKI {
        certificate_ref: X509CertificateReference,
    },
    AccountVault {
        operations: Arc<dyn AccountVaultOperations>,
    },
    OpenBao {
        operations: Arc<dyn OpenBaoDeviceOperations>,
    },
}

impl DevicePrimaryProtectionStrategy {
    pub fn ty(&self) -> AvailableDeviceType {
        match self {
            Self::Keyring => AvailableDeviceType::Keyring,
            Self::Password { .. } => AvailableDeviceType::Password,
            Self::PKI {
                certificate_ref, ..
            } => AvailableDeviceType::PKI {
                certificate_ref: certificate_ref.to_owned(),
            },
            Self::AccountVault { .. } => AvailableDeviceType::AccountVault,
            Self::OpenBao { operations, .. } => AvailableDeviceType::OpenBao {
                openbao_entity_id: operations.openbao_entity_id().to_owned(),
                openbao_preferred_auth_id: operations.openbao_preferred_auth_id().to_owned(),
            },
        }
    }
}

/// Represent how (not where) to save a device file
#[derive(Debug, Clone)]
pub struct DeviceSaveStrategy {
    pub totp_protection: Option<(TOTPOpaqueKeyID, SecretKey)>,
    pub primary_protection: DevicePrimaryProtectionStrategy,
}

impl DeviceSaveStrategy {
    pub fn into_access(self, key_file: PathBuf) -> DeviceAccessStrategy {
        DeviceAccessStrategy {
            key_file,
            totp_protection: self.totp_protection,
            primary_protection: self.primary_protection,
        }
    }

    // Convenient builders for keyring & password since they are the most
    // commonly used in the tests.

    pub fn new_keyring() -> Self {
        Self {
            totp_protection: None,
            primary_protection: DevicePrimaryProtectionStrategy::Keyring,
        }
    }

    pub fn new_password(password: Password) -> Self {
        Self {
            totp_protection: None,
            primary_protection: DevicePrimaryProtectionStrategy::Password { password },
        }
    }
}

/*
 * DeviceAccessStrategy
 */

/// Represent how to load a device file
#[derive(Debug, Clone)]
pub struct DeviceAccessStrategy {
    pub key_file: PathBuf,
    pub totp_protection: Option<(TOTPOpaqueKeyID, SecretKey)>,
    pub primary_protection: DevicePrimaryProtectionStrategy,
}

impl From<DeviceAccessStrategy> for DeviceSaveStrategy {
    fn from(value: DeviceAccessStrategy) -> Self {
        DeviceSaveStrategy {
            totp_protection: value.totp_protection,
            primary_protection: value.primary_protection,
        }
    }
}

impl DeviceAccessStrategy {
    // Convenient builders for keyring & password since they are the most
    // commonly used in the tests.

    pub fn new_keyring(key_file: PathBuf) -> Self {
        Self {
            key_file,
            totp_protection: None,
            primary_protection: DevicePrimaryProtectionStrategy::Keyring,
        }
    }

    pub fn new_password(key_file: PathBuf, password: Password) -> Self {
        Self {
            key_file,
            totp_protection: None,
            primary_protection: DevicePrimaryProtectionStrategy::Password { password },
        }
    }
}

/*
 * AvailableDevice
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AvailableDeviceType {
    Keyring,
    Password,
    Recovery,
    PKI {
        certificate_ref: X509CertificateReference,
    },
    AccountVault,
    OpenBao {
        openbao_entity_id: String,
        openbao_preferred_auth_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// TOTP consists of an opaque key stored server-side and only accessible
    /// through a TOTP-challenge.
    /// However this security cannot be used alone (otherwise the server would
    /// be able to decrypt the device!), hence the `next` field to combine with
    /// another layer of security.
    pub totp_opaque_key_id: Option<TOTPOpaqueKeyID>,
    pub ty: AvailableDeviceType,
}
