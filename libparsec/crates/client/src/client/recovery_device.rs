// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError, ProxyConfig};
use libparsec_platform_device_loader::{
    get_default_key_file, save_device, AvailableDevice, DeviceSaveStrategy,
    LoadRecoveryDeviceError, RemoteOperationServer, SaveDeviceError,
};
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_protocol::authenticated_cmds::latest::device_create;
use libparsec_types::prelude::*;

use crate::{
    greater_timestamp, CertifPollServerError, GreaterTimestampOffset, InvalidCertificateError,
};

use super::Client;

pub async fn export_recovery_device(
    client: &Client,
    device_label: DeviceLabel,
) -> Result<(SecretKeyPassphrase, Vec<u8>), ClientExportRecoveryDeviceError> {
    // 1. Generate recovery device (device ID, signing key etc.)

    let recovery_device = LocalDevice::from_existing_device_for_user(&client.device, device_label);

    // 2. Upload the recovery device on the server

    let latest_known_timestamp = register_new_device(
        &client.cmds,
        &recovery_device,
        DevicePurpose::PassphraseRecovery,
        &client.device,
    )
    .await?;

    // From now on, our newly generated recovery device has an actual existence !

    // 3. Fetch the new device certificate from the server

    let latest_known_timestamp = PerTopicLastTimestamps::new_for_common(latest_known_timestamp);

    client
        .certificates_ops
        .poll_server_for_new_certificates(Some(&latest_known_timestamp))
        .await
        .map_err(|e| match e {
            CertifPollServerError::Stopped => ClientExportRecoveryDeviceError::Stopped,
            CertifPollServerError::Offline(e) => ClientExportRecoveryDeviceError::Offline(e),
            CertifPollServerError::InvalidCertificate(err) => {
                ClientExportRecoveryDeviceError::InvalidCertificate(err)
            }
            CertifPollServerError::Internal(err) => err
                .context("Cannot poll server for new certificates")
                .into(),
        })?;

    // 4. Serialize and return the recovery device

    let (passphrase, data) =
        libparsec_platform_device_loader::dump_recovery_device(&recovery_device);

    Ok((passphrase, data))
}

#[derive(Debug, thiserror::Error)]
pub enum ClientExportRecoveryDeviceError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),

    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
}

