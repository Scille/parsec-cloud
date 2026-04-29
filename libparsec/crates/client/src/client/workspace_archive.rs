// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use super::Client;
use crate::{
    certif::{
        CertifArchiveRealmError, CertifBootstrapWorkspaceError, CertifPollServerError,
        CertificateBasedActionOutcome, InvalidCertificateError, InvalidEncryptedRealmNameError,
        InvalidKeysBundleError, RequestedRealmArchivingConfiguration,
    },
    ClientRefreshWorkspacesListError,
};

#[derive(Debug, thiserror::Error)]
pub enum ClientArchiveWorkspaceError {
    #[error("Workspace not found")]
    WorkspaceNotFound,
    #[error("Not allowed")]
    AuthorNotAllowed,
    #[error("Workspace has been deleted")]
    WorkspaceDeleted,
    #[error("Archiving period is too short")]
    ArchivingPeriodTooShort,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
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
    InvalidEncryptedRealmName(#[from] Box<InvalidEncryptedRealmNameError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn archive_workspace(
    client: &Client,
    realm_id: VlobID,
    configuration: RequestedRealmArchivingConfiguration,
) -> Result<(), ClientArchiveWorkspaceError> {
    // 0) Quick check to filter out invalid realm ID.
    // This is useful given otherwise the next step will create a realm with
    // this invalid ID.

    let user_manifest = client.user_ops.get_user_manifest();
    let entry = user_manifest
        .get_local_workspace_entry(realm_id)
        .ok_or(ClientArchiveWorkspaceError::WorkspaceNotFound)?;

    // 1) Make sure the workspace is bootstrapped

    let outcome = match client
        .certificates_ops
        .bootstrap_workspace(realm_id, &entry.name)
        .await
    {
        Ok(outcome) => outcome,
        Err(err) => match err {
            // If our author is not allowed, it means the workspace has been shared
            // with us... and hence the bootstrap has been done already !
            CertifBootstrapWorkspaceError::AuthorNotAllowed => {
                CertificateBasedActionOutcome::LocalIdempotent
            }
            CertifBootstrapWorkspaceError::Offline(e) => {
                return Err(ClientArchiveWorkspaceError::Offline(e))
            }
            CertifBootstrapWorkspaceError::Stopped => {
                return Err(ClientArchiveWorkspaceError::Stopped)
            }
            CertifBootstrapWorkspaceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => {
                return Err(ClientArchiveWorkspaceError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                })
            }
            CertifBootstrapWorkspaceError::RealmDeleted => {
                return Err(ClientArchiveWorkspaceError::WorkspaceDeleted)
            }
            CertifBootstrapWorkspaceError::InvalidKeysBundle(err) => {
                return Err(ClientArchiveWorkspaceError::InvalidKeysBundle(err))
            }
            CertifBootstrapWorkspaceError::InvalidCertificate(err) => {
                return Err(ClientArchiveWorkspaceError::InvalidCertificate(err))
            }
            CertifBootstrapWorkspaceError::Internal(err) => {
                return Err(err
                    .context("Cannot ensure workspace is bootstrapped")
                    .into())
            }
        },
    };

    match outcome {
        CertificateBasedActionOutcome::Uploaded {
            certificate_timestamp,
        }
        | CertificateBasedActionOutcome::RemoteIdempotent {
            certificate_timestamp,
        } => {
            // Bootstrap just occurred, must fetch the new certificates.
            let latest_known_timestamps =
                PerTopicLastTimestamps::new_for_realm(realm_id, certificate_timestamp);
            client
                .certificates_ops
                .poll_server_for_new_certificates(Some(&latest_known_timestamps))
                .await
                .map_err(|e| match e {
                    CertifPollServerError::Stopped => ClientArchiveWorkspaceError::Stopped,
                    CertifPollServerError::Offline(e) => ClientArchiveWorkspaceError::Offline(e),
                    CertifPollServerError::InvalidCertificate(err) => {
                        ClientArchiveWorkspaceError::InvalidCertificate(err)
                    }
                    CertifPollServerError::Internal(err) => err
                        .context("Cannot poll server for new certificates")
                        .into(),
                })?;
        }
        CertificateBasedActionOutcome::LocalIdempotent => (),
    }

    // 2) Do the actual archive operation (i.e. upload realm archiving certificate)

    let outcome = client
        .certificates_ops
        .archive_realm(realm_id, configuration)
        .await
        .map_err(|e| match e {
            CertifArchiveRealmError::Stopped => ClientArchiveWorkspaceError::Stopped,
            CertifArchiveRealmError::Offline(e) => ClientArchiveWorkspaceError::Offline(e),
            CertifArchiveRealmError::UnknownRealm => ClientArchiveWorkspaceError::WorkspaceNotFound,
            CertifArchiveRealmError::RealmDeleted => ClientArchiveWorkspaceError::WorkspaceDeleted,
            CertifArchiveRealmError::AuthorNotAllowed => {
                ClientArchiveWorkspaceError::AuthorNotAllowed
            }
            CertifArchiveRealmError::ArchivingPeriodTooShort => {
                ClientArchiveWorkspaceError::ArchivingPeriodTooShort
            }
            CertifArchiveRealmError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => ClientArchiveWorkspaceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
            CertifArchiveRealmError::Internal(err) => err.context("Cannot archive realm").into(),
        })?;

    // 3) Poll the server to fetch back the new realm archiving certificate

    let latest_known_timestamps = match outcome {
        CertificateBasedActionOutcome::LocalIdempotent => {
            return Ok(());
        }
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
            CertifPollServerError::Stopped => ClientArchiveWorkspaceError::Stopped,
            CertifPollServerError::Offline(e) => ClientArchiveWorkspaceError::Offline(e),
            CertifPollServerError::InvalidCertificate(err) => {
                ClientArchiveWorkspaceError::InvalidCertificate(err)
            }
            CertifPollServerError::Internal(err) => err
                .context("Cannot poll server for new certificates")
                .into(),
        })?;

    // 4) Refresh the workspace list to take into account archiving status update

    client
        .refresh_workspaces_list()
        .await
        .map_err(|e| match e {
            ClientRefreshWorkspacesListError::Offline(e) => ClientArchiveWorkspaceError::Offline(e),
            ClientRefreshWorkspacesListError::Stopped => ClientArchiveWorkspaceError::Stopped,
            ClientRefreshWorkspacesListError::InvalidEncryptedRealmName(err) => {
                ClientArchiveWorkspaceError::InvalidEncryptedRealmName(err)
            }
            ClientRefreshWorkspacesListError::InvalidKeysBundle(err) => {
                ClientArchiveWorkspaceError::InvalidKeysBundle(err)
            }
            ClientRefreshWorkspacesListError::Internal(err) => {
                err.context("Cannot refresh workspace list").into()
            }
        })?;

    Ok(())
}
