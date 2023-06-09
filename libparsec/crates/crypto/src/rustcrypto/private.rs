// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// TODO: This should be part of crypto_box crate, provide a PR ?
// Taken verbatim from https://github.com/Karrq/sealed_box/blob/master/src/lib.rs
mod sealed_box {

    use blake2::{
        digest::{Update, VariableOutput},
        Blake2bVar,
    };
    use crypto_box::{
        aead::{self, Aead},
        SalsaBox,
    };

    //re-export keys
    pub use crypto_box::{PublicKey, SecretKey};

    const BOX_NONCE_LENGTH: usize = 24;
    const BOX_OVERHEAD: usize = 16;

    //32 = PublicKey length
    const SEALED_OVERHEAD: usize = 32 + BOX_OVERHEAD;

    ///generate the nonce for the given public keys
    ///
    /// nonce = Blake2b(ephemeral_pk||target_pk)
    /// nonce_length = 24
    fn get_nonce(ephemeral_pk: &PublicKey, target_pk: &PublicKey) -> [u8; BOX_NONCE_LENGTH] {
        let mut hasher = Blake2bVar::new(BOX_NONCE_LENGTH).unwrap();

        hasher.update(ephemeral_pk.as_bytes());
        hasher.update(target_pk.as_bytes());

        let out = hasher.finalize_boxed();

        let mut array = [0u8; BOX_NONCE_LENGTH];
        array.copy_from_slice(&out);

        array
    }

    ///encrypts the given buffer for the given public key
    ///
    /// overhead = 48 = (32 ephemeral_pk||16 box_overhead)
    pub fn seal(data: &[u8], pk: &PublicKey) -> Vec<u8> {
        let mut out = Vec::with_capacity(SEALED_OVERHEAD + data.len());

        let ep_sk = SecretKey::generate(&mut rand_08::thread_rng());
        let ep_pk = ep_sk.public_key();
        out.extend_from_slice(ep_pk.as_bytes());

        let nonce = get_nonce(&ep_pk, pk);
        let nonce = aead::generic_array::GenericArray::from_slice(&nonce);

        let salsabox = SalsaBox::new(pk, &ep_sk);
        let encrypted = salsabox.encrypt(nonce, data).unwrap();

        out.extend_from_slice(&encrypted);
        out
    }

    ///attempt to decrypt the given ciphertext with the given secret key
    /// will fail if the secret key doesn't match the public key used to encrypt the payload
    /// or if the ciphertext is not long enough
    pub fn open(ciphertext: &[u8], sk: &SecretKey) -> Option<Vec<u8>> {
        if ciphertext.len() <= 32 {
            //not long enough
            return None;
        }

        let pk = sk.public_key();

        let ephemeral_pk = {
            let bytes = &ciphertext[..32];
            let mut array = [0u8; 32];
            array.copy_from_slice(bytes);
            array.into()
        };

        let nonce = get_nonce(&ephemeral_pk, &pk);
        let nonce = aead::generic_array::GenericArray::from_slice(&nonce);

        let encrypted = &ciphertext[32..];
        let salsabox = SalsaBox::new(&ephemeral_pk, sk);

        salsabox.decrypt(nonce, encrypted).ok()
    }
}

use crypto_box::KEY_SIZE;
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;
use x25519_dalek::x25519;

use crate::{CryptoError, SecretKey};

/*
 * PrivateKey
 */

#[derive(Clone, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct PrivateKey(crypto_box::SecretKey);

crate::impl_key_debug!(PrivateKey);

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for PrivateKey {}

impl PrivateKey {
    pub const ALGORITHM: &'static str = "curve25519blake2bxsalsa20poly1305";
    pub const SIZE: usize = KEY_SIZE;

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public_key())
    }

    pub fn generate() -> Self {
        Self(crypto_box::SecretKey::generate(&mut rand_08::thread_rng()))
    }

    pub fn decrypt_from_self(&self, ciphered: &[u8]) -> Result<Vec<u8>, CryptoError> {
        sealed_box::open(ciphered, &self.0).ok_or(CryptoError::Decryption)
    }

    pub fn generate_shared_secret_key(&self, peer_public_key: &PublicKey) -> SecretKey {
        SecretKey::from(x25519(*self.0.as_bytes(), *peer_public_key.0.as_bytes()))
    }
}

impl AsRef<[u8]> for PrivateKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl TryFrom<&[u8]> for PrivateKey {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        <[u8; Self::SIZE]>::try_from(data)
            .map(Self::from)
            .map_err(|_| CryptoError::DataSize)
    }
}

impl From<[u8; Self::SIZE]> for PrivateKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        Self(key.into())
    }
}

impl TryFrom<&Bytes> for PrivateKey {
    type Error = CryptoError;
    fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
    }
}

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.as_ref())
    }
}

/*
 * PublicKey
 */

#[derive(Clone, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct PublicKey(crypto_box::PublicKey);

crate::impl_key_debug!(PublicKey);

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for PublicKey {}

impl PublicKey {
    pub const ALGORITHM: &'static str = "curve25519blake2bxsalsa20poly1305";
    pub const SIZE: usize = KEY_SIZE;

    pub fn encrypt_for_self(&self, data: &[u8]) -> Vec<u8> {
        sealed_box::seal(data, &self.0)
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        <[u8; Self::SIZE]>::try_from(data)
            .map(Self::from)
            .map_err(|_| CryptoError::DataSize)
    }
}

impl From<[u8; Self::SIZE]> for PublicKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        Self(key.into())
    }
}

impl TryFrom<&Bytes> for PublicKey {
    type Error = CryptoError;
    fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.as_ref())
    }
}
