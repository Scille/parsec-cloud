// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_::{
    curve25519xsalsa20poly1305, gen_keypair, PUBLICKEYBYTES, SECRETKEYBYTES,
};
use sodiumoxide::crypto::scalarmult::{scalarmult, GroupElement, Scalar};
use sodiumoxide::crypto::sealedbox::{open, seal};

use crate::{CryptoError, SecretKey};

/*
 * PrivateKey
 */

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PrivateKey(curve25519xsalsa20poly1305::SecretKey);

crate::macros::impl_key_debug!(PrivateKey);

super::utils::impl_try_from!(PrivateKey, curve25519xsalsa20poly1305::SecretKey);

impl PrivateKey {
    pub const ALGORITHM: &'static str = "curve25519blake2bxsalsa20poly1305";
    pub const SIZE: usize = SECRETKEYBYTES;

    pub fn public_key(&self) -> PublicKey {
        PublicKey::try_from(self.0.public_key().0).unwrap()
    }

    pub fn generate() -> Self {
        Self(gen_keypair().1)
    }

    pub fn decrypt_from_self(&self, ciphered: &[u8]) -> Result<Vec<u8>, CryptoError> {
        open(ciphered, &self.0.public_key(), &self.0).or(Err(CryptoError::Decryption))
    }

    pub fn generate_shared_secret_key(&self, peer_public_key: &PublicKey) -> SecretKey {
        let scalar = Scalar(self.as_ref().try_into().unwrap());
        let group_element = GroupElement(peer_public_key.as_ref().try_into().unwrap());
        let mult = scalarmult(&scalar, &group_element).unwrap();
        // TODO: too many copies...
        let x: [u8; 32] = mult.as_ref().try_into().unwrap();
        SecretKey::from(x)
    }
}

impl AsRef<[u8]> for PrivateKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
}

/*
 * PublicKey
 */

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PublicKey(curve25519xsalsa20poly1305::PublicKey);

crate::macros::impl_key_debug!(PublicKey);

super::utils::impl_try_from!(PublicKey, curve25519xsalsa20poly1305::PublicKey);

impl PublicKey {
    pub const ALGORITHM: &'static str = "curve25519blake2bxsalsa20poly1305";
    pub const SIZE: usize = PUBLICKEYBYTES;

    pub fn encrypt_for_self(&self, data: &[u8]) -> Vec<u8> {
        seal(data, &self.0)
    }
}

impl AsRef<[u8]> for PublicKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
}
