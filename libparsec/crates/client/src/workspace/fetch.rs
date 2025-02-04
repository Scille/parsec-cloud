// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, AuthenticatedCmds, ConnectionError,
};
use libparsec_types::prelude::*;

use crate::{
    certif::{
        CertifValidateManifestError, CertificateOps, InvalidCertificateError,
        InvalidKeysBundleError, InvalidManifestError,
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
    certificates_ops: &CertificateOps,
    realm_id: VlobID,
    vlob_id: VlobID,
    at: Option<DateTime>,
) -> Result<ChildManifest, FetchRemoteManifestError> {
    let data = fetch_vlob(cmds, realm_id, vlob_id, at).await?;

    certificates_ops
        .validate_child_manifest(
            data.needed_realm_certificate_timestamp,
            data.needed_common_certificate_timestamp,
            realm_id,
            data.key_index,
            vlob_id,
            data.expected_author,
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

pub(super) async fn fetch_remote_workspace_manifest(
    cmds: &AuthenticatedCmds,
    certificates_ops: &CertificateOps,
    realm_id: VlobID,
    at: Option<DateTime>,
) -> Result<FolderManifest, FetchRemoteManifestError> {
    let vlob_id = realm_id; // Remember: workspace manifest's ID *is* the realm ID !
    let data = fetch_vlob(cmds, realm_id, vlob_id, at).await?;

    certificates_ops
        .validate_workspace_manifest(
            data.needed_realm_certificate_timestamp,
            data.needed_common_certificate_timestamp,
            realm_id,
            data.key_index,
            data.expected_author,
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
    at: Option<DateTime>,
) -> Result<VlobData, FetchRemoteManifestError> {
    use authenticated_cmds::latest::vlob_read_batch::{Rep, Req};

    let req = Req {
        realm_id,
        vlobs: vec![vlob_id],
        at,
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
    #[error("The realm's manifest doesn't exist on the server")]
    RealmNotFound,
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
    certificates_ops: &CertificateOps,
    realm_id: VlobID,
    manifest: &FileManifest,
    access: &BlockAccess,
) -> Result<Bytes, FetchRemoteBlockError> {
    let (needed_realm_certificate_timestamp, key_index, encrypted) = {
        use authenticated_cmds::latest::block_read::{Rep, Req};

        let req = Req {
            realm_id,
            block_id: access.id,
        };

        let rep = cmds.send(req).await?;

        match rep {
            Rep::Ok {
                needed_realm_certificate_timestamp,
                key_index,
                block,
            } => Ok((needed_realm_certificate_timestamp, key_index, block)),
            // Expected errors
            Rep::StoreUnavailable => Err(FetchRemoteBlockError::StoreUnavailable),
            Rep::AuthorNotAllowed => Err(FetchRemoteBlockError::NoRealmAccess),
            Rep::RealmNotFound => Err(FetchRemoteBlockError::RealmNotFound),
            Rep::BlockNotFound => Err(FetchRemoteBlockError::BlockNotFound),
            // Unexpected errors :(
            rep @ Rep::UnknownStatus { .. } => {
                Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
            }
        }?
    };

    let data: Bytes = certificates_ops
        .validate_block(
            needed_realm_certificate_timestamp,
            realm_id,
            key_index,
            manifest,
            access,
            &encrypted,
        )
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

#[derive(Debug, thiserror::Error)]
pub enum FetchVersionsRemoteManifestError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error("The manifest's realm doesn't exist on the server")]
    RealmNotFound,
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

impl From<ConnectionError> for FetchVersionsRemoteManifestError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub(super) async fn fetch_versions_remote_workspace_manifest(
    cmds: &AuthenticatedCmds,
    certificates_ops: &CertificateOps,
    realm_id: VlobID,
    versions: &[VersionInt],
) -> Result<Vec<FolderManifest>, FetchVersionsRemoteManifestError> {
    let VlobVersionsData {
        needed_common_certificate_timestamp,
        needed_realm_certificate_timestamp,
        items,
    } = fetch_versions_vlob(
        cmds,
        realm_id,
        versions.iter().map(|version| (realm_id, *version)),
    )
    .await?;

    let mut manifests = Vec::with_capacity(items.len());
    for (_, key_index, expected_author, expected_version, expected_timestamp, blob) in items {
        let manifest = certificates_ops
            .validate_workspace_manifest(
                needed_realm_certificate_timestamp,
                needed_common_certificate_timestamp,
                realm_id,
                key_index,
                expected_author,
                expected_version,
                expected_timestamp,
                &blob,
            )
            .await
            .map_err(|err| match err {
                CertifValidateManifestError::Offline => FetchVersionsRemoteManifestError::Offline,
                CertifValidateManifestError::Stopped => FetchVersionsRemoteManifestError::Stopped,
                CertifValidateManifestError::NotAllowed => {
                    FetchVersionsRemoteManifestError::NoRealmAccess
                }
                CertifValidateManifestError::InvalidManifest(err) => {
                    FetchVersionsRemoteManifestError::InvalidManifest(err)
                }
                CertifValidateManifestError::InvalidCertificate(err) => {
                    FetchVersionsRemoteManifestError::InvalidCertificate(err)
                }
                CertifValidateManifestError::InvalidKeysBundle(err) => {
                    FetchVersionsRemoteManifestError::InvalidKeysBundle(err)
                }
                CertifValidateManifestError::Internal(err) => {
                    err.context("Cannot validate vlob").into()
                }
            })?;
        manifests.push(manifest);
    }

    Ok(manifests)
}

struct VlobVersionsData {
    needed_common_certificate_timestamp: DateTime,
    needed_realm_certificate_timestamp: DateTime,
    /// Taken verbatim from `vlob_read_batch`'s server response.
    ///
    /// Fields are: vlob ID, key index, author, version, created on, blob
    ///
    /// Not it would be tempting to introduce a dedicated type to improve readability
    /// here, but this is not worth it since this type is private and only used in a
    /// single place to split code into two functions (so no matter what we will have
    /// a loop dealing with this tuple type).
    items: Vec<(VlobID, IndexInt, DeviceID, VersionInt, DateTime, Bytes)>,
}

async fn fetch_versions_vlob(
    cmds: &AuthenticatedCmds,
    realm_id: VlobID,
    items: impl Iterator<Item = (VlobID, VersionInt)>,
) -> Result<VlobVersionsData, FetchVersionsRemoteManifestError> {
    use authenticated_cmds::latest::vlob_read_versions::{Rep, Req};

    let req = Req {
        realm_id,
        items: items.collect(),
    };

    let rep = cmds.send(req).await?;

    match rep {
        Rep::Ok { needed_common_certificate_timestamp, needed_realm_certificate_timestamp, items } => {
            Ok(VlobVersionsData {
                needed_common_certificate_timestamp,
                needed_realm_certificate_timestamp,
                items,
            })
        },
        // Expected errors
        Rep::AuthorNotAllowed => Err(FetchVersionsRemoteManifestError::NoRealmAccess),
        Rep::RealmNotFound => Err(FetchVersionsRemoteManifestError::RealmNotFound),
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
