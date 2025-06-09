// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_with::*;

use libparsec_crypto::{PrivateKey, PublicKey, SecretKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types,
    data_macros::impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    DataError, DeviceID, DeviceLabel, HumanHandle, ShamirShare, UserID, UserProfile, VlobID,
};

/*
 * InvitationType
 */

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvitationType {
    User,
    Device,
    ShamirRecovery,
}

#[derive(Debug, Clone)]
pub struct InvitationTypeParseError;

impl std::error::Error for InvitationTypeParseError {}

impl std::fmt::Display for InvitationTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid InvitationType")
    }
}

impl FromStr for InvitationType {
    type Err = InvitationTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "USER" => Ok(Self::User),
            "DEVICE" => Ok(Self::Device),
            _ => Err(InvitationTypeParseError),
        }
    }
}

impl Display for InvitationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "USER"),
            Self::Device => write!(f, "DEVICE"),
            Self::ShamirRecovery => write!(f, "SHAMIR_RECOVERY"),
        }
    }
}

/*
 * InvitationStatus
 */

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvitationStatus {
    Pending,
    Finished,
    Cancelled,
}

/*
 * GreeterOrClaimer
 */

#[derive(Debug, Clone)]
pub struct GreeterOrClaimerParseError;

impl std::error::Error for GreeterOrClaimerParseError {}

impl std::fmt::Display for GreeterOrClaimerParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid GreeterOrClaimer")
    }
}

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GreeterOrClaimer {
    Greeter,
    Claimer,
}

impl FromStr for GreeterOrClaimer {
    type Err = GreeterOrClaimerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GREETER" => Ok(Self::Greeter),
            "CLAIMER" => Ok(Self::Claimer),
            _ => Err(GreeterOrClaimerParseError),
        }
    }
}

impl Display for GreeterOrClaimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Greeter => write!(f, "greeter"),
            Self::Claimer => write!(f, "claimer"),
        }
    }
}

/*
* Cancel greeting attempt reason
*/

#[derive(Debug, Clone)]
pub struct CancelledGreetingAttemptReasonParseError;

impl std::error::Error for CancelledGreetingAttemptReasonParseError {}

impl std::fmt::Display for CancelledGreetingAttemptReasonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid CancelledGreetingAttemptReason")
    }
}

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelledGreetingAttemptReason {
    /// the user manually cancelled the greeting attempt
    ManuallyCancelled,
    /// the hashed nonce didn't match the provided nonce
    InvalidNonceHash,
    /// the SAS code communicated to the user was invalid
    InvalidSasCode,
    /// the payload could not be deciphered
    UndecipherablePayload,
    /// the payload could not be deserialized
    UndeserializablePayload,
    /// the payload contained inconsistent information
    InconsistentPayload,
    /// the greeting attempt has been automatically cancelled by a new start_greeting_attempt command
    AutomaticallyCancelled,
}

impl FromStr for CancelledGreetingAttemptReason {
    type Err = CancelledGreetingAttemptReasonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "MANUALLY_CANCELLED" => Ok(Self::ManuallyCancelled),
            "INVALID_NONCE_HASH" => Ok(Self::InvalidNonceHash),
            "INVALID_SAS_CODE" => Ok(Self::InvalidSasCode),
            "UNDECIPHERABLE_PAYLOAD" => Ok(Self::UndecipherablePayload),
            "UNDESERIALIZABLE_PAYLOAD" => Ok(Self::UndeserializablePayload),
            "INCONSISTENT_PAYLOAD" => Ok(Self::InconsistentPayload),
            "AUTOMATICALLY_CANCELLED" => Ok(Self::AutomaticallyCancelled),
            _ => Err(CancelledGreetingAttemptReasonParseError),
        }
    }
}

impl std::fmt::Display for CancelledGreetingAttemptReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ManuallyCancelled => write!(f, "manually cancelled"),
            Self::InvalidNonceHash => write!(f, "invalid nonce hash"),
            Self::InvalidSasCode => write!(f, "invalid sas code"),
            Self::UndecipherablePayload => write!(f, "undecipherable payload"),
            Self::UndeserializablePayload => write!(f, "undeserializable payload"),
            Self::InconsistentPayload => write!(f, "inconsistent payload"),
            Self::AutomaticallyCancelled => write!(f, "automatically cancelled"),
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

/*
 * InviteShamirRecoveryData
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "InviteShamirRecoveryDataData",
    from = "InviteShamirRecoveryDataData"
)]
pub struct InviteShamirRecoveryData;

parsec_data!("schema/invite/invite_shamir_recovery_data.json5");

impl_dump_and_encrypt!(InviteShamirRecoveryData);
impl_decrypt_and_load!(InviteShamirRecoveryData);

impl_transparent_data_format_conversion!(InviteShamirRecoveryData, InviteShamirRecoveryDataData,);

/*
 * InviteDeviceConfirmation
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "InviteShamirRecoveryConfirmationData",
    from = "InviteShamirRecoveryConfirmationData"
)]
pub struct InviteShamirRecoveryConfirmation {
    pub weighted_share: Vec<ShamirShare>,
}

parsec_data!("schema/invite/invite_shamir_recovery_confirmation.json5");

impl_dump_and_encrypt!(InviteShamirRecoveryConfirmation);
impl_decrypt_and_load!(InviteShamirRecoveryConfirmation);

impl_transparent_data_format_conversion!(
    InviteShamirRecoveryConfirmation,
    InviteShamirRecoveryConfirmationData,
    weighted_share,
);

#[cfg(test)]
#[path = "../tests/unit/invite.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
