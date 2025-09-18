// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub mod errors;
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
        CertificateDer, CertificateReference, EncryptMessageError, EncryptedMessage,
        GetDerEncodedCertificateError, SignMessageError, SignedMessage,
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

    pub fn encrypt_message(
        message: &[u8],
        certificate_ref: &CertificateReference,
    ) -> Result<EncryptedMessage, EncryptMessageError> {
        let _ = (message, certificate_ref);
        unimplemented!("platform not supported")
    }
}

pub use errors::GetDerEncodedCertificateError;
pub use platform::get_der_encoded_certificate;

pub struct CertificateDer {
    pub cert_ref: CertificateReferenceIdOrHash,
    pub der_content: Bytes,
}

pub struct SignedMessage {
    pub cert_ref: CertificateReferenceIdOrHash,
    pub signed_message: Bytes,
}

pub use errors::SignMessageError;
pub use platform::sign_message;

pub struct EncryptedMessage {
    pub cert_ref: CertificateReferenceIdOrHash,
    pub ciphered: Bytes,
}

pub use errors::EncryptMessageError;
pub use platform::encrypt_message;
