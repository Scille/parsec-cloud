// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::{
    collections::HashSet,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::LocalDevice;
use crate::{LocalDeviceError, LocalDeviceResult};
use libparsec_types::{DeviceID, DeviceLabel, HumanHandle, OrganizationID};

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
    // Handle legacy device with option
    pub slug: Option<String>,
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
    pub fn save(&self, key_file_path: &Path) -> LocalDeviceResult<()> {
        if let Some(parent) = key_file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|_| LocalDeviceError::Access(key_file_path.to_path_buf()))?;
        }

        let data = rmp_serde::to_vec_named(self)
            .map_err(|_| LocalDeviceError::Serialization(key_file_path.to_path_buf()))?;

        std::fs::write(key_file_path, data)
            .map_err(|_| LocalDeviceError::Access(key_file_path.to_path_buf()))
    }

    fn load(key_file_path: &Path) -> LocalDeviceResult<Self> {
        let data = std::fs::read(&key_file_path)
            .map_err(|_| LocalDeviceError::Access(key_file_path.to_path_buf()))?;

        rmp_serde::from_slice::<DeviceFile>(&data)
            .map_err(|_| LocalDeviceError::Deserialization(key_file_path.to_path_buf()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceFileType {
    Password,
    Recovery,
    Smartcard,
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
    pub fn user_display(&self) -> String {
        self.human_handle
            .as_ref()
            .map(HumanHandle::to_string)
            .unwrap_or_else(|| self.device_id.user_id.to_string())
    }

    pub fn short_user_display(&self) -> String {
        self.human_handle
            .as_ref()
            .map(|hh| hh.label.clone())
            .unwrap_or_else(|| self.device_id.user_id.to_string())
    }

    pub fn device_display(&self) -> String {
        self.device_label
            .as_ref()
            .map(DeviceLabel::to_string)
            .unwrap_or_else(|| self.device_id.device_name.to_string())
    }

    /// For the legacy device files, the slug is contained in the device filename
    fn load(key_file_path: PathBuf) -> LocalDeviceResult<Self> {
        let (ty, organization_id, device_id, human_handle, device_label, slug) =
            match DeviceFile::load(&key_file_path)? {
                DeviceFile::Password(device) => (
                    DeviceFileType::Password,
                    device.organization_id,
                    device.device_id,
                    device.human_handle,
                    device.device_label,
                    // Handle legacy device
                    match device.slug {
                        Some(slug) => slug,
                        None => {
                            let slug = key_file_path
                                .file_stem()
                                .expect("Unreachable because deserialization succeed")
                                .to_str()
                                .expect("It may be unreachable")
                                .to_string();

                            if LocalDevice::load_slug(&slug).is_err() {
                                return Err(LocalDeviceError::InvalidSlug);
                            }

                            slug
                        }
                    },
                ),
                DeviceFile::Recovery(device) => (
                    DeviceFileType::Recovery,
                    device.organization_id,
                    device.device_id,
                    device.human_handle,
                    device.device_label,
                    device.slug,
                ),
                DeviceFile::Smartcard(device) => (
                    DeviceFileType::Smartcard,
                    device.organization_id,
                    device.device_id,
                    device.human_handle,
                    device.device_label,
                    device.slug,
                ),
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

/// Return the default keyfile path for a given device.
///
/// Note that the filename does not carry any intrinsic meaning.
/// Here, we simply use the slughash to avoid name collision.
pub fn get_default_key_file(config_dir: &Path, device: &LocalDevice) -> PathBuf {
    let devices_dir = config_dir.join("devices");
    let _ = std::fs::create_dir_all(&devices_dir);
    devices_dir.join(device.slughash() + ".keys")
}

fn read_key_file_paths(path: PathBuf) -> LocalDeviceResult<Vec<PathBuf>> {
    let mut key_file_paths = vec![];

    for path in std::fs::read_dir(&path)
        .map_err(|_| LocalDeviceError::Access(path))?
        .filter_map(|path| path.ok())
        .map(|entry| entry.path())
    {
        if path.extension() == Some(OsStr::new("keys")) {
            key_file_paths.push(path)
        } else if path.is_dir() {
            key_file_paths.append(&mut read_key_file_paths(path)?)
        }
    }

    Ok(key_file_paths)
}

pub fn list_available_devices(config_dir: &Path) -> LocalDeviceResult<Vec<AvailableDevice>> {
    let mut list = vec![];
    // Set of seen slugs
    let mut seen = HashSet::new();

    let key_file_paths = PathBuf::from(config_dir).join("devices");

    // Consider `.keys` files in devices directory
    let mut key_file_paths = read_key_file_paths(key_file_paths)?;

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
