// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    CertifDecryptForRealmError, CertifGetLatestRealmKeyForEncryptionError, EncryptionUsage,
    InvalidCertificateError, InvalidKeysBundleError, workspace::WorkspaceOps,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceEncryptEditicsError {
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

pub(crate) async fn editics_encrypt(
    ops: &WorkspaceOps,
    document_id: VlobID,
    cleartext: &[u8],
) -> Result<(IndexInt, Vec<u8>), WorkspaceEncryptEditicsError> {
    let (key, key_index) = ops
        .certificates_ops
        .get_latest_realm_key_for_encryption(EncryptionUsage::Editics(document_id), ops.realm_id)
        .await
        .map_err(|err| match err {
            CertifGetLatestRealmKeyForEncryptionError::Stopped => {
                WorkspaceEncryptEditicsError::Stopped
            }
            CertifGetLatestRealmKeyForEncryptionError::Offline(err) => {
                WorkspaceEncryptEditicsError::Offline(err)
            }
            CertifGetLatestRealmKeyForEncryptionError::NotAllowed => {
                WorkspaceEncryptEditicsError::NoRealmAccess
            }
            CertifGetLatestRealmKeyForEncryptionError::RealmDeleted => {
                WorkspaceEncryptEditicsError::RealmDeleted
            }
            CertifGetLatestRealmKeyForEncryptionError::NoKey => WorkspaceEncryptEditicsError::NoKey,
            CertifGetLatestRealmKeyForEncryptionError::InvalidKeysBundle(err) => {
                WorkspaceEncryptEditicsError::InvalidKeysBundle(err)
            }
            CertifGetLatestRealmKeyForEncryptionError::Internal(err) => err
                .context("Cannot encrypt the editics payload for realm")
                .into(),
        })?;

    Ok((key_index, key.encrypt(cleartext)))
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceDecryptEditicsError {
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
    #[error("Cannot decrypt the editics payload")]
    CorruptedData,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(crate) async fn editics_decrypt(
    ops: &WorkspaceOps,
    document_id: VlobID,
    key_index: IndexInt,
    encrypted: &[u8],
) -> Result<Vec<u8>, WorkspaceDecryptEditicsError> {
    ops.certificates_ops
        .decrypt_opaque_data_for_realm(
            EncryptionUsage::Editics(document_id),
            ops.realm_id,
            key_index,
            encrypted,
        )
        .await
        .map_err(|err| match err {
            CertifDecryptForRealmError::Stopped => WorkspaceDecryptEditicsError::Stopped,
            CertifDecryptForRealmError::Offline(e) => WorkspaceDecryptEditicsError::Offline(e),
            CertifDecryptForRealmError::NotAllowed => WorkspaceDecryptEditicsError::NoRealmAccess,
            CertifDecryptForRealmError::RealmDeleted => WorkspaceDecryptEditicsError::RealmDeleted,
            CertifDecryptForRealmError::KeyNotFound => WorkspaceDecryptEditicsError::KeyNotFound,
            CertifDecryptForRealmError::CorruptedKey => WorkspaceDecryptEditicsError::CorruptedKey,
            CertifDecryptForRealmError::CorruptedData => {
                WorkspaceDecryptEditicsError::CorruptedData
            }
            CertifDecryptForRealmError::InvalidCertificate(err) => {
                WorkspaceDecryptEditicsError::InvalidCertificate(err)
            }
            CertifDecryptForRealmError::InvalidKeysBundle(err) => {
                WorkspaceDecryptEditicsError::InvalidKeysBundle(err)
            }
            CertifDecryptForRealmError::Internal(err) => err
                .context("Cannot decrypt the editics payload for realm")
                .into(),
        })
}
