// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::*;

use libparsec_crypto::{PrivateKey, PublicKey, SecretKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types,
    data_macros::impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    DataError, DeviceID, DeviceLabel, HumanHandle, UserID, UserProfile, VlobID,
};

/*
 * InvitationType
 */

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InvitationType {
    User,
    Device,
}

/*
 * InvitationStatus
 */

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum InvitationStatus {
    Idle,
    Ready,
    Finished,
    Cancelled,
}

impl FromStr for InvitationType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "USER" => Ok(Self::User),
            "DEVICE" => Ok(Self::Device),
            _ => Err("Invalid InvitationType"),
        }
    }
}

impl ToString for InvitationType {
    fn to_string(&self) -> String {
        match self {
            Self::User => String::from("USER"),
            Self::Device => String::from("DEVICE"),
        }
    }
}

/*
 * Helpers
 */

macro_rules! impl_dump_and_encrypt {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_encrypt(&self, key: &::libparsec_crypto::SecretKey) -> Vec<u8> {
                let serialized = format_v0_dump(&self);
                key.encrypt(&serialized)
            }
        }
    };
}

macro_rules! impl_decrypt_and_load {
    ($name:ident) => {
        impl $name {
            pub fn decrypt_and_load(
                encrypted: &[u8],
                key: &::libparsec_crypto::SecretKey,
            ) -> Result<$name, DataError> {
                let serialized = key.decrypt(encrypted).map_err(|_| DataError::Decryption)?;
                format_vx_load(&serialized)
            }
        }
    };
}

/*
 * InviteUserData
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "InviteUserDataData", from = "InviteUserDataData")]
pub struct InviteUserData {
    // Claimer ask for device_label/human_handle, but greeter has final word on this
    pub requested_device_label: DeviceLabel,
    pub requested_human_handle: HumanHandle,
    // Note claiming user also imply creating a first device
    pub public_key: PublicKey,
    pub verify_key: VerifyKey,
}

parsec_data!("schema/invite/invite_user_data.json5");

impl_dump_and_encrypt!(InviteUserData);
impl_decrypt_and_load!(InviteUserData);

impl_transparent_data_format_conversion!(
    InviteUserData,
    InviteUserDataData,
    requested_device_label,
    requested_human_handle,
    public_key,
    verify_key,
);

/*
 * InviteUserConfirmation
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "InviteUserConfirmationData",
    from = "InviteUserConfirmationData"
)]
pub struct InviteUserConfirmation {
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub device_label: DeviceLabel,
    pub human_handle: HumanHandle,
    pub profile: UserProfile,
    pub root_verify_key: VerifyKey,
}

parsec_data!("schema/invite/invite_user_confirmation.json5");

impl_dump_and_encrypt!(InviteUserConfirmation);
impl_decrypt_and_load!(InviteUserConfirmation);

impl_transparent_data_format_conversion!(
    InviteUserConfirmation,
    InviteUserConfirmationData,
    user_id,
    device_id,
    device_label,
    human_handle,
    profile,
    root_verify_key,
);

/*
 * InviteDeviceData
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "InviteDeviceDataData", from = "InviteDeviceDataData")]
pub struct InviteDeviceData {
    pub requested_device_label: DeviceLabel,
    pub verify_key: VerifyKey,
}

parsec_data!("schema/invite/invite_device_data.json5");

impl_dump_and_encrypt!(InviteDeviceData);
impl_decrypt_and_load!(InviteDeviceData);

impl_transparent_data_format_conversion!(
    InviteDeviceData,
    InviteDeviceDataData,
    requested_device_label,
    verify_key,
);

/*
 * InviteDeviceConfirmation
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "InviteDeviceConfirmationData",
    from = "InviteDeviceConfirmationData"
)]
pub struct InviteDeviceConfirmation {
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub device_label: DeviceLabel,
    pub human_handle: HumanHandle,
    pub profile: UserProfile,
    pub private_key: PrivateKey,
    pub user_realm_id: VlobID,
    pub user_realm_key: SecretKey,
    pub root_verify_key: VerifyKey,
}

parsec_data!("schema/invite/invite_device_confirmation.json5");

impl_dump_and_encrypt!(InviteDeviceConfirmation);
impl_decrypt_and_load!(InviteDeviceConfirmation);

impl_transparent_data_format_conversion!(
    InviteDeviceConfirmation,
    InviteDeviceConfirmationData,
    user_id,
    device_id,
    device_label,
    human_handle,
    profile,
    private_key,
    user_realm_id,
    user_realm_key,
    root_verify_key,
);

#[cfg(test)]
#[path = "../tests/unit/invite.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
