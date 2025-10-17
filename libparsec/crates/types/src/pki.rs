// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Context;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

use libparsec_crypto::{PublicKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    DataError, DataResult, DateTime, DeviceID, DeviceLabel, EnrollmentID, HumanHandle, ParsecAddr,
    ParsecPkiEnrollmentAddr, PkiEnrollmentLocalPendingError, PkiEnrollmentLocalPendingResult,
    UserID, UserProfile,
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
    pub requested_device_label: DeviceLabel,
}

parsec_data!("schema/pki/pki_enrollment_submit_payload.json5");

impl_transparent_data_format_conversion!(
    PkiEnrollmentSubmitPayload,
    PkiEnrollmentSubmitPayloadData,
    verify_key,
    public_key,
    requested_device_label,
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
    pub x509_certificate: X509Certificate,
    pub addr: ParsecPkiEnrollmentAddr,
    pub submitted_on: DateTime,
    pub enrollment_id: EnrollmentID,
    pub submit_payload: PkiEnrollmentSubmitPayload,
    pub encrypted_key: Bytes,
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
            x509_certificate: data.x509_certificate,
            submitted_on: data.submitted_on,
            enrollment_id: data.enrollment_id,
            submit_payload: data.submit_payload,
            encrypted_key: data.encrypted_key,
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
            x509_certificate: obj.x509_certificate,
            submitted_on: obj.submitted_on,
            enrollment_id: obj.enrollment_id,
            submit_payload: obj.submit_payload,
            encrypted_key: obj.encrypted_key,
            ciphertext: obj.ciphertext,
        }
    }
}

#[derive(
    Clone, Eq, PartialEq, serde_with::DeserializeFromStr, serde_with::SerializeDisplay, Debug,
)]
pub enum X509CertificateHash {
    SHA256(Box<[u8; 32]>),
}

impl X509CertificateHash {
    pub fn fake_sha256() -> Self {
        Self::SHA256(Default::default())
    }
}

impl std::fmt::Display for X509CertificateHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (hash_str, data) = match self {
            X509CertificateHash::SHA256(data) => ("sha256", data.as_ref()),
        };
        write!(
            f,
            "{hash_str}-{}",
            ::data_encoding::BASE64.encode_display(data)
        )
    }
}

impl FromStr for X509CertificateHash {
    type Err = DataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hash_ty, b64_hash) = s.split_once('-').ok_or(DataError::BadSerialization {
            format: None,
            step: "Missing `-` delimiter",
        })?;
        let raw_data = data_encoding::BASE64
            .decode(b64_hash.as_bytes())
            .map_err(|_| DataError::BadSerialization {
                format: None,
                step: "error decoding hash",
            })?;
        if hash_ty.eq_ignore_ascii_case("sha256") {
            Ok(X509CertificateHash::SHA256(raw_data.try_into().map_err(
                |_| DataError::BadSerialization {
                    format: None,
                    step: "Invalid data size",
                },
            )?))
        } else {
            Err(DataError::BadSerialization {
                format: None,
                step: "Unsupported hash type ",
            })
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct X509CertificateReference {
    uris: Vec<X509URIFlavorValue>,
    pub hash: X509CertificateHash,
}

impl X509CertificateReference {
    pub fn with_uri(mut self, new_uri: X509URIFlavorValue) -> Self {
        match self
            .uris
            .binary_search_by(|uri| uri.flavor().cmp(&new_uri.flavor()))
        {
            // A URI already use that flavor, replacing it.
            Ok(pos) => self.uris[pos] = new_uri,
            // No URIs use that flavor, inserting so the list is keep sorted.
            Err(pos) => self.uris.insert(pos, new_uri),
        }

        self
    }

    pub fn uris(&self) -> impl Iterator<Item = &X509URIFlavorValue> {
        self.uris.iter()
    }

    pub fn get_uri<Flavor: X509URIFlavor>(&self, flavor: X509URIFlavor) -> Option<&Flavor> {
        self.uris.iter().find_map(|uri| Flavor::may_unwrap(uri))
    }
}

trait X509URIFlavor {
    fn may_unwrap(wrapper: X509URIFlavorValue) -> Option<Self>;
}

impl From<X509CertificateHash> for X509CertificateReference {
    fn from(hash: X509CertificateHash) -> Self {
        Self {
            uris: Vec::new(),
            hash,
        }
    }
}

pub struct X509WindowCngURI(Bytes);

impl X509URIFlavor for X509WindowCngURI {
    fn may_unwrap(wrapper: X509URIFlavorValue) -> Option<Self> {
        match wrapper {
            X509URIFlavorValue::WindowsCNG(uri) => Some(uri),
            _ => None,
        }
    }
}

impl AsRef<[u8]> for X509WindowCngURI {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

pub struct X509Pkcs11URI(
    // TODO: See if we store raw pkcs11 attribute in a list serialized in msgpack
    // Or a string representing a valid pkcs11-URI (following RFC-7512)
    Bytes,
);

impl X509URIFlavor for X509Pkcs11URI {
    fn may_unwrap(wrapper: X509URIFlavorValue) -> Option<Self> {
        match wrapper {
            X509URIFlavorValue::PKCS11(uri) => Some(uri),
            _ => None,
        }
    }
}

#[derive(
    Debug, serde_with::DeserializeFromStr, serde_with::SerializeDisplay, PartialEq, Eq, Clone,
)]
pub enum X509URIFlavorValue {
    WindowsCNG(X509WindowCngURI),
    PKCS11(X509Pkcs11URI),
}

impl Display for X509URIFlavorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let encoder = data_encoding::BASE64;
        let (header, value) = match self {
            Self::WindowsCNG(data) => ("windows-cng", encoder.encode_display(data)),
            Self::PKCS11(data) => ("pkcs11", encoder.encode_display(data)),
        };
        write!(f, "{header}:{value}")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum X509URIFlavorValueParsingError {
    #[error("missing delimiter")]
    MissingDelimiter,
    #[error("unknown flavor `{0}`")]
    UnknownFlavor(String),
    #[error("invalid encoding: {0}")]
    InvalidEncoding(#[from] anyhow::Error),
}

impl FromStr for X509URIFlavorValue {
    type Err = X509URIFlavorValueParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((header, value)) = s.split_once(':') else {
            return Err(X509URIFlavorValueParsingError::MissingDelimiter);
        };
        if header.eq_ignore_ascii_case("pkcs11") {
            let val = data_encoding::BASE64
                .decode(value.as_bytes())
                .context("Invalid pkcs11 value encoding")?;
            Ok(Self::PKCS11(val.into()))
        } else if header.eq_ignore_ascii_case("windows-cng") {
            let val = data_encoding::BASE64
                .decode(value.as_bytes())
                .context("Invalid windows CNG value encoding")?;
            Ok(Self::WindowsCNG(val.into()))
        } else {
            Err(X509URIFlavorValueParsingError::UnknownFlavor(
                header.to_string(),
            ))
        }
    }
}

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

#[cfg(test)]
#[path = "../tests/unit/pki.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
