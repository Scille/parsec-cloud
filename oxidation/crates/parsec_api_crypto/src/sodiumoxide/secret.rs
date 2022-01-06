// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
// use sodiumoxide::crypto::kdf::blake2b;
use sodiumoxide::crypto::secretbox::xsalsa20poly1305::{
    gen_key, Key, KEYBYTES, MACBYTES, NONCEBYTES,
};
use sodiumoxide::crypto::secretbox::{gen_nonce, open, seal, Nonce};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SecretKey(Key);

super::utils::impl_try_from!(SecretKey, Key);

impl SecretKey {
    pub const ALGORITHM: &'static str = "xsalsa20poly1305";
    pub const SIZE: usize = KEYBYTES;

    pub fn generate() -> Self {
        Self(gen_key())
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Returned format: NONCE | MAC | CIPHERTEXT
        let mut ciphered = Vec::with_capacity(NONCEBYTES + MACBYTES + data.len());
        let nonce = gen_nonce();
        ciphered.extend_from_slice(&nonce.0);
        ciphered.append(&mut seal(data, &nonce, &self.0));
        ciphered
    }

    pub fn decrypt(&self, ciphered: &[u8]) -> Result<Vec<u8>, &'static str> {
        let nonce_slice = ciphered.get(..NONCEBYTES).ok_or("Invalid data size")?;
        let nonce = Nonce::from_slice(nonce_slice).ok_or("Invalid data size")?;
        let plaintext =
            open(&ciphered[NONCEBYTES..], &nonce, &self.0).or(Err("Decryption error"))?;
        Ok(plaintext)
    }

    // TODO
    pub fn hmac(&self, _data: &[u8], digest_size: usize) -> Vec<u8> {
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
        self.0.as_ref()
    }
}
