// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;
use libparsec_client_connection::{protocol::authenticated_cmds, ConnectionError};

use super::WorkspaceOps;
use crate::certificates_ops::{
        InvalidCertificateError, InvalidManifestError, ValidateManifestError,
};


#[derive(Debug, thiserror::Error)]
pub enum FetchRemoteManifestError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Server has no such version for this manifest")]
    BadVersion,
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    InvalidManifest(#[from] InvalidManifestError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for FetchRemoteManifestError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

async fn fetch_remote_child_manifest(
    ops: &WorkspaceOps,
    entry_id: EntryID,
    version: Option<VersionInt>,
) -> Result<UserManifest, FetchRemoteManifestError> {
    // let encryption_revision = ops.encryption_revision;
    let encryption_revision = 1;  // TODO
    let (certificate_index, expected_author, version_according_to_server, expected_timestamp, blob) = fetch_vlob(ops, entry_id.into(), version, encryption_revision).await?;

    let outcome = ops.certificates_ops.validate_child_manifest(certificate_index, &expected_author, expected_version, expected_timestamp, &blob).await;
    outcome.map_err(|err| match err {
        ValidateManifestError::InvalidCertificate(err) => {
            FetchRemoteManifestError::InvalidCertificate(err)
        }
        ValidateManifestError::InvalidManifest(err) => {
            FetchRemoteManifestError::InvalidManifest(err)
        }
        ValidateManifestError::Offline => FetchRemoteManifestError::Offline,
        err @ ValidateManifestError::Internal(_) => {
            FetchRemoteManifestError::Internal(err.into())
        }
    })

    todo!()
}

async fn fetch_remote_workspace_manifest(
    ops: &WorkspaceOps,
    version: Option<VersionInt>,
) -> Result<UserManifest, FetchRemoteManifestError> {
    let vlob_id = VlobID::from(ops.realm_id.as_ref().to_owned());
    // let encryption_revision = ops.encryption_revision;
    let encryption_revision = 1;  // TODO
    let _vlob = fetch_vlob(ops, vlob_id, version, encryption_revision).await?;
    todo!()
}

async fn fetch_vlob(
    ops: &WorkspaceOps,
    vlob_id: VlobID,
    version: Option<VersionInt>,
    encryption_revision: IndexInt,

) -> Result<(IndexInt, DeviceID, VersionInt, DateTime, Bytes), FetchRemoteManifestError> {
    use authenticated_cmds::latest::vlob_read::{Rep, Req};

    let req = Req {
        encryption_revision,
        timestamp: None,
        version,
        vlob_id,
    };

    let rep = ops.cmds.send(req).await?;

    match rep {
        Rep::Ok { certificate_index, author: expected_author, version: version_according_to_server, timestamp: expected_timestamp, blob } => {
            let expected_version = match version {
                Some(version) => version,
                None => version_according_to_server,
            };
            Ok((certificate_index, expected_author, version_according_to_server, expected_timestamp, blob))
        }
        // Expected errors
        Rep::BadVersion if version.is_some() => {
            Err(FetchRemoteManifestError::BadVersion)
        }
        // Unexpected errors :(
        rep @ (
            // We didn't specified a `version` argument in the request
            Rep::BadVersion |
            // User never loses access to it user manifest's workspace
            Rep::NotAllowed |
            // User manifest's workspace never gets reencrypted !
            Rep::InMaintenance |
            Rep::BadEncryptionRevision |
            // User manifest's vlob is supposed to exists !
            Rep::NotFound { .. } |
            // Don't know what to do with this status :/
            Rep::UnknownStatus { .. }
        ) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}
