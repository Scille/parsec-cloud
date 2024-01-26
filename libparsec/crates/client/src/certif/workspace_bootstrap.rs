// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::{
    store::CertifStoreError, CertifEnsureRealmCreatedError, CertifOps, CertifPollServerError,
    CertifRenameRealmError, CertifRotateRealmKeyError, CertificateBasedActionOutcome,
    InvalidCertificateError, InvalidKeysBundleError,
};

#[derive(Debug, thiserror::Error)]
pub enum CertifBootstrapWorkspaceError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Author not allowed")]
    AuthorNotAllowed,
    // Note `InvalidManifest` here, this is because we self-repair in case of invalid
    // user manifest (given otherwise the client would be stuck for good !)
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    InvalidKeysBundle(#[from] InvalidKeysBundleError),
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for CertifBootstrapWorkspaceError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

impl From<CertifStoreError> for CertifBootstrapWorkspaceError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub(super) async fn bootstrap_workspace(
    ops: &CertifOps,
    realm_id: VlobID,
    name: &EntryName,
) -> Result<CertificateBasedActionOutcome, CertifBootstrapWorkspaceError> {
    // Workspace bootstrap is composed of three server-side steps:
    // 1. Realm creation & initial realm role certificate upload.
    // 2. Initial realm key rotation upload.
    // 3. Initial realm name certificate upload.
    //
    // Given those operations are not done in an atomic way, they are all
    // idempotent and we retry them until they are all done.

    // Cheap check: only OWNER can do the bootstrap
    let current_role = ops.get_current_self_realm_role(realm_id).await?;
    match current_role {
        // `None` can mean:
        // - We are the original OWNER, and the realm hasn't been created on the server yet
        //   (hence there is not realm role certificate so far).
        // - We never had access to the realm (unlikely: why do we know about it then ?).
        // - The local certificates database got cleared (or we are upgrading from Parsec <v3
        //   where certificates were only kept in RAM).
        // So we have to give it the benefit of the doubt here and rely on server access
        // control instead.
        Some(Some(RealmRole::Owner)) | None => (),
        _ => return Err(CertifBootstrapWorkspaceError::AuthorNotAllowed),
    }

    // Step 1

    super::realm_create::ensure_realm_created(ops, realm_id)
        .await
        .map_err(|e| match e {
            CertifEnsureRealmCreatedError::Offline => CertifBootstrapWorkspaceError::Offline,
            CertifEnsureRealmCreatedError::Stopped => CertifBootstrapWorkspaceError::Stopped,
            CertifEnsureRealmCreatedError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => CertifBootstrapWorkspaceError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
            CertifEnsureRealmCreatedError::Internal(err) => {
                err.context("Cannot do server-side realm creation").into()
            }
        })?;

    // Step 2

    super::realm_key_rotation::ensure_realm_intial_key_rotation(ops, realm_id).await.map_err(|e| match e {
        CertifRotateRealmKeyError::Offline => CertifBootstrapWorkspaceError::Offline,
        CertifRotateRealmKeyError::Stopped => CertifBootstrapWorkspaceError::Stopped,
        CertifRotateRealmKeyError::AuthorNotAllowed => CertifBootstrapWorkspaceError::AuthorNotAllowed,
        CertifRotateRealmKeyError::TimestampOutOfBallpark {
            server_timestamp, client_timestamp, ballpark_client_early_offset, ballpark_client_late_offset
        } => CertifBootstrapWorkspaceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        },
        CertifRotateRealmKeyError::InvalidCertificate(err) => CertifBootstrapWorkspaceError::InvalidCertificate(err),
        CertifRotateRealmKeyError::Internal(err) => err.context("Cannot do initial server-side key rotation").into(),
        // Unexpected given we have just made sure the realm was created
        CertifRotateRealmKeyError::UnknownRealm => anyhow::anyhow!(
            "Cannot do initial server-side key rotation: server doesn't know the realm while we have just made sure it exists !"
        ).into(),
    })?;

    // Step 3

    loop {
        let outcome =
            super::realm_rename::ensure_realm_initial_rename(ops, realm_id, name.to_owned()).await;

        return match outcome {
            Ok(outcome) => Ok(outcome),
            Err(err) => match err {
                // Step 3 depends on the certificate uploaded on step 2, hence it is likely
                // our local storage is missing said certificate (remember a monitor only
                // updates the local storage once it has been notified by the server).
                // In that case we will poll the server for the new certificates and retry.
                CertifRenameRealmError::NoKey => {
                    // Fetch new certificates and retry
                    ops.poll_server_for_new_certificates(None)
                        .await
                        .map_err(|e| match e {
                            CertifPollServerError::Offline => CertifBootstrapWorkspaceError::Offline,
                            CertifPollServerError::Stopped => CertifBootstrapWorkspaceError::Stopped,
                            CertifPollServerError::InvalidCertificate(err) => {
                                CertifBootstrapWorkspaceError::InvalidCertificate(err)
                            }
                            CertifPollServerError::Internal(err) => {
                                err.context("Cannot poll new certificates").into()
                            }
                        })?;
                        continue;
                },

                // Other cases are proper errors that we propagate

                CertifRenameRealmError::Offline => Err(CertifBootstrapWorkspaceError::Offline),
                CertifRenameRealmError::Stopped => Err(CertifBootstrapWorkspaceError::Stopped),
                CertifRenameRealmError::AuthorNotAllowed => Err(CertifBootstrapWorkspaceError::AuthorNotAllowed),
                CertifRenameRealmError::TimestampOutOfBallpark {
                    server_timestamp, client_timestamp, ballpark_client_early_offset, ballpark_client_late_offset
                } => Err(CertifBootstrapWorkspaceError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                }),
                // It is unlikely the keys bundle is invalid given we've most likely just
                // uploaded it. Anyway, we don't try to play smart here and just propagate
                // the error: a dedicated monitor should detect the issue, heal the keys
                // bundle and finally send an event that will re-trigger the bootstrap.
                CertifRenameRealmError::InvalidKeysBundle(err) => Err(CertifBootstrapWorkspaceError::InvalidKeysBundle(err)),
                CertifRenameRealmError::InvalidCertificate(err) => Err(CertifBootstrapWorkspaceError::InvalidCertificate(err)),
                // Unexpected given we have just made sure the realm was created
                CertifRenameRealmError::UnknownRealm => Err(anyhow::anyhow!(
                    "Cannot upload initial realm name certificate: server doesn't know the realm while we have just made sure it exists !"
                ).into()),
                CertifRenameRealmError::Internal(err) => Err(err.context("Cannot upload initial realm name certificate").into()),
            }
        };
    }
}
