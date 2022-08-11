// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;
use sha2::{Digest, Sha256};
use std::hash::Hash;

use crate::CryptoError;

#[derive(Clone, PartialEq, Eq, Deserialize, Hash)]
#[serde(try_from = "&Bytes")]
pub struct HashDigest(digest::Output<Sha256>);

impl HashDigest {
    pub const ALGORITHM: &'static str = "sha256";
    pub const SIZE: usize = 32;

    pub fn from_data(data: &[u8]) -> Self {
        Self(Sha256::digest(data))
    }

    pub fn hexdigest(&self) -> String {
        hex::encode(self.0.as_slice())
    }
}

impl std::fmt::Debug for HashDigest {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "{}({})",
            stringify!(HashDigest),
            &self.hexdigest()
        )
    }
}

impl AsRef<[u8]> for HashDigest {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&[u8]> for HashDigest {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        <[u8; Self::SIZE]>::try_from(data)
            .map(Self::from)
            .map_err(|_| CryptoError::DataSize)
    }
}

impl From<[u8; Self::SIZE]> for HashDigest {
    fn from(digest: [u8; Self::SIZE]) -> Self {
        Self(digest.into())
    }
}

impl TryFrom<&Bytes> for HashDigest {
    type Error = CryptoError;
    fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
    }
}

impl Serialize for HashDigest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait BaseHashDigest<'de, 'f>:
        serde::Serialize
        + serde::Deserialize<'de>
        + AsRef<[u8]>
        + TryFrom<&'f [u8]>
        + From<[u8; HashDigest::SIZE]>
        + TryFrom<&'f Bytes>
    {
        const ALGORITHM: &'static str;
        const SIZE: usize;

        fn from_data(data: &[u8]) -> Self;
        fn hexdigest(&self) -> String;
    }

    impl BaseHashDigest<'_, '_> for HashDigest {
        const ALGORITHM: &'static str = HashDigest::ALGORITHM;
        const SIZE: usize = HashDigest::SIZE;

        fn from_data(data: &[u8]) -> Self {
            HashDigest::from_data(data)
        }
        fn hexdigest(&self) -> String {
            self.hexdigest()
        }
    }
}
