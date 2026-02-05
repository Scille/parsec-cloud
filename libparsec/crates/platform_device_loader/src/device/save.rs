// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{encrypt_device, platform};
use libparsec_platform_filesystem::{save_content, SaveContentError};
use std::path::PathBuf;

use crate::{
    AccountVaultOperationsUploadOpaqueKeyError, AvailableDevice, DeviceSaveStrategy,
    OpenBaoOperationsUploadOpaqueKeyError, RemoteOperationServer,
};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum SaveDeviceError {
    #[error("No space available")]
    NoSpaceAvailable,
    #[error("Path is invalid")]
    InvalidPath,
    /// Note only a subset of save strategies requires server access to
    /// upload an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("No response from {server} server: {error}")]
    // We don't use `ConnectionError` here since this type only corresponds to
    // an answer from the Parsec server and here any arbitrary server may have
    // been (unsuccessfully) requested (e.g. OpenBao server).
    RemoteOpaqueKeyUploadOffline {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    /// Note only a subset of save strategies requires server access to
    /// upload an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("{server} server opaque key upload failed: {error}")]
    RemoteOpaqueKeyUploadFailed {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<SaveContentError> for SaveDeviceError {
    fn from(value: SaveContentError) -> Self {
        match value {
            SaveContentError::NotAFile
            | SaveContentError::InvalidParent
            | SaveContentError::InvalidPath
            | SaveContentError::ParentNotFound
            | SaveContentError::CannotEdit => SaveDeviceError::InvalidPath,

            SaveContentError::StorageNotAvailable | SaveContentError::NoSpaceLeft => {
                SaveDeviceError::NoSpaceAvailable
            }
            SaveContentError::Internal(_) => SaveDeviceError::Internal(value.into()),
        }
    }
}

pub(crate) async fn save_device(
    strategy: &DeviceSaveStrategy,
    device: &LocalDevice,
    created_on: DateTime,
    key_file: PathBuf,
) -> Result<AvailableDevice, SaveDeviceError> {
    let protected_on = device.now();
    let server_addr: ParsecAddr = device.organization_addr.clone().into();

    match strategy {
        DeviceSaveStrategy::Keyring => {
            platform::save_device_keyring(
                device,
                &created_on,
                &key_file,
                &server_addr,
                &protected_on,
            )
            .await?;
        }

        DeviceSaveStrategy::Password { password } => {
            let key_algo =
                PasswordAlgorithm::generate_argon2id(PasswordAlgorithmSaltStrategy::Random);
            let key = key_algo
                .compute_secret_key(password)
                .expect("Failed to derive key from password");

            let ciphertext = encrypt_device(device, &key);

            let file_content = DeviceFile::Password(DeviceFilePassword {
                created_on,
                protected_on,
                server_url: server_addr.clone(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.to_owned(),
                device_label: device.device_label.to_owned(),
                algorithm: key_algo,
                ciphertext,
            });

            let file_content = file_content.dump();

            save_content(&key_file, &file_content).await?;
        }

        DeviceSaveStrategy::PKI { certificate_ref } => {
            platform::save_device_pki(
                device,
                &created_on,
                &key_file,
                &server_addr,
                &protected_on,
                certificate_ref,
            )
            .await?;
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

            let ciphertext = encrypt_device(device, &ciphertext_key);

            let file_content = DeviceFile::AccountVault(DeviceFileAccountVault {
                created_on,
                protected_on,
                server_url: server_addr.clone(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.to_owned(),
                device_label: device.device_label.to_owned(),
                ciphertext_key_id,
                ciphertext,
            });

            let file_content = file_content.dump();

            save_content(&key_file, &file_content).await?;
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

            let ciphertext = encrypt_device(device, &ciphertext_key);

            let file_content = DeviceFile::OpenBao(DeviceFileOpenBao {
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

            let file_content = file_content.dump();

            save_content(&key_file, &file_content).await?;
        }
    }

    Ok(AvailableDevice {
        key_file_path: key_file,
        server_addr,
        created_on,
        protected_on,
        organization_id: device.organization_id().to_owned(),
        user_id: device.user_id,
        device_id: device.device_id,
        device_label: device.device_label.clone(),
        human_handle: device.human_handle.clone(),
        ty: strategy.ty(),
    })
}
