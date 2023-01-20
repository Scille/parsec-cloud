// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use sha2::{Digest, Sha256};

use libparsec_types::{DeviceID, DeviceLabel, HumanHandle, OrganizationID};

use crate::StrPath;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceFilePassword {
    #[serde_as(as = "Bytes")]
    pub salt: Vec<u8>,

    #[serde_as(as = "Bytes")]
    pub ciphertext: Vec<u8>,

    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,

    pub device_id: DeviceID,
    pub organization_id: OrganizationID,
    pub slug: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceFileRecovery {
    #[serde_as(as = "Bytes")]
    pub ciphertext: Vec<u8>,

    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,

    pub device_id: DeviceID,
    pub organization_id: OrganizationID,
    pub slug: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceFileSmartcard {
    #[serde_as(as = "Bytes")]
    pub encrypted_key: Vec<u8>,

    pub certificate_id: String,

    #[serde_as(as = "Option<Bytes>")]
    pub certificate_sha1: Option<Vec<u8>>,

    #[serde_as(as = "Bytes")]
    pub ciphertext: Vec<u8>,

    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,

    pub device_id: DeviceID,
    pub organization_id: OrganizationID,
    pub slug: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
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

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct LegacyDeviceFilePassword {
    #[serde_as(as = "Bytes")]
    pub salt: Vec<u8>,
    #[serde_as(as = "Bytes")]
    pub ciphertext: Vec<u8>,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct AvailableDevice {
    pub key_file_path: StrPath,
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
