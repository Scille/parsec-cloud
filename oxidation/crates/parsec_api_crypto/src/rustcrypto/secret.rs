// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use blake2::Blake2bMac;
use digest::{consts::U5, Mac};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use xsalsa20poly1305::aead::{Aead, NewAead};
use xsalsa20poly1305::{generate_nonce, Key, XSalsa20Poly1305, KEY_SIZE, NONCE_SIZE};

use crate::CryptoError;

type Blake2bMac40 = Blake2bMac<U5>;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "ByteBuf", try_from = "ByteBuf")]
pub struct SecretKey(Key);

crate::macros::impl_key_debug!(SecretKey);

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

impl AsRef<[u8]> for SecretKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&[u8]> for SecretKey {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // if you wonder, `try_into` will also fail if data is too small
        let arr: [u8; Self::SIZE] = data.try_into().map_err(|_| CryptoError::DataSize)?;
        Ok(Self(Key::from(arr)))
    }
}

impl From<[u8; Self::SIZE]> for SecretKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        Self(Key::from(key))
    }
}

impl TryFrom<ByteBuf> for SecretKey {
    type Error = CryptoError;
    fn try_from(data: ByteBuf) -> Result<Self, Self::Error> {
        // if you wonder, `try_into` will also fail if data is too small
        let arr: [u8; Self::SIZE] = data
            .into_vec()
            .try_into()
            .map_err(|_| CryptoError::DataSize)?;
        Ok(Self(Key::from(arr)))
    }
}

impl From<SecretKey> for ByteBuf {
    fn from(data: SecretKey) -> Self {
        Self::from(data.0.to_vec())
    }
}
