// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    CertifDecryptForRealmError, CertifEncryptForRealmError, EncrytionUsage,
    InvalidCertificateError, InvalidKeysBundleError, WorkspaceOps,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceGeneratePathAddrError {
    /// Stopped is not used by `encrypt_for_realm`, but is convenient anyways given
    /// it is needed by the wrapper `CertificateOps::encrypt_for_realm`.
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("There is no key available in this realm for encryption")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn generate_path_addr(
    ops: &WorkspaceOps,
    path: &FsPath,
) -> Result<ParsecWorkspacePathAddr, WorkspaceGeneratePathAddrError> {
    // String is already in UTF8, so no need for encoding
    let path_as_str = path.to_string();
    let cleartext = path_as_str.as_bytes();

    let (encrypted, key_index) = ops
        .certificates_ops
        .encrypt_for_realm(EncrytionUsage::PathUrl, ops.realm_id, cleartext)
        .await
        .map_err(|err| match err {
            CertifEncryptForRealmError::Stopped => WorkspaceGeneratePathAddrError::Stopped,
            CertifEncryptForRealmError::Offline(e) => WorkspaceGeneratePathAddrError::Offline(e),
            CertifEncryptForRealmError::NotAllowed => WorkspaceGeneratePathAddrError::NotAllowed,
            CertifEncryptForRealmError::NoKey => WorkspaceGeneratePathAddrError::NoKey,
            CertifEncryptForRealmError::InvalidKeysBundle(err) => {
                WorkspaceGeneratePathAddrError::InvalidKeysBundle(err)
            }
            CertifEncryptForRealmError::Internal(err) => {
                err.context("cannot encrypt path for realm").into()
            }
        })?;

    Ok(ParsecWorkspacePathAddr::new(
        ops.device.organization_addr.clone(),
        ops.device.organization_id().to_owned(),
        ops.realm_id,
        key_index,
        encrypted,
    ))
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceDecryptPathAddrError {
    /// Stopped is not used by `encrypt_for_realm`, but is convenient anyways given
    /// it is needed by the wrapper `CertificateOps::encrypt_for_realm`.
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("The referenced key doesn't exist yet in this realm")]
    KeyNotFound,
    #[error("The referenced key appears to be corrupted !")]
    CorruptedKey,
    #[error("Cannot decrypt data")]
    CorruptedData,
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn decrypt_path_addr(
    ops: &WorkspaceOps,
    link: &ParsecWorkspacePathAddr,
) -> Result<FsPath, WorkspaceDecryptPathAddrError> {
    let cleartext = ops
        .certificates_ops
        .decrypt_opaque_data_for_realm(
            EncrytionUsage::PathUrl,
            ops.realm_id,
            link.key_index(),
            link.encrypted_path(),
        )
        .await
        .map_err(|err| match err {
            CertifDecryptForRealmError::Stopped => WorkspaceDecryptPathAddrError::Stopped,
            CertifDecryptForRealmError::Offline(e) => WorkspaceDecryptPathAddrError::Offline(e),
            CertifDecryptForRealmError::NotAllowed => WorkspaceDecryptPathAddrError::NotAllowed,
            CertifDecryptForRealmError::KeyNotFound => WorkspaceDecryptPathAddrError::KeyNotFound,
            CertifDecryptForRealmError::CorruptedKey => WorkspaceDecryptPathAddrError::CorruptedKey,
            CertifDecryptForRealmError::CorruptedData => {
                WorkspaceDecryptPathAddrError::CorruptedData
            }
            CertifDecryptForRealmError::InvalidCertificate(err) => {
                WorkspaceDecryptPathAddrError::InvalidCertificate(err)
            }
            CertifDecryptForRealmError::InvalidKeysBundle(err) => {
                WorkspaceDecryptPathAddrError::InvalidKeysBundle(err)
            }
            CertifDecryptForRealmError::Internal(err) => err.context("").into(),
        })?;

    let path_as_str = std::str::from_utf8(&cleartext)
        .map_err(|_| WorkspaceDecryptPathAddrError::CorruptedData)?;
    path_as_str
        .parse()
        .map_err(|_| WorkspaceDecryptPathAddrError::CorruptedData)
}
