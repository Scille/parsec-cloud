// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use libparsec_serialization_format::parsec_data;

use crate::{self as libparsec_types, PKIEncryptionAlgorithm, X509CertificateReference};
use crate::{
    impl_transparent_data_format_conversion, AccountVaultItemOpaqueKeyID, DateTime, DeviceID,
    DeviceLabel, HumanHandle, OrganizationID, ParsecAddr, PasswordAlgorithm, TOTPOpaqueKeyID,
    UserID,
};

/*
 * AdvisoryDeviceFileProtection
 */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdvisoryDeviceFilePrimaryProtection {
    Password,
    Keyring,
    PKI,
    OpenBao,
    AccountVault,
}

/// Advisory device protection configuration combining a primary strategy with an
/// optional TOTP second factor.
/// This gets serialized as a string (e.g. `"PASSWORD+TOTP"`, `"PKI"`).
///
/// Note this type doesn't implement serde (de)serialization but instead relies
/// on manual conversion from/to string. This is because this deserialization
/// is expected to be run manually *after* the object containing it (i.e. server
/// response to a `server_config` command) is deserialized, hence allowing to
/// ignore unknown strings to stays forward-compatible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvisoryDeviceFileProtection {
    pub primary: AdvisoryDeviceFilePrimaryProtection,
    pub with_totp: bool,
}

impl FromStr for AdvisoryDeviceFileProtection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (primary_str, with_totp) = if let Some(base) = s.strip_suffix("+TOTP") {
            (base, true)
        } else {
            (s, false)
        };

        let primary = match primary_str {
            "PASSWORD" => AdvisoryDeviceFilePrimaryProtection::Password,
            "KEYRING" => AdvisoryDeviceFilePrimaryProtection::Keyring,
            "PKI" => AdvisoryDeviceFilePrimaryProtection::PKI,
            "OPENBAO" => AdvisoryDeviceFilePrimaryProtection::OpenBao,
            "ACCOUNT_VAULT" => AdvisoryDeviceFilePrimaryProtection::AccountVault,
            _ => return Err(()),
        };

        Ok(AdvisoryDeviceFileProtection { primary, with_totp })
    }
}

impl AdvisoryDeviceFileProtection {
    pub fn as_str(&self) -> &'static str {
        match (&self.primary, self.with_totp) {
            (AdvisoryDeviceFilePrimaryProtection::Password, false) => "PASSWORD",
            (AdvisoryDeviceFilePrimaryProtection::Password, true) => "PASSWORD+TOTP",
            (AdvisoryDeviceFilePrimaryProtection::Keyring, false) => "KEYRING",
            (AdvisoryDeviceFilePrimaryProtection::Keyring, true) => "KEYRING+TOTP",
            (AdvisoryDeviceFilePrimaryProtection::PKI, false) => "PKI",
            (AdvisoryDeviceFilePrimaryProtection::PKI, true) => "PKI+TOTP",
            (AdvisoryDeviceFilePrimaryProtection::OpenBao, false) => "OPENBAO",
            (AdvisoryDeviceFilePrimaryProtection::OpenBao, true) => "OPENBAO+TOTP",
            (AdvisoryDeviceFilePrimaryProtection::AccountVault, false) => "ACCOUNT_VAULT",
            (AdvisoryDeviceFilePrimaryProtection::AccountVault, true) => "ACCOUNT_VAULT+TOTP",
        }
    }
}

/*
 * DeviceFileKeyring
 */

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
    pub totp_opaque_key_id: Option<TOTPOpaqueKeyID>,
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
    totp_opaque_key_id,
);

/*
 * DeviceFilePassword
 */

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
    pub totp_opaque_key_id: Option<TOTPOpaqueKeyID>,
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
    totp_opaque_key_id,
);

/*
 * DeviceFileRecovery
 */

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

/*
 * DeviceFilePKI
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFilePKIData", try_from = "DeviceFilePKIData")]
pub struct DeviceFilePKI {
    pub created_on: DateTime,
    pub protected_on: DateTime,
    pub server_url: ParsecAddr,
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub certificate_ref: X509CertificateReference,
    pub algorithm: PKIEncryptionAlgorithm,
    pub encrypted_key: Bytes,
    pub ciphertext: Bytes,
    pub totp_opaque_key_id: Option<TOTPOpaqueKeyID>,
}

parsec_data!("schema/local_device/device_file_pki.json5");

impl_transparent_data_format_conversion!(
    DeviceFilePKI,
    DeviceFilePKIData,
    created_on,
    protected_on,
    server_url,
    organization_id,
    user_id,
    device_id,
    human_handle,
    device_label,
    certificate_ref,
    algorithm,
    encrypted_key,
    ciphertext,
    totp_opaque_key_id,
);

/*
 * DeviceFileAccountVault
 */

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
    pub totp_opaque_key_id: Option<TOTPOpaqueKeyID>,
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
    totp_opaque_key_id,
);

/*
 * DeviceFileOpenBao
 */

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
    pub totp_opaque_key_id: Option<TOTPOpaqueKeyID>,
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
    totp_opaque_key_id,
);

/*
 * DeviceFile
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum DeviceFile {
    Keyring(DeviceFileKeyring),
    Password(DeviceFilePassword),
    Recovery(DeviceFileRecovery),
    PKI(DeviceFilePKI),
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
            DeviceFile::PKI(device) => &device.ciphertext,
            DeviceFile::AccountVault(device) => &device.ciphertext,
            DeviceFile::OpenBao(device) => &device.ciphertext,
        }
    }

    pub fn created_on(&self) -> DateTime {
        match self {
            DeviceFile::Keyring(device) => device.created_on,
            DeviceFile::Password(device) => device.created_on,
            DeviceFile::Recovery(device) => device.created_on,
            DeviceFile::PKI(device) => device.created_on,
            DeviceFile::AccountVault(device) => device.created_on,
            DeviceFile::OpenBao(device) => device.created_on,
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/local_device_file.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
