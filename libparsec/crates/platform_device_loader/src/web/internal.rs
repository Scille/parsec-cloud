// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    rc::Rc,
};

use libparsec_platform_async::{lock::Mutex, stream::StreamExt};
use libparsec_platform_filesystem::save_content;
use libparsec_types::prelude::*;

use crate::SaveDeviceError;
use crate::{
    web::wrapper::{DirEntry, DirOrFileHandle, OpenOptions},
    AccountVaultOperationsUploadOpaqueKeyError, AvailableDevice, DeviceSaveStrategy,
    OpenBaoOperationsUploadOpaqueKeyError, RemoteOperationServer,
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

        let file_entries = self
            .list_file_entries(&devices_dir, crate::DEVICE_FILE_EXT)
            .await?;
        let mut devices = Vec::with_capacity(file_entries.len());
        for file_entry in file_entries {
            match load_available_device(&file_entry).await {
                Ok(device) => {
                    devices.push(device);
                }
                // Ignore invalid files
                Err(LoadAvailableDeviceError::RmpDecode(_)) => continue,
                // Unexpected error, make it bubble up
                Err(e) => return Err(ListAvailableDevicesError::from(e)),
            }
        }

        devices.sort_by(|a, b| a.key_file_path.cmp(&b.key_file_path));

        Ok(devices)
    }

    pub(super) async fn list_file_entries(
        &self,
        dir: &Path,
        extension: impl AsRef<OsStr> + std::fmt::Display,
    ) -> Result<Vec<File>, ListFileEntriesError> {
        log::debug!(
            "Listing file entries in {} with extension {extension}",
            dir.display()
        );
        let dir = match self.root_dir.get_directory_from_path(&dir, None).await {
            Ok(dir) => dir,
            Err(GetDirectoryHandleError::NotFound { .. }) => {
                log::debug!("Could not find devices dir");
                return Ok(Vec::new());
            }
            Err(e) => return Err(e.into()),
        };
        let mut files = Vec::<File>::new();
        let dirs_to_explore = Rc::new(Mutex::new(Vec::from([dir])));
        while let Some(dir) = {
            let mut handle = dirs_to_explore.lock().await;
            let res = handle.pop();
            drop(handle);
            res
        } {
            log::trace!("Exploring directory {}", dir.path.display());
            let mut entries_stream = dir.entries();
            while let Some(entry) = entries_stream.next().await {
                let DirEntry { path, handle } = entry;
                match handle {
                    DirOrFileHandle::File(handle)
                        if path.extension() == Some(extension.as_ref()) =>
                    {
                        log::trace!("File {} with correct extension", path.display());
                        files.push(File { path, handle })
                    }
                    DirOrFileHandle::File(_) => {
                        log::trace!("Ignoring file {} because of bad suffix", path.display());
                    }
                    DirOrFileHandle::Dir(handle) => {
                        log::trace!("New directory to explore: {}", path.display());
                        dirs_to_explore
                            .lock()
                            .await
                            .push(Directory { path, handle });
                    }
                }
            }
        }
        Ok(files)
    }

    pub(crate) async fn read_file(&self, file: &Path) -> Result<Vec<u8>, ReadFile> {
        let file = self.root_dir.get_file_from_path(file, None).await?;
        file.read_to_end().await.map_err(|e| e.into())
    }

    pub(crate) async fn save_device(
        strategy: &DeviceSaveStrategy,
        key_file: PathBuf,
        device: &LocalDevice,
        created_on: DateTime,
    ) -> Result<AvailableDevice, SaveDeviceError> {
        let protected_on = device.now();
        let server_addr: ParsecAddr = device.organization_addr.clone().into();

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
                    server_url: server_addr.clone(),
                    organization_id: device.organization_id().to_owned(),
                    user_id: device.user_id,
                    device_id: device.device_id,
                    human_handle: device.human_handle.clone(),
                    device_label: device.device_label.clone(),
                    algorithm: key_algo,
                    ciphertext,
                });

                save_content(&key_file, &file_data.dump()).await?
            }

            DeviceSaveStrategy::AccountVault { operations } => {
                let (ciphertext_key_id, ciphertext_key) = operations
                    .upload_opaque_key()
                    .await
                    .map_err(|err| match err {
                        AccountVaultOperationsUploadOpaqueKeyError::BadVaultKeyAccess(_)
                        | AccountVaultOperationsUploadOpaqueKeyError::BadServerResponse(_) => {
                            SaveDeviceError::RemoteOpaqueKeyUploadFailed {
                                server: RemoteOperationServer::ParsecAccount,
                                error: err.into(),
                            }
                        }
                        AccountVaultOperationsUploadOpaqueKeyError::Offline(_) => {
                            SaveDeviceError::RemoteOpaqueKeyUploadOffline {
                                server: RemoteOperationServer::ParsecAccount,
                                error: err.into(),
                            }
                        }
                    })?;

                let ciphertext = crate::encrypt_device(device, &ciphertext_key);
                let file_data = DeviceFile::AccountVault(DeviceFileAccountVault {
                    created_on,
                    protected_on,
                    server_url: server_addr.clone(),
                    organization_id: device.organization_id().to_owned(),
                    user_id: device.user_id,
                    device_id: device.device_id,
                    human_handle: device.human_handle.clone(),
                    device_label: device.device_label.clone(),
                    ciphertext_key_id,
                    ciphertext,
                });

                save_content(&key_file, &file_data.dump()).await?
            }

            DeviceSaveStrategy::OpenBao { operations } => {
                let (openbao_ciphertext_key_path, ciphertext_key) = operations
                    .upload_opaque_key()
                    .await
                    .map_err(|err| match err {
                        OpenBaoOperationsUploadOpaqueKeyError::NoServerResponse(_) => {
                            SaveDeviceError::RemoteOpaqueKeyUploadOffline {
                                server: RemoteOperationServer::OpenBao,
                                error: err.into(),
                            }
                        }
                        OpenBaoOperationsUploadOpaqueKeyError::BadURL(_)
                        | OpenBaoOperationsUploadOpaqueKeyError::BadServerResponse(_) => {
                            SaveDeviceError::RemoteOpaqueKeyUploadFailed {
                                server: RemoteOperationServer::OpenBao,
                                error: err.into(),
                            }
                        }
                    })?;

                let ciphertext = crate::encrypt_device(device, &ciphertext_key);

                let file_data = DeviceFile::OpenBao(DeviceFileOpenBao {
                    created_on,
                    protected_on,
                    server_url: server_addr.clone(),
                    organization_id: device.organization_id().to_owned(),
                    user_id: device.user_id,
                    device_id: device.device_id,
                    human_handle: device.human_handle.to_owned(),
                    device_label: device.device_label.to_owned(),
                    openbao_preferred_auth_id: operations.openbao_preferred_auth_id().to_owned(),
                    openbao_entity_id: operations.openbao_entity_id().to_owned(),
                    openbao_ciphertext_key_path,
                    ciphertext,
                });

                save_content(&key_file, &file_data.dump()).await?
            }

            DeviceSaveStrategy::Keyring { .. } => panic!("Keyring not supported on Web"),
            DeviceSaveStrategy::PKI { .. } => panic!("PKI not supported on Web"),
        }

        Ok(AvailableDevice {
            key_file_path: key_file.to_owned(),
            created_on,
            protected_on,
            server_addr,
            organization_id: device.organization_id().to_owned(),
            user_id: device.user_id,
            device_id: device.device_id,
            human_handle: device.human_handle.clone(),
            device_label: device.device_label.clone(),
            ty: strategy.ty(),
        })
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

    pub(crate) async fn list_pki_local_pending(
        &self,
        config_dir: &Path,
    ) -> Result<Vec<PKILocalPendingEnrollment>, ListPkiLocalPendingError> {
        let pending_dir = crate::get_local_pending_dir(config_dir);
        let file_entries = self
            .list_file_entries(&pending_dir, crate::LOCAL_PENDING_EXT)
            .await?;
        let mut entries = Vec::with_capacity(file_entries.len());
        for file_entry in file_entries {
            match load_pki_local_pending(&file_entry).await {
                Ok(device) => {
                    entries.push(device);
                }
                // Ignore invalid files
                Err(LoadPkiLocalPendingError::DataError(_)) => continue,
                // Unexpected error, make it bubble up
                Err(e) => return Err(ListPkiLocalPendingError::from(e)),
            }
        }

        entries.sort_by_key(|v| v.enrollment_id);

        Ok(entries)
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

async fn load_pki_local_pending(
    file: &File,
) -> Result<PKILocalPendingEnrollment, LoadPkiLocalPendingError> {
    let raw_data = file
        .read_to_end()
        .await
        .map_err(LoadPkiLocalPendingError::ReadToEnd)?;
    PKILocalPendingEnrollment::load(&raw_data)
        .inspect_err(|err| {
            log::warn!(
                "Failed to decode local pending at {}: {err}",
                file.path().display()
            )
        })
        .map_err(LoadPkiLocalPendingError::DataError)
}
