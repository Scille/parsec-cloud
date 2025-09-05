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

impl AsRef<[u8]> for CertificateHash {
    fn as_ref(&self) -> &[u8] {
        match self {
            CertificateHash::SHA256(items) => items.as_ref(),
        }
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
    use crate::{CertificateDer, CertificateReference, GetDerEncodedCertificateError};
    pub fn get_der_encoded_certificate(
        certificate_ref: &CertificateReference,
    ) -> Result<CertificateDer, GetDerEncodedCertificateError> {
        let _ = certificate_ref;
        unimplemented!("platform not supported")
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
