// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec_platform_filesystem::save_content;
use libparsec_types::prelude::*;

use crate::{
    AccountVaultOperationsUploadOpaqueKeyError, AvailableDevice, DeviceSaveStrategy,
    OpenBaoOperationsUploadOpaqueKeyError, RemoteOperationServer, SaveDeviceError,
};

pub struct Storage {}

impl Storage {
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
}
