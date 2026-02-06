// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use blake2::Blake2bMac;
use digest::Mac;
use generic_array::{
    typenum::{
        consts::{U32, U64},
        IsLessOrEqual, LeEq, NonZero,
    },
    ArrayLength, GenericArray,
};
use rand::{rngs::SysRng, Rng, TryRng};
use serde::Deserialize;
use serde_bytes::Bytes;

use crate::SecretKey;

// see https://github.com/jedisct1/libsodium/blob/fd8c876bb5ad9d5ad79074ccd6b509f845631807/src/libsodium/include/sodium/crypto_generichash_blake2b.h#L61-L65
const BLAKE2B_SALTBYTES: usize = 16;
const BLAKE2B_PERSONALBYTES: usize = 16;

#[derive(Clone, PartialEq, Eq, Deserialize, Hash)]
#[serde(try_from = "&Bytes")]
pub struct KeyDerivation(GenericArray<u8, U32>);

impl KeyDerivation {
    pub const ALGORITHM: &'static str = "blake2b";
    pub const SIZE: usize = 32;

    pub fn generate() -> Self {
        let mut bytes = [0u8; Self::SIZE];
        SysRng
            .try_fill_bytes(&mut bytes)
            .expect("Failed to generate random data");
        Self(bytes.into())
    }

    pub fn derive_secret_key_from_uuid(&self, id: uuid::Uuid) -> SecretKey {
        SecretKey(self.derive_raw_from_uuid(id))
    }

    pub fn derive_uuid_from_uuid(&self, id: uuid::Uuid) -> uuid::Uuid {
        uuid::Uuid::from_bytes(self.derive_raw_from_uuid(id).into())
    }

    fn derive_raw_from_uuid<Size>(&self, id: uuid::Uuid) -> GenericArray<u8, Size>
    where
        Size: ArrayLength<u8> + IsLessOrEqual<U64>,
        LeEq<Size, U64>: NonZero,
    {
        // We follow what libsodium does here
        // (see https://github.com/jedisct1/libsodium/blob/4a15ab7cd0a4b78a7356e5f488d5345b8d314549/src/libsodium/crypto_kdf/blake2b/kdf_blake2b.c#L31-L52)

        let mut salt = [0u8; BLAKE2B_SALTBYTES];
        salt[..8].copy_from_slice(&id.as_bytes()[..8]);

        let mut personal = [0u8; BLAKE2B_PERSONALBYTES];
        personal[..8].copy_from_slice(&id.as_bytes()[8..]);

        let subkey = Blake2bMac::new_with_salt_and_personal(&self.0, &salt, &personal)
            .expect("subkey has always a valid size")
            .finalize();

        subkey.into_bytes()
    }
}

impl From<[u8; KeyDerivation::SIZE]> for KeyDerivation {
    fn from(key: [u8; KeyDerivation::SIZE]) -> Self {
        Self(key.into())
    }
}

crate::common::impl_key_derivation!(KeyDerivation);
