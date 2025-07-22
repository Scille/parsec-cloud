// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};
use libsodium_rs::crypto_kdf::blake2b;
use serde::Deserialize;
use serde_bytes::Bytes;

use crate::SecretKey;

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct KeyDerivation(blake2b::Key);

impl std::hash::Hash for KeyDerivation {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.as_ref().hash(state)
    }
}

impl KeyDerivation {
    pub const ALGORITHM: &'static str = "blake2b";
    pub const SIZE: usize = blake2b::KEYBYTES;

    pub fn generate() -> Self {
        Self(blake2b::Key::generate().expect("Failed to generate key"))
    }

    pub fn derive_secret_key_from_uuid(&self, id: uuid::Uuid) -> SecretKey {
        let raw: [u8; SecretKey::SIZE] = self.derive_raw_from_uuid(id).into();
        SecretKey::from(raw)
    }

    pub fn derive_uuid_from_uuid(&self, id: uuid::Uuid) -> uuid::Uuid {
        uuid::Uuid::from_bytes(self.derive_raw_from_uuid(id).into())
    }

    fn derive_raw_from_uuid<Size>(&self, id: uuid::Uuid) -> GenericArray<u8, Size>
    where
        Size: ArrayLength<u8> + IsLessOrEqual<U64>,
        LeEq<Size, U64>: NonZero,
    {
        let id_low: &[u8; 8] = id.as_bytes()[..8].try_into().unwrap();
        let id_high: &[u8; 8] = id.as_bytes()[8..].try_into().unwrap();

        let subkey_id = u64::from_le_bytes(*id_low);
        let context = id_high;
        debug_assert_eq!(context.len(), blake2b::CONTEXTBYTES);

        let subkey = blake2b::derive_from_key(Size::USIZE, subkey_id, context, &self.0)
            .expect("subkey has always a valid size");

        // generic-array@1.2.0 provide better way to convert from an array but we are stuck by
        // crypto_common from rustcrypto.
        GenericArray::from_exact_iter(subkey).expect("Invalid derivation size")
    }
}

impl From<[u8; KeyDerivation::SIZE]> for KeyDerivation {
    fn from(key: [u8; KeyDerivation::SIZE]) -> Self {
        Self(blake2b::Key::from(key))
    }
}

crate::common::impl_key_derivation!(KeyDerivation);
