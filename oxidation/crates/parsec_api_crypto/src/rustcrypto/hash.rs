// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use sha2::{Digest, Sha256};
use std::hash::Hash;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(into = "ByteBuf", try_from = "ByteBuf")]
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
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&[u8]> for HashDigest {
    type Error = &'static str;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // if you wonder, `try_into` will also fail if data is too small
        let arr: [u8; Self::SIZE] = data.try_into().map_err(|_| ("Invalid data size"))?;
        Ok(Self(arr.into()))
    }
}

impl From<[u8; Self::SIZE]> for HashDigest {
    fn from(digest: [u8; Self::SIZE]) -> Self {
        Self(digest.into())
    }
}

impl TryFrom<ByteBuf> for HashDigest {
    type Error = &'static str;
    fn try_from(data: ByteBuf) -> Result<Self, Self::Error> {
        // if you wonder, `try_into` will also fail if data is too small
        let arr: [u8; Self::SIZE] = data
            .into_vec()
            .try_into()
            .map_err(|_| ("Invalid data size"))?;
        Ok(Self(arr.into()))
    }
}

impl From<HashDigest> for ByteBuf {
    fn from(data: HashDigest) -> Self {
        Self::from(data.0.to_vec())
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
        + TryFrom<ByteBuf>
        + Into<ByteBuf>
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
