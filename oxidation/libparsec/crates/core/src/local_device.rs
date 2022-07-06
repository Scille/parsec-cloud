// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::{
    collections::HashSet,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{LocalDeviceError, LocalDeviceResult};
use api_types::{DeviceID, DeviceLabel, HumanHandle, OrganizationID};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum DeviceFile {
    Password {
        #[serde_as(as = "Bytes")]
        salt: Vec<u8>,

        #[serde_as(as = "Bytes")]
        ciphertext: Vec<u8>,

        human_handle: Option<HumanHandle>,
        device_label: Option<DeviceLabel>,

        device_id: DeviceID,
        organization_id: OrganizationID,
        slug: String,
    },
    Recovery {
        #[serde_as(as = "Bytes")]
        ciphertext: Vec<u8>,

        human_handle: Option<HumanHandle>,
        device_label: Option<DeviceLabel>,

        device_id: DeviceID,
        organization_id: OrganizationID,
        slug: String,
    },
    Smartcard {
        #[serde_as(as = "Bytes")]
        encrypted_key: Vec<u8>,

        certificate_id: String,

        #[serde_as(as = "Option<Bytes>")]
        certificate_sha1: Option<Vec<u8>>,

        #[serde_as(as = "Bytes")]
        ciphertext: Vec<u8>,

        human_handle: Option<HumanHandle>,
        device_label: Option<DeviceLabel>,

        device_id: DeviceID,
        organization_id: OrganizationID,
        slug: String,
    },
}

pub enum DeviceFileType {
    Password,
    Recovery,
    Smartcard,
}

pub struct AvailableDevice {
    pub key_file_path: PathBuf,
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,
    pub slug: String,
    pub ty: DeviceFileType,
}

impl AvailableDevice {
    pub fn load(key_file_path: PathBuf) -> LocalDeviceResult<Self> {
        let data = std::fs::read(&key_file_path)
            .map_err(|_| LocalDeviceError::Access(key_file_path.clone()))?;
        let (ty, organization_id, device_id, human_handle, device_label, slug) =
            match rmp_serde::from_slice::<DeviceFile>(&data) {
                Ok(DeviceFile::Password {
                    human_handle,
                    device_label,
                    device_id,
                    organization_id,
                    slug,
                    ..
                }) => (
                    DeviceFileType::Password,
                    organization_id,
                    device_id,
                    human_handle,
                    device_label,
                    slug,
                ),
                Ok(DeviceFile::Recovery {
                    human_handle,
                    device_label,
                    device_id,
                    organization_id,
                    slug,
                    ..
                }) => (
                    DeviceFileType::Recovery,
                    organization_id,
                    device_id,
                    human_handle,
                    device_label,
                    slug,
                ),
                Ok(DeviceFile::Smartcard {
                    human_handle,
                    device_label,
                    device_id,
                    organization_id,
                    slug,
                    ..
                }) => (
                    DeviceFileType::Smartcard,
                    organization_id,
                    device_id,
                    human_handle,
                    device_label,
                    slug,
                ),
                Err(_) => return Err(LocalDeviceError::Deserialization(key_file_path)),
            };

        Ok(Self {
            key_file_path,
            organization_id,
            device_id,
            human_handle,
            device_label,
            slug,
            ty,
        })
    }
}

pub fn list_available_devices(config_dir: &Path) -> LocalDeviceResult<Vec<AvailableDevice>> {
    let mut list = vec![];
    // Set of seen slugs
    let mut seen = HashSet::new();

    let key_file_paths = PathBuf::from(config_dir).join("devices");

    // Consider `.keys` files in devices directory
    let mut key_file_paths = std::fs::read_dir(&key_file_paths)
        .map_err(|_| LocalDeviceError::Access(key_file_paths))?
        .filter_map(|path| path.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension() == Some(OsStr::new("keys")))
        .collect::<Vec<_>>();

    // Sort paths so the discovery order is deterministic
    // In the case of duplicate files, that means only the first discovered device is considered
    key_file_paths.sort();

    for key_file_path in key_file_paths {
        let device = match AvailableDevice::load(key_file_path) {
            // Load the device file
            Ok(device) => device,
            // Ignore invalid files
            Err(_) => continue,
        };

        // Ignore duplicate files
        if seen.contains(&device.slug) {
            continue;
        }

        seen.insert(device.slug.clone());

        list.push(device);
    }

    Ok(list)
}
