// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashSet, path::Path, sync::Arc};

use base64::prelude::{Engine as _, BASE64_STANDARD};
use libparsec_types::{
    AvailableDevice, DateTime, DeviceAccessStrategy, DeviceFile, DeviceFilePassword, LocalDevice,
};

use super::error::*;

pub struct Storage {
    storage: web_sys::Storage,
}

impl Storage {
    /// Entry in the local storage where the devices keys are stored.
    const LIST_DEV_KEY: &'static str = "parsec_devices";
    /// Entry in the local storage where archived devices keys are stored.
    const ARCHIVED_DEV_KEY_LIST: &'static str = "archived_parsec_devices";

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
        let device_prefix =
            config_dir
                .to_str()
                .ok_or_else(|| ListAvailableDevicesError::InvalidPath {
                    path: config_dir.to_owned(),
                })?;
        let device_prefix = format!("{device_prefix}/");
        let Some(raw_data) = self.storage.get_item(Self::LIST_DEV_KEY).map_err(|e| {
            ListAvailableDevicesError::GetItemStorage {
                key: Self::LIST_DEV_KEY.to_owned(),
                error: e,
            }
        })?
        else {
            return Ok(Vec::new());
        };
        let mut entries = serde_json::from_str::<Vec<&str>>(&raw_data)
            .inspect_err(|e| log::warn!("Invalid device list: {e}"))
            .map_err(ListAvailableDevicesError::JsonDecode)?;
        entries.sort();
        entries
            .into_iter()
            .filter(|key| key.starts_with(&device_prefix))
            .map(|key| self.load_available_device(key).map_err(Into::into))
            .collect::<Result<Vec<_>, _>>()
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
        BASE64_STANDARD
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
        let key = key_path
            .to_str()
            .ok_or_else(|| LoadDeviceError::InvalidPath {
                path: key_path.to_owned(),
            })?;
        let raw_data = self.get_raw_device(key)?;
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
        let key_file = key_file_path
            .to_str()
            .ok_or_else(|| SaveDeviceError::InvalidPath {
                path: key_file_path.to_owned(),
            })?;

        match access {
            DeviceAccessStrategy::Keyring { .. } => todo!("Save keyring device"),
            DeviceAccessStrategy::Password { password, .. } => {
                let key_algo = crate::generate_default_password_algorithm_parameters();
                let key = crate::secret_key_from_password(password, &key_algo)
                    .expect("Salt has the correct length");
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

                self.save_device_file(key_file, &file_data)?;
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
        let b64_data = BASE64_STANDARD.encode(&data);
        self.storage
            .set_item(key, &b64_data)
            .map_err(|e| SaveDeviceFileError::SetItemStorage {
                key: key.to_owned(),
                error: e,
            })?;

        add_item_to_list(&self.storage, Self::LIST_DEV_KEY, key)
            .and(Ok(()))
            .map_err(Into::into)
    }

    pub(crate) fn archive_device(&self, path: &Path) -> Result<(), ArchiveDeviceError> {
        let key = path
            .to_str()
            .ok_or_else(|| ArchiveDeviceError::InvalidPath {
                path: path.to_owned(),
            })?;
        if remove_item_from_list(&self.storage, Self::LIST_DEV_KEY, key)? {
            add_item_to_list(&self.storage, Self::ARCHIVED_DEV_KEY_LIST, key)?;
        }
        Ok(())
    }

    pub(crate) fn remove_device(&self, path: &Path) -> Result<(), RemoveDeviceError> {
        let key = path
            .to_str()
            .ok_or_else(|| RemoveDeviceError::InvalidPath {
                path: path.to_owned(),
            })?;
        if remove_item_from_list(&self.storage, Self::LIST_DEV_KEY, key)? {
            self.storage
                .delete(key)
                .map_err(|e| RemoveDeviceError::RemoveItemStorage {
                    key: key.to_owned(),
                    error: e,
                })?;
        }
        Ok(())
    }
}

/// Add the given item to the list identified by `list_id`.
///
/// If the list does not exist, it will create it.
///
/// # Returns
///
/// Will return true if the item was added to the list, false if the item was already in the list.
pub(crate) fn add_item_to_list(
    storage: &web_sys::Storage,
    list_id: &str,
    item: &str,
) -> Result<bool, AddItemToListError> {
    let raw_list = storage
        .get_item(list_id)
        .map_err(|e| AddItemToListError::GetItemStorage {
            key: list_id.to_owned(),
            error: e,
        })?;

    let mut list = raw_list
        .as_ref()
        .map(|v| serde_json::from_str::<HashSet<&str>>(v).map_err(AddItemToListError::JsonDecode))
        .unwrap_or_else(|| Ok(HashSet::new()))?;
    if list.insert(item) {
        let raw_data = serde_json::to_string(&list).map_err(AddItemToListError::JsonEncode)?;
        storage
            .set_item(list_id, &raw_data)
            .map_err(|e| AddItemToListError::SetItemStorage {
                key: list_id.to_owned(),
                error: e,
            })?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Remove the given item from the list identified by `list_id`.
///
/// If the list does not exist, it is not created.
///
/// # Returns
///
/// Will return trun if the item was removed from the list, false if the item was not in the list.
pub(crate) fn remove_item_from_list(
    storage: &web_sys::Storage,
    list_id: &str,
    item: &str,
) -> Result<bool, RemoveItemFromListError> {
    let raw_list =
        storage
            .get_item(list_id)
            .map_err(|e| RemoveItemFromListError::GetItemStorage {
                key: list_id.to_owned(),
                error: e,
            })?;

    let Some(mut list) = raw_list
        .as_ref()
        .map(|v| {
            serde_json::from_str::<HashSet<&str>>(v).map_err(RemoveItemFromListError::JsonDecode)
        })
        .transpose()?
    else {
        return Ok(false);
    };
    if list.remove(item) {
        let raw_data = serde_json::to_string(&list).map_err(RemoveItemFromListError::JsonEncode)?;
        storage.set_item(list_id, &raw_data).map_err(|e| {
            RemoveItemFromListError::SetItemStorage {
                key: list_id.to_owned(),
                error: e,
            }
        })?;
        Ok(true)
    } else {
        Ok(false)
    }
}
