// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use serde::{Deserialize, Serialize};

use libparsec_serialization_format::parsec_data;

use crate::{self as libparsec_types, PKIEncryptionAlgorithm, X509CertificateReference};
use crate::{
    impl_transparent_data_format_conversion, AccountVaultItemOpaqueKeyID, DateTime, DeviceID,
    DeviceLabel, HumanHandle, OrganizationID, ParsecAddr, PasswordAlgorithm, UserID,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFileKeyringData", try_from = "DeviceFileKeyringData")]
pub struct DeviceFileKeyring {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: ParsecAddr,
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
#[serde(into = "DeviceFilePasswordData", try_from = "DeviceFilePasswordData")]
pub struct DeviceFilePassword {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: ParsecAddr,
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
#[serde(into = "DeviceFileRecoveryData", try_from = "DeviceFileRecoveryData")]
pub struct DeviceFileRecovery {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: ParsecAddr,
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
    pub server_url: ParsecAddr,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub certificate_ref: X509CertificateReference,
    pub algorithm_for_encrypted_key: PKIEncryptionAlgorithm,
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
    certificate_ref,
    algorithm_for_encrypted_key,
    encrypted_key,
    ciphertext,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "DeviceFileAccountVaultData",
    try_from = "DeviceFileAccountVaultData"
)]
pub struct DeviceFileAccountVault {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: ParsecAddr,
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
#[serde(into = "DeviceFileOpenBaoData", try_from = "DeviceFileOpenBaoData")]
pub struct DeviceFileOpenBao {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: ParsecAddr,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub openbao_preferred_auth_id: String,
    pub openbao_entity_id: String,
    pub openbao_ciphertext_key_path: String,
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
    openbao_preferred_auth_id,
    openbao_entity_id,
    openbao_ciphertext_key_path,
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

#[cfg(test)]
#[path = "../tests/unit/local_device_file.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
