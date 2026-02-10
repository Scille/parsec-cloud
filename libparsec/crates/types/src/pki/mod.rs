// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod cert_ref;

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use libparsec_crypto::{PublicKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    DataResult, DeviceID, DeviceLabel, UserID, UserProfile,
};
pub use cert_ref::{
    X509CertificateHash, X509CertificateReference, X509Pkcs11URI, X509URIFlavorValue,
    X509WindowsCngURI,
};

/*
 * PkiEnrollmentAnswerPayload
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "PkiEnrollmentAnswerPayloadData",
    from = "PkiEnrollmentAnswerPayloadData"
)]
pub struct PkiEnrollmentAnswerPayload {
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub device_label: DeviceLabel,
    pub profile: UserProfile,
    pub root_verify_key: VerifyKey,
}

parsec_data!("schema/pki/pki_enrollment_answer_payload.json5");

impl_transparent_data_format_conversion!(
    PkiEnrollmentAnswerPayload,
    PkiEnrollmentAnswerPayloadData,
    user_id,
    device_id,
    device_label,
    profile,
    root_verify_key,
);

impl PkiEnrollmentAnswerPayload {
    pub fn load(raw: &[u8]) -> DataResult<Self> {
        format_vx_load(raw)
    }
    pub fn dump(&self) -> Vec<u8> {
        format_v0_dump(&self)
    }
}

/*
 * PkiEnrollmentSubmitPayload
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "PkiEnrollmentSubmitPayloadData",
    from = "PkiEnrollmentSubmitPayloadData"
)]
pub struct PkiEnrollmentSubmitPayload {
    pub verify_key: VerifyKey,
    pub public_key: PublicKey,
    pub device_label: DeviceLabel,
}

parsec_data!("schema/pki/pki_enrollment_submit_payload.json5");

impl_transparent_data_format_conversion!(
    PkiEnrollmentSubmitPayload,
    PkiEnrollmentSubmitPayloadData,
    verify_key,
    public_key,
    device_label,
);

impl PkiEnrollmentSubmitPayload {
    pub fn load(raw: &[u8]) -> DataResult<Self> {
        format_vx_load(raw)
    }
    pub fn dump(&self) -> Vec<u8> {
        format_v0_dump(&self)
    }
}

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
