// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub mod errors;
mod shared;
#[cfg(target_os = "windows")]
mod windows;

use std::{fmt::Display, str::FromStr};

use bytes::Bytes;

#[cfg(target_os = "windows")]
pub(crate) use windows as platform;

#[derive(Clone)]
#[cfg_attr(not(feature = "hash-sri-display"), derive(Debug))]
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

#[cfg(feature = "hash-sri-display")]
impl std::fmt::Debug for CertificateHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
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
        CertificateDer, CertificateReference, DecryptMessageError, DecryptedMessage,
        EncryptMessageError, EncryptedMessage, EncryptionAlgorithm, GetDerEncodedCertificateError,
        SignMessageError, SignedMessageFromPki,
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
    ) -> Result<SignedMessageFromPki, SignMessageError> {
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

    pub fn decrypt_message(
        algo: EncryptionAlgorithm,
        encrypted_message: &[u8],
        certificate_ref: &CertificateReference,
    ) -> Result<DecryptedMessage, DecryptMessageError> {
        let _ = (algo, encrypted_message, certificate_ref);
        unimplemented!("platform not supported")
    }
}

pub use errors::GetDerEncodedCertificateError;
pub use platform::get_der_encoded_certificate;

pub struct CertificateDer {
    pub cert_ref: CertificateReferenceIdOrHash,
    pub der_content: Bytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureAlgorithm {
    RsassaPssSha256,
}

impl From<SignatureAlgorithm> for &'static str {
    fn from(value: SignatureAlgorithm) -> Self {
        match value {
            SignatureAlgorithm::RsassaPssSha256 => "RSASSA-PSS-SHA256",
        }
    }
}

impl Display for SignatureAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str((*self).into())
    }
}

impl FromStr for SignatureAlgorithm {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RSASSA-PSS-SHA256" => Ok(Self::RsassaPssSha256),
            _ => Err("Unknown signature algorithm"),
        }
    }
}

pub struct SignedMessageFromPki {
    pub algo: SignatureAlgorithm,
    pub cert_ref: CertificateReferenceIdOrHash,
    pub signature: Bytes,
}

pub use errors::SignMessageError;
pub use platform::sign_message;

pub use shared::{verify_message, Certificate, SignedMessage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    RsaesOaepSha256,
}

impl From<EncryptionAlgorithm> for &'static str {
    fn from(value: EncryptionAlgorithm) -> Self {
        match value {
            EncryptionAlgorithm::RsaesOaepSha256 => "RSAES-OAEP-SHA256",
        }
    }
}

impl FromStr for EncryptionAlgorithm {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RSAES-OAEP-SHA256" => Ok(Self::RsaesOaepSha256),
            _ => Err("Unknown encryption algorithm"),
        }
    }
}

impl Display for EncryptionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str((*self).into())
    }
}

pub struct EncryptedMessage {
    pub algo: EncryptionAlgorithm,
    pub cert_ref: CertificateReferenceIdOrHash,
    pub ciphered: Bytes,
}

pub use errors::EncryptMessageError;
pub use platform::encrypt_message;

pub struct DecryptedMessage {
    pub cert_ref: CertificateReferenceIdOrHash,
    pub data: Bytes,
}

pub use errors::DecryptMessageError;
pub use platform::decrypt_message;
