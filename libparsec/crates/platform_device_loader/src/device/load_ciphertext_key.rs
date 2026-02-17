// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    platform, AccountVaultOperationsFetchOpaqueKeyError, DeviceAccessStrategy,
    DeviceCiphertextKeys, OpenBaoOperationsFetchOpaqueKeyError, RemoteOperationServer,
};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum LoadCiphertextKeyError {
    /// Typically returned if the access strategy doesn't match the device type
    #[error("Invalid data")]
    InvalidData,
    #[error("Decryption failed")]
    #[cfg_attr(target_arch = "wasm32", expect(dead_code))]
    DecryptionFailed,
    /// Note only a subset of load strategies requires server access to
    /// fetch an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("No response from {server} server: {error}")]
    // We don't use `ConnectionError` here since this type only corresponds to
    // an answer from the Parsec server and here any arbitrary server may have
    // been (unsuccessfully) requested (e.g. OpenBao server).
    RemoteOpaqueKeyFetchOffline {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    /// Note only a subset of load strategies requires server access to
    /// fetch an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("{server} server opaque key fetch failed: {error}")]
    RemoteOpaqueKeyFetchFailed {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    #[error(transparent)]
    Internal(anyhow::Error),
}

pub(super) async fn load_ciphertext_key(
    access: &DeviceAccessStrategy,
    device_file: &DeviceFile,
) -> Result<DeviceCiphertextKeys, LoadCiphertextKeyError> {
    // Don't do `match (access, device_file)` since we would end up with a catch-all
    // `(_, _) => return <error>` condition that would prevent this code from breaking
    // whenever a new variant is introduced (hence hiding the fact this code has
    // to be updated).
    match access {
        DeviceAccessStrategy::Keyring { .. } => {
            let ciphertext_key = platform::load_ciphertext_key_keyring(device_file).await?;
            Ok(DeviceCiphertextKeys {
                ciphertext_key,
                totp_opaque_key: None,
            })
        }

        DeviceAccessStrategy::Password { password, .. } => {
            if let DeviceFile::Password(device) = device_file {
                let ciphertext_key = device
                    .algorithm
                    .compute_secret_key(password)
                    .map_err(|_| LoadCiphertextKeyError::InvalidData)?;

                Ok(DeviceCiphertextKeys {
                    ciphertext_key,
                    totp_opaque_key: None,
                })
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }

        DeviceAccessStrategy::PKI { .. } => {
            let ciphertext_key = platform::load_ciphertext_key_pki(device_file).await?;
            Ok(DeviceCiphertextKeys {
                ciphertext_key,
                totp_opaque_key: None,
            })
        }

        DeviceAccessStrategy::AccountVault { operations, .. } => {
            if let DeviceFile::AccountVault(device) = device_file {
                let ciphertext_key = operations
                    .fetch_opaque_key(device.ciphertext_key_id)
                    .await
                    .map_err(|err| match err {
                        AccountVaultOperationsFetchOpaqueKeyError::BadVaultKeyAccess(_)
                        | AccountVaultOperationsFetchOpaqueKeyError::UnknownOpaqueKey
                        | AccountVaultOperationsFetchOpaqueKeyError::CorruptedOpaqueKey => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed {
                                server: RemoteOperationServer::ParsecAccount,
                                error: err.into(),
                            }
                        }
                        AccountVaultOperationsFetchOpaqueKeyError::Offline(_) => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline {
                                server: RemoteOperationServer::ParsecAccount,
                                error: err.into(),
                            }
                        }
                        AccountVaultOperationsFetchOpaqueKeyError::Internal(err) => {
                            LoadCiphertextKeyError::Internal(err)
                        }
                    })?;
                Ok(DeviceCiphertextKeys {
                    ciphertext_key,
                    totp_opaque_key: None,
                })
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }

        DeviceAccessStrategy::OpenBao { operations, .. } => {
            if let DeviceFile::OpenBao(device) = device_file {
                let ciphertext_key = operations
                    .fetch_opaque_key(device.openbao_ciphertext_key_path.clone())
                    .await
                    .map_err(|err| match err {
                        err @ (OpenBaoOperationsFetchOpaqueKeyError::BadURL(_)
                        | OpenBaoOperationsFetchOpaqueKeyError::BadServerResponse(_)) => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed {
                                server: RemoteOperationServer::OpenBao,
                                error: err.into(),
                            }
                        }
                        OpenBaoOperationsFetchOpaqueKeyError::NoServerResponse(_) => {
                            LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline {
                                server: RemoteOperationServer::OpenBao,
                                error: err.into(),
                            }
                        }
                    })?;
                Ok(DeviceCiphertextKeys {
                    ciphertext_key,
                    totp_opaque_key: None,
                })
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }
        DeviceAccessStrategy::TOTP {
            totp_opaque_key,
            next,
        } => {
            let mut keys = Box::pin(load_ciphertext_key(next, device_file)).await?;
            if keys.totp_opaque_key.is_some() {
                return Err(LoadCiphertextKeyError::Internal(anyhow::anyhow!(
                    "Cannot have multiple nested TOTP protections"
                )));
            }
            keys.totp_opaque_key = Some(totp_opaque_key.clone());
            Ok(keys)
        }
    }
}
