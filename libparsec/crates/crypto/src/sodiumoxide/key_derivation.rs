// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};
use serde::Deserialize;
use serde_bytes::Bytes;
use sodiumoxide::crypto::kdf::{derive_from_key, gen_key, Key, KEYBYTES};

use crate::{from_argon2id_password, CryptoError, Password, SecretKey};

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct KeyDerivation(Key);

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
    pub const SIZE: usize = KEYBYTES;

    pub fn generate() -> Self {
        Self(gen_key())
    }

    pub fn derive_secret_key_from_uuid(&self, id: uuid::Uuid) -> SecretKey {
        let raw: [u8; SecretKey::SIZE] = self.derive_raw_from_uuid(id).into();
        SecretKey::from(raw)
    }

    pub fn derive_uuid_from_uuid(&self, id: uuid::Uuid) -> uuid::Uuid {
        uuid::Uuid::from_bytes(self.derive_raw_from_uuid(id).into())
    }

    fn derive_raw_form_uuid<Size>(&self, id: uuid::Uuid) -> GenericArray<u8, Size>
    where
        Size: ArrayLength<u8> + IsLessOrEqual<U64>,
        LeEq<Size, U64>: NonZero,
    {
        let id_low: &[u8; 8] = id.as_bytes()[..8].try_into().unwrap();
        let id_high: &[u8; 8] = id.as_bytes()[8..].try_into().unwrap();

        let subkey_id = u64::from_le_bytes(*id_low);
        let context = id_high;

        let mut subkey = GenericArray::default();
        derive_from_key(&mut subkey, subkey_id, *context, &self.0)
            .expect("subkey has always a valid size");

        subkey
    }

    pub fn from_argon2id_password(
        password: &Password,
        salt: &[u8],
        opslimit: u32,
        memlimit_kb: u32,
        parallelism: u32,
    ) -> Result<Self, CryptoError> {
        let raw: [u8; Self::SIZE] =
            from_argon2id_password(password, salt, opslimit, memlimit_kb, parallelism)?.into();
        Ok(Self::from(raw))
    }
}

impl From<[u8; KeyDerivation::SIZE]> for KeyDerivation {
    fn from(key: [u8; KeyDerivation::SIZE]) -> Self {
        Self(Key(key))
    }
}

crate::common::impl_key_derivation!(KeyDerivation);
