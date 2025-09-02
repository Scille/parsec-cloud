// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libsodium_rs::{
    crypto_box::{
        self, curve25519xsalsa20poly1305, open_sealed_box, seal_box, PUBLICKEYBYTES, SECRETKEYBYTES,
    },
    crypto_scalarmult::curve25519,
};
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;

use crate::{CryptoError, SecretKey};

/*
 * PrivateKey
 */

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct PrivateKey(curve25519xsalsa20poly1305::SecretKey);

crate::impl_key_debug!(PrivateKey);

super::impl_bytes_traits!(PrivateKey, curve25519xsalsa20poly1305::SecretKey);

impl PrivateKey {
    pub const ALGORITHM: &'static str = "curve25519blake2bxsalsa20poly1305";
    pub const SIZE: usize = SECRETKEYBYTES;

    pub fn public_key(&self) -> PublicKey {
        curve25519::scalarmult_base(self.0.as_bytes())
            .map(PublicKey::from)
            .expect("Cannot obtain public key from secret key")
    }

    pub fn generate() -> Self {
        Self(
            curve25519xsalsa20poly1305::KeyPair::generate()
                .expect("Failed to generate keypair")
                .secret_key,
        )
    }

    pub fn decrypt_from_self(&self, ciphered: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let pk = crypto_box::PublicKey::from_bytes_exact(self.public_key().to_bytes());
        let sk = crypto_box::SecretKey::from_bytes_exact(self.to_bytes());
        open_sealed_box(ciphered, &pk, &sk).map_err(|_| CryptoError::Decryption)
    }

    pub fn generate_shared_secret_key(
        &self,
        peer_public_key: &PublicKey,
    ) -> Result<SecretKey, CryptoError> {
        // libsodium_rs::crypto_kx::server_session_keys(server_pk, server_sk, client_pk)
        // let curve25519::scalarmult(self.0.as_bytes(), peer_public_key.0.as_bytes())
        //     .map(SecretKey::from)
        //     .expect("Failed to compute shared key");

        curve25519::scalarmult(self.0.as_bytes(), peer_public_key.0.as_bytes())
            .map(SecretKey::from)
            .map_err(|e| CryptoError::SharedSecretKey(e.to_string()))
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        self.0.as_bytes().to_owned()
    }
}

super::impl_serde_traits!(PrivateKey);

/*
 * PublicKey
 */

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct PublicKey(curve25519xsalsa20poly1305::PublicKey);

crate::impl_key_debug!(PublicKey);

impl std::hash::Hash for PublicKey {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.as_ref().hash(state)
    }
}

super::impl_bytes_traits!(PublicKey, curve25519xsalsa20poly1305::PublicKey);

impl PublicKey {
    pub const ALGORITHM: &'static str = "curve25519blake2bxsalsa20poly1305";
    pub const SIZE: usize = PUBLICKEYBYTES;

    pub fn encrypt_for_self(&self, data: &[u8]) -> Vec<u8> {
        let pk = crypto_box::PublicKey::from(self.to_bytes());
        seal_box(data, &pk).expect("Cannot encrypt data")
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        self.0.as_bytes().to_owned()
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
