// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::{
    CertifDecryptForRealmError, CertifEncryptForRealmError, InvalidCertificateError,
    InvalidKeysBundleError, WorkspaceOps,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceGenerateFileLinkError {
    /// Stopped is not used by `encrypt_for_realm`, but is convenient anyways given
    /// it is needed by the wrapper `CertificateOps::encrypt_for_realm`.
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("There is no key available in this realm for encryption")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn generate_file_link(
    ops: &WorkspaceOps,
    path: &FsPath,
) -> Result<ParsecOrganizationFileLinkAddr, WorkspaceGenerateFileLinkError> {
    // String is already in UTF8, so no need for encoding
    let path_as_str = path.to_string();
    let cleartext = path_as_str.as_bytes();

    let (encrypted, key_index) = ops
        .certificates_ops
        .encrypt_for_realm(ops.realm_id, cleartext)
        .await
        .map_err(|err| match err {
            CertifEncryptForRealmError::Stopped => WorkspaceGenerateFileLinkError::Stopped,
            CertifEncryptForRealmError::Offline => WorkspaceGenerateFileLinkError::Offline,
            CertifEncryptForRealmError::NotAllowed => WorkspaceGenerateFileLinkError::NotAllowed,
            CertifEncryptForRealmError::NoKey => WorkspaceGenerateFileLinkError::NoKey,
            CertifEncryptForRealmError::InvalidKeysBundle(err) => {
                WorkspaceGenerateFileLinkError::InvalidKeysBundle(err)
            }
            CertifEncryptForRealmError::Internal(err) => {
                err.context("cannot encrypt path for realm").into()
            }
        })?;

    // ops.certificates_ops.

    Ok(ParsecOrganizationFileLinkAddr::new(
        ops.device.organization_addr.clone(),
        ops.device.organization_id().to_owned(),
        ops.realm_id,
        key_index,
        encrypted,
    ))
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceDecryptFileLinkPathError {
    /// Stopped is not used by `encrypt_for_realm`, but is convenient anyways given
    /// it is needed by the wrapper `CertificateOps::encrypt_for_realm`.
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
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

pub async fn decrypt_file_link_path(
    ops: &WorkspaceOps,
    link: &ParsecOrganizationFileLinkAddr,
) -> Result<FsPath, WorkspaceDecryptFileLinkPathError> {
    let cleartext = ops
        .certificates_ops
        .decrypt_opaque_data_for_realm(ops.realm_id, link.key_index(), link.encrypted_path())
        .await
        .map_err(|err| match err {
            CertifDecryptForRealmError::Stopped => WorkspaceDecryptFileLinkPathError::Stopped,
            CertifDecryptForRealmError::Offline => WorkspaceDecryptFileLinkPathError::Offline,
            CertifDecryptForRealmError::NotAllowed => WorkspaceDecryptFileLinkPathError::NotAllowed,
            CertifDecryptForRealmError::KeyNotFound => {
                WorkspaceDecryptFileLinkPathError::KeyNotFound
            }
            CertifDecryptForRealmError::CorruptedKey => {
                WorkspaceDecryptFileLinkPathError::CorruptedKey
            }
            CertifDecryptForRealmError::CorruptedData => {
                WorkspaceDecryptFileLinkPathError::CorruptedData
            }
            CertifDecryptForRealmError::InvalidCertificate(err) => {
                WorkspaceDecryptFileLinkPathError::InvalidCertificate(err)
            }
            CertifDecryptForRealmError::InvalidKeysBundle(err) => {
                WorkspaceDecryptFileLinkPathError::InvalidKeysBundle(err)
            }
            CertifDecryptForRealmError::Internal(err) => err.context("").into(),
        })?;

    let path_as_str = std::str::from_utf8(&cleartext)
        .map_err(|_| WorkspaceDecryptFileLinkPathError::CorruptedData)?;
    path_as_str
        .parse()
        .map_err(|_| WorkspaceDecryptFileLinkPathError::CorruptedData)
}
