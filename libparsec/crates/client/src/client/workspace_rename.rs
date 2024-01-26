// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use super::Client;
use crate::{
    certif::{
        CertifBootstrapWorkspaceError, CertifPollServerError, CertifRenameRealmError,
        CertificateBasedActionOutcome, InvalidCertificateError, InvalidEncryptedRealmNameError,
        InvalidKeysBundleError,
    },
    RefreshWorkspacesListError,
};

#[derive(Debug, thiserror::Error)]
pub enum ClientRenameWorkspaceError {
    #[error("Workspace not found")]
    WorkspaceNotFound,
    #[error("Not allowed")]
    AuthorNotAllowed,
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error("There is no key available in this realm for encryption")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] InvalidKeysBundleError),
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    InvalidEncryptedRealmName(#[from] InvalidEncryptedRealmNameError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn rename_workspace(
    client_ops: &Client,
    realm_id: VlobID,
    new_name: EntryName,
) -> Result<(), ClientRenameWorkspaceError> {
    // 0) Quick check to filter out invalid realm ID.
    // This is useful given otherwise the next step will create a realm with
    // this invalid ID.

    client_ops
        .user_ops
        .get_user_manifest()
        .get_local_workspace_entry(realm_id)
        .ok_or(ClientRenameWorkspaceError::WorkspaceNotFound)?;

    // 1) Make sure the workspace is bootstrapped

    let outcome = client_ops
        .certificates_ops
        .bootstrap_workspace(realm_id, &new_name)
        .await
        .map_err(|e| match e {
            CertifBootstrapWorkspaceError::Offline => ClientRenameWorkspaceError::Offline,
            CertifBootstrapWorkspaceError::Stopped => ClientRenameWorkspaceError::Stopped,
            CertifBootstrapWorkspaceError::AuthorNotAllowed => {
                ClientRenameWorkspaceError::AuthorNotAllowed
            }
            CertifBootstrapWorkspaceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => ClientRenameWorkspaceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
            CertifBootstrapWorkspaceError::InvalidKeysBundle(err) => {
                ClientRenameWorkspaceError::InvalidKeysBundle(err)
            }
            CertifBootstrapWorkspaceError::InvalidCertificate(err) => {
                ClientRenameWorkspaceError::InvalidCertificate(err)
            }
            CertifBootstrapWorkspaceError::Internal(err) => {
                err.context("Cannot ensure workspace is boostrapped").into()
            }
        })?;

    let rename_done_in_bootstrap = match outcome {
        CertificateBasedActionOutcome::Uploaded {
            certificate_timestamp,
        } => {
            // It seems the workspace wasn't bootstrapped and we've just done it.
            // Hence there is no need for a rename given our name has been used as
            // the workspace's initial name.
            client_ops
                .certificates_ops
                .poll_server_for_new_certificates(Some(&PerTopicLastTimestamps::new_for_realm(
                    realm_id,
                    certificate_timestamp,
                )))
                .await
                .map_err(|e| match e {
                    CertifPollServerError::Stopped => ClientRenameWorkspaceError::Stopped,
                    CertifPollServerError::Offline => ClientRenameWorkspaceError::Offline,
                    CertifPollServerError::InvalidCertificate(err) => {
                        ClientRenameWorkspaceError::InvalidCertificate(err)
                    }
                    CertifPollServerError::Internal(err) => err
                        .context("Cannot poll server for new certificates")
                        .into(),
                })?;
            true
        }
        CertificateBasedActionOutcome::RemoteIdempotent => {
            client_ops
                .certificates_ops
                .poll_server_for_new_certificates(None)
                .await
                .map_err(|e| match e {
                    CertifPollServerError::Stopped => ClientRenameWorkspaceError::Stopped,
                    CertifPollServerError::Offline => ClientRenameWorkspaceError::Offline,
                    CertifPollServerError::InvalidCertificate(err) => {
                        ClientRenameWorkspaceError::InvalidCertificate(err)
                    }
                    CertifPollServerError::Internal(err) => err
                        .context("Cannot poll server for new certificates")
                        .into(),
                })?;
            false
        }
        CertificateBasedActionOutcome::LocalIdempotent => false,
    };

    if !rename_done_in_bootstrap {
        // 2) Then do the actual rename (i.e. send the realm name certificate to the server)

        let outcome = client_ops
            .certificates_ops
            .rename_realm(realm_id, new_name.clone())
            .await
            .map_err(|e| match e {
                CertifRenameRealmError::Stopped => ClientRenameWorkspaceError::Stopped,
                CertifRenameRealmError::Offline => ClientRenameWorkspaceError::Offline,
                CertifRenameRealmError::UnknownRealm => {
                    ClientRenameWorkspaceError::WorkspaceNotFound
                }
                CertifRenameRealmError::AuthorNotAllowed => {
                    ClientRenameWorkspaceError::AuthorNotAllowed
                }
                CertifRenameRealmError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                } => ClientRenameWorkspaceError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                },
                CertifRenameRealmError::NoKey => ClientRenameWorkspaceError::NoKey,
                CertifRenameRealmError::InvalidKeysBundle(err) => {
                    ClientRenameWorkspaceError::InvalidKeysBundle(err)
                }
                CertifRenameRealmError::InvalidCertificate(err) => {
                    ClientRenameWorkspaceError::InvalidCertificate(err)
                }
                CertifRenameRealmError::Internal(err) => err.context("Cannot rename realm").into(),
            })?;

        // At this point the rename is done, but our local workspace list is not aware of
        // it. So now is the time for a proactive refresh.
        // Note that if we were to do nothing (or if we crash right now), a monitor should
        // eventually detect the new certificates and refresh the workspace list accordingly.

        // 3) Poll the server to get back the new realm name certificate

        let latest_known_timestamps = match outcome {
            CertificateBasedActionOutcome::LocalIdempotent => return Ok(()),
            CertificateBasedActionOutcome::Uploaded {
                certificate_timestamp,
            } => Some(PerTopicLastTimestamps::new_for_realm(
                realm_id,
                certificate_timestamp,
            )),
            CertificateBasedActionOutcome::RemoteIdempotent => None,
        };
        client_ops
            .certificates_ops
            .poll_server_for_new_certificates(latest_known_timestamps.as_ref())
            .await
            .map_err(|e| match e {
                CertifPollServerError::Stopped => ClientRenameWorkspaceError::Stopped,
                CertifPollServerError::Offline => ClientRenameWorkspaceError::Offline,
                CertifPollServerError::InvalidCertificate(err) => {
                    ClientRenameWorkspaceError::InvalidCertificate(err)
                }
                CertifPollServerError::Internal(err) => err
                    .context("Cannot poll server for new certificates")
                    .into(),
            })?;
    }

    // 4) Refresh the workspace list to take into account the rename

    client_ops
        .refresh_workspaces_list()
        .await
        .map_err(|e| match e {
            RefreshWorkspacesListError::Offline => ClientRenameWorkspaceError::Offline,
            RefreshWorkspacesListError::Stopped => ClientRenameWorkspaceError::Stopped,
            RefreshWorkspacesListError::InvalidEncryptedRealmName(err) => {
                ClientRenameWorkspaceError::InvalidEncryptedRealmName(err)
            }
            RefreshWorkspacesListError::InvalidKeysBundle(err) => {
                ClientRenameWorkspaceError::InvalidKeysBundle(err)
            }
            RefreshWorkspacesListError::Internal(err) => {
                err.context("Cannot refresh workspace list").into()
            }
        })?;

    Ok(())
}
