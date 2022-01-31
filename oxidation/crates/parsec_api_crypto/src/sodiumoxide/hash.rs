// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::hash::sha256;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HashDigest(sha256::Digest);

impl HashDigest {
    pub const ALGORITHM: &'static str = "sha256";
    pub const SIZE: usize = sha256::DIGESTBYTES;

    pub fn from_data(data: &[u8]) -> Self {
        Self(sha256::hash(data))
    }

    pub fn hexdigest(&self) -> String {
        hex::encode(self.0.as_ref())
    }
}

impl AsRef<[u8]> for HashDigest {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl TryFrom<&[u8]> for HashDigest {
    type Error = &'static str;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // if you wonder, `try_into` will also fail if data is too small
        let arr: [u8; Self::SIZE] = data.try_into().map_err(|_| ("Invalid data size"))?;
        Ok(Self(sha256::Digest(arr)))
    }
}

impl From<[u8; Self::SIZE]> for HashDigest {
    fn from(digest: [u8; Self::SIZE]) -> Self {
        Self(sha256::Digest(digest))
    }
}
