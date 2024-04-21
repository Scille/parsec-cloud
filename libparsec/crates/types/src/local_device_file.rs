// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

use libparsec_crypto::Password;
use libparsec_serialization_format::parsec_data;

use crate as libparsec_types;
use crate::{
    impl_transparent_data_format_conversion, DeviceID, DeviceLabel, HumanHandle, OrganizationID,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFileKeyringData", from = "DeviceFileKeyringData")]
pub struct DeviceFileKeyring {
    pub ciphertext: Bytes,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub device_id: DeviceID,
    pub organization_id: OrganizationID,
    pub slug: String,
    pub keyring_service: String,
    pub keyring_user: String,
}

parsec_data!("schema/local_device/device_file_keyring.json5");

impl_transparent_data_format_conversion!(
    DeviceFileKeyring,
    DeviceFileKeyringData,
    ciphertext,
    human_handle,
    device_label,
    device_id,
    organization_id,
    slug,
    keyring_service,
    keyring_user,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFilePasswordData", from = "DeviceFilePasswordData")]
pub struct DeviceFilePassword {
    pub ciphertext: Bytes,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub device_id: DeviceID,
    pub organization_id: OrganizationID,
    pub slug: String,
    pub algorithm: DeviceFilePasswordAlgorithm,
}

parsec_data!("schema/local_device/device_file_password.json5");

impl_transparent_data_format_conversion!(
    DeviceFilePassword,
    DeviceFilePasswordData,
    ciphertext,
    human_handle,
    device_label,
    device_id,
    organization_id,
    slug,
    algorithm,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFileRecoveryData", from = "DeviceFileRecoveryData")]
pub struct DeviceFileRecovery {
    pub ciphertext: Bytes,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub device_id: DeviceID,
    pub organization_id: OrganizationID,
    pub slug: String,
}

parsec_data!("schema/local_device/device_file_recovery.json5");

impl_transparent_data_format_conversion!(
    DeviceFileRecovery,
    DeviceFileRecoveryData,
    ciphertext,
    human_handle,
    device_label,
    device_id,
    organization_id,
    slug,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFileSmartcardData", from = "DeviceFileSmartcardData")]
pub struct DeviceFileSmartcard {
    pub ciphertext: Bytes,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub device_id: DeviceID,
    pub organization_id: OrganizationID,
    pub slug: String,

    pub encrypted_key: Bytes,
    pub certificate_id: String,
    pub certificate_sha1: Option<Bytes>,
}

parsec_data!("schema/local_device/device_file_smartcard.json5");

impl_transparent_data_format_conversion!(
    DeviceFileSmartcard,
    DeviceFileSmartcardData,
    ciphertext,
    human_handle,
    device_label,
    device_id,
    organization_id,
    slug,
    encrypted_key,
    certificate_id,
    certificate_sha1,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum DeviceFile {
    Keyring(DeviceFileKeyring),
    Password(DeviceFilePassword),
    Recovery(DeviceFileRecovery),
    Smartcard(DeviceFileSmartcard),
}

impl DeviceFile {
    pub fn dump(&self) -> Vec<u8> {
        rmp_serde::to_vec_named(self).expect("Unreachable")
    }

    pub fn load(data: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(data)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceFileType {
    Keyring,
    Password,
    Recovery,
    Smartcard,
}

impl DeviceFileType {
    pub fn dump(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec_named(self)
    }

    pub fn load(bytes: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(bytes)
    }
}

/// Represent how to load/save a device file
/// Note this is only for regular device given recovery device has dedicated
/// import/export functions.
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
    // Future API that will be use for parsec-web
    // ServerSide{
    //     url: ParsecOrganizationAddr,
    //     email: String,
    //     password: Password,
    // }
}

impl DeviceAccessStrategy {
    pub fn key_file(&self) -> &Path {
        match self {
            Self::Keyring { key_file } => key_file,
            Self::Password { key_file, .. } => key_file,
            Self::Smartcard { key_file } => key_file,
        }
    }

    pub fn ty(&self) -> DeviceFileType {
        match self {
            Self::Keyring { .. } => DeviceFileType::Keyring,
            Self::Password { .. } => DeviceFileType::Password,
            Self::Smartcard { .. } => DeviceFileType::Smartcard,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct AvailableDevice {
    pub key_file_path: PathBuf,
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    pub human_handle: HumanHandle,
    pub device_label: DeviceLabel,
    pub slug: String,
    #[serde(rename = "type")]
    pub ty: DeviceFileType,
}

impl AvailableDevice {
    /// Return a `sha256` hash of device slug as hex string
    pub fn slughash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.slug.as_bytes());
        let hash_digest = hasher.finalize();

        format!("{:x}", hash_digest)
    }
}

#[cfg(test)]
#[path = "../tests/unit/local_device_file.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
