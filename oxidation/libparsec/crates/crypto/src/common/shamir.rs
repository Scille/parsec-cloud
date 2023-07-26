// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use super::error::CryptoError;
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;

#[derive(Clone, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct ShamirShare(sharks::Share);

crate::impl_key_debug!(ShamirShare);

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
