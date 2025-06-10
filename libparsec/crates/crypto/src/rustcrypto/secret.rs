// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use blake2::Blake2bMac;
use crypto_box::aead::Aead;
use crypto_secretbox::{
    aead::{rand_core::RngCore, OsRng},
    AeadCore, Key, XSalsa20Poly1305,
};
use digest::{KeyInit, Mac};
use generic_array::{
    typenum::{
        consts::{U5, U64},
        IsLessOrEqual, LeEq, NonZero,
    },
    ArrayLength, GenericArray,
};
use serde::Deserialize;
use serde_bytes::Bytes;

use crate::CryptoError;

// https://github.com/sodiumoxide/sodiumoxide/blob/3057acb1a030ad86ed8892a223d64036ab5e8523/libsodium-sys/src/sodium_bindings.rs#L137
const SALTBYTES: usize = 16;

#[derive(Clone, PartialEq, Eq, Deserialize, Hash)]
#[serde(try_from = "&Bytes")]
pub struct SecretKey(pub(crate) Key);

impl SecretKey {
    pub const ALGORITHM: &'static str = "xsalsa20poly1305";
    pub const SIZE: usize = XSalsa20Poly1305::KEY_SIZE;

    pub fn generate() -> Self {
        Self(XSalsa20Poly1305::generate_key(rand::thread_rng()))
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Returned format: NONCE | MAC | CIPHERTEXT
        // TODO: zero copy with pre-allocated buffer
        // let mut ciphered = Vec::with_capacity(NONCE_SIZE + TAG_SIZE + data.len());
        let cipher = XSalsa20Poly1305::new(&self.0);
        let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::thread_rng());
        // TODO: handle this error?
        let mut ciphered = cipher.encrypt(&nonce, data).expect("encryption failure !");
        let mut res = vec![];
        res.append(&mut nonce.to_vec());
        res.append(&mut ciphered);
        res
    }

    pub fn decrypt(&self, ciphered: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = XSalsa20Poly1305::new(&self.0);
        let nonce_slice = ciphered
            .get(..XSalsa20Poly1305::NONCE_SIZE)
            .ok_or(CryptoError::Nonce)?;
        cipher
            .decrypt(
                nonce_slice.into(),
                &ciphered[XSalsa20Poly1305::NONCE_SIZE..],
            )
            .map_err(|_| CryptoError::Decryption)
    }

    fn mac<Size>(&self, data: &[u8]) -> GenericArray<u8, Size>
    where
        Size: ArrayLength<u8> + IsLessOrEqual<U64>,
        LeEq<Size, U64>: NonZero,
    {
        let mut hasher = <Blake2bMac<Size> as KeyInit>::new_from_slice(self.0.as_ref())
            .unwrap_or_else(|_| unreachable!());
        hasher.update(data);
        let res = hasher.finalize();
        res.into_bytes()
    }

    pub fn mac_512(&self, data: &[u8]) -> [u8; 64] {
        self.mac::<U64>(data).into()
    }

    pub fn sas_code(&self, data: &[u8]) -> [u8; 5] {
        self.mac::<U5>(data).into()
    }

    pub fn generate_salt() -> Vec<u8> {
        let mut salt = vec![0; SALTBYTES];
        OsRng.fill_bytes(&mut salt);

        salt
    }
}

impl From<[u8; XSalsa20Poly1305::KEY_SIZE]> for SecretKey {
    fn from(key: [u8; XSalsa20Poly1305::KEY_SIZE]) -> Self {
        Self(key.into())
    }
}

crate::common::impl_secret_key!(SecretKey);
