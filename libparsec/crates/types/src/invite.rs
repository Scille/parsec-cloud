// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::*;

use libparsec_serialization_format::parsec_data;

use crate::{self as libparsec_types, DataError};

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
                let serialized =
                    ::rmp_serde::to_vec_named(&self).expect("object should be serializable");
                let mut e =
                    ::flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                use std::io::Write;
                let compressed = e
                    .write_all(&serialized)
                    .and_then(|_| e.finish())
                    .expect("in-memory buffer should not fail");
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
            ) -> Result<$name, DataError> {
                let compressed = key.decrypt(encrypted).map_err(|_| DataError::Decryption)?;
                let mut serialized = vec![];
                use std::io::Read;
                ::flate2::read::ZlibDecoder::new(&compressed[..])
                    .read_to_end(&mut serialized)
                    .map_err(|_| DataError::Compression)?;
                let obj: $name =
                    ::rmp_serde::from_slice(&serialized).map_err(|_| DataError::Serialization)?;
                Ok(obj)
            }
        }
    };
}

/*
 * Invitate exchange schemas
 */

parsec_data!("schema/invite/invite_0_wait_peer.json5");
pub type Invite0WaitPeer = Invite0WaitPeerData;
impl_dump_and_encrypt!(Invite0WaitPeer);
impl_decrypt_and_load!(Invite0WaitPeer);

parsec_data!("schema/invite/invite_1_claimer_send_hashed_nonce.json5");
pub type Invite1ClaimerSendHashedNonce = Invite1ClaimerSendHashedNonceData;
impl_dump_and_encrypt!(Invite1ClaimerSendHashedNonce);
impl_decrypt_and_load!(Invite1ClaimerSendHashedNonce);

parsec_data!("schema/invite/invite_2_greeter_send_nonce.json5");
pub type Invite2GreeterSendNonce = Invite2GreeterSendNonceData;
impl_dump_and_encrypt!(Invite2GreeterSendNonce);
impl_decrypt_and_load!(Invite2GreeterSendNonce);

parsec_data!("schema/invite/invite_3_claimer_send_nonce.json5");
pub type Invite3ClaimerSendNonce = Invite3ClaimerSendNonceData;
impl_dump_and_encrypt!(Invite3ClaimerSendNonce);
impl_decrypt_and_load!(Invite3ClaimerSendNonce);

parsec_data!("schema/invite/invite_4_claimer_signify_trust.json5");
pub type Invite4ClaimerSignifyTrust = Invite4ClaimerSignifyTrustData;
impl_dump_and_encrypt!(Invite4ClaimerSignifyTrust);
impl_decrypt_and_load!(Invite4ClaimerSignifyTrust);

parsec_data!("schema/invite/invite_5_greeter_signify_trust.json5");
pub type Invite5GreeterSignifyTrust = Invite5GreeterSignifyTrustData;
impl_dump_and_encrypt!(Invite5GreeterSignifyTrust);
impl_decrypt_and_load!(Invite5GreeterSignifyTrust);

parsec_data!("schema/invite/invite_6_claimer_request_device.json5");
pub type Invite6ClaimerRequestDevice = Invite6ClaimerRequestDeviceData;
impl_dump_and_encrypt!(Invite6ClaimerRequestDevice);
impl_decrypt_and_load!(Invite6ClaimerRequestDevice);

parsec_data!("schema/invite/invite_6_claimer_request_user.json5");
pub type Invite6ClaimerRequestUser = Invite6ClaimerRequestUserData;
impl_dump_and_encrypt!(Invite6ClaimerRequestUser);
impl_decrypt_and_load!(Invite6ClaimerRequestUser);

parsec_data!("schema/invite/invite_7_greeter_confirm_device.json5");
pub type Invite7GreeterConfirmDevice = Invite7GreeterConfirmDeviceData;
impl_dump_and_encrypt!(Invite7GreeterConfirmDevice);
impl_decrypt_and_load!(Invite7GreeterConfirmDevice);

parsec_data!("schema/invite/invite_7_greeter_confirm_user.json5");
pub type Invite7GreeterConfirmUser = Invite7GreeterConfirmUserData;
impl_dump_and_encrypt!(Invite7GreeterConfirmUser);
impl_decrypt_and_load!(Invite7GreeterConfirmUser);

#[cfg(test)]
#[path = "../tests/unit/invite.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
