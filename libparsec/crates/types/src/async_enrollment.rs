// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use serde::{Deserialize, Serialize};

use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types,
    data_macros::impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    AsyncEnrollmentID, DataError, DateTime, DeviceID, DeviceLabel, HumanHandle, OrganizationID,
    ParsecAddr, PublicKey, UserID, UserProfile, VerifyKey,
};

/*
 * Helpers
 */

macro_rules! impl_dump {
    ($name:ident) => {
        impl $name {
            pub fn dump(&self) -> Vec<u8> {
                format_v0_dump(&self)
            }
        }
    };
}

macro_rules! impl_load {
    ($name:ident) => {
        impl $name {
            pub fn load(serialized: &[u8]) -> Result<$name, DataError> {
                format_vx_load(&serialized)
            }
        }
    };
}

/*
 * AsyncEnrollmentSubmitPayload
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "AsyncEnrollmentSubmitPayloadData",
    from = "AsyncEnrollmentSubmitPayloadData"
)]
pub struct AsyncEnrollmentSubmitPayload {
    pub verify_key: VerifyKey,
    pub public_key: PublicKey,
    pub requested_device_label: DeviceLabel,
    pub requested_human_handle: HumanHandle,
}

parsec_data!("schema/async_enrollment/async_enrollment_submit_payload.json5");

impl_transparent_data_format_conversion!(
    AsyncEnrollmentSubmitPayload,
    AsyncEnrollmentSubmitPayloadData,
    verify_key,
    public_key,
    requested_device_label,
    requested_human_handle,
);

impl_dump!(AsyncEnrollmentSubmitPayload);
impl_load!(AsyncEnrollmentSubmitPayload);

/*
 * AsyncEnrollmentAcceptPayload
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "AsyncEnrollmentAcceptPayloadData",
    from = "AsyncEnrollmentAcceptPayloadData"
)]
pub struct AsyncEnrollmentAcceptPayload {
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub device_label: DeviceLabel,
    pub human_handle: HumanHandle,
    pub profile: UserProfile,
    pub root_verify_key: VerifyKey,
}

parsec_data!("schema/async_enrollment/async_enrollment_accept_payload.json5");

impl_transparent_data_format_conversion!(
    AsyncEnrollmentAcceptPayload,
    AsyncEnrollmentAcceptPayloadData,
    user_id,
    device_id,
    device_label,
    human_handle,
    profile,
    root_verify_key,
);

impl_dump!(AsyncEnrollmentAcceptPayload);
impl_load!(AsyncEnrollmentAcceptPayload);

/*
 * AsyncEnrollmentLocalPending
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "AsyncEnrollmentLocalPendingData",
    from = "AsyncEnrollmentLocalPendingData"
)]
pub struct AsyncEnrollmentLocalPending {
    pub cleartext_content: Bytes,
    pub ciphertext_cleartext_content_digest: Bytes,
    pub ciphertext_signing_key: Bytes,
    pub ciphertext_private_key: Bytes,
}

parsec_data!("schema/async_enrollment/async_enrollment_local_pending.json5");

impl_transparent_data_format_conversion!(
    AsyncEnrollmentLocalPending,
    AsyncEnrollmentLocalPendingData,
    cleartext_content,
    ciphertext_cleartext_content_digest,
    ciphertext_signing_key,
    ciphertext_private_key,
);

impl_dump!(AsyncEnrollmentLocalPending);
impl_load!(AsyncEnrollmentLocalPending);

/*
 * AsyncEnrollmentLocalPendingCleartextContent
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "AsyncEnrollmentLocalPendingCleartextContentData",
    from = "AsyncEnrollmentLocalPendingCleartextContentData"
)]
pub struct AsyncEnrollmentLocalPendingCleartextContent {
    pub server_url: ParsecAddr,
    pub organization_id: OrganizationID,
    pub submitted_on: DateTime,
    pub enrollment_id: AsyncEnrollmentID,
    pub requested_device_label: DeviceLabel,
    pub requested_human_handle: HumanHandle,
    pub identity_system: AsyncEnrollmentLocalPendingIdentitySystem,
}

parsec_data!("schema/async_enrollment/async_enrollment_local_pending_cleartext_content.json5");

impl_transparent_data_format_conversion!(
    AsyncEnrollmentLocalPendingCleartextContent,
    AsyncEnrollmentLocalPendingCleartextContentData,
    server_url,
    organization_id,
    submitted_on,
    enrollment_id,
    requested_device_label,
    requested_human_handle,
    identity_system,
);

impl_dump!(AsyncEnrollmentLocalPendingCleartextContent);
impl_load!(AsyncEnrollmentLocalPendingCleartextContent);

#[cfg(test)]
#[path = "../tests/unit/async_enrollment.rs"]
mod tests;
