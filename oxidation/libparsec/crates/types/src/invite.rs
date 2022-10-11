// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::str::FromStr;

use libparsec_crypto::{PrivateKey, PublicKey, SecretKey, VerifyKey};
use serialization_format::parsec_data;

use crate as libparsec_types;
use crate::data_macros::impl_transparent_data_format_conversion;
use crate::ext_types::new_uuid_type;
use crate::{DeviceID, DeviceLabel, EntryID, HumanHandle, UserProfile};

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

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum InvitationStatus {
    Idle,
    Ready,
    Deleted,
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
 * InvitationToken
 */

new_uuid_type!(pub InvitationToken);

/*
 * SASCode
 */

// SAS code is composed of 4 hexadecimal characters
macro_rules! SAS_CODE_CHARS {
    () => {
        "ABCDEFGHJKLMNPQRSTUVWXYZ23456789"
    };
}
const SAS_CODE_CHARS: &str = SAS_CODE_CHARS!();
const SAS_CODE_PATTERN: &str = concat!("^[", SAS_CODE_CHARS!(), "]{4}$");
const SAS_CODE_LEN: usize = 4;
const SAS_CODE_BITS: u32 = 20;

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct SASCode(String);

impl std::fmt::Display for SASCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for SASCode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PATTERN: Regex =
                Regex::new(SAS_CODE_PATTERN).unwrap_or_else(|_| unreachable!());
        }
        if PATTERN.is_match(s) {
            Ok(Self(s.to_string()))
        } else {
            Err("Invalid SAS code")
        }
    }
}

impl TryFrom<u32> for SASCode {
    type Error = &'static str;
    fn try_from(mut num: u32) -> Result<SASCode, Self::Error> {
        let mut str = String::with_capacity(SAS_CODE_LEN);

        for _ in 0..SAS_CODE_LEN {
            let subcode = num % SAS_CODE_CHARS.len() as u32;
            let char = SAS_CODE_CHARS
                .chars()
                .nth(subcode as usize)
                .unwrap_or_else(|| unreachable!());
            str.push(char);
            num /= SAS_CODE_CHARS.len() as u32;
        }
        if num != 0 {
            Err("Provided integer is too large")
        } else {
            Ok(Self(str))
        }
    }
}

impl std::convert::AsRef<str> for SASCode {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<SASCode> for String {
    fn from(item: SASCode) -> String {
        item.0
    }
}

impl SASCode {
    pub fn generate_sas_code_candidates(&self, size: usize) -> Vec<SASCode> {
        if size == 0 {
            return vec![];
        }

        let mut sas_codes = Vec::<SASCode>::with_capacity(size);

        sas_codes.push(SASCode(self.to_string()));
        while sas_codes.len() < size {
            let num = rand::thread_rng().gen_range(0..(2u32.pow(SAS_CODE_BITS) - 1));
            let candidate = SASCode::try_from(num).unwrap_or_else(|_| unreachable!());
            if &candidate != self {
                sas_codes.push(candidate);
            }
        }
        sas_codes.shuffle(&mut rand::thread_rng());
        sas_codes
    }

    pub fn generate_sas_codes(
        claimer_nonce: &[u8],
        greeter_nonce: &[u8],
        shared_secret_key: &SecretKey,
    ) -> (SASCode, SASCode) {
        // Computes combined HMAC
        let mut combined_nonce = Vec::with_capacity(claimer_nonce.len() + greeter_nonce.len());
        combined_nonce.extend_from_slice(claimer_nonce);
        combined_nonce.extend_from_slice(greeter_nonce);

        // Digest size of 5 bytes so we can split it beween two 20bits SAS
        // Note we have to store is as a 8bytes array to be able to convert it into u64
        let mut combined_hmac = [0; 8];
        combined_hmac[3..8].clone_from_slice(&shared_secret_key.hmac(&combined_nonce, 5)[..]);
        let hmac_as_int = u64::from_be_bytes(combined_hmac);

        let claimer_part_as_int = (hmac_as_int % 2u64.pow(SAS_CODE_BITS) as u64) as u32;
        let greeter_part_as_int =
            ((hmac_as_int >> SAS_CODE_BITS) % 2u64.pow(SAS_CODE_BITS) as u64) as u32;

        // Big endian number extracted from bits [0, 20[
        let claimer_sas = SASCode::try_from(claimer_part_as_int).unwrap_or_else(|_| unreachable!());
        // Big endian number extracted from bits [20, 40[
        let greeter_sas = SASCode::try_from(greeter_part_as_int).unwrap_or_else(|_| unreachable!());

        (claimer_sas, greeter_sas)
    }
}

/*
 * Helpers
 */

macro_rules! impl_dump_and_encrypt {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_encrypt(&self, key: &::libparsec_crypto::SecretKey) -> Vec<u8> {
                let serialized =
                    ::rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
                let mut e =
                    ::flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                use std::io::Write;
                e.write_all(&serialized).unwrap_or_else(|_| unreachable!());
                let compressed = e.finish().unwrap_or_else(|_| unreachable!());
                key.encrypt(&compressed)
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
            ) -> Result<$name, &'static str> {
                let compressed = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
                let mut serialized = vec![];
                use std::io::Read;
                ::flate2::read::ZlibDecoder::new(&compressed[..])
                    .read_to_end(&mut serialized)
                    .map_err(|_| "Invalid compression")?;
                let obj: $name =
                    ::rmp_serde::from_slice(&serialized).map_err(|_| "Invalid serialization")?;
                Ok(obj)
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
    pub requested_device_label: Option<DeviceLabel>,
    pub requested_human_handle: Option<HumanHandle>,
    // Note claiming user also imply creating a first device
    pub public_key: PublicKey,
    pub verify_key: VerifyKey,
}

parsec_data!("schema/invite/invite_user_data.json");

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
    pub device_id: DeviceID,
    pub device_label: Option<DeviceLabel>,
    pub human_handle: Option<HumanHandle>,
    pub profile: UserProfile,
    pub root_verify_key: VerifyKey,
}

parsec_data!("schema/invite/invite_user_confirmation.json");

impl_dump_and_encrypt!(InviteUserConfirmation);
impl_decrypt_and_load!(InviteUserConfirmation);

impl_transparent_data_format_conversion!(
    InviteUserConfirmation,
    InviteUserConfirmationData,
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
    pub requested_device_label: Option<DeviceLabel>,
    pub verify_key: VerifyKey,
}

parsec_data!("schema/invite/invite_device_data.json");

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
    pub device_id: DeviceID,
    pub device_label: Option<DeviceLabel>,
    pub human_handle: Option<HumanHandle>,
    pub profile: UserProfile,
    pub private_key: PrivateKey,
    pub user_manifest_id: EntryID,
    pub user_manifest_key: SecretKey,
    pub root_verify_key: VerifyKey,
}

parsec_data!("schema/invite/invite_device_confirmation.json");

impl_dump_and_encrypt!(InviteDeviceConfirmation);
impl_decrypt_and_load!(InviteDeviceConfirmation);

impl_transparent_data_format_conversion!(
    InviteDeviceConfirmation,
    InviteDeviceConfirmationData,
    device_id,
    device_label,
    human_handle,
    profile,
    private_key,
    user_manifest_id,
    user_manifest_key,
    root_verify_key,
);
