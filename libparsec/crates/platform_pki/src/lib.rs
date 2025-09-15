// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(target_os = "windows")]
mod windows;

use bytes::Bytes;

#[cfg(target_os = "windows")]
pub(crate) use windows as platform;

#[derive(Debug, Clone)]
pub enum CertificateHash {
    SHA256(Box<[u8; 32]>),
}

#[cfg(feature = "hash-sri-display")]
impl std::fmt::Display for CertificateHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (hash_str, data) = match self {
            CertificateHash::SHA256(data) => ("sha256", data.as_ref()),
        };
        write!(
            f,
            "{hash_str}-{}",
            ::data_encoding::BASE64.encode_display(data)
        )
    }
}

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

pub struct CertificateReferenceIdOrHash {
    pub id: Bytes,
    pub hash: CertificateHash,
}

#[cfg(target_os = "windows")]
pub use windows::show_certificate_selection_dialog;

// Mock module for unsupported platform
#[cfg(not(target_os = "windows"))]
mod platform {
    use crate::{
        CertificateDer, CertificateReference, GetDerEncodedCertificateError, SignMessageError,
        SignedMessage,
    };

    pub fn get_der_encoded_certificate(
        certificate_ref: &CertificateReference,
    ) -> Result<CertificateDer, GetDerEncodedCertificateError> {
        let _ = certificate_ref;
        unimplemented!("platform not supported")
    }

    pub fn sign_message(
        message: &[u8],
        certificate_ref: &CertificateReference,
    ) -> Result<SignedMessage, SignMessageError> {
        let _ = message;
        let _ = certificate_ref;
        unimplemented!("platform not supported")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetDerEncodedCertificateError {
    #[error("Cannot open certificate store: {}", .0)]
    CannotOpenStore(std::io::Error),
    #[error("Cannot find certificate")]
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
pub enum SignMessageError {
    #[error("Cannot open certificate store: {}", .0)]
    CannotOpenStore(std::io::Error),
    #[error("Cannot find certificate")]
    NotFound,
    #[error("Cannot get certificate info: {}", .0)]
    CannotGetCertificateInfo(std::io::Error),
    #[error("Cannot acquire keypair related to certificate: {}", .0)]
    CannotAcquireKeypair(std::io::Error),
    #[error("Cannot sign message: {}", .0)]
    CannotSign(std::io::Error),
}

pub struct SignedMessage {
    pub cert_ref: CertificateReferenceIdOrHash,
    pub signed_message: Bytes,
}

pub use platform::sign_message;
