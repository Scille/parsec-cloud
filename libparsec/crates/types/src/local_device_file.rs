// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use libparsec_crypto::{Password, SecretKey};
use libparsec_serialization_format::parsec_data;

use crate::{self as libparsec_types};
use crate::{
    impl_transparent_data_format_conversion, AccountVaultItemOpaqueKeyID, DateTime, DeviceID,
    DeviceLabel, HumanHandle, OrganizationID, PasswordAlgorithm, UserID,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFileKeyringData", from = "DeviceFileKeyringData")]
pub struct DeviceFileKeyring {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: String,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub keyring_service: String,
    pub keyring_user: String,
    pub ciphertext: Bytes,
}

parsec_data!("schema/local_device/device_file_keyring.json5");

impl_transparent_data_format_conversion!(
    DeviceFileKeyring,
    DeviceFileKeyringData,
    created_on,
    protected_on,
    server_url,
    organization_id,
    user_id,
    device_id,
    human_handle,
    device_label,
    keyring_service,
    keyring_user,
    ciphertext,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFilePasswordData", from = "DeviceFilePasswordData")]
pub struct DeviceFilePassword {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: String,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub algorithm: PasswordAlgorithm,
    pub ciphertext: Bytes,
}

parsec_data!("schema/local_device/device_file_password.json5");

impl_transparent_data_format_conversion!(
    DeviceFilePassword,
    DeviceFilePasswordData,
    created_on,
    protected_on,
    server_url,
    organization_id,
    user_id,
    device_id,
    human_handle,
    device_label,
    algorithm,
    ciphertext,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFileRecoveryData", from = "DeviceFileRecoveryData")]
pub struct DeviceFileRecovery {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: String,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub ciphertext: Bytes,
}

parsec_data!("schema/local_device/device_file_recovery.json5");

impl_transparent_data_format_conversion!(
    DeviceFileRecovery,
    DeviceFileRecoveryData,
    created_on,
    protected_on,
    server_url,
    organization_id,
    user_id,
    device_id,
    human_handle,
    device_label,
    ciphertext,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFileSmartcardData", from = "DeviceFileSmartcardData")]
pub struct DeviceFileSmartcard {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: String,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub certificate_id: String,
    pub certificate_sha1: Option<Bytes>,
    pub encrypted_key: Bytes,
    pub ciphertext: Bytes,
}

parsec_data!("schema/local_device/device_file_smartcard.json5");

impl_transparent_data_format_conversion!(
    DeviceFileSmartcard,
    DeviceFileSmartcardData,
    created_on,
    protected_on,
    server_url,
    organization_id,
    user_id,
    device_id,
    human_handle,
    device_label,
    certificate_id,
    certificate_sha1,
    encrypted_key,
    ciphertext,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "DeviceFileAccountVaultData",
    from = "DeviceFileAccountVaultData"
)]
pub struct DeviceFileAccountVault {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: String,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub ciphertext_key_id: AccountVaultItemOpaqueKeyID,
    pub ciphertext: Bytes,
}

parsec_data!("schema/local_device/device_file_account_vault.json5");

impl_transparent_data_format_conversion!(
    DeviceFileAccountVault,
    DeviceFileAccountVaultData,
    created_on,
    protected_on,
    server_url,
    organization_id,
    user_id,
    device_id,
    human_handle,
    device_label,
    ciphertext_key_id,
    ciphertext,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum DeviceFile {
    Keyring(DeviceFileKeyring),
    Password(DeviceFilePassword),
    Recovery(DeviceFileRecovery),
    Smartcard(DeviceFileSmartcard),
    AccountVault(DeviceFileAccountVault),
}

impl DeviceFile {
    pub fn dump(&self) -> Vec<u8> {
        rmp_serde::to_vec_named(self).expect("Unreachable")
    }

    pub fn load(data: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(data)
    }

    pub fn ciphertext(&self) -> &Bytes {
        match self {
            DeviceFile::Keyring(device) => &device.ciphertext,
            DeviceFile::Password(device) => &device.ciphertext,
            DeviceFile::Recovery(device) => &device.ciphertext,
            DeviceFile::Smartcard(device) => &device.ciphertext,
            DeviceFile::AccountVault(device) => &device.ciphertext,
        }
    }

    pub fn created_on(&self) -> DateTime {
        match self {
            DeviceFile::Keyring(device) => device.created_on,
            DeviceFile::Password(device) => device.created_on,
            DeviceFile::Recovery(device) => device.created_on,
            DeviceFile::Smartcard(device) => device.created_on,
            DeviceFile::AccountVault(device) => device.created_on,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DeviceSaveStrategy {
    Keyring,
    Password {
        password: Password,
    },
    Smartcard,
    AccountVault {
        /// This key is the one stored in the account vault.
        ///
        /// Note it is `libparsec_account`'s job to deal with encrypting&uploading
        /// the account vault item containing this key.
        ciphertext_key_id: AccountVaultItemOpaqueKeyID,
        ciphertext_key: SecretKey,
    },
}

impl DeviceSaveStrategy {
    pub fn into_access(self, key_file: PathBuf) -> DeviceAccessStrategy {
        match self {
            DeviceSaveStrategy::Keyring => DeviceAccessStrategy::Keyring { key_file },
            DeviceSaveStrategy::Password { password } => {
                DeviceAccessStrategy::Password { key_file, password }
            }
            DeviceSaveStrategy::Smartcard => DeviceAccessStrategy::Smartcard { key_file },
            DeviceSaveStrategy::AccountVault {
                ciphertext_key_id,
                ciphertext_key,
            } => DeviceAccessStrategy::AccountVault {
                key_file,
                ciphertext_key_id,
                ciphertext_key,
            },
        }
    }
}

/// Represent how to load/save a device file
#[derive(Debug, Clone, PartialEq)]
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
        /// This key is the one stored in the account vault.
        ///
        /// Note it is `libparsec_account`'s job to deal with fetching&decrypting
        /// the account vault item containing this key.
        ciphertext_key_id: AccountVaultItemOpaqueKeyID,
        ciphertext_key: SecretKey,
    },
}

impl DeviceAccessStrategy {
    pub fn key_file(&self) -> &Path {
        match self {
            Self::Keyring { key_file } => key_file,
            Self::Password { key_file, .. } => key_file,
            Self::Smartcard { key_file } => key_file,
            Self::AccountVault { key_file, .. } => key_file,
        }
    }

    pub fn ty(&self) -> AvailableDeviceType {
        match self {
            Self::Keyring { .. } => AvailableDeviceType::Keyring,
            Self::Password { .. } => AvailableDeviceType::Password,
            Self::Smartcard { .. } => AvailableDeviceType::Smartcard,
            Self::AccountVault {
                ciphertext_key_id, ..
            } => AvailableDeviceType::AccountVault {
                ciphertext_key_id: *ciphertext_key_id,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AvailableDeviceType {
    Keyring,
    Password,
    Recovery,
    Smartcard,
    AccountVault {
        ciphertext_key_id: AccountVaultItemOpaqueKeyID,
    },
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

#[cfg(test)]
#[path = "../tests/unit/local_device_file.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
