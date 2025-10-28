// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
#[path = "../tests/unit/local_device_file.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
