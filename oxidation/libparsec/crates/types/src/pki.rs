// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use libparsec_crypto::{PublicKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, impl_transparent_data_format_conversion, BackendPkiEnrollmentAddr,
    DataError, DataResult, DateTime, DeviceID, DeviceLabel, EnrollmentID, HumanHandle,
    PkiEnrollmentLocalPendingError, PkiEnrollmentLocalPendingResult, UserProfile,
};

fn load<T: DeserializeOwned>(raw: &[u8]) -> DataResult<T> {
    let mut decompressed = vec![];

    ZlibDecoder::new(raw)
        .read_to_end(&mut decompressed)
        .map_err(|_| DataError::Compression)?;

    rmp_serde::from_slice(&decompressed).map_err(|_| Box::new(DataError::Serialization))
}

fn dump<T: Serialize>(data: &T) -> Vec<u8> {
    let serialized = rmp_serde::to_vec_named(data).expect("Unreachable");

    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&serialized).expect("Unreachable");

    e.finish().expect("Unreachable")
}

/*
 * PkiEnrollmentAnswerPayload
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "PkiEnrollmentAnswerPayloadData",
    from = "PkiEnrollmentAnswerPayloadData"
)]
pub struct PkiEnrollmentAnswerPayload {
    pub device_id: DeviceID,
    pub device_label: Option<DeviceLabel>,
    pub human_handle: Option<HumanHandle>,
    pub profile: UserProfile,
    pub root_verify_key: VerifyKey,
}

parsec_data!("schema/pki/pki_enrollment_answer_payload.json");

impl_transparent_data_format_conversion!(
    PkiEnrollmentAnswerPayload,
    PkiEnrollmentAnswerPayloadData,
    device_id,
    device_label,
    human_handle,
    profile,
    root_verify_key,
);

impl PkiEnrollmentAnswerPayload {
    pub fn load(raw: &[u8]) -> DataResult<Self> {
        load(raw)
    }
    pub fn dump(&self) -> Vec<u8> {
        dump(&self)
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

parsec_data!("schema/pki/pki_enrollment_submit_payload.json");

impl_transparent_data_format_conversion!(
    PkiEnrollmentSubmitPayload,
    PkiEnrollmentSubmitPayloadData,
    verify_key,
    public_key,
    requested_device_label,
);

impl PkiEnrollmentSubmitPayload {
    pub fn load(raw: &[u8]) -> DataResult<Self> {
        load(raw)
    }
    pub fn dump(&self) -> Vec<u8> {
        dump(&self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "X509CertificateData", from = "X509CertificateData")]
pub struct X509Certificate {
    pub issuer: HashMap<String, String>,
    pub subject: HashMap<String, String>,
    pub der_x509_certificate: Vec<u8>,
    pub certificate_sha1: Vec<u8>,
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

parsec_data!("schema/pki/x509_certificate.json");

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
    from = "LocalPendingEnrollmentData"
)]
pub struct LocalPendingEnrollment {
    pub x509_certificate: X509Certificate,
    pub addr: BackendPkiEnrollmentAddr,
    pub submitted_on: DateTime,
    pub enrollment_id: EnrollmentID,
    pub submit_payload: PkiEnrollmentSubmitPayload,
    pub encrypted_key: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

impl LocalPendingEnrollment {
    const DIRECTORY_NAME: &str = "enrollment_requests";

    fn path_from_enrollment_id(config_dir: &Path, enrollment_id: EnrollmentID) -> PathBuf {
        config_dir
            .join(Self::DIRECTORY_NAME)
            .join(enrollment_id.hex())
    }

    pub fn load(raw: &[u8]) -> DataResult<Self> {
        rmp_serde::from_slice(raw).map_err(|_| Box::new(DataError::Serialization))
    }

    pub fn dump(&self) -> Vec<u8> {
        rmp_serde::to_vec_named(&self).expect("Unreachable")
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
        Self::load(&data)
            .map_err(|exc| Box::new(PkiEnrollmentLocalPendingError::Validation { exc: *exc }))
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
        std::fs::remove_file(&path).map_err(|e| {
            Box::new(PkiEnrollmentLocalPendingError::CannotRemove {
                path,
                exc: e.to_string(),
            })
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

parsec_data!("schema/pki/local_pending_enrollment.json");

impl_transparent_data_format_conversion!(
    LocalPendingEnrollment,
    LocalPendingEnrollmentData,
    x509_certificate,
    addr,
    submitted_on,
    enrollment_id,
    submit_payload,
    encrypted_key,
    ciphertext,
);
