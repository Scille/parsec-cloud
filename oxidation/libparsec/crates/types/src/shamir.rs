// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;

use libparsec_crypto::{PrivateKey, PublicKey, SecretKey, SigningKey, VerifyKey};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Write},
    num::NonZeroU64,
};

use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, impl_transparent_data_format_conversion, DataError, DataResult,
    DateTime, DeviceID, ShamirRevealToken, UserID,
};

fn verify_and_load<T>(signed: &[u8], author_verify_key: &VerifyKey) -> DataResult<T>
where
    T: for<'a> Deserialize<'a>,
{
    let compressed = author_verify_key.verify(signed)?;
    let mut serialized = vec![];
    ZlibDecoder::new(&compressed[..])
        .read_to_end(&mut serialized)
        .map_err(|_| DataError::Compression)?;
    rmp_serde::from_slice(&serialized).map_err(|_| Box::new(DataError::Serialization))
}

macro_rules! impl_unsecure_load {
    ($name:ident) => {
        impl $name {
            pub fn unsecure_load(signed: &[u8]) -> DataResult<$name> {
                let compressed = VerifyKey::unsecure_unwrap(signed).ok_or(DataError::Signature)?;
                let mut serialized = vec![];
                ZlibDecoder::new(&compressed[..])
                    .read_to_end(&mut serialized)
                    .map_err(|_| DataError::Compression)?;
                ::rmp_serde::from_slice(&serialized).map_err(|_| Box::new(DataError::Serialization))
            }
        }
    };
}

macro_rules! impl_dump_and_sign {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
                let serialized =
                    ::rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
                let mut e = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                e.write_all(&serialized).unwrap_or_else(|_| unreachable!());
                let compressed = e.finish().unwrap_or_else(|_| unreachable!());
                author_signkey.sign(&compressed)
            }
        }
    };
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

impl_unsecure_load!(ShamirRecoveryBriefCertificate);
impl_dump_and_sign!(ShamirRecoveryBriefCertificate);

impl ShamirRecoveryBriefCertificate {
    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
    ) -> DataResult<Self> {
        let r = verify_and_load::<Self>(signed, author_verify_key)?;

        if &r.author != expected_author {
            return Err(Box::new(DataError::UnexpectedAuthor {
                expected: expected_author.clone(),
                got: Some(r.author),
            }));
        }
        Ok(r)
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

impl_unsecure_load!(ShamirRecoveryShareCertificate);
impl_dump_and_sign!(ShamirRecoveryShareCertificate);

impl ShamirRecoveryShareCertificate {
    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
    ) -> DataResult<Self> {
        let r = verify_and_load::<Self>(signed, author_verify_key)?;

        if &r.author != expected_author {
            return Err(Box::new(DataError::UnexpectedAuthor {
                expected: expected_author.clone(),
                got: Some(r.author),
            }));
        }
        Ok(r)
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
    pub weighted_share: Vec<Vec<u8>>,
}

parsec_data!("schema/shamir/shamir_recovery_share_data.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoveryShareData,
    ShamirRecoveryShareDataData,
    weighted_share,
);

impl ShamirRecoveryShareData {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "ShamirRecoveryCommunicatedDataData",
    from = "ShamirRecoveryCommunicatedDataData"
)]
pub struct ShamirRecoveryCommunicatedData {
    pub weighted_share: Vec<Vec<u8>>,
}

parsec_data!("schema/shamir/shamir_recovery_communicated_data.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoveryCommunicatedData,
    ShamirRecoveryCommunicatedDataData,
    weighted_share,
);

impl ShamirRecoveryCommunicatedData {
    pub fn dump(&self) -> Result<Vec<u8>, &'static str> {
        ::rmp_serde::to_vec_named(self).map_err(|_| "Serialization failed")
    }

    pub fn load(buf: &[u8]) -> Result<Self, &'static str> {
        ::rmp_serde::from_slice(buf).map_err(|_| "Deserialization failed")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "ShamirRecoverySecretData", from = "ShamirRecoverySecretData")]
pub struct ShamirRecoverySecret {
    pub data_key: SecretKey,
    pub reveal_token: ShamirRevealToken,
}

parsec_data!("schema/shamir/shamir_recovery_secret.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoverySecret,
    ShamirRecoverySecretData,
    data_key,
    reveal_token,
);

impl ShamirRecoverySecret {
    pub fn dump(&self) -> Result<Vec<u8>, &'static str> {
        ::rmp_serde::to_vec_named(self).map_err(|_| "Serialization failed")
    }

    pub fn load(buf: &[u8]) -> Result<Self, &'static str> {
        ::rmp_serde::from_slice(buf).map_err(|_| "Deserialization failed")
    }
}
