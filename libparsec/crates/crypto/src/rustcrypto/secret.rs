// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use argon2::{Algorithm, Argon2, Params, Version};
use blake2::Blake2bMac;
use crypto_box::aead::Aead;
use crypto_secretbox::{AeadCore, Key, XSalsa20Poly1305};
use digest::{consts::U5, KeyInit, Mac};
use rand_08::{rngs::OsRng, RngCore};
use serde::Deserialize;
use serde_bytes::Bytes;

use crate::{CryptoError, Password};

type Blake2bMac40 = Blake2bMac<U5>;

/// Memory block size in bytes
const BLOCKSIZE: u32 = 1024;
/// The maximum amount of RAM that the functions in this module will use, in bytes.
/// https://github.com/sodiumoxide/sodiumoxide/blob/master/libsodium-sys/src/sodium_bindings.rs#L128
const MEMLIMIT_INTERACTIVE: u32 = 33_554_432;
/// The maximum number of computations to perform when using the functions.
/// https://github.com/sodiumoxide/sodiumoxide/blob/master/libsodium-sys/src/sodium_bindings.rs#L127
const OPSLIMIT_INTERACTIVE: u32 = 4;
/// Degree of parallelism
const PARALLELISM: u32 = 1;
// https://github.com/sodiumoxide/sodiumoxide/blob/master/libsodium-sys/src/sodium_bindings.rs#L121
const SALTBYTES: usize = 16;

lazy_static::lazy_static! {
    static ref ARGON2: Argon2<'static> =
        Argon2::new(
            Algorithm::Argon2i,
            Version::V0x13,
            Params::new(
                MEMLIMIT_INTERACTIVE / BLOCKSIZE,
                OPSLIMIT_INTERACTIVE,
                PARALLELISM,
                Some(XSalsa20Poly1305::KEY_SIZE),
            )
            .expect("Can't fail"),
        );
}

#[derive(Clone, PartialEq, Eq, Deserialize, Hash)]
#[serde(try_from = "&Bytes")]
pub struct SecretKey(Key);

impl SecretKey {
    pub const ALGORITHM: &'static str = "xsalsa20poly1305";
    pub const SIZE: usize = XSalsa20Poly1305::KEY_SIZE;

    pub fn generate() -> Self {
        Self(XSalsa20Poly1305::generate_key(rand_08::thread_rng()))
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Returned format: NONCE | MAC | CIPHERTEXT
        // TODO: zero copy with preallocated buffer
        // let mut ciphered = Vec::with_capacity(NONCE_SIZE + TAG_SIZE + data.len());
        let cipher = XSalsa20Poly1305::new(&self.0);
        let nonce = XSalsa20Poly1305::generate_nonce(&mut rand_08::thread_rng());
        // TODO: handle this error ?
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

    pub fn hmac(&self, data: &[u8], digest_size: usize) -> Vec<u8> {
        // TODO only work for 5 bytes -> need to improve
        if digest_size != 5 {
            panic!("Not implemented for this digest size");
        }
        let mut hasher = <Blake2bMac40 as KeyInit>::new_from_slice(self.0.as_ref())
            .unwrap_or_else(|_| unreachable!());
        hasher.update(data);
        let res = hasher.finalize();
        res.into_bytes().to_vec()
    }

    pub fn generate_salt() -> Vec<u8> {
        let mut salt = vec![0; SALTBYTES];
        OsRng.fill_bytes(&mut salt);

        salt
    }

    pub fn from_password(password: &Password, salt: &[u8]) -> Result<Self, CryptoError> {
        let mut key = [0; XSalsa20Poly1305::KEY_SIZE];

        ARGON2
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|_| CryptoError::DataSize)?;

        Ok(Self::from(key))
    }
}

impl From<[u8; XSalsa20Poly1305::KEY_SIZE]> for SecretKey {
    fn from(key: [u8; XSalsa20Poly1305::KEY_SIZE]) -> Self {
        Self(key.into())
    }
}

crate::common::impl_secret_key!(SecretKey);
