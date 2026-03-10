// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use super::Client;
use crate::{
    certif::{
        CertifPollServerError, CertifSelfPromoteToOwnerError, CertificateBasedActionOutcome,
        InvalidCertificateError,
    },
    ClientRefreshWorkspacesListError,
};

#[derive(Debug, thiserror::Error)]
pub enum ClientSelfPromoteToWorkspaceOwnerError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Workspace realm not found")]
    WorkspaceNotFound,
    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error("An active user is already OWNER of this workspace")]
    ActiveOwnerAlreadyExists,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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
    Internal(#[from] anyhow::Error),
}

pub async fn self_promote_to_workspace_owner(
    client: &Client,
    realm_id: VlobID,
) -> Result<(), ClientSelfPromoteToWorkspaceOwnerError> {
    // Quick check to filter out invalid realm ID
    let user_manifest = client.user_ops.get_user_manifest();
    user_manifest
        .get_local_workspace_entry(realm_id)
        .ok_or(ClientSelfPromoteToWorkspaceOwnerError::WorkspaceNotFound)?;

    // Do the actual self-promotion (i.e. upload realm role certificate)
    let outcome = client
        .certificates_ops
        .self_promote_to_owner(realm_id)
        .await
        .map_err(|e| match e {
            CertifSelfPromoteToOwnerError::Stopped => {
                ClientSelfPromoteToWorkspaceOwnerError::Stopped
            }
            CertifSelfPromoteToOwnerError::AuthorNotAllowed => {
                ClientSelfPromoteToWorkspaceOwnerError::AuthorNotAllowed
            }
            CertifSelfPromoteToOwnerError::RealmNotFound => {
                ClientSelfPromoteToWorkspaceOwnerError::WorkspaceNotFound
            }
            CertifSelfPromoteToOwnerError::ActiveOwnerAlreadyExists => {
                ClientSelfPromoteToWorkspaceOwnerError::ActiveOwnerAlreadyExists
            }
            CertifSelfPromoteToOwnerError::Offline(e) => {
                ClientSelfPromoteToWorkspaceOwnerError::Offline(e)
            }
            CertifSelfPromoteToOwnerError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => ClientSelfPromoteToWorkspaceOwnerError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
            CertifSelfPromoteToOwnerError::InvalidCertificate(err) => {
                ClientSelfPromoteToWorkspaceOwnerError::InvalidCertificate(err)
            }
            CertifSelfPromoteToOwnerError::Internal(err) => err.into(),
        })?;

    // Poll the server to fetch back the new realm role certificate
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
            CertifPollServerError::Stopped => ClientSelfPromoteToWorkspaceOwnerError::Stopped,
            CertifPollServerError::Offline(e) => ClientSelfPromoteToWorkspaceOwnerError::Offline(e),
            CertifPollServerError::InvalidCertificate(err) => {
                ClientSelfPromoteToWorkspaceOwnerError::InvalidCertificate(err)
            }
            CertifPollServerError::Internal(err) => err
                .context("Cannot poll server for new certificates")
                .into(),
        })?;

    // Refresh the workspace list so that `list_workspaces` reflects the new OWNER role
    client
        .refresh_workspaces_list()
        .await
        .map_err(|e| match e {
            ClientRefreshWorkspacesListError::Offline(e) => {
                ClientSelfPromoteToWorkspaceOwnerError::Offline(e)
            }
            ClientRefreshWorkspacesListError::Stopped => {
                ClientSelfPromoteToWorkspaceOwnerError::Stopped
            }
            ClientRefreshWorkspacesListError::InvalidEncryptedRealmName(err) => {
                anyhow::anyhow!("Cannot refresh workspace list: {err}").into()
            }
            ClientRefreshWorkspacesListError::InvalidKeysBundle(err) => {
                anyhow::anyhow!("Cannot refresh workspace list: {err}").into()
            }
            ClientRefreshWorkspacesListError::Internal(err) => {
                err.context("Cannot refresh workspace list").into()
            }
        })?;

    Ok(())
}
