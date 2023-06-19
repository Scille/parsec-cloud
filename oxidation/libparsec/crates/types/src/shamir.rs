// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;

use libparsec_crypto::{PrivateKey, PublicKey, SigningKey, VerifyKey};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Write},
    num::NonZeroU64,
};

use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, impl_transparent_data_format_conversion, DataError, DataResult,
    DateTime, DeviceID, UserID,
};

fn load<T: DeserializeOwned>(raw: &[u8]) -> DataResult<T> {
    rmp_serde::from_slice(raw).map_err(|_| Box::new(DataError::Serialization))
}

fn dump<T: Serialize>(data: &T) -> Vec<u8> {
    rmp_serde::to_vec_named(data).expect("Unreachable")
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "ShamirRecoveryBriefCertificateData",
    from = "ShamirRecoveryBriefCertificateData"
)]
pub struct ShamirRecoveryBriefCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,
    pub threshold: NonZeroU64,
    pub per_recipient_shares: HashMap<UserID, NonZeroU64>,
}

impl ShamirRecoveryBriefCertificate {
    pub fn load(raw: &[u8]) -> DataResult<Self> {
        load(raw)
    }
    pub fn dump(&self) -> Vec<u8> {
        dump(self)
    }
}

parsec_data!("schema/shamir/shamir_recovery_brief_certificate.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryBriefCertificateData,
    author,
    timestamp,
    threshold,
    per_recipient_shares,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "ShamirRecoveryShareCertificateData",
    from = "ShamirRecoveryShareCertificateData"
)]
pub struct ShamirRecoveryShareCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,
    pub recipient: UserID,
    pub ciphered_share: Vec<u8>,
}

impl ShamirRecoveryShareCertificate {
    pub fn load(raw: &[u8]) -> DataResult<Self> {
        load(raw)
    }
    pub fn dump(&self) -> Vec<u8> {
        dump(self)
    }
}

parsec_data!("schema/shamir/shamir_recovery_share_certificate.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoveryShareCertificate,
    ShamirRecoveryShareCertificateData,
    author,
    timestamp,
    recipient,
    ciphered_share,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "ShamirRecoveryShareDataData",
    from = "ShamirRecoveryShareDataData"
)]
pub struct ShamirRecoveryShareData {
    pub reveal_token_share: Vec<u8>,
    pub data_key_share: Vec<u8>,
}

parsec_data!("schema/shamir/shamir_recovery_share_data.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoveryShareData,
    ShamirRecoveryShareDataData,
    reveal_token_share,
    data_key_share,
);

impl ShamirRecoveryShareData {
    pub fn dump(&self) -> Result<Vec<u8>, &'static str> {
        ::rmp_serde::to_vec_named(self).map_err(|_| "Serialization failed")
    }

    pub fn load(buf: &[u8]) -> Result<Self, &'static str> {
        ::rmp_serde::from_slice(buf).map_err(|_| "Deserialization failed")
    }

    pub fn decrypt_verify_and_load_for(
        ciphered: &[u8],
        recipient_privkey: &PrivateKey,
        author_verify_key: &VerifyKey,
    ) -> Result<ShamirRecoveryShareData, &'static str> {
        let signed = recipient_privkey
            .decrypt_from_self(ciphered)
            .map_err(|_| "Invalid encryption")?;
        let compressed = author_verify_key
            .verify(&signed)
            .map_err(|_| "Invalid signature")?;
        let mut serialized = vec![];
        ZlibDecoder::new(&compressed[..])
            .read_to_end(&mut serialized)
            .map_err(|_| "Invalid compression")?;
        let data: ShamirRecoveryShareData =
            rmp_serde::from_slice(&serialized).map_err(|_| "Invalid serialization")?;
        Ok(data)
    }

    pub fn dump_sign_and_encrypt_for(
        &self,
        author_signkey: &SigningKey,
        recipient_pubkey: &PublicKey,
    ) -> Vec<u8> {
        let serialized = rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
        let mut e = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        e.write_all(&serialized).unwrap_or_else(|_| unreachable!());
        let compressed = e.finish().unwrap_or_else(|_| unreachable!());
        let signed = author_signkey.sign(&compressed);
        recipient_pubkey.encrypt_for_self(&signed)
    }
}
