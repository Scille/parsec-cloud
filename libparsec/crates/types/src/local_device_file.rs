// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use libparsec_crypto::{Password, SecretKey};
use libparsec_serialization_format::parsec_data;

use crate::{self as libparsec_types, DataError, EncryptionAlgorithm, X509CertificateReference};
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
#[serde(into = "DeviceFileSmartcardData", try_from = "DeviceFileSmartcardData")]
pub struct DeviceFileSmartcard {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: String,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub certificate_ref: X509CertificateReference,
    pub algorithm_for_encrypted_key: EncryptionAlgorithm,
    pub encrypted_key: Bytes,
    pub ciphertext: Bytes,
}

parsec_data!("schema/local_device/device_file_smartcard.json5");

impl TryFrom<DeviceFileSmartcardData> for DeviceFileSmartcard {
    type Error = DataError;

    fn try_from(value: DeviceFileSmartcardData) -> Result<Self, Self::Error> {
        let algorithm_for_encrypted_key =
            value
                .algorithm_for_encrypted_key
                .parse()
                .map_err(|_| DataError::DataIntegrity {
                    data_type: "algorithm for encrypted key",
                    invariant: "unknown algorithm",
                })?;
        Ok(DeviceFileSmartcard {
            created_on: value.created_on,
            protected_on: value.protected_on,
            server_url: value.server_url,
            organization_id: value.organization_id,
            user_id: value.user_id,
            device_id: value.device_id,
            human_handle: value.human_handle,
            device_label: value.device_label,
            certificate_ref: value.certificate_ref,
            algorithm_for_encrypted_key,
            encrypted_key: value.encrypted_key,
            ciphertext: value.ciphertext,
        })
    }
}

impl From<DeviceFileSmartcard> for DeviceFileSmartcardData {
    fn from(value: DeviceFileSmartcard) -> Self {
        let algorithm_for_encrypted_key = value.algorithm_for_encrypted_key.to_string();
        DeviceFileSmartcardData {
            created_on: value.created_on,
            protected_on: value.protected_on,
            server_url: value.server_url,
            organization_id: value.organization_id,
            user_id: value.user_id,
            device_id: value.device_id,
            human_handle: value.human_handle,
            device_label: value.device_label,
            certificate_ref: value.certificate_ref,
            algorithm_for_encrypted_key,
            encrypted_key: value.encrypted_key,
            ciphertext: value.ciphertext,
            ty: DeviceFileSmartcardDataType,
        }
    }
}

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
#[serde(into = "DeviceFileOpenBaoData", from = "DeviceFileOpenBaoData")]
pub struct DeviceFileOpenBao {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: String,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub openbao_url: String,
    pub openbao_ciphertext_key_path: String,
    pub openbao_auth_path: String,
    pub openbao_auth_type: OpenBaoAuthType,
    pub ciphertext: Bytes,
}

parsec_data!("schema/local_device/device_file_openbao.json5");