impl From<RegisterNewDeviceError> for ClientExportRecoveryDeviceError {
    fn from(value: RegisterNewDeviceError) -> Self {
        match value {
            RegisterNewDeviceError::Offline(e) => ClientExportRecoveryDeviceError::Offline(e),
            RegisterNewDeviceError::Internal(error) => {
                ClientExportRecoveryDeviceError::Internal(error)
            }
            RegisterNewDeviceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => ClientExportRecoveryDeviceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ImportRecoveryDeviceError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Storage is not available")]
    StorageNotAvailable,

    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error("Cannot deserialize file content")]
    InvalidData,
    #[error("Passphrase format is invalid")]
    InvalidPassphrase,
    #[error("Failed to decrypt file content")]
    DecryptionFailed,
    #[error(transparent)]
    InvalidPath(anyhow::Error),
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
}

impl From<LoadRecoveryDeviceError> for ImportRecoveryDeviceError {
    fn from(value: LoadRecoveryDeviceError) -> Self {
        match value {
            LoadRecoveryDeviceError::InvalidData => ImportRecoveryDeviceError::InvalidData,
            LoadRecoveryDeviceError::InvalidPassphrase => {
                ImportRecoveryDeviceError::InvalidPassphrase
            }
            LoadRecoveryDeviceError::DecryptionFailed => {
                ImportRecoveryDeviceError::DecryptionFailed
            }
        }
    }
}

impl From<RegisterNewDeviceError> for ImportRecoveryDeviceError {
    fn from(value: RegisterNewDeviceError) -> Self {
        match value {
            RegisterNewDeviceError::Offline(e) => ImportRecoveryDeviceError::Offline(e),
            RegisterNewDeviceError::Internal(error) => ImportRecoveryDeviceError::Internal(error),
            RegisterNewDeviceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => ImportRecoveryDeviceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
        }
    }
}

impl From<SaveDeviceError> for ImportRecoveryDeviceError {
    fn from(value: SaveDeviceError) -> Self {
        match value {
            SaveDeviceError::StorageNotAvailable => ImportRecoveryDeviceError::StorageNotAvailable,
            SaveDeviceError::InvalidPath(error) => ImportRecoveryDeviceError::InvalidPath(error),
            SaveDeviceError::Internal(error) => ImportRecoveryDeviceError::Internal(error),
            SaveDeviceError::RemoteOpaqueKeyUploadOffline { server, error } => {
                ImportRecoveryDeviceError::RemoteOpaqueKeyUploadOffline { server, error }
            }
            SaveDeviceError::RemoteOpaqueKeyUploadFailed { server, error } => {
                ImportRecoveryDeviceError::RemoteOpaqueKeyUploadFailed { server, error }
            }
        }
    }
}

pub async fn register_new_device(
    cmds: &AuthenticatedCmds,
    new_device: &LocalDevice,
    new_device_purpose: DevicePurpose,
    author: &LocalDevice,
) -> Result<DateTime, RegisterNewDeviceError> {
    // Loop is needed to deal with server requiring greater timestamp
    let mut timestamp = author.now();

    loop {
        let outcome =
            internal_register_new_device(cmds, new_device, new_device_purpose, author, timestamp)
                .await?;

        match outcome {
            DeviceInternalsOutcome::Done(timestamp) => return Ok(timestamp),
            DeviceInternalsOutcome::RequireGreaterTimestamp(strictly_greater_than) => {
                // TODO: handle `strictly_greater_than` out of the client ballpark by
                // returning an error
                timestamp = greater_timestamp(
                    &author.time_provider,
                    GreaterTimestampOffset::User,
                    strictly_greater_than,
                );
            }
        }
    }
}

pub(crate) struct DeviceCertificatesBytes {
    pub full: Bytes,
    pub redacted: Bytes,
}

/// generates certificates for new device signed by author at now
pub(crate) fn generate_new_device_certificates(
    new_device: &LocalDevice,
    new_device_purpose: DevicePurpose,
    author: &LocalDevice,
    now: DateTime,
) -> DeviceCertificatesBytes {
    let device_cert = DeviceCertificate {
        author: CertificateSigner::User(author.device_id),
        timestamp: now,
        purpose: new_device_purpose,
        user_id: new_device.user_id,
        device_id: new_device.device_id,
        device_label: MaybeRedacted::Real(new_device.device_label.clone()),
        verify_key: new_device.verify_key(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    };

    let device_certificate = device_cert.dump_and_sign(&author.signing_key).into();

    let redacted_device_cert = device_cert.into_redacted();

    let redacted_device_certificate = redacted_device_cert
        .dump_and_sign(&author.signing_key)
        .into();

    DeviceCertificatesBytes {
        full: device_certificate,
        redacted: redacted_device_certificate,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegisterNewDeviceError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
}

#[derive(Debug)]
enum DeviceInternalsOutcome {
    Done(DateTime),
    RequireGreaterTimestamp(DateTime),
}

async fn internal_register_new_device(
    cmds: &AuthenticatedCmds,
    new_device: &LocalDevice,
    new_device_purpose: DevicePurpose,
    author: &LocalDevice,
    now: DateTime,
) -> Result<DeviceInternalsOutcome, RegisterNewDeviceError> {
    let DeviceCertificatesBytes {
        full: device_certificate,
        redacted: redacted_device_certificate,
    } = generate_new_device_certificates(new_device, new_device_purpose, author, now);
    match cmds
        .send(device_create::Req {
            device_certificate,
            redacted_device_certificate,
        })
        .await?
    {
        device_create::Rep::Ok => Ok(DeviceInternalsOutcome::Done(now)),
        device_create::Rep::RequireGreaterTimestamp {
            strictly_greater_than,
        } =>
        // The retry is handled by the caller
        {
            Ok(DeviceInternalsOutcome::RequireGreaterTimestamp(
                strictly_greater_than,
            ))
        }
        device_create::Rep::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => Err(RegisterNewDeviceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        }),
        bad_rep @ (device_create::Rep::UnknownStatus { .. }
        | device_create::Rep::InvalidCertificate
        | device_create::Rep::DeviceAlreadyExists) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

pub async fn import_recovery_device(
    config_dir: &Path,
    recovery_device: &[u8],
    passphrase: String,
    device_label: DeviceLabel,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, ImportRecoveryDeviceError> {
    // 1. Load the recovery device

    let recovery_device =
        libparsec_platform_device_loader::load_recovery_device(recovery_device, passphrase.into())?;

    // 2. Generate the new device (device ID, signing key etc.)

    let new_device = LocalDevice::from_existing_device_for_user(&recovery_device, device_label);

    // We first register the new device to the server, and only then save it on disk.
    // This is to ensure the device found on disk are always valid, however the drawback
    // is it can lead to losing the device if something goes wrong between the two steps.
    //
    // This is considered acceptable given 1) the error window is small and
    // 2) if this occurs, the user can always re-import the recovery device.

    // 3. Upload the new device on the server

    let cmds = AuthenticatedCmds::new(config_dir, recovery_device.clone(), ProxyConfig::default())?;
    register_new_device(
        &cmds,
        &new_device,
        DevicePurpose::Standard,
        &recovery_device,
    )
    .await?;

    // From now on, our new device has an actual existence !

    // 4. Save the new device on disk

    let key_file = get_default_key_file(config_dir, new_device.device_id);

    let new_available_device =
        save_device(config_dir, &save_strategy, &new_device, key_file).await?;

    Ok(new_available_device)
}
