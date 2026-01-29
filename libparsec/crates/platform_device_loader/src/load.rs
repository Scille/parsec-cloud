// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    platform, AccountVaultOperationsFetchOpaqueKeyError, DeviceAccessStrategy,
    LoadCiphertextKeyError, OpenBaoOperationsFetchOpaqueKeyError, RemoteOperationServer,
};
use libparsec_types::prelude::*;

pub(super) async fn load_ciphertext_key(
    access: &DeviceAccessStrategy,
    device_file: &DeviceFile,
) -> Result<SecretKey, LoadCiphertextKeyError> {
    // Don't do `match (access, device_file)` since we would end up with a catch-all
    // `(_, _) => return <error>` condition that would prevent this code from breaking
    // whenever a new variant is introduced (hence hiding the fact this code has
    // to be updated).
    match access {
        DeviceAccessStrategy::Keyring { .. } => {
            platform::load_ciphertext_key_keyring(device_file).await
        }

        DeviceAccessStrategy::Password { password, .. } => {
            if let DeviceFile::Password(device) = device_file {
                let key = device
                    .algorithm
                    .compute_secret_key(password)
                    .map_err(|_| LoadCiphertextKeyError::InvalidData)?;

                Ok(key)
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }

        DeviceAccessStrategy::PKI { .. } => platform::load_ciphertext_key_pki(device_file).await,

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
                Ok(ciphertext_key)
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
                Ok(ciphertext_key)
            } else {
                Err(LoadCiphertextKeyError::InvalidData)
            }
        }
    }
}
