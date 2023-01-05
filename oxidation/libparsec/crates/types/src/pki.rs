// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::io::{Read, Write};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use libparsec_crypto::{PublicKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, impl_transparent_data_format_conversion, DataError, DataResult,
    DeviceID, DeviceLabel, HumanHandle, UserProfile,
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
