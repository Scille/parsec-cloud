// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use super::Client;
use crate::certif::{
    CertifBootstrapWorkspaceError, CertifPollServerError, CertifShareRealmError,
    CertificateBasedActionOutcome, InvalidCertificateError, InvalidKeysBundleError,
};

#[derive(Debug, thiserror::Error)]
pub enum ClientShareWorkspaceError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot share with oneself")]
    RecipientIsSelf,
    #[error("Recipient user not found")]
    RecipientNotFound,
    #[error("Workspace realm not found")]
    WorkspaceNotFound,
    #[error("Cannot share with a revoked user")]
    RecipientRevoked,
    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error("Role incompatible with the user, as it has profile OUTSIDER")]
    RoleIncompatibleWithOutsider,
    #[error("Cannot reach the server")]
    Offline,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for ClientShareWorkspaceError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn share_workspace(
    client: &Client,
    realm_id: VlobID,
    recipient: UserID,
    role: Option<RealmRole>,
) -> Result<(), ClientShareWorkspaceError> {
    // 0) Quick check to filter out invalid realm ID.
    // This is useful given otherwise the next step will create a realm with
    // this invalid ID.

    let user_manifest = client.user_ops.get_user_manifest();
    let entry = user_manifest
        .get_local_workspace_entry(realm_id)
        .ok_or(ClientShareWorkspaceError::WorkspaceNotFound)?;

    // 1) Make sure the workspace is bootstrapped

    let outcome = client
        .certificates_ops
        .bootstrap_workspace(realm_id, &entry.name)
        .await
        .map_err(|e| match e {
            CertifBootstrapWorkspaceError::Offline => ClientShareWorkspaceError::Offline,
            CertifBootstrapWorkspaceError::Stopped => ClientShareWorkspaceError::Stopped,
            CertifBootstrapWorkspaceError::AuthorNotAllowed => {
                ClientShareWorkspaceError::AuthorNotAllowed
            }
            CertifBootstrapWorkspaceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => ClientShareWorkspaceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
            CertifBootstrapWorkspaceError::InvalidKeysBundle(err) => {
                ClientShareWorkspaceError::InvalidKeysBundle(err)
            }
            CertifBootstrapWorkspaceError::InvalidCertificate(err) => {
                ClientShareWorkspaceError::InvalidCertificate(err)
            }
            CertifBootstrapWorkspaceError::Internal(err) => err
                .context("Cannot ensure workspace is bootstrapped")
                .into(),
        })?;

    match outcome {
        CertificateBasedActionOutcome::Uploaded {
            certificate_timestamp,
        }
        | CertificateBasedActionOutcome::RemoteIdempotent {
            certificate_timestamp,
        } => {
            // Bootstrap just occured, must fetch the new certificates.
            let latest_known_timestamps =
                PerTopicLastTimestamps::new_for_realm(realm_id, certificate_timestamp);
            client
                .certificates_ops
                .poll_server_for_new_certificates(Some(&latest_known_timestamps))
                .await
                .map_err(|e| match e {
                    CertifPollServerError::Stopped => ClientShareWorkspaceError::Stopped,
                    CertifPollServerError::Offline => ClientShareWorkspaceError::Offline,
                    CertifPollServerError::InvalidCertificate(err) => {
                        ClientShareWorkspaceError::InvalidCertificate(err)
                    }
                    CertifPollServerError::Internal(err) => err
                        .context("Cannot poll server for new certificates")
                        .into(),
                })?;
        }
        CertificateBasedActionOutcome::LocalIdempotent => (),
    }

    // 2) Do the actual share (i.e. upload realm role certificate)

    let outcome = client.certificates_ops
        .share_realm(realm_id, recipient, role)
        .await
        .map_err(|e| match e {
            CertifShareRealmError::Stopped => ClientShareWorkspaceError::Stopped,
            CertifShareRealmError::RecipientIsSelf => ClientShareWorkspaceError::RecipientIsSelf,
            CertifShareRealmError::RecipientNotFound => ClientShareWorkspaceError::RecipientNotFound,
            CertifShareRealmError::RealmNotFound => ClientShareWorkspaceError::WorkspaceNotFound,
            CertifShareRealmError::RecipientRevoked => ClientShareWorkspaceError::RecipientRevoked,
            CertifShareRealmError::AuthorNotAllowed => ClientShareWorkspaceError::AuthorNotAllowed,
            CertifShareRealmError::RoleIncompatibleWithOutsider => ClientShareWorkspaceError::RoleIncompatibleWithOutsider,
            CertifShareRealmError::Offline => ClientShareWorkspaceError::Offline,
            CertifShareRealmError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => ClientShareWorkspaceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
            CertifShareRealmError::InvalidKeysBundle(err) => ClientShareWorkspaceError::InvalidKeysBundle(err),
            CertifShareRealmError::InvalidCertificate(err) => ClientShareWorkspaceError::InvalidCertificate(err),
            bad_rep @ CertifShareRealmError::NoKey => anyhow::anyhow!("Unexpected server response: {} while we've just made sure the realm was bootstrapped", bad_rep).into(),
            CertifShareRealmError::Internal(err) => err.into(),
        })?;

    // 3) Poll the server to fetch back the new realm role certificate

    let latest_known_timestamps = match outcome {
        CertificateBasedActionOutcome::LocalIdempotent => return Ok(()),
        CertificateBasedActionOutcome::Uploaded {
            certificate_timestamp,
        }
        | CertificateBasedActionOutcome::RemoteIdempotent {
            certificate_timestamp,
        } => PerTopicLastTimestamps::new_for_realm(realm_id, certificate_timestamp),
    };
    client
        .certificates_ops
        .poll_server_for_new_certificates(Some(&latest_known_timestamps))
        .await
        .map_err(|e| match e {
            CertifPollServerError::Stopped => ClientShareWorkspaceError::Stopped,
            CertifPollServerError::Offline => ClientShareWorkspaceError::Offline,
            CertifPollServerError::InvalidCertificate(err) => {
                ClientShareWorkspaceError::InvalidCertificate(err)
            }
            CertifPollServerError::Internal(err) => err
                .context("Cannot poll server for new certificates")
                .into(),
        })?;

    Ok(())
}
