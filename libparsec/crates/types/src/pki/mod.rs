// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod cert_ref;

use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use bytes::Bytes;
use serde::{Deserialize, Serialize};

use libparsec_crypto::{PublicKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    DataResult, DateTime, DeviceID, DeviceLabel, EnrollmentID, HumanHandle, ParsecAddr,
    ParsecPkiEnrollmentAddr, PkiEnrollmentLocalPendingError, PkiEnrollmentLocalPendingResult,
    UserID, UserProfile,
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
    pub human_handle: HumanHandle,
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
    human_handle,
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
    pub human_handle: HumanHandle,
}

parsec_data!("schema/pki/pki_enrollment_submit_payload.json5");

impl_transparent_data_format_conversion!(
    PkiEnrollmentSubmitPayload,
    PkiEnrollmentSubmitPayloadData,
    verify_key,
    public_key,
    device_label,
    human_handle,
);

impl PkiEnrollmentSubmitPayload {
    pub fn load(raw: &[u8]) -> DataResult<Self> {
        format_vx_load(raw)
    }
    pub fn dump(&self) -> Vec<u8> {
        format_v0_dump(&self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "X509CertificateData", from = "X509CertificateData")]
pub struct X509Certificate {
    pub issuer: HashMap<String, String>,
    pub subject: HashMap<String, String>,
    pub der_x509_certificate: Bytes,
    pub certificate_sha1: Bytes,
    pub certificate_id: Option<String>,
}

impl X509Certificate {
    /// Certificates that are received from another peer are not available locally.
    pub fn is_available_locally(&self) -> bool {
        self.certificate_id.is_some()
    }

    pub fn subject_common_name(&self) -> Option<&String> {
        self.subject.get("common_name")
    }

    pub fn subject_email_address(&self) -> Option<&String> {
        self.subject.get("email_address")
    }

    pub fn issuer_common_name(&self) -> Option<&String> {
        self.issuer.get("common_name")
    }
}

parsec_data!("schema/pki/x509_certificate.json5");

impl_transparent_data_format_conversion!(
    X509Certificate,
    X509CertificateData,
    issuer,
    subject,
    der_x509_certificate,
    certificate_sha1,
    certificate_id,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "LocalPendingEnrollmentData",
    try_from = "LocalPendingEnrollmentData"
)]
pub struct LocalPendingEnrollment {
    pub cert_ref: X509CertificateReference,
    pub addr: ParsecPkiEnrollmentAddr,
    pub submitted_on: DateTime,
    pub enrollment_id: EnrollmentID,
    pub payload: PkiEnrollmentSubmitPayload,
    pub encrypted_key: Bytes,
    pub encrypted_key_algo: EncryptionAlgorithm,
    pub ciphertext: Bytes,
}

impl LocalPendingEnrollment {
    const DIRECTORY_NAME: &'static str = "enrollment_requests";

    fn path_from_enrollment_id(config_dir: &Path, enrollment_id: EnrollmentID) -> PathBuf {
        config_dir
            .join(Self::DIRECTORY_NAME)
            .join(enrollment_id.hex())
    }

    pub fn load(raw: &[u8]) -> DataResult<Self> {
        format_vx_load(raw)
    }

    pub fn dump(&self) -> Vec<u8> {
        format_v0_dump(&self)
    }

    pub fn save(&self, config_dir: &Path) -> PkiEnrollmentLocalPendingResult<PathBuf> {
        let path = Self::path_from_enrollment_id(config_dir, self.enrollment_id);
        let parent = path.parent().expect("Unreachable");
        std::fs::create_dir_all(parent).map_err(|e| {
            PkiEnrollmentLocalPendingError::CannotSave {
                path: path.clone(),
                exc: e.to_string(),
            }
        })?;
        std::fs::write(&path, self.dump()).map_err(|e| {
            PkiEnrollmentLocalPendingError::CannotSave {
                path: path.clone(),
                exc: e.to_string(),
            }
        })?;

        Ok(path)
    }

    pub fn load_from_path(path: &Path) -> PkiEnrollmentLocalPendingResult<Self> {
        let data = std::fs::read(path).map_err(|e| PkiEnrollmentLocalPendingError::CannotRead {
            path: path.to_path_buf(),
            exc: e.to_string(),
        })?;
        Self::load(&data).map_err(|exc| PkiEnrollmentLocalPendingError::Validation { exc })
    }

    pub fn load_from_enrollment_id(
        config_dir: &Path,
        enrollment_id: EnrollmentID,
    ) -> PkiEnrollmentLocalPendingResult<Self> {
        let path = Self::path_from_enrollment_id(config_dir, enrollment_id);
        Self::load_from_path(&path)
    }

    pub fn remove_from_enrollment_id(
        config_dir: &Path,
        enrollment_id: EnrollmentID,
    ) -> PkiEnrollmentLocalPendingResult<()> {
        let path = Self::path_from_enrollment_id(config_dir, enrollment_id);
        std::fs::remove_file(&path).map_err(|e| PkiEnrollmentLocalPendingError::CannotRemove {
            path,
            exc: e.to_string(),
        })
    }

    pub fn list(config_dir: &Path) -> Vec<Self> {
        config_dir
            .join(Self::DIRECTORY_NAME)
            .read_dir()
            .map(|read_dir| {
                read_dir
                    .filter_map(|entry| entry.ok())
                    .filter_map(|entry| Self::load_from_path(&entry.path()).ok())
                    .collect()
            })
            .unwrap_or_default()
    }
}

parsec_data!("schema/pki/local_pending_enrollment.json5");

impl TryFrom<LocalPendingEnrollmentData> for LocalPendingEnrollment {
    type Error = &'static str;

    fn try_from(data: LocalPendingEnrollmentData) -> Result<Self, Self::Error> {
        let addr = {
            let server_addr =
                ParsecAddr::from_http_url(&data.server_url).map_err(|_| "Invalid server URL")?;
            ParsecPkiEnrollmentAddr::new(server_addr, data.organization_id)
        };
        Ok(Self {
            addr,
            cert_ref: data.x509_certificate_ref,
            submitted_on: data.submitted_on,
            enrollment_id: data.enrollment_id,
            payload: data.payload,
            encrypted_key: data.encrypted_key,
            encrypted_key_algo: data.encrypted_key_algo,
            ciphertext: data.ciphertext,
        })
    }
}

impl From<LocalPendingEnrollment> for LocalPendingEnrollmentData {
    fn from(obj: LocalPendingEnrollment) -> Self {
        let server_url = {
            let server_addr = ParsecAddr::new(
                obj.addr.hostname().to_string(),
                Some(obj.addr.port()),
                obj.addr.use_ssl(),
            );
            server_addr.to_http_url(None).to_string()
        };
        Self {
            ty: Default::default(),
            server_url,
            organization_id: obj.addr.organization_id().clone(),
            x509_certificate_ref: obj.cert_ref,
            submitted_on: obj.submitted_on,
            enrollment_id: obj.enrollment_id,
            payload: obj.payload,
            encrypted_key: obj.encrypted_key,
            encrypted_key_algo: obj.encrypted_key_algo,
            ciphertext: obj.ciphertext,
        }
    }
}

impl PrivateParts {
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

#[cfg(test)]
#[path = "../../tests/unit/pki.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
