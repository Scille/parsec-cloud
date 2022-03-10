// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
// use sodiumoxide::crypto::kdf::blake2b;
use sodiumoxide::crypto::secretbox::xsalsa20poly1305::{
    gen_key, Key, KEYBYTES, MACBYTES, NONCEBYTES,
};
use sodiumoxide::crypto::secretbox::{gen_nonce, open, seal, Nonce};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SecretKey(Key);

crate::macros::impl_key_debug!(SecretKey);

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
        let nonce_slice = ciphered
            .get(..NONCEBYTES)
            .ok_or("The nonce must be exactly 24 bytes long")?;
        let nonce = Nonce::from_slice(nonce_slice).ok_or("Invalid data size")?;
        let plaintext =
            open(&ciphered[NONCEBYTES..], &nonce, &self.0).or(Err("Decryption error"))?;
        Ok(plaintext)
    }

    pub fn hmac(&self, data: &[u8], digest_size: usize) -> Vec<u8> {
        // Sodiumoxide doesn't expose those methods, so we have to access
        // the libsodium C API directly
        unsafe {
            let mut state = libsodium_sys::crypto_generichash_blake2b_state {
                opaque: [0u8; 384usize],
            };
            libsodium_sys::crypto_generichash_blake2b_init(
                &mut state,
                self.as_ref().as_ptr(),
                self.as_ref().len(),
                digest_size,
            );
            libsodium_sys::crypto_generichash_blake2b_update(
                &mut state,
                data.as_ptr(),
                data.len() as u64,
            );
            let mut out = Vec::with_capacity(digest_size);
            libsodium_sys::crypto_generichash_blake2b_final(
                &mut state,
                out.as_mut_ptr(),
                digest_size,
            );
            out.set_len(digest_size);
            out
        }
    }
}

impl AsRef<[u8]> for SecretKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
