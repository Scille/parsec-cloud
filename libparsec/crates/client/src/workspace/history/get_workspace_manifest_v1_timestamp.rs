// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::WorkspaceHistoryOps;
use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    server_fetch::{server_fetch_versions_workspace_manifest, ServerFetchVersionsManifestError},
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryGetWorkspaceManifestV1TimestampError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

/// Earliest date we can go back.
///
/// It's possible to have non root manifest older than this date, however
/// they were in practice not accessible until the workspace manifest got updated.
pub async fn get_workspace_manifest_v1_timestamp(
    ops: &WorkspaceHistoryOps,
) -> Result<Option<DateTime>, WorkspaceHistoryGetWorkspaceManifestV1TimestampError> {
    // Cache lookup...

    {
        let guard = ops.cache.lock().expect("Mutex is poisoned");
        if let Some(timestamp) = guard.workspace_manifest_v1_timestamp {
            return Ok(Some(timestamp));
        }
    }

    // Cache miss !

    let outcome = server_fetch_versions_workspace_manifest(
        &ops.cmds,
        &ops.certificates_ops,
        ops.realm_id,
        &[1],
    )
    .await;

    let workspace_manifest_v1_timestamp = match outcome {
        Ok(manifest) => match manifest.first() {
            Some(manifest) => manifest.timestamp,
            // Manifest has never been uploaded to the server yet
            None => return Ok(None),
        },
        Err(err) => {
            return match err {
                // The realm doesn't exist on server side, hence we are its creator and
                // the manifest has never been uploaded to the server yet.
                ServerFetchVersionsManifestError::RealmNotFound => Ok(None),

                // Actual errors
                ServerFetchVersionsManifestError::Stopped => {
                    Err(WorkspaceHistoryGetWorkspaceManifestV1TimestampError::Stopped)
                }
                ServerFetchVersionsManifestError::Offline(e) => {
                    Err(WorkspaceHistoryGetWorkspaceManifestV1TimestampError::Offline(e))
                }
                ServerFetchVersionsManifestError::NoRealmAccess => {
                    Err(WorkspaceHistoryGetWorkspaceManifestV1TimestampError::NoRealmAccess)
                }
                ServerFetchVersionsManifestError::InvalidKeysBundle(err) => Err(
                    WorkspaceHistoryGetWorkspaceManifestV1TimestampError::InvalidKeysBundle(err),
                ),
                ServerFetchVersionsManifestError::InvalidCertificate(err) => Err(
                    WorkspaceHistoryGetWorkspaceManifestV1TimestampError::InvalidCertificate(err),
                ),
                ServerFetchVersionsManifestError::InvalidManifest(err) => {
                    Err(WorkspaceHistoryGetWorkspaceManifestV1TimestampError::InvalidManifest(err))
                }
                ServerFetchVersionsManifestError::Internal(err) => Err(err.into()),
            };
        }
    };

    // Update cache and return

    let mut guard = ops.cache.lock().expect("Mutex is poisoned");
    match guard.workspace_manifest_v1_timestamp {
        None => {
            guard.workspace_manifest_v1_timestamp = Some(workspace_manifest_v1_timestamp);
            Ok(guard.workspace_manifest_v1_timestamp)
        }
        // Concurrent operation has updated the cache, let's just pretend it was
        // available when we first checked it.
        Some(timestamp) => Ok(Some(timestamp)),
    }
}
