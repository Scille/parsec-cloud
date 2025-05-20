// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use digest::{
    consts::{U5, U64},
    typenum::{IsLessOrEqual, LeEq, NonZero},
};
use generic_array::{ArrayLength, GenericArray};
use serde::Deserialize;
use serde_bytes::Bytes;
use sodiumoxide::{
    crypto::{
        pwhash::argon2id13::{derive_key, MemLimit, OpsLimit, Salt, SALTBYTES},
        secretbox::{
            gen_nonce, open, seal,
            xsalsa20poly1305::{gen_key, Key, KEYBYTES, MACBYTES, NONCEBYTES},
            Nonce,
        },
    },
    randombytes::randombytes,
};

use crate::{CryptoError, Password};

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
    fn mac<Size>(&self, data: &[u8]) -> GenericArray<u8, Size>
    where
        Size: ArrayLength<u8> + IsLessOrEqual<U64>,
        LeEq<Size, U64>: NonZero,
    {
        let mut state = libsodium_sys::crypto_generichash_blake2b_state {
            opaque: [0u8; 384usize],
        };
        // TODO: replace `from_exact_iter` by `from_array` once generic-array is updated to v1.0+
        let mut out =
            GenericArray::<u8, Size>::from_exact_iter(std::iter::repeat(0u8).take(Size::USIZE))
                .expect("correct iterator size");

        // SAFETY: Sodiumoxide doesn't expose those methods, so we have to access
        // the libsodium C API directly.
        // this remains safe because we provide bounds defined in Rust land when passing vectors.
        // The only data structure provided by remote code is dropped
        // at the end of the function.
        unsafe {
            libsodium_sys::crypto_generichash_blake2b_init(
                &mut state,
                self.as_ref().as_ptr(),
                self.as_ref().len(),
                Size::USIZE,
            );
            libsodium_sys::crypto_generichash_blake2b_update(
                &mut state,
                data.as_ptr(),
                data.len() as u64,
            );
            libsodium_sys::crypto_generichash_blake2b_final(
                &mut state,
                out.as_mut_ptr(),
                Size::USIZE,
            );
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
        randombytes(SALTBYTES)
    }

    pub fn from_argon2id_password(
        password: &Password,
        salt: &[u8],
        opslimit: u32,
        memlimit_kb: u32,
        parallelism: u32,
    ) -> Result<Self, CryptoError> {
        let mut key = [0; KEYBYTES];

        // Libsodium only support parallelism of 1
        if parallelism != 1 {
            return Err(CryptoError::DataSize);
        }

        let salt = Salt::from_slice(salt).ok_or(CryptoError::DataSize)?;

        let opslimit = OpsLimit(opslimit.try_into().map_err(|_| CryptoError::DataSize)?);
        let memlimit_kb: usize = memlimit_kb.try_into().map_err(|_| CryptoError::DataSize)?;
        let memlimit = MemLimit(memlimit_kb * 1024);

        derive_key(&mut key, password.as_bytes(), &salt, opslimit, memlimit)
            .map_err(|_| CryptoError::DataSize)?;

        Ok(Self::from(key))
    }
}

impl From<[u8; KEYBYTES]> for SecretKey {
    fn from(key: [u8; KEYBYTES]) -> Self {
        Self(Key(key))
    }
}

crate::common::impl_secret_key!(SecretKey);
