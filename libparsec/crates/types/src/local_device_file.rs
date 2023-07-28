// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

use libparsec_crypto::Password;
use libparsec_serialization_format::parsec_data;

use crate as libparsec_types;
use crate::{
    impl_transparent_data_format_conversion, DeviceID, DeviceLabel, HumanHandle, OrganizationID,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFilePasswordData", from = "DeviceFilePasswordData")]
pub struct DeviceFilePassword {
    pub ciphertext: Bytes,
    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,
    pub device_id: DeviceID,
    pub organization_id: OrganizationID,
    pub slug: String,

    pub salt: Bytes,
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
    salt,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceFileRecoveryData", from = "DeviceFileRecoveryData")]
pub struct DeviceFileRecovery {
    pub ciphertext: Bytes,
    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,
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
    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,
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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct LegacyDeviceFilePassword {
    pub salt: Bytes,
    pub ciphertext: Bytes,
    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,
}

/// Represents a legacy device file. This enum is mandatory because legacy device
/// files used to be serialized with a `type` field set to `password`. In order to
/// enforce this property serde's `tag` attribute is set to `type` field here.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum LegacyDeviceFile {
    Password(LegacyDeviceFilePassword),
}

impl LegacyDeviceFile {
    pub fn load(serialized: &[u8]) -> Result<Self, &'static str> {
        rmp_serde::from_slice(serialized).map_err(|_| "Invalid serialization")
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceFileType {
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
    Password {
        key_file: PathBuf,
        password: Password,
    },
    Smartcard {
        key_file: PathBuf,
    },
    // Future API that will be use for parsec-web
    // ServerSide{
    //     url: BackendOrganizationAddr,
    //     email: String,
    //     password: Password,
    // }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct AvailableDevice {
    pub key_file_path: PathBuf,
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,
    pub slug: String,
    #[serde(rename = "type")]
    pub ty: DeviceFileType,
}

impl AvailableDevice {
    pub fn user_display(&self) -> &str {
        self.human_handle
            .as_ref()
            .map(|x| x.as_ref())
            .unwrap_or_else(|| self.device_id.user_id().as_ref())
    }

    pub fn short_user_display(&self) -> &str {
        self.human_handle
            .as_ref()
            .map(|hh| hh.label())
            .unwrap_or_else(|| self.device_id.user_id().as_ref())
    }

    pub fn device_display(&self) -> &str {
        self.device_label
            .as_ref()
            .map(|x| x.as_ref())
            .unwrap_or_else(|| self.device_id.device_name().as_ref())
    }

    /// Return a `sha256` hash of device slug as hex string
    pub fn slughash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.slug.as_bytes());
        let hash_digest = hasher.finalize();

        format!("{:x}", hash_digest)
    }
}
