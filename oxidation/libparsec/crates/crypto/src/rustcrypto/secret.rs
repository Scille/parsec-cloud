// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use blake2::Blake2bMac;
use digest::{consts::U5, Mac};
use serde::Deserialize;
use serde_bytes::Bytes;
use xsalsa20poly1305::{
    aead::{Aead, NewAead},
    generate_nonce, Key, XSalsa20Poly1305, KEY_SIZE, NONCE_SIZE,
};

use crate::CryptoError;

type Blake2bMac40 = Blake2bMac<U5>;

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct SecretKey(Key);

impl SecretKey {
    pub const ALGORITHM: &'static str = "xsalsa20poly1305";
    pub const SIZE: usize = KEY_SIZE;

    pub fn generate() -> Self {
        Self(XSalsa20Poly1305::generate_key(rand_08::thread_rng()))
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Returned format: NONCE | MAC | CIPHERTEXT
        // TODO: zero copy with preallocated buffer
        // let mut ciphered = Vec::with_capacity(NONCE_SIZE + TAG_SIZE + data.len());
        let cipher = XSalsa20Poly1305::new(&self.0);
        let nonce = generate_nonce(&mut rand_08::thread_rng());
        // TODO: handle this error ?
        let mut ciphered = cipher.encrypt(&nonce, data).expect("encryption failure !");
        let mut res = vec![];
        res.append(&mut nonce.to_vec());
        res.append(&mut ciphered);
        res
    }

    pub fn decrypt(&self, ciphered: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cipher = XSalsa20Poly1305::new(&self.0);
        let nonce_slice = ciphered.get(..NONCE_SIZE).ok_or(CryptoError::Nonce)?;
        cipher
            .decrypt(nonce_slice.into(), &ciphered[NONCE_SIZE..])
            .map_err(|_| CryptoError::Decryption)
    }

    pub fn hmac(&self, data: &[u8], digest_size: usize) -> Vec<u8> {
        // TODO only work for 5 bytes -> need to improve
        if digest_size != 5 {
            panic!("Not implemeted for this digest size");
        }
        // TODO investigate why new() is not working
        // let mut hasher = Blake2bMac40::new(&self.0);
        // &self.0 always provide the correct key size
        let mut hasher = Blake2bMac40::new_from_slice(&self.0).unwrap_or_else(|_| unreachable!());
        hasher.update(data);
        let res = hasher.finalize();
        res.into_bytes().to_vec()
    }
}

impl From<[u8; KEY_SIZE]> for SecretKey {
    fn from(key: [u8; KEY_SIZE]) -> Self {
        Self(key.into())
    }
}

crate::common::impl_secret_key!(SecretKey);
