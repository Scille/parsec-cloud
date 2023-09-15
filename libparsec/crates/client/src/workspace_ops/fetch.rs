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
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("Realm ")]
    BadEncryptionRevision,
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

pub(super) async fn fetch_remote_child_manifest(
    ops: &WorkspaceOps,
    entry_id: EntryID,
    version: Option<VersionInt>,
) -> Result<ChildManifest, FetchRemoteManifestError> {
    let data = fetch_vlob(ops, entry_id.into(), version).await?;

    ops.certificates_ops.validate_child_manifest(
        data.certificate_index, &data.expected_author, data.expected_version, data.expected_timestamp, &data.blob
    ).await
    .map_err(|err| match err {
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
    })}

pub(super) async fn fetch_remote_workspace_manifest(
    ops: &WorkspaceOps,
    version: Option<VersionInt>,
) -> Result<WorkspaceManifest, FetchRemoteManifestError> {

    let vlob_id = VlobID::from(ops.realm_id.as_ref().to_owned());
    let data = fetch_vlob(ops, vlob_id, version).await?;

    ops.certificates_ops.validate_workspace_manifest(
        data.certificate_index, &data.expected_author, data.expected_version, data.expected_timestamp, &data.blob
    ).await
    .map_err(|err| match err {
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
}


struct VlobData {
    certificate_index: IndexInt,
    expected_author: DeviceID,
    expected_version: VersionInt,
    expected_timestamp: DateTime,
    blob: Bytes,
}

async fn fetch_vlob(
    ops: &WorkspaceOps,
    vlob_id: VlobID,
    version: Option<VersionInt>,
) -> Result<VlobData, FetchRemoteManifestError> {
    use authenticated_cmds::latest::vlob_read::{Rep, Req};

    let req = Req {
        encryption_revision: 1,  // TODO
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
            Ok(VlobData {
                certificate_index, expected_author, expected_version, expected_timestamp, blob
            })
        }
        // Expected errors
        Rep::NotAllowed => Err(FetchRemoteManifestError::NotAllowed),
        Rep::NotFound { .. } => Err(FetchRemoteManifestError::NotFound),
        Rep::BadVersion if version.is_some() => {
            Err(FetchRemoteManifestError::BadVersion)
        }
        // TODO: InMaintenance status for vlob_read is only returned if we provide the
        // latest encryption_revision while under still under reencryption. But doing
        // this means our client is buggy (in theory we know we should not use this
        // revision before we received the end of maintenance event)
        Rep::InMaintenance => Err(FetchRemoteManifestError::InMaintenance),
        // TODO: in case of BadEncryptionRevision, we should double check what is the
        // encryption revision (concurrent operation should have updated it) and retry
        Rep::BadEncryptionRevision => Err(FetchRemoteManifestError::BadEncryptionRevision),
        // Unexpected errors :(
        rep @ (
            // We didn't specified a `version` argument in the request
            Rep::BadVersion |
            // Don't know what to do with this status :/
            Rep::UnknownStatus { .. }
        ) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    }
}
