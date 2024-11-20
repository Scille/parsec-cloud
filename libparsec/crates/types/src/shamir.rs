// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    self as libparsec_types, /*To use parsec_data!*/
    data_macros::impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    DataError, DataResult, InvitationToken,
};
use libparsec_crypto::{
    impl_key_debug, CryptoError, CryptoResult, PrivateKey, PublicKey, SecretKey,
};
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
    fn load(raw: &[u8]) -> DataResult<Self> {
        format_vx_load(raw)
    }

    fn dump(&self) -> Vec<u8> {
        format_v0_dump(&self)
    }

    pub fn dump_and_encrypt_into_shares(&self, threshold: u8, shares: usize) -> Vec<ShamirShare> {
        let secret = self.dump();

        let sharks = sharks::Sharks(threshold);
        sharks
            .dealer(&secret)
            .map(ShamirShare)
            .take(shares)
            .collect()
    }

    pub fn decrypt_and_load_from_shares<'a, T>(threshold: u8, shares: T) -> DataResult<Self>
    where
        T: IntoIterator<Item = &'a ShamirShare>,
        T::IntoIter: Iterator<Item = &'a ShamirShare>,
    {
        let sharks = sharks::Sharks(threshold);
        let shares = shares.into_iter().map(|x| &x.0);
        let secret = sharks.recover(shares).map_err(|_| DataError::Decryption)?;

        Self::load(&secret)
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
    // Note `ShamirRecoveryShareData` doesn't need to be signed since it is
    // embedded into a `ShamirRecoveryShareCertificate` which itself is signed.

    pub fn decrypt_and_load_for(
        encrypted: &[u8],
        recipient_privkey: &PrivateKey,
    ) -> DataResult<ShamirRecoveryShareData> {
        let serialized = recipient_privkey
            .decrypt_from_self(encrypted)
            .map_err(|_| DataError::Decryption)?;
        let obj: Self = format_vx_load(&serialized)?;
        Ok(obj)
    }

    pub fn dump_and_encrypt_for(&self, recipient_pubkey: &PublicKey) -> Vec<u8> {
        let serialized = format_v0_dump(self);
        recipient_pubkey.encrypt_for_self(&serialized)
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
    pub fn load(raw: &[u8]) -> CryptoResult<Self> {
        let share = sharks::Share::try_from(raw).map_err(|_| CryptoError::KeySize {
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
