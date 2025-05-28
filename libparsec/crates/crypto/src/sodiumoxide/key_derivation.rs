// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde::Deserialize;
use serde_bytes::Bytes;
use sodiumoxide::crypto::kdf::{derive_from_key, gen_key, Key, KEYBYTES};

use crate::SecretKey;

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
        let id_low: &[u8; 8] = id.as_bytes()[..8].try_into().unwrap();
        let id_high: &[u8; 8] = id.as_bytes()[8..].try_into().unwrap();

        let subkey_id = u64::from_le_bytes(*id_low);
        let context = id_high;

        let mut subkey = [0u8; SecretKey::SIZE];
        derive_from_key(&mut subkey, subkey_id, *context, &self.0)
            .expect("subkey has always a valid size");

        SecretKey::from(subkey)
    }

    #[expect(unused_variables)]
    pub fn derive_uuid_from_uuid(&self, id: uuid::Uuid) -> uuid::Uuid {
        todo!("done in #10471")
    }
}

impl From<[u8; KeyDerivation::SIZE]> for KeyDerivation {
    fn from(key: [u8; KeyDerivation::SIZE]) -> Self {
        Self(Key(key))
    }
}

crate::common::impl_key_derivation!(KeyDerivation);
