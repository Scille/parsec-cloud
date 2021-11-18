// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

// TODO: This should be part of crypto_box crate, provide a PR ?
// Taken verbatim from https://github.com/Karrq/sealed_box/blob/master/src/lib.rs
mod sealed_box {

    use blake2::{
        digest::{Update, VariableOutput},
        VarBlake2b,
    };
    use crypto_box::{
        aead::{self, Aead},
        SalsaBox,
    };

    //re-export keys
    pub use crypto_box::{PublicKey, SecretKey};

    const BOX_NONCELENGTH: usize = 24;
    const BOX_OVERHEAD: usize = 16;

    //32 = PublicKey length
    const SEALED_OVERHEAD: usize = 32 + BOX_OVERHEAD;

    ///generate the nonce for the given public keys
    ///
    /// nonce = Blake2b(ephemeral_pk||target_pk)
    /// nonce_length = 24
    fn get_nonce(ephemeral_pk: &PublicKey, target_pk: &PublicKey) -> [u8; BOX_NONCELENGTH] {
        let mut hasher = VarBlake2b::new(BOX_NONCELENGTH).unwrap();

        hasher.update(ephemeral_pk.as_bytes());
        hasher.update(target_pk.as_bytes());

        let out = hasher.finalize_boxed();

        let mut array = [0u8; BOX_NONCELENGTH];
        array.copy_from_slice(&out);

        array
    }

    ///encrypts the given buffer for the given public key
    ///
    /// overhead = 48 = (32 ephemeral_pk||16 box_overhead)
    pub fn seal(data: &[u8], pk: &PublicKey) -> Vec<u8> {
        let mut out = Vec::with_capacity(SEALED_OVERHEAD + data.len());

        let mut rng = crypto_box::rand_core::OsRng;
        let ep_sk = SecretKey::generate(&mut rng);
        let ep_pk = ep_sk.public_key();
        out.extend_from_slice(ep_pk.as_bytes());

        let nonce = get_nonce(&ep_pk, &pk);
        let nonce = aead::generic_array::GenericArray::from_slice(&nonce);

        let salsabox = SalsaBox::new(&pk, &ep_sk);
        let encrypted = salsabox.encrypt(&nonce, &data[..]).unwrap();

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
            array.copy_from_slice(&bytes);
            array.into()
        };

        let nonce = get_nonce(&ephemeral_pk, &pk);
        let nonce = aead::generic_array::GenericArray::from_slice(&nonce);

        let encrypted = &ciphertext[32..];
        let salsabox = SalsaBox::new(&ephemeral_pk, &sk);

        salsabox.decrypt(&nonce, &encrypted[..]).ok()
    }
}

use crypto_box::KEY_SIZE;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::convert::TryInto;
use x25519_dalek::x25519;

use crate::SecretKey;

/*
 * PrivateKey
 */

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(into = "ByteBuf", try_from = "ByteBuf")]
pub struct PrivateKey(crypto_box::SecretKey);

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
        let mut rng = crypto_box::rand_core::OsRng;
        Self(crypto_box::SecretKey::generate(&mut rng))
    }

    pub fn decrypt_from_self(&self, ciphered: &[u8]) -> Result<Vec<u8>, &'static str> {
        sealed_box::open(ciphered, &self.0).ok_or("Decryption error")
    }

    pub fn generate_shared_secret_key(&self, peer_public_key: &PublicKey) -> SecretKey {
        SecretKey::from(x25519(
            self.0.to_bytes(),
            peer_public_key.0.as_bytes().to_owned(),
        ))
    }
}

impl AsRef<[u8]> for PrivateKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        // Unsafe on private key, what could possibly go wrong ?
        // TODO: crypto_box::SecretKey should implement AsRef<[u8]> instead
        unsafe {
            ::std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                ::std::mem::size_of::<Self>(),
            )
        }
    }
}

impl TryFrom<&[u8]> for PrivateKey {
    type Error = &'static str;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let key: [u8; KEY_SIZE] = data.try_into().map_err(|_| ("Invalid data size"))?;
        Ok(Self(crypto_box::SecretKey::from(key)))
    }
}

impl From<[u8; Self::SIZE]> for PrivateKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        // TODO: zerocopy
        Self::try_from(key.as_ref()).unwrap()
    }
}

impl TryFrom<ByteBuf> for PrivateKey {
    type Error = &'static str;
    fn try_from(data: ByteBuf) -> Result<Self, Self::Error> {
        let key: [u8; KEY_SIZE] = data
            .to_vec()
            .try_into()
            .map_err(|_| ("Invalid data size"))?;
        Ok(Self(crypto_box::SecretKey::from(key)))
    }
}

impl From<PrivateKey> for ByteBuf {
    fn from(data: PrivateKey) -> Self {
        Self::from(data.0.to_bytes())
    }
}

/*
 * PublicKey
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(into = "ByteBuf", try_from = "ByteBuf")]
pub struct PublicKey(crypto_box::PublicKey);

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}
impl Eq for PublicKey {}

impl PublicKey {
    pub const ALGORITHM: &'static str = "curve25519blake2bxsalsa20poly1305";
    pub const SIZE: usize = KEY_SIZE;

    pub fn encrypt_from_self(&self, data: &[u8]) -> Vec<u8> {
        sealed_box::seal(data, &self.0)
    }
}

impl AsRef<[u8]> for PublicKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = &'static str;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let key: [u8; KEY_SIZE] = data.try_into().map_err(|_| ("Invalid data size"))?;
        Ok(Self(crypto_box::PublicKey::from(key)))
    }
}

impl From<[u8; Self::SIZE]> for PublicKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        // TODO: zerocopy
        Self::try_from(key.as_ref()).unwrap()
    }
}

impl TryFrom<ByteBuf> for PublicKey {
    type Error = &'static str;
    fn try_from(data: ByteBuf) -> Result<Self, Self::Error> {
        let key: [u8; KEY_SIZE] = data
            .to_vec()
            .try_into()
            .map_err(|_| ("Invalid data size"))?;
        Ok(Self(crypto_box::PublicKey::from(key)))
    }
}

impl From<PublicKey> for ByteBuf {
    fn from(data: PublicKey) -> Self {
        // TODO: convert to Bytes instead of ByteBuf for zerocopy ?
        Self::from(data.0.as_bytes().to_owned())
    }
}
