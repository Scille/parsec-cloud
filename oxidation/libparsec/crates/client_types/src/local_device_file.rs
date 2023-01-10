// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

use libparsec_crypto::SecretKey;
use libparsec_types::{DeviceID, DeviceLabel, HumanHandle, OrganizationID};

use crate::{LocalDevice, LocalDeviceError, LocalDeviceResult, StrPath};

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
        let data = std::fs::read(key_file_path)
            .map_err(|_| LocalDeviceError::Access(key_file_path.to_path_buf()))?;

        rmp_serde::from_slice::<DeviceFile>(&data)
            .map_err(|_| LocalDeviceError::Deserialization(key_file_path.to_path_buf()))
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum LegacyDeviceFile {
    Password(LegacyDeviceFilePassword),
}

impl LegacyDeviceFile {
    pub fn decode(serialized: &[u8]) -> Result<Self, &'static str> {
        rmp_serde::from_slice(serialized).map_err(|_| "Invalid serialization")
    }

    #[cfg(test)]
    pub fn dump(&self) -> Vec<u8> {
        rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!())
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
            key_file_path: key_file_path.into(),
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

    let key_file_paths = config_dir.join("devices");

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

pub async fn save_recovery_device(
    key_file: &Path,
    device: LocalDevice,
    force: bool,
) -> LocalDeviceResult<String> {
    if File::open(key_file).is_ok() && !force {
        return Err(LocalDeviceError::AlreadyExists(key_file.to_path_buf()));
    }

    let (passphrase, key) = SecretKey::generate_recovery_passphrase();

    let ciphertext = key.encrypt(&device.dump());

    let key_file_content = DeviceFile::Recovery(DeviceFileRecovery {
        ciphertext,
        organization_id: device.organization_id().clone(),
        slug: device.slug(),
        human_handle: device.human_handle,
        device_label: device.device_label,
        device_id: device.device_id,
    });

    key_file_content.save(key_file)?;

    Ok(passphrase)
}

/// TODO: need test (backend_cmds required)
pub async fn load_recovery_device(
    key_file: &Path,
    passphrase: &str,
) -> LocalDeviceResult<LocalDevice> {
    let ciphertext =
        std::fs::read(key_file).map_err(|_| LocalDeviceError::Access(key_file.to_path_buf()))?;
    let data = rmp_serde::from_slice::<DeviceFile>(&ciphertext)
        .map_err(|_| LocalDeviceError::Deserialization(key_file.to_path_buf()))?;

    let device = match data {
        DeviceFile::Recovery(device) => device,
        _ => return Err(LocalDeviceError::Validation(DeviceFileType::Recovery)),
    };

    let key =
        SecretKey::from_recovery_passphrase(passphrase).map_err(LocalDeviceError::CryptoError)?;
    let plaintext = key
        .decrypt(&device.ciphertext)
        .map_err(LocalDeviceError::CryptoError)?;
    LocalDevice::load(&plaintext)
        .map_err(|_| LocalDeviceError::Deserialization(key_file.to_path_buf()))
}
