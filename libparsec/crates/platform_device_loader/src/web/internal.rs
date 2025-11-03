// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use libparsec_platform_async::{lock::Mutex, stream::StreamExt};
use libparsec_types::prelude::*;

use crate::{
    web::wrapper::{DirEntry, DirOrFileHandle, OpenOptions},
    AccountVaultOperationsUploadOpaqueKeyError, AvailableDevice, DeviceSaveStrategy,
};

use super::{
    error::*,
    wrapper::{Directory, File},
};

pub struct Storage {
    root_dir: Directory,
}

impl Storage {
    pub(crate) async fn new() -> Result<Self, NewStorageError> {
        let root_dir = Directory::get_root().await?;
        Ok(Self { root_dir })
    }

    pub(crate) async fn list_available_devices(
        &self,
        config_dir: &Path,
    ) -> Result<Vec<AvailableDevice>, ListAvailableDevicesError> {
        let devices_dir = crate::get_devices_dir(config_dir);
        log::debug!(
            "Will list devices starting with `{}` and having suffix {}",
            devices_dir.display(),
            crate::DEVICE_FILE_EXT
        );
        let dir = match self
            .root_dir
            .get_directory_from_path(&devices_dir, None)
            .await
        {
            Ok(dir) => dir,
            Err(GetDirectoryHandleError::NotFound { .. }) => {
                log::debug!("Could not found devices dir");
                return Ok(Vec::new());
            }
            Err(e) => return Err(e.into()),
        };
        let mut devices = Vec::<AvailableDevice>::new();
        let dirs_to_explore = Rc::new(Mutex::new(Vec::from([dir])));
        while let Some(dir) = {
            let mut handle = dirs_to_explore.lock().await;
            let res = handle.pop();
            drop(handle);
            res
        } {
            log::trace!("Explore dir {}", dir.path.display());
            let mut entries_stream = dir.entries();
            while let Some(entry) = entries_stream.next().await {
                let DirEntry { path, handle } = entry;
                log::trace!(
                    "Testing entry {} with extension {:?}",
                    path.display(),
                    path.extension()
                );
                match handle {
                    DirOrFileHandle::File(handle)
                        if path.extension() == Some(crate::DEVICE_FILE_EXT.as_ref()) =>
                    {
                        log::trace!("Try to load device {}", path.display());
                        let dev = match load_available_device(&File { path, handle }).await {
                            Ok(dev) => dev,
                            // Ignore devices that we cannot deserialize
                            Err(LoadAvailableDeviceError::RmpDecode(_)) => continue,
                            Err(LoadAvailableDeviceError::ReadToEnd(e)) => {
                                return Err(ListAvailableDevicesError::from(e))
                            }
                        };
                        devices.push(dev)
                    }
                    DirOrFileHandle::File(_) => {
                        log::trace!("Ignoring file {} because of bad suffix", path.display());
                    }
                    DirOrFileHandle::Dir(handle) => {
                        dirs_to_explore
                            .lock()
                            .await
                            .push(Directory { path, handle });
                    }
                }
            }
        }

        devices.sort_by(|a, b| a.key_file_path.cmp(&b.key_file_path));

        Ok(devices)
    }

    pub(crate) async fn read_file(&self, file: &Path) -> Result<Vec<u8>, ReadFile> {
        let file = self.root_dir.get_file_from_path(file, None).await?;
        file.read_to_end().await.map_err(|e| e.into())
    }

