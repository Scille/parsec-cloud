// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, AuthenticatedCmds, ConnectionError,
};
use libparsec_types::prelude::*;

use crate::{
    certif::{
        CertifOps, CertifValidateManifestError, InvalidCertificateError, InvalidKeysBundleError,
        InvalidManifestError,
    },
    CertifValidateBlockError, InvalidBlockAccessError,
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

impl From<ConnectionError> for FetchRemoteManifestError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub(super) async fn fetch_remote_child_manifest(
    cmds: &AuthenticatedCmds,
    certificates_ops: &CertifOps,
    realm_id: VlobID,
    vlob_id: VlobID,
) -> Result<ChildManifest, FetchRemoteManifestError> {
    let data = fetch_vlob(cmds, realm_id, vlob_id).await?;

    certificates_ops
        .validate_child_manifest(
            data.needed_realm_certificate_timestamp,
            data.needed_common_certificate_timestamp,
            realm_id,
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
            CertifValidateManifestError::NotAllowed => FetchRemoteManifestError::NoRealmAccess,
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
    cmds: &AuthenticatedCmds,
    certificates_ops: &CertifOps,
    realm_id: VlobID,
) -> Result<WorkspaceManifest, FetchRemoteManifestError> {
    let vlob_id = realm_id; // Remember: workspace manifest's ID *is* the realm ID !
    let data = fetch_vlob(cmds, realm_id, vlob_id).await?;

    certificates_ops
        .validate_workspace_manifest(
            data.needed_realm_certificate_timestamp,
            data.needed_common_certificate_timestamp,
            realm_id,
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
            CertifValidateManifestError::NotAllowed => FetchRemoteManifestError::NoRealmAccess,
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
    cmds: &AuthenticatedCmds,
    realm_id: VlobID,
    vlob_id: VlobID,
) -> Result<VlobData, FetchRemoteManifestError> {
    use authenticated_cmds::latest::vlob_read_batch::{Rep, Req};

    let req = Req {
        realm_id,
        vlobs: vec![vlob_id],
        at: None,
    };

    let rep = cmds.send(req).await?;

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
        Rep::AuthorNotAllowed => Err(FetchRemoteManifestError::NoRealmAccess),
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

#[derive(Debug, thiserror::Error)]
pub enum FetchRemoteBlockError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error("The block doesn't exist on the server")]
    BlockNotFound,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Block access is temporary unavailable on the server")]
    StoreUnavailable,
    #[error(transparent)]
    InvalidBlockAccess(#[from] Box<InvalidBlockAccessError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for FetchRemoteBlockError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub(super) async fn fetch_block(
    cmds: &AuthenticatedCmds,
    certificates_ops: &CertifOps,
    realm_id: VlobID,
    manifest: &FileManifest,
    access: &BlockAccess,
) -> Result<Bytes, FetchRemoteBlockError> {
    let encrypted = {
        use authenticated_cmds::latest::block_read::{Rep, Req};

        let req = Req {
            block_id: access.id,
        };

        let rep = cmds.send(req).await?;

        match rep {
            Rep::Ok { block } => Ok(block),
            // Expected errors
            Rep::StoreUnavailable => Err(FetchRemoteBlockError::StoreUnavailable),
            Rep::AuthorNotAllowed => Err(FetchRemoteBlockError::NoRealmAccess),
            Rep::BlockNotFound => Err(FetchRemoteBlockError::BlockNotFound),
            // Unexpected errors :(
            rep @ Rep::UnknownStatus { .. } => {
                Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
            }
        }?
    };

    let data: Bytes = certificates_ops
        .validate_block(realm_id, manifest, access, &encrypted)
        .await
        .map_err(|err| match err {
            CertifValidateBlockError::Offline => FetchRemoteBlockError::Offline,
            CertifValidateBlockError::Stopped => FetchRemoteBlockError::Stopped,
            CertifValidateBlockError::NotAllowed => FetchRemoteBlockError::NoRealmAccess,
            CertifValidateBlockError::InvalidBlockAccess(err) => {
                FetchRemoteBlockError::InvalidBlockAccess(err)
            }
            CertifValidateBlockError::InvalidCertificate(err) => {
                FetchRemoteBlockError::InvalidCertificate(err)
            }
            CertifValidateBlockError::InvalidKeysBundle(err) => {
                FetchRemoteBlockError::InvalidKeysBundle(err)
            }
            CertifValidateBlockError::Internal(err) => {
                err.context("cannot validate block from server").into()
            }
        })?
        .into();

    Ok(data)
}
