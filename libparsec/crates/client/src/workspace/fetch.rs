// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol::authenticated_cmds, ConnectionError};
use libparsec_types::prelude::*;

use super::WorkspaceOps;
use crate::certif::{
    CertifValidateManifestError, InvalidCertificateError, InvalidKeysBundleError,
    InvalidManifestError,
};

#[derive(Debug, thiserror::Error)]
pub enum FetchRemoteManifestError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error("The manifest's realm doesn't exist on the server")]
    RealmNotFound,
    #[error("This manifest doesn't exist on the server")]
    VlobNotFound,
    #[error("Server has no such version for this manifest")]
    BadVersion,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error(transparent)]
    InvalidKeysBundle(#[from] InvalidKeysBundleError),
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
    vlob_id: VlobID,
    version: Option<VersionInt>,
) -> Result<ChildManifest, FetchRemoteManifestError> {
    let data = fetch_vlob(ops, vlob_id, version).await?;

    ops.certificates_ops
        .validate_child_manifest(
            data.needed_realm_certificate_timestamp,
            data.needed_common_certificate_timestamp,
            ops.realm_id,
            data.key_index,
            vlob_id,
            &data.expected_author,
            data.expected_version,
            data.expected_timestamp,
            &data.blob,
        )
        .await
        .map_err(|err| match err {
            CertifValidateManifestError::Offline => FetchRemoteManifestError::Offline,
            CertifValidateManifestError::Stopped => FetchRemoteManifestError::Stopped,
            CertifValidateManifestError::NotAllowed => FetchRemoteManifestError::NotAllowed,
            CertifValidateManifestError::InvalidManifest(err) => {
                FetchRemoteManifestError::InvalidManifest(err)
            }
            CertifValidateManifestError::InvalidCertificate(err) => {
                FetchRemoteManifestError::InvalidCertificate(err)
            }
            CertifValidateManifestError::InvalidKeysBundle(err) => {
                FetchRemoteManifestError::InvalidKeysBundle(err)
            }
            CertifValidateManifestError::Internal(err) => {
                err.context("Cannot validate vlob").into()
            }
        })
}

#[allow(unused)]
pub(super) async fn fetch_remote_workspace_manifest(
    ops: &WorkspaceOps,
    version: Option<VersionInt>,
) -> Result<WorkspaceManifest, FetchRemoteManifestError> {
    let vlob_id = ops.realm_id;
    let data = fetch_vlob(ops, vlob_id, version).await?;

    ops.certificates_ops
        .validate_workspace_manifest(
            data.needed_realm_certificate_timestamp,
            data.needed_common_certificate_timestamp,
            ops.realm_id,
            data.key_index,
            &data.expected_author,
            data.expected_version,
            data.expected_timestamp,
            &data.blob,
        )
        .await
        .map_err(|err| match err {
            CertifValidateManifestError::Offline => FetchRemoteManifestError::Offline,
            CertifValidateManifestError::Stopped => FetchRemoteManifestError::Stopped,
            CertifValidateManifestError::NotAllowed => FetchRemoteManifestError::NotAllowed,
            CertifValidateManifestError::InvalidManifest(err) => {
                FetchRemoteManifestError::InvalidManifest(err)
            }
            CertifValidateManifestError::InvalidCertificate(err) => {
                FetchRemoteManifestError::InvalidCertificate(err)
            }
            CertifValidateManifestError::InvalidKeysBundle(err) => {
                FetchRemoteManifestError::InvalidKeysBundle(err)
            }
            CertifValidateManifestError::Internal(err) => {
                err.context("Cannot validate vlob").into()
            }
        })
}

struct VlobData {
    needed_common_certificate_timestamp: DateTime,
    needed_realm_certificate_timestamp: DateTime,
    key_index: IndexInt,
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
    if version.is_some() {
        todo!()
    }

    use authenticated_cmds::latest::vlob_read_batch::{Rep, Req};

    let req = Req {
        realm_id: ops.realm_id,
        vlobs: vec![vlob_id],
        at: None,
    };

    let rep = ops.cmds.send(req).await?;

    match rep {
        Rep::Ok { mut items, needed_common_certificate_timestamp, needed_realm_certificate_timestamp } => {
            let (_, key_index, expected_author, expected_version, expected_timestamp, blob) = items.pop().ok_or(FetchRemoteManifestError::VlobNotFound)?;
            Ok(VlobData {
                needed_common_certificate_timestamp,
                needed_realm_certificate_timestamp,
                key_index,
                expected_author,
                expected_version,
                expected_timestamp,
                blob
            })
        },
        // Expected errors
        Rep::AuthorNotAllowed => Err(FetchRemoteManifestError::NotAllowed),
        Rep::RealmNotFound => Err(FetchRemoteManifestError::RealmNotFound),
        // Unexpected errors :(
        rep @ (
            // One item is too many ???? Really ????
            Rep::TooManyElements |
            // Don't know what to do with this status :/
            Rep::UnknownStatus { .. }
        ) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        },
    }
}
