// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::io::{Read, Write};

use flate2::{read::ZlibDecoder, write::ZlibEncoder};

use crate::{self as libparsec_types /*To use parsec_data!*/, InvitationToken};
use crate::{data_macros::impl_transparent_data_format_conversion, DataError, DataResult};
use libparsec_crypto::{impl_key_debug, CryptoError, PrivateKey, PublicKey, SecretKey, VerifyKey};
use libparsec_serialization_format::parsec_data;
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "ShamirRecoverySecretData", from = "ShamirRecoverySecretData")]
pub struct ShamirRecoverySecret {
    pub data_key: SecretKey,
    pub reveal_token: InvitationToken,
}

parsec_data!("schema/shamir/shamir_recovery_secret.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoverySecret,
    ShamirRecoverySecretData,
    data_key,
    reveal_token,
);

impl ShamirRecoverySecret {
    pub fn dump(&self) -> DataResult<Vec<u8>> {
        ::rmp_serde::to_vec_named(self).map_err(|_| DataError::BadSerialization {
            format: None,
            step: "dump shamir recovery secret",
        })
    }

    pub fn load(buf: &[u8]) -> DataResult<Self> {
        ::rmp_serde::from_slice(buf).map_err(|_| DataError::BadSerialization {
            format: None,
            step: "load shamir recovery secret",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "ShamirRecoveryShareDataData",
    from = "ShamirRecoveryShareDataData"
)]
pub struct ShamirRecoveryShareData {
    pub weighted_share: Vec<ShamirShare>,
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
    ) -> DataResult<ShamirRecoveryShareData> {
        let signed = recipient_privkey.decrypt_from_self(ciphered)?;
        let compressed = author_verify_key.verify(&signed)?;
        let mut serialized = vec![];
        ZlibDecoder::new(compressed)
            .read_to_end(&mut serialized)
            .map_err(|_| DataError::BadSerialization {
                format: None,
                step: "compression shamir recovery share data",
            })?;
        let data: ShamirRecoveryShareData =
            rmp_serde::from_slice(&serialized).map_err(|_| DataError::BadSerialization {
                format: None,
                step: "shamir recovery share data",
            })?;
        Ok(data)
    }

    pub fn dump_and_encrypt_for(&self, recipient_pubkey: &PublicKey) -> Vec<u8> {
        let serialized = rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
        let mut e = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        e.write_all(&serialized).unwrap_or_else(|_| unreachable!());
        let compressed = e.finish().unwrap_or_else(|_| unreachable!());
        recipient_pubkey.encrypt_for_self(&compressed)
    }
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct ShamirShare(sharks::Share);

impl_key_debug!(ShamirShare);

impl ShamirShare {
    pub fn dump(&self) -> Vec<u8> {
        Vec::from(&self.0)
    }
}

impl PartialEq for ShamirShare {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

impl Eq for ShamirShare {}

impl TryFrom<&[u8]> for ShamirShare {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(
            sharks::Share::try_from(data).map_err(|_| CryptoError::DataSize)?,
        ))
    }
}

impl TryFrom<&Bytes> for ShamirShare {
    type Error = CryptoError;
    fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
    }
}

impl Serialize for ShamirShare {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.dump())
    }
}

pub fn shamir_make_shares(threshold: u8, secret: &[u8], shares: usize) -> Vec<ShamirShare> {
    let sharks = sharks::Sharks(threshold);
    sharks
        .dealer(secret)
        .map(ShamirShare)
        .take(shares)
        .collect()
}

pub fn shamir_recover_secret<'a, T>(threshold: u8, shares: T) -> Result<Vec<u8>, CryptoError>
where
    T: IntoIterator<Item = &'a ShamirShare>,
    T::IntoIter: Iterator<Item = &'a ShamirShare>,
{
    let sharks = sharks::Sharks(threshold);
    let it = shares.into_iter().map(|x| &x.0);
    sharks
        .recover(it)
        .map_err(|x| CryptoError::ShamirRecovery(x.to_string()))
}