    pub(crate) async fn save_device(
        &self,
        strategy: &DeviceSaveStrategy,
        key_file: PathBuf,
        device: &LocalDevice,
        created_on: DateTime,
    ) -> Result<AvailableDevice, SaveDeviceError> {
        let protected_on = device.now();
        let server_url = crate::server_url_from_device(device);

        match strategy {
            DeviceSaveStrategy::Password { password, .. } => {
                let key_algo =
                    PasswordAlgorithm::generate_argon2id(PasswordAlgorithmSaltStrategy::Random);
                let key = key_algo
                    .compute_secret_key(password)
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

                self.save_device_file(&key_file, &file_data).await?;
            }
            DeviceSaveStrategy::AccountVault { operations } => {
                let (ciphertext_key_id, ciphertext_key) = operations
                    .upload_opaque_key()
                    .await
                    .map_err(|err| match err {
                        AccountVaultOperationsUploadOpaqueKeyError::BadVaultKeyAccess(err) => {
                            SaveDeviceError::RemoteOpaqueKeyUploadFailed(err.into())
                        }
                        AccountVaultOperationsUploadOpaqueKeyError::Offline(err) => {
                            SaveDeviceError::RemoteOpaqueKeyUploadOffline(err)
                        }
                        AccountVaultOperationsUploadOpaqueKeyError::Internal(err) => {
                            SaveDeviceError::Internal(err)
                        }
                    })?;

                let ciphertext = crate::encrypt_device(device, &ciphertext_key);
                let file_data = DeviceFile::AccountVault(DeviceFileAccountVault {
                    created_on,
                    protected_on,
                    server_url: server_url.clone(),
                    organization_id: device.organization_id().to_owned(),
                    user_id: device.user_id,
                    device_id: device.device_id,
                    human_handle: device.human_handle.clone(),
                    device_label: device.device_label.clone(),
                    ciphertext_key_id,
                    ciphertext,
                });

                self.save_device_file(&key_file, &file_data).await?;
            }
            DeviceSaveStrategy::Keyring { .. } => panic!("Keyring not supported on Web"),
            DeviceSaveStrategy::Smartcard { .. } => panic!("Smartcard not supported on Web"),
        }

        Ok(AvailableDevice {
            key_file_path: key_file.to_owned(),
            created_on,
            protected_on,
            server_url,
            organization_id: device.organization_id().to_owned(),
            user_id: device.user_id,
            device_id: device.device_id,
            human_handle: device.human_handle.clone(),
            device_label: device.device_label.clone(),
            ty: strategy.ty(),
        })
    }

    pub(crate) async fn save_device_file(
        &self,
        key: &Path,
        file_data: &DeviceFile,
    ) -> Result<(), SaveRawDataError> {
        let data = file_data.dump();
        self.save_raw_data(key, &data).await
    }

    pub(crate) async fn save_raw_data(
        &self,
        path: &Path,
        data: &[u8],
    ) -> Result<(), SaveRawDataError> {
        log::trace!("Saving device file at {}", path.display());
        let parent = if let Some(parent) = path.parent() {
            Some(self.root_dir.create_dir_all(parent).await?)
        } else {
            None
        };
        let file = parent
            .as_ref()
            .unwrap_or(&self.root_dir)
            .get_file(
                path.file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .expect("Missing filename"),
                Some(OpenOptions::create()),
            )
            .await?;
        file.write_all(&data).await.map_err(Into::into)
    }

    pub(crate) async fn archive_device(&self, path: &Path) -> Result<(), ArchiveDeviceError> {
        let old_device = self
            .root_dir
            .get_file_from_path(path, None)
            .await
            .map_err(ArchiveDeviceError::GetDeviceToArchive)?;
        let old_data = old_device
            .read_to_end()
            .await
            .map_err(ArchiveDeviceError::ReadDeviceToArchive)?;

        let archive_path = crate::get_device_archive_path(path);
        let archive_device = self
            .root_dir
            .get_file_from_path(&archive_path, Some(OpenOptions::create()))
            .await
            .map_err(ArchiveDeviceError::CreateArchiveDevice)?;
        archive_device
            .write_all(&old_data)
            .await
            .map_err(ArchiveDeviceError::WriteArchiveDevice)?;

        self.root_dir
            .remove_entry_from_path(path)
            .await
            .map_err(Into::into)
    }

    pub(crate) async fn remove_device(&self, path: &Path) -> Result<(), RemoveDeviceError> {
        self.root_dir
            .remove_entry_from_path(path)
            .await
            .map_err(Into::into)
    }

    pub(crate) async fn save_pki_local_pending(
        &self,
        local_file: PathBuf,
        file_data: LocalPendingEnrollment,
    ) -> Result<(), SaveRawDataError> {
        let data = file_data.dump();
        self.save_raw_data(&local_file, &data).await
    }
}

async fn load_available_device(file: &File) -> Result<AvailableDevice, LoadAvailableDeviceError> {
    let raw_data = file
        .read_to_end()
        .await
        .map_err(LoadAvailableDeviceError::ReadToEnd)?;
    crate::load_available_device_from_blob(file.path().to_owned(), &raw_data)
        .inspect_err(|e| {
            log::warn!(
                "Failed to decode device from {}: {e}",
                file.path().display()
            )
        })
        .map_err(LoadAvailableDeviceError::RmpDecode)
}
