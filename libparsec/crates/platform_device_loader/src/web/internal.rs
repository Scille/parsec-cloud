// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use itertools::Itertools;
use libparsec_types::{
    AvailableDevice, DateTime, DeviceAccessStrategy, DeviceFile, DeviceFilePassword, LocalDevice,
};

use super::error::*;

pub struct Storage {
    storage: web_sys::Storage,
}

impl Storage {
    pub(crate) fn new() -> Result<Self, NewStorageError> {
        let window = web_sys::window().ok_or(NewStorageError::NoWindow)?;
        let storage = if cfg!(test) {
            // Use session storage for tests, So the browser does not try to persist the data.
            window.session_storage()
        } else {
            window.local_storage()
        }
        .map_err(NewStorageError::WindowError)
        .and_then(|v| v.ok_or(NewStorageError::NoLocalStorage))?;
        Ok(Self { storage })
    }

    pub(crate) fn list_available_devices(
        &self,
        config_dir: &Path,
    ) -> Result<Vec<AvailableDevice>, ListAvailableDevicesError> {
        let devices_dir = crate::get_devices_dir(config_dir);
        let device_prefix = format!("{}/", devices_dir.display());
        log::debug!("Will list devices starting with `{device_prefix}`");
        let items_count =
            self.storage
                .length()
                .map_err(|e| ListAvailableDevicesError::GetItemStorage {
                    key: "length".to_owned(),
                    error: e,
                })?;
        Ok((0..items_count)
            .filter_map(|i| {
                let key = self.storage.key(i).ok().flatten();
                match key {
                    Some(key)
                        if key.starts_with(&device_prefix)
                            && key.ends_with(crate::DEVICE_FILE_EXT) =>
                    {
                        log::trace!("Device {key} included in list");
                        Some(key)
                    }
                    _ => {
                        log::trace!("Device {key:?} not included in list");
                        None
                    }
                }
            })
            .sorted()
            .filter_map(|key| {
                self.load_available_device(&key)
                    .inspect_err(|e| log::warn!("Cannot load device {key}: {e}"))
                    .ok()
            })
            .collect::<Vec<_>>())
    }

    fn get_raw_device(&self, key: &str) -> Result<Vec<u8>, GetRawDeviceError> {
        let raw_b64_data = self
            .storage
            .get_item(key)
            .map_err(|e| GetRawDeviceError::GetItemStorage {
                key: key.to_owned(),
                error: e,
            })
            .and_then(|v| {
                v.ok_or_else(|| GetRawDeviceError::Missing {
                    key: key.to_owned(),
                })
            })?;
        data_encoding::BASE64
            .decode(raw_b64_data.as_bytes())
            .map_err(GetRawDeviceError::B64Decode)
    }

    fn load_available_device(
        &self,
        key: &str,
    ) -> Result<AvailableDevice, LoadAvailableDeviceError> {
        let raw_data = self.get_raw_device(key)?;
        crate::load_available_device_from_blob(key.into(), &raw_data)
            .inspect_err(|e| log::warn!("Failed to load device from {key}: {e}"))
            .map_err(Into::into)
    }

    pub(crate) fn load_device(
        &self,
        access: &DeviceAccessStrategy,
    ) -> Result<(Arc<LocalDevice>, DateTime), LoadDeviceError> {
        let key_path = access.key_file();
        let key = key_path.to_string_lossy();
        let raw_data = self.get_raw_device(&key)?;
        let device = DeviceFile::load(&raw_data)?;
        let (key, created_on) = match (access, &device) {
            (DeviceAccessStrategy::Keyring { .. }, DeviceFile::Keyring(_device)) => {
                todo!("Access keyring device")
            }
            (DeviceAccessStrategy::Password { password, .. }, DeviceFile::Password(device)) => {
                let key = crate::secret_key_from_password(password, &device.algorithm)
                    .map_err(LoadDeviceError::GetSecretKey)?;
                Ok((key, device.created_on))
            }
            (DeviceAccessStrategy::Smartcard { .. }, DeviceFile::Smartcard(_device)) => {
                todo!("Access smartcard device")
            }
            _ => Err(LoadDeviceError::InvalidFileType),
        }?;

        let device = crate::decrypt_device_file(&device, &key)?;
        Ok((Arc::new(device), created_on))
    }

    pub(crate) fn save_device(
        &self,
        access: &DeviceAccessStrategy,
        device: &LocalDevice,
        created_on: DateTime,
    ) -> Result<AvailableDevice, SaveDeviceError> {
        let protected_on = device.now();
        let server_url = crate::server_url_from_device(device);
        let key_file_path = access.key_file();
        let key_file = key_file_path.to_string_lossy();

        match access {
            DeviceAccessStrategy::Keyring { .. } => todo!("Save keyring device"),
            DeviceAccessStrategy::Password { password, .. } => {
                let key_algo = crate::generate_default_password_algorithm_parameters();
                let key = crate::secret_key_from_password(password, &key_algo)
                    .expect("Failed to derive key from password");
                let ciphertext = crate::encrypt_device(device, &key);
                let file_data = DeviceFile::Password(DeviceFilePassword {
                    created_on,
                    protected_on,
                    server_url: server_url.clone(),
                    organization_id: device.organization_id().to_owned(),
                    user_id: device.user_id,
                    device_id: device.device_id,
                    human_handle: device.human_handle.clone(),
                    device_label: device.device_label.clone(),
                    algorithm: key_algo,
                    ciphertext,
                });

                self.save_device_file(&key_file, &file_data)?;
            }
            DeviceAccessStrategy::Smartcard { .. } => todo!("Save smartcard device"),
        }

        Ok(AvailableDevice {
            key_file_path: key_file_path.to_owned(),
            created_on,
            protected_on,
            server_url,
            organization_id: device.organization_id().to_owned(),
            user_id: device.user_id,
            device_id: device.device_id,
            human_handle: device.human_handle.clone(),
            device_label: device.device_label.clone(),
            ty: access.ty(),
        })
    }

    pub(crate) fn save_device_file(
        &self,
        key: &str,
        file_data: &DeviceFile,
    ) -> Result<(), SaveDeviceFileError> {
        let data = file_data.dump();
        let b64_data = data_encoding::BASE64.encode(&data);
        log::trace!("Saving device using key={key}");
        self.storage
            .set_item(key, &b64_data)
            .map_err(|e| SaveDeviceFileError::SetItemStorage {
                key: key.to_owned(),
                error: e,
            })
    }

    pub(crate) fn archive_device(&self, path: &Path) -> Result<(), ArchiveDeviceError> {
        let key = path.to_string_lossy();
        let device_data = self
            .storage
            .get_item(&key)
            .map_err(|error| ArchiveDeviceError::GetItemStorage {
                key: key.to_string(),
                error,
            })
            .and_then(|v| {
                v.ok_or_else(|| ArchiveDeviceError::Missing {
                    key: key.to_string(),
                })
            })?;

        let archived_key = format!("{key}.archived");
        self.storage
            .set_item(&archived_key, &device_data)
            .map_err(|error| ArchiveDeviceError::SetItemStorage {
                key: key.to_string(),
                error,
            })?;
        self.storage
            .remove_item(&key)
            .map_err(|error| ArchiveDeviceError::RemoveItemStorage {
                key: key.to_string(),
                error,
            })
    }

    pub(crate) fn remove_device(&self, path: &Path) -> Result<(), RemoveDeviceError> {
        let key = path.to_string_lossy();
        self.storage
            .delete(&key)
            .map_err(|e| RemoveDeviceError::RemoveItemStorage {
                key: key.to_string(),
                error: e,
            })
    }
}
