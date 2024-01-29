// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::certif::{
    CertifBootstrapWorkspaceError, InvalidCertificateError, InvalidKeysBundleError,
};

use super::Client;

#[derive(Debug, thiserror::Error)]
pub enum ClientEnsureWorkspacesBootstrappedError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    // Note the is no `InvalidManifest` here, this is because we self-repair in case of
    // invalid user manifest (given otherwise the client would be stuck for good !)
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

pub(crate) async fn ensure_workspaces_bootstrapped(
    client_ops: &Client,
) -> Result<(), ClientEnsureWorkspacesBootstrappedError> {
    let user_manifest = client_ops.user_ops.get_user_manifest();

    for entry in &user_manifest.local_workspaces {
        // Having a name certificate is a proof that the workspace has been bootstrapped
        // (as it is the last step during bootstrap).
        if let CertificateBasedInfoOrigin::Certificate { .. } = entry.name_origin {
            continue;
        }

        // The name is not synchronized, this *may* mean the workspace hasn't been bootstrapped.
        // In any case, we can rely on the fact that the bootstrap is idempotent.

        client_ops
            .certificates_ops
            .bootstrap_workspace(entry.id, &entry.name)
            .await
            .map(|_| ())
            .or_else(|e| match e {
                // Our user is not OWNER of the workspace, so we cannot bootstrap it.
                //
                // This is unlikely (but not impossible !) given the workspace is only
                // shared once entirely bootstrapped.
                //
                // The two reasons for that are:
                // - Our local certificates database has been cleared (e.g. switch from/to OUTSIDER)
                // - The most likely reason: this is the workspace has been created with Parsec < v3
                //   (where workspace bootstrap didn't exist and hence workspace got shared with no
                //   initial key rotation nor realm name certificate).
                //
                // In any case, we can just ignore the error.
                CertifBootstrapWorkspaceError::AuthorNotAllowed => Ok(()),
                // Propagate the other errors
                CertifBootstrapWorkspaceError::Offline => {
                    Err(ClientEnsureWorkspacesBootstrappedError::Offline)
                }
                CertifBootstrapWorkspaceError::Stopped => {
                    Err(ClientEnsureWorkspacesBootstrappedError::Stopped)
                }
                CertifBootstrapWorkspaceError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                } => Err(
                    ClientEnsureWorkspacesBootstrappedError::TimestampOutOfBallpark {
                        server_timestamp,
                        client_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                    },
                ),
                CertifBootstrapWorkspaceError::InvalidKeysBundle(err) => Err(
                    ClientEnsureWorkspacesBootstrappedError::InvalidKeysBundle(err),
                ),
                CertifBootstrapWorkspaceError::InvalidCertificate(err) => Err(
                    ClientEnsureWorkspacesBootstrappedError::InvalidCertificate(err),
                ),
                CertifBootstrapWorkspaceError::Internal(err) => {
                    Err(err.context("Cannot bootstrap workspace").into())
                }
            })?;
    }

    Ok(())
}