impl_transparent_data_format_conversion!(
    DeviceFileOpenBao,
    DeviceFileOpenBaoData,
    created_on,
    protected_on,
    server_url,
    organization_id,
    user_id,
    device_id,
    human_handle,
    device_label,
    openbao_url,
    openbao_ciphertext_key_path,
    openbao_auth_path,
    openbao_auth_type,
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
    OpenBao(DeviceFileOpenBao),
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
            DeviceFile::OpenBao(device) => &device.ciphertext,
        }
    }

    pub fn created_on(&self) -> DateTime {
        match self {
            DeviceFile::Keyring(device) => device.created_on,
            DeviceFile::Password(device) => device.created_on,
            DeviceFile::Recovery(device) => device.created_on,
            DeviceFile::Smartcard(device) => device.created_on,
            DeviceFile::AccountVault(device) => device.created_on,
            DeviceFile::OpenBao(device) => device.created_on,
        }
    }
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
        /// This key is the one stored in the account vault.
        ///
        /// Note it is `libparsec_account`'s job to deal with encrypting&uploading
        /// the account vault item containing this key.
        ciphertext_key_id: AccountVaultItemOpaqueKeyID,
        ciphertext_key: SecretKey,
    },
    OpenBao {
        /// This key is the one stored in OpenBao.
        ///
        /// Note it is `libparsec_openbao`'s job to deal with interfacing with
        /// the OpenBao service to fetch the the key.
        openbao_url: String,
        openbao_ciphertext_key_path: String,
        openbao_auth_path: String,
        openbao_auth_type: OpenBaoAuthType,
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
            DeviceSaveStrategy::Smartcard { .. } => DeviceAccessStrategy::Smartcard { key_file },
            DeviceSaveStrategy::AccountVault {
                ciphertext_key_id,
                ciphertext_key,
            } => DeviceAccessStrategy::AccountVault {
                key_file,
                ciphertext_key_id,
                ciphertext_key,
            },
            DeviceSaveStrategy::OpenBao {
                openbao_url,
                openbao_ciphertext_key_path,
                openbao_auth_path,
                openbao_auth_type,
                ciphertext_key,
            } => DeviceAccessStrategy::OpenBao {
                key_file,
                openbao_url,
                openbao_ciphertext_key_path,
                openbao_auth_path,
                openbao_auth_type,
                ciphertext_key,
            },
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
            Self::OpenBao {
                openbao_url,
                openbao_ciphertext_key_path,
                openbao_auth_path,
                openbao_auth_type,
                ..
            } => AvailableDeviceType::OpenBao {
                openbao_url: openbao_url.clone(),
                openbao_ciphertext_key_path: openbao_ciphertext_key_path.clone(),
                openbao_auth_path: openbao_auth_path.clone(),
                openbao_auth_type: *openbao_auth_type,
            },
        }
    }
}

/// Represent how to load a device file
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
    OpenBao {
        key_file: PathBuf,
        /// This key is the one stored in OpenBao.
        ///
        /// Note it is `libparsec_openbao`'s job to deal with interfacing with
        /// the OpenBao service to fetch the the key.
        openbao_url: String,
        openbao_ciphertext_key_path: String,
        openbao_auth_path: String,
        openbao_auth_type: OpenBaoAuthType,
        ciphertext_key: SecretKey,
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
            Self::OpenBao {
                openbao_url,
                openbao_ciphertext_key_path,
                openbao_auth_path,
                openbao_auth_type,
                ..
            } => AvailableDeviceType::OpenBao {
                openbao_url: openbao_url.clone(),
                openbao_ciphertext_key_path: openbao_ciphertext_key_path.clone(),
                openbao_auth_path: openbao_auth_path.clone(),
                openbao_auth_type: *openbao_auth_type,
            },
        }
    }

    pub fn into_save_strategy(self) -> DeviceSaveStrategy {
        match self {
            DeviceAccessStrategy::Keyring { .. } => DeviceSaveStrategy::Keyring,
            DeviceAccessStrategy::Password { password, .. } => {
                DeviceSaveStrategy::Password { password }
            }
            DeviceAccessStrategy::Smartcard { .. } => todo!(),
            DeviceAccessStrategy::AccountVault {
                ciphertext_key_id,
                ciphertext_key,
                ..
            } => DeviceSaveStrategy::AccountVault {
                ciphertext_key_id,
                ciphertext_key,
            },
            DeviceAccessStrategy::OpenBao {
                openbao_url,
                openbao_ciphertext_key_path,
                openbao_auth_path,
                openbao_auth_type,
                ciphertext_key,
                ..
            } => DeviceSaveStrategy::OpenBao {
                openbao_url,
                openbao_ciphertext_key_path,
                openbao_auth_path,
                openbao_auth_type,
                ciphertext_key,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AvailableDeviceType {
    Keyring,
    Password,
    Recovery,
    Smartcard,
    AccountVault {
        ciphertext_key_id: AccountVaultItemOpaqueKeyID,
    },
    OpenBao {
        openbao_url: String,
        openbao_ciphertext_key_path: String,
        openbao_auth_path: String,
        openbao_auth_type: OpenBaoAuthType,
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
