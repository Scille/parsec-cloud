// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use blake2::Blake2bMac;
use digest::{Mac, consts::U32, generic_array::GenericArray};
use rand::RngCore;
use serde::Deserialize;
use serde_bytes::Bytes;

use crate::SecretKey;

type Blake2bMac256 = Blake2bMac<U32>;

#[derive(Clone, PartialEq, Eq, Deserialize, Hash)]
#[serde(try_from = "&Bytes")]
pub struct KeyDerivation(GenericArray<u8, U32>);

impl KeyDerivation {
    pub const ALGORITHM: &'static str = "blake2b";
    pub const SIZE: usize = 32;

    pub fn generate() -> Self {
        let mut bytes = [0u8; Self::SIZE];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self(bytes.into())
    }

    pub fn derive_secret_key_from_uuid(&self, id: uuid::Uuid) -> SecretKey {
        // We follow what libsodium does here
        // (see https://github.com/jedisct1/libsodium/blob/4a15ab7cd0a4b78a7356e5f488d5345b8d314549/src/libsodium/crypto_kdf/blake2b/kdf_blake2b.c#L31-L52)

        let mut salt: [u8; 16] = [0u8; 16];
        salt[..8].copy_from_slice(&id.as_bytes()[..8]);

        let mut personal: [u8; 16] = [0u8; 16];
        personal[..8].copy_from_slice(&id.as_bytes()[8..]);

        let subkey = Blake2bMac256::new_with_salt_and_personal(&self.0, &salt, &personal)
            .expect("subkey has always a valid size")
            .finalize();

        SecretKey(subkey.into_bytes())
    }
}

impl From<[u8; KeyDerivation::SIZE]> for KeyDerivation {
    fn from(key: [u8; KeyDerivation::SIZE]) -> Self {
        Self(key.into())
    }
}

crate::common::impl_key_derivation!(KeyDerivation);
