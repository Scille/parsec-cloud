// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::num::NonZeroU8;

use crate::{
    self as libparsec_types,
    data_macros::impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    AccessToken, DataError, DataResult, DateTime, DeviceID,
};
use libparsec_crypto::{
    impl_key_debug, CryptoError, CryptoResult, PrivateKey, PublicKey, SecretKey, SigningKey,
    VerifyKey,
};
use libparsec_serialization_format::parsec_data;
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "ShamirRecoverySecretData", from = "ShamirRecoverySecretData")]
pub struct ShamirRecoverySecret {
    pub data_key: SecretKey,
    pub reveal_token: AccessToken,
}

parsec_data!("schema/shamir/shamir_recovery_secret.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoverySecret,
    ShamirRecoverySecretData,
    data_key,
    reveal_token,
);

impl ShamirRecoverySecret {
    fn load(raw: &[u8]) -> DataResult<Self> {
        format_vx_load(raw)
    }

    fn dump(&self) -> Vec<u8> {
        format_v0_dump(&self)
    }

    pub fn dump_and_encrypt_into_shares(
        &self,
        threshold: NonZeroU8,
        shares: NonZeroU8,
    ) -> Vec<ShamirShare> {
        let secret = self.dump();

        let blahaj = blahaj::Sharks(threshold.get());
        blahaj
            .dealer(&secret)
            .map(ShamirShare)
            // Note, unlike what Sharks's documentation claims, at most
            // 255 shares can be generated !
            // (hence why we use `NonZeroU8` to represent this number)
            .take(shares.get() as usize)
            .collect()
    }

    pub fn decrypt_and_load_from_shares<'a, T>(threshold: NonZeroU8, shares: T) -> DataResult<Self>
    where
        T: IntoIterator<Item = &'a ShamirShare>,
        T::IntoIter: Iterator<Item = &'a ShamirShare>,
    {
        let blahaj = blahaj::Sharks(threshold.get());
        let shares = shares.into_iter().map(|x| &x.0);
        let secret = blahaj.recover(shares).map_err(|_| DataError::Decryption)?;

        Self::load(&secret)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "ShamirRecoveryShareDataData",
    from = "ShamirRecoveryShareDataData"
)]
pub struct ShamirRecoveryShareData {
    pub author: DeviceID,
    pub timestamp: DateTime,
    pub weighted_share: Vec<ShamirShare>,
}

parsec_data!("schema/shamir/shamir_recovery_share_data.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoveryShareData,
    ShamirRecoveryShareDataData,
    author,
    timestamp,
    weighted_share,
);

impl ShamirRecoveryShareData {
    // Note `ShamirRecoveryShareData` doesn't need to be signed since it is
    // embedded into a `ShamirRecoveryShareCertificate` which itself is signed.

    fn check_data_integrity(&self) -> DataResult<()> {
        if self.weighted_share.len() > 255 {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "weighted_share <= 255",
            });
        }

        Ok(())
    }

    pub fn decrypt_verify_and_load_for(
        encrypted: &[u8],
        recipient_privkey: &PrivateKey,
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
    ) -> DataResult<ShamirRecoveryShareData> {
        let signed = recipient_privkey
            .decrypt_from_self(encrypted)
            .map_err(|_| DataError::Decryption)?;

        let serialized = author_verify_key
            .verify(&signed)
            .map_err(|_| DataError::Signature)?;

        let obj: Self = format_vx_load(serialized)?;

        if obj.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author,
                got: Some(obj.author),
            });
        }

        if obj.timestamp != expected_timestamp {
            return Err(DataError::UnexpectedTimestamp {
                expected: expected_timestamp,
                got: obj.timestamp,
            });
        }

        obj.check_data_integrity()?;

        Ok(obj)
    }

    pub fn dump_sign_and_encrypt_for(
        &self,
        author_signkey: &SigningKey,
        recipient_pubkey: &PublicKey,
    ) -> Vec<u8> {
        let serialized = format_v0_dump(self);
        let signed = author_signkey.sign(&serialized);
        recipient_pubkey.encrypt_for_self(&signed)
    }
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct ShamirShare(blahaj::Share);

impl_key_debug!(ShamirShare);

impl ShamirShare {
    pub fn dump(&self) -> Vec<u8> {
        Vec::from(&self.0)
    }
    pub fn load(raw: &[u8]) -> CryptoResult<Self> {
        let share = blahaj::Share::try_from(raw).map_err(|_| CryptoError::KeySize {
            expected: 2,
            got: raw.len(),
        })?;
        Ok(Self(share))
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
        Self::load(data)
    }
}

impl TryFrom<&Bytes> for ShamirShare {
    type Error = CryptoError;
    fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
        Self::load(data.as_ref())
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

#[cfg(test)]
#[path = "../tests/unit/shamir.rs"]
mod tests;
