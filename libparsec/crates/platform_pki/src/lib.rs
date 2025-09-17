// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(target_os = "windows")]
mod windows;

pub use platform::{decrypt_secret_key, encrypt_secret_key};

use bytes::Bytes;

#[cfg(target_os = "windows")]
pub(crate) use windows as platform;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateHash {
    SHA256(Box<[u8; 32]>),
}

impl AsRef<[u8]> for CertificateHash {
    fn as_ref(&self) -> &[u8] {
        match self {
            CertificateHash::SHA256(items) => items.as_ref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CertificateReference {
    Id(Bytes),
    Hash(CertificateHash),
    IdOrHash(CertificateReferenceIdOrHash),
}

impl From<CertificateReferenceIdOrHash> for CertificateReference {
    fn from(value: CertificateReferenceIdOrHash) -> Self {
        Self::IdOrHash(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CertificateReferenceIdOrHash {
    pub id: Bytes,
    pub hash: CertificateHash,
}

#[cfg(target_os = "windows")]
pub use windows::show_certificate_selection_dialog;

// Mock module for unsupported platform
#[cfg(not(target_os = "windows"))]
mod platform {
    use bytes::Bytes;
    use libparsec_crypto::SecretKey;

    use crate::{
        CertificateDer, CertificateHash, CertificateReference, DecryptSecretKeyError,
        EncryptSecretKeyError, GetDerEncodedCertificateError,
    };
    pub fn get_der_encoded_certificate(
        certificate_ref: &CertificateReference,
    ) -> Result<CertificateDer, GetDerEncodedCertificateError> {
        let _ = certificate_ref;
        unimplemented!("platform not supported")
    }

    /// returns encrypted key, certificated id and certificate hash
    pub fn encrypt_secret_key(
        key: &SecretKey,
        certificate_ref: &CertificateReference,
    ) -> Result<(Bytes, Bytes, CertificateHash), EncryptSecretKeyError> {
        let _ = certificate_ref;
        let _ = key;

        unimplemented!()
    }

    pub fn decrypt_secret_key(
        encrypted_key: &Bytes,
        certificate_ref: &CertificateReference,
    ) -> Result<SecretKey, DecryptSecretKeyError> {
        let _ = certificate_ref;
        let _ = encrypted_key;
        unimplemented!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetDerEncodedCertificateError {
    #[error("Cannot open certificate store: {}", .0)]
    CannotOpenStore(std::io::Error),
    #[error("Cannot found certificate")]
    NotFound,
    #[error("Cannot get certificate info: {}", .0)]
    CannotGetCertificateInfo(std::io::Error),
}

pub use platform::get_der_encoded_certificate;

pub struct CertificateDer {
    pub cert_ref: CertificateReferenceIdOrHash,
    pub der_content: Bytes,
}

#[derive(Debug, thiserror::Error)]
pub enum EncryptSecretKeyError {}

#[derive(Debug, thiserror::Error)]
pub enum DecryptSecretKeyError {}
