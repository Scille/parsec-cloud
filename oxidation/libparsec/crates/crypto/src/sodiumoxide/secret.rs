// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::Deserialize;
use serde_bytes::Bytes;
use sodiumoxide::{
    crypto::{
        pwhash::argon2i13::{
            derive_key, Salt, MEMLIMIT_INTERACTIVE, OPSLIMIT_INTERACTIVE, SALTBYTES,
        },
        secretbox::{
            gen_nonce, open, seal,
            xsalsa20poly1305::{gen_key, Key, KEYBYTES, MACBYTES, NONCEBYTES},
            Nonce,
        },
    },
    randombytes::randombytes,
};

use crate::CryptoError;

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct SecretKey(Key);

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

    pub fn decrypt(&self, ciphered: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce_slice = ciphered.get(..NONCEBYTES).ok_or(CryptoError::Nonce)?;
        let nonce = Nonce::from_slice(nonce_slice).ok_or(CryptoError::DataSize)?;
        let plaintext =
            open(&ciphered[NONCEBYTES..], &nonce, &self.0).or(Err(CryptoError::Decryption))?;
        Ok(plaintext)
    }

    /// # Safety
    ///
    /// This function requires access to libsodium methods that are not
    /// exposed directly, so it uses the unsafe C API
    /// ...
    pub fn hmac(&self, data: &[u8], digest_size: usize) -> Vec<u8> {
        // SAFETY: Sodiumoxide doesn't expose those methods, so we have to access
        // the libsodium C API directly.
        // this remains safe because we provide bounds defined in Rust land when passing vectors.
        // The only data structure provided by remote code is dropped
        // at the end of the function.
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

    pub fn generate_salt() -> Vec<u8> {
        randombytes(SALTBYTES)
    }

    pub fn from_password(password: &str, salt: &[u8]) -> Self {
        let mut key = [0; KEYBYTES];
        let salt = Salt::from_slice(salt).expect("Invalid salt");

        derive_key(
            &mut key,
            password.as_bytes(),
            &salt,
            OPSLIMIT_INTERACTIVE,
            MEMLIMIT_INTERACTIVE,
        )
        .expect("Can't fail");

        Self::from(key)
    }
}

impl From<[u8; KEYBYTES]> for SecretKey {
    fn from(key: [u8; KEYBYTES]) -> Self {
        Self(Key(key))
    }
}

crate::common::impl_secret_key!(SecretKey);
