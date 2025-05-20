// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use super::Client;
use crate::{
    CertifDeleteShamirRecoveryError, CertificateBasedActionOutcome,
    certif::{CertifPollServerError, InvalidCertificateError},
};

#[derive(Debug, thiserror::Error)]
pub enum ClientDeleteShamirRecoveryError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(
        "Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart"
    )]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn delete_shamir_recovery(
    client: &Client,
) -> Result<(), ClientDeleteShamirRecoveryError> {
    let outcome = client
        .certificates_ops
        .delete_shamir_recovery()
        .await
        .map_err(|err| match err {
            CertifDeleteShamirRecoveryError::Stopped => ClientDeleteShamirRecoveryError::Stopped,
            CertifDeleteShamirRecoveryError::Offline(e) => {
                ClientDeleteShamirRecoveryError::Offline(e)
            }
            CertifDeleteShamirRecoveryError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => ClientDeleteShamirRecoveryError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
            CertifDeleteShamirRecoveryError::Internal(error) => error.into(),
        })?;

    match outcome {
        CertificateBasedActionOutcome::Uploaded {
            certificate_timestamp,
        }
        | CertificateBasedActionOutcome::RemoteIdempotent {
            certificate_timestamp,
        } => {
            // Bootstrap just occurred, must fetch the new certificates.
            let latest_known_timestamps =
                PerTopicLastTimestamps::new_for_shamir(certificate_timestamp);
            client
                .certificates_ops
                .poll_server_for_new_certificates(Some(&latest_known_timestamps))
                .await
                .map_err(|e| match e {
                    CertifPollServerError::Stopped => ClientDeleteShamirRecoveryError::Stopped,
                    CertifPollServerError::Offline(e) => {
                        ClientDeleteShamirRecoveryError::Offline(e)
                    }
                    CertifPollServerError::InvalidCertificate(err) => {
                        ClientDeleteShamirRecoveryError::InvalidCertificate(err)
                    }
                    CertifPollServerError::Internal(err) => err
                        .context("Cannot poll server for new certificates")
                        .into(),
                })?;
        }
        CertificateBasedActionOutcome::LocalIdempotent => (),
    }

    Ok(())
}
