// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_platform_storage::certificates::GetCertificateError;
use libparsec_protocol::authenticated_cmds::v4::device_create;
use libparsec_types::{
    anyhow, thiserror, Bytes, CertificateSignerOwned, DateTime, DeviceCertificate, LocalDevice,
    MaybeRedacted, SigningKeyAlgorithm, UserID,
};

use super::{
    encrypt::CertifEncryptForUserError, greater_timestamp, CertificateBasedActionOutcome,
    GreaterTimestampOffset, InvalidCertificateError,
};

pub async fn create_new_device(
    cmds: Arc<AuthenticatedCmds>,
    new_device: LocalDevice,
    author: Arc<LocalDevice>,
) -> Result<CertificateBasedActionOutcome, CertifDeviceError> {
    // Loop is needed to deal with server requiring greater timestamp
    let mut timestamp = author.now();

    loop {
        let outcome =
            internal_create_new_device(cmds.clone(), new_device.clone(), author.clone(), timestamp)
                .await?;

        match outcome {
            DeviceInternalsOutcome::Done(outcome) => return Ok(outcome),
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

/// returns (device_certificate, redacted_device_certificate)
pub(crate) fn generate_new_device_certificates(
    new_device: &LocalDevice,
    author: Arc<LocalDevice>,
    now: DateTime,
) -> (Bytes, Bytes) {
    let device_cert = DeviceCertificate {
        author: CertificateSignerOwned::User(author.device_id),
        timestamp: now,
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

    (device_certificate, redacted_device_certificate)
}

#[derive(Debug, thiserror::Error)]
pub enum CertifDeviceError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("User {0} is revoked")]
    UserRevoked(UserID),
    #[error("Threshold is 0")]
    ThresholdIsZero,
    #[error("Share recipient declared in brief has no share certificate")]
    ShareRecipientHasZeroShares,
    #[error("User {0} not found.")]
    MissingUser(UserID),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    DataError(#[from] libparsec_types::DataError),
    #[error(transparent)]
    EncryptionError(#[from] CertifEncryptForUserError),
    #[error(transparent)]
    GetCertificateError(#[from] GetCertificateError),
    #[error(transparent)]
    ConnectionError(#[from] ConnectionError),
}

#[derive(Debug)]
enum DeviceInternalsOutcome {
    Done(CertificateBasedActionOutcome),
    RequireGreaterTimestamp(DateTime),
}

async fn internal_create_new_device(
    cmds: Arc<AuthenticatedCmds>,
    new_device: LocalDevice,
    author: Arc<LocalDevice>,
    now: DateTime,
) -> Result<DeviceInternalsOutcome, CertifDeviceError> {
    let (device_certificate, redacted_device_certificate) =
        generate_new_device_certificates(&new_device, author, now);
    match cmds
        .send(device_create::Req {
            device_certificate,
            redacted_device_certificate,
        })
        .await?
    {
        device_create::Rep::Ok => Ok(DeviceInternalsOutcome::Done(
            CertificateBasedActionOutcome::Uploaded {
                certificate_timestamp: now,
            },
        )),
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
        } => Err(CertifDeviceError::TimestampOutOfBallpark {
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
