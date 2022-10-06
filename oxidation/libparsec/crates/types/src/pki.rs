// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::{Deserialize, Serialize};

use libparsec_crypto::{PublicKey, VerifyKey};
use serialization_format::parsec_data;

use crate::{
    self as libparsec_types, impl_transparent_data_format_conversion, DeviceID, DeviceLabel,
    HumanHandle, UserProfile,
};

fn load<'a, T: Deserialize<'a>>(raw: &'a [u8]) -> Result<T, rmp_serde::decode::Error> {
    rmp_serde::from_slice(raw)
}

fn dump<T: Serialize>(data: &T) -> Vec<u8> {
    rmp_serde::to_vec_named(data).expect("Unreachable")
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
    pub fn load(raw: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
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
    pub fn load(raw: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        load(raw)
    }
    pub fn dump(&self) -> Vec<u8> {
        dump(&self)
    }
}
