// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use digest::{
    consts::{U5, U64},
    typenum::{IsLessOrEqual, LeEq, NonZero},
};
use generic_array::{ArrayLength, GenericArray};
use libsodium_rs::{
    crypto_pwhash::argon2id::SALTBYTES,
    crypto_secretbox::{open, seal, Key, Nonce, KEYBYTES, MACBYTES, NONCEBYTES},
    random,
};
use serde::Deserialize;
use serde_bytes::Bytes;

use crate::CryptoError;

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct SecretKey(Key);

impl std::hash::Hash for SecretKey {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.as_ref().hash(state)
    }
}

impl SecretKey {
    pub const ALGORITHM: &'static str = "xsalsa20poly1305";
    pub const SIZE: usize = KEYBYTES;

    pub fn generate() -> Self {
        Self(Key::generate())
    }

    /// Returned format: NONCE | MAC | CIPHERTEXT
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut ciphered = Vec::with_capacity(NONCEBYTES + MACBYTES + data.len());
        let nonce = Nonce::generate();
        ciphered.extend_from_slice(nonce.as_bytes());
        ciphered.append(&mut seal(data, &nonce, &self.0));
        ciphered
    }

    pub fn decrypt(&self, ciphered: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce_slice = ciphered.get(..NONCEBYTES).ok_or(CryptoError::Nonce)?;
        let nonce = Nonce::try_from_slice(nonce_slice).or(Err(CryptoError::DataSize))?;
        let plaintext =
            open(&ciphered[NONCEBYTES..], &nonce, &self.0).or(Err(CryptoError::Decryption))?;
        Ok(plaintext)
    }

    fn mac<Size>(&self, data: &[u8]) -> GenericArray<u8, Size>
    where
        Size: ArrayLength<u8> + IsLessOrEqual<U64>,
        LeEq<Size, U64>: NonZero,
    {
        let mut out = GenericArray::<u8, Size>::default();
        let raw_key = self.0.as_bytes();
        // SAFETY: We provide valid pointers to `crypto_generichash_blake2b`
        // The lib call to check that `outlen` is of correct size for cryptography purpose (at
        // min `crypto_generichash_BYTES_MIN`).
        // `outlen` will be smaller than that when generating SAS code.
        // https://doc.libsodium.org/hashing/generic_hashing#usage
        let res = unsafe {
            libsodium_sys::crypto_generichash_blake2b(
                out.as_mut_ptr(),
                out.len(),
                data.as_ptr(),
                data.len() as u64,
                raw_key.as_ptr(),
                raw_key.len(),
            )
        };
        if res != 0 {
            panic!("Error while generating MAC");
        } else {
            out
        }
    }

    pub fn mac_512(&self, data: &[u8]) -> [u8; 64] {
        self.mac::<U64>(data).into()
    }

    pub fn sas_code(&self, data: &[u8]) -> [u8; 5] {
        self.mac::<U5>(data).into()
    }

    pub fn generate_salt() -> Vec<u8> {
        random::bytes(SALTBYTES)
    }
}

impl From<[u8; Self::SIZE]> for SecretKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        Self(Key::from(key))
    }
}

crate::common::impl_secret_key!(SecretKey);
