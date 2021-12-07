// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use xsalsa20poly1305::aead::{Aead, NewAead};
use xsalsa20poly1305::{generate_nonce, Key, XSalsa20Poly1305, KEY_SIZE, NONCE_SIZE};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "ByteBuf", try_from = "ByteBuf")]
pub struct SecretKey(Key);

super::macros::impl_key_debug!(SecretKey);

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

    pub fn decrypt(&self, ciphered: &[u8]) -> Result<Vec<u8>, &'static str> {
        let cipher = XSalsa20Poly1305::new(&self.0);
        let nonce_slice = ciphered
            .get(..NONCE_SIZE)
            .ok_or("The nonce must be exactly 24 bytes long")?;
        cipher
            .decrypt(nonce_slice.into(), &ciphered[NONCE_SIZE..])
            .map_err(|_| "Decryption error")
    }

    // TODO
    pub fn hmac(&self, _data: &[u8], _digest_size: usize) -> Vec<u8> {
        // // blake2b(data, digest_size=digest_size, key=self, encoder=RawEncoder)
        // let key = blake2b::Key::from_slice(&self.0);
        // blake2b::derive_from_key(
        //     subkey: &mut [u8],
        //     subkey_id: u64,
        //     ctx: [u8; 8],
        //     key,
        // );
        vec![]
    }
}

impl AsRef<[u8]> for SecretKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&[u8]> for SecretKey {
    type Error = &'static str;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // if you wonder, `try_into` will also fail if data is too small
        let arr: [u8; Self::SIZE] = data.try_into().map_err(|_| ("Invalid data size"))?;
        Ok(Self(Key::from(arr)))
    }
}

impl From<[u8; Self::SIZE]> for SecretKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        Self(Key::from(key))
    }
}

impl TryFrom<ByteBuf> for SecretKey {
    type Error = &'static str;
    fn try_from(data: ByteBuf) -> Result<Self, Self::Error> {
        // if you wonder, `try_into` will also fail if data is too small
        let arr: [u8; Self::SIZE] = data
            .into_vec()
            .try_into()
            .map_err(|_| ("Invalid data size"))?;
        Ok(Self(Key::from(arr)))
    }
}

impl From<SecretKey> for ByteBuf {
    fn from(data: SecretKey) -> Self {
        Self::from(data.0.to_vec())
    }
}
