// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use argon2::{Algorithm, Argon2, Params, Version};
use blake2::Blake2bMac;
use crypto_box::aead::Aead;
use crypto_secretbox::{
    AeadCore, Key, XSalsa20Poly1305,
    aead::{OsRng, rand_core::RngCore},
};
use digest::{
    KeyInit, Mac,
    consts::{U5, U64},
    typenum::{IsLessOrEqual, LeEq, NonZero},
};
use generic_array::ArrayLength;
use serde::Deserialize;
use serde_bytes::Bytes;

use crate::{CryptoError, Password};

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

    pub fn hmac<Size>(&self, data: &[u8]) -> Vec<u8>
    where
        Size: ArrayLength<u8> + IsLessOrEqual<U64>,
        LeEq<Size, U64>: NonZero,
    {
        let mut hasher = <Blake2bMac<Size> as KeyInit>::new_from_slice(self.0.as_ref())
            .unwrap_or_else(|_| unreachable!());
        hasher.update(data);
        let res = hasher.finalize();
        let mut out = res.into_bytes().to_vec();
        out.resize(Size::USIZE, 0);
        out
    }

    pub fn hmac_full(&self, data: &[u8]) -> Vec<u8> {
        self.hmac::<U64>(data)
    }

    pub fn sas_code(&self, data: &[u8]) -> Vec<u8> {
        self.hmac::<U5>(data)
    }

    pub fn generate_salt() -> Vec<u8> {
        let mut salt = vec![0; SALTBYTES];
        OsRng.fill_bytes(&mut salt);

        salt
    }

    pub fn from_argon2id_password(
        password: &Password,
        salt: &[u8],
        opslimit: u32,
        memlimit_kb: u32,
        parallelism: u32,
    ) -> Result<Self, CryptoError> {
        let mut key = [0; XSalsa20Poly1305::KEY_SIZE];

        let argon = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(
                memlimit_kb,
                opslimit,
                parallelism,
                Some(XSalsa20Poly1305::KEY_SIZE),
            )
            .map_err(|_| CryptoError::DataSize)?,
        );

        argon
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
