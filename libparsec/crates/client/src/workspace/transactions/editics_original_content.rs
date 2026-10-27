// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

// The archive is serialized with msgpack: it is only ever produced and
// consumed by this crate's encrypt/decrypt pair, the server and the host
// pages treat it as an opaque blob.
use rmp_serde::{from_slice, to_vec_named};

use crate::{
    CertifDecryptForRealmError, CertifGetLatestRealmKeyForEncryptionError, EncryptionUsage,
    InvalidCertificateError, InvalidKeysBundleError, workspace::WorkspaceOps,
};

/// The archive of a document's original content shared through the collaborative
/// edition session (see RFC 1030 §1.2): the OnlyOffice `.bin` obtained by the
/// x2t conversion of the workspace file, along with the media files (e.g.
/// images) the conversion extracted. It is encrypted with the workspace key
/// and uploaded to (or fetched from) the editics session, so all the
/// participants edit against a single canonical base independently of their
/// own x2t version.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct EditicsOriginalContentArchive {
    pub bin: Vec<u8>,
    pub medias: Vec<(String, Vec<u8>)>,
}

/// A media file of an original content archive (see
/// `EditicsOriginalContentArchive`): e.g. an image extracted by the x2t
/// conversion, referenced by the `.bin` under its `name`.
///
/// This is the bindings-level shape of a media entry (the archive's internal
/// serialization keeps plain tuples, which the wasm-bindgen boundary cannot
/// carry).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditicsOriginalContentMedia {
    pub name: String,
    pub content: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceEncryptEditicsOriginalContentError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("The workspace's realm has been deleted on the server")]
    RealmDeleted,
    #[error("There is no key available in this realm for encryption")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(crate) async fn encrypt_editics_original_content(
    ops: &WorkspaceOps,
    document_id: VlobID,
    bin: &[u8],
    medias: &[(String, Vec<u8>)],
) -> Result<(IndexInt, Vec<u8>), WorkspaceEncryptEditicsOriginalContentError> {
    let (key, key_index) = ops
        .certificates_ops
        .get_latest_realm_key_for_encryption(
            EncryptionUsage::EditicsOriginalContent(document_id),
            ops.realm_id,
        )
        .await
        .map_err(|err| match err {
            CertifGetLatestRealmKeyForEncryptionError::Stopped => {
                WorkspaceEncryptEditicsOriginalContentError::Stopped
            }
            CertifGetLatestRealmKeyForEncryptionError::Offline(err) => {
                WorkspaceEncryptEditicsOriginalContentError::Offline(err)
            }
            CertifGetLatestRealmKeyForEncryptionError::NotAllowed => {
                WorkspaceEncryptEditicsOriginalContentError::NoRealmAccess
            }
            CertifGetLatestRealmKeyForEncryptionError::RealmDeleted => {
                WorkspaceEncryptEditicsOriginalContentError::RealmDeleted
            }
            CertifGetLatestRealmKeyForEncryptionError::NoKey => {
                WorkspaceEncryptEditicsOriginalContentError::NoKey
            }
            CertifGetLatestRealmKeyForEncryptionError::InvalidKeysBundle(err) => {
                WorkspaceEncryptEditicsOriginalContentError::InvalidKeysBundle(err)
            }
            CertifGetLatestRealmKeyForEncryptionError::Internal(err) => err
                .context("Cannot encrypt the editics original content for realm")
                .into(),
        })?;

    let archive = EditicsOriginalContentArchive {
        bin: bin.to_vec(),
        medias: medias.to_vec(),
    };
    let msgpack = to_vec_named(&archive)
        .map_err(|err| anyhow::anyhow!("Cannot serialize the editics original content: {err}"))?;

    let encrypted = key.encrypt(&msgpack);

    Ok((key_index, encrypted))
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceDecryptEditicsOriginalContentError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("The workspace's realm has been deleted on the server")]
    RealmDeleted,
    #[error("The referenced key doesn't exist yet in this realm")]
    KeyNotFound,
    #[error("The referenced key appears to be corrupted !")]
    CorruptedKey,
    #[error("Cannot decrypt the editics original content")]
    CorruptedData,
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(crate) async fn decrypt_editics_original_content(
    ops: &WorkspaceOps,
    document_id: VlobID,
    key_index: IndexInt,
    encrypted: &[u8],
) -> Result<(Vec<u8>, Vec<(String, Vec<u8>)>), WorkspaceDecryptEditicsOriginalContentError> {
    let msgpack = ops
        .certificates_ops
        .decrypt_opaque_data_for_realm(
            EncryptionUsage::EditicsOriginalContent(document_id),
            ops.realm_id,
            key_index,
            encrypted,
        )
        .await
        .map_err(|err| match err {
            CertifDecryptForRealmError::Stopped => {
                WorkspaceDecryptEditicsOriginalContentError::Stopped
            }
            CertifDecryptForRealmError::Offline(e) => {
                WorkspaceDecryptEditicsOriginalContentError::Offline(e)
            }
            CertifDecryptForRealmError::NotAllowed => {
                WorkspaceDecryptEditicsOriginalContentError::NoRealmAccess
            }
            CertifDecryptForRealmError::RealmDeleted => {
                WorkspaceDecryptEditicsOriginalContentError::RealmDeleted
            }
            CertifDecryptForRealmError::KeyNotFound => {
                WorkspaceDecryptEditicsOriginalContentError::KeyNotFound
            }
            CertifDecryptForRealmError::CorruptedKey => {
                WorkspaceDecryptEditicsOriginalContentError::CorruptedKey
            }
            CertifDecryptForRealmError::CorruptedData => {
                WorkspaceDecryptEditicsOriginalContentError::CorruptedData
            }
            CertifDecryptForRealmError::InvalidCertificate(err) => {
                WorkspaceDecryptEditicsOriginalContentError::InvalidCertificate(err)
            }
            CertifDecryptForRealmError::InvalidKeysBundle(err) => anyhow::Error::from(err)
                .context("Cannot decrypt the editics original content for realm")
                .into(),
            CertifDecryptForRealmError::Internal(err) => err
                .context("Cannot decrypt the editics original content for realm")
                .into(),
        })?;

    let archive: EditicsOriginalContentArchive = from_slice(&msgpack)
        .map_err(|_| WorkspaceDecryptEditicsOriginalContentError::CorruptedData)?;

    Ok((archive.bin, archive.medias))
}
