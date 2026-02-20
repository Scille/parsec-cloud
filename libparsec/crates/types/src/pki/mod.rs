// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod cert_ref;

use std::{fmt::Display, str::FromStr};

pub use cert_ref::{
    X509CertificateHash, X509CertificateReference, X509Pkcs11URI, X509URIFlavorValue,
    X509WindowsCngURI,
};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde_with::DeserializeFromStr, serde_with::SerializeDisplay,
)]
pub enum PKIEncryptionAlgorithm {
    RsaesOaepSha256,
}

impl From<PKIEncryptionAlgorithm> for &'static str {
    fn from(value: PKIEncryptionAlgorithm) -> Self {
        match value {
            PKIEncryptionAlgorithm::RsaesOaepSha256 => "RSAES-OAEP-SHA256",
        }
    }
}

impl FromStr for PKIEncryptionAlgorithm {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RSAES-OAEP-SHA256" => Ok(Self::RsaesOaepSha256),
            _ => Err("Unknown encryption algorithm"),
        }
    }
}

impl Display for PKIEncryptionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str((*self).into())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    serde_with::DeserializeFromStr,
    serde_with::SerializeDisplay,
)]
pub enum PkiSignatureAlgorithm {
    RsassaPssSha256,
}

impl From<PkiSignatureAlgorithm> for &'static str {
    fn from(value: PkiSignatureAlgorithm) -> Self {
        match value {
            PkiSignatureAlgorithm::RsassaPssSha256 => "RSASSA-PSS-SHA256",
        }
    }
}

impl Display for PkiSignatureAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str((*self).into())
    }
}

impl FromStr for PkiSignatureAlgorithm {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RSASSA-PSS-SHA256" => Ok(Self::RsassaPssSha256),
            _ => Err("Unknown signature algorithm"),
        }
    }
}

#[cfg(test)]
#[path = "../../tests/unit/pki.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
