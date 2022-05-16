// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use sodiumoxide::crypto::sign::{
    ed25519, gen_keypair, sign, verify, PUBLICKEYBYTES, SEEDBYTES, SIGNATUREBYTES,
};

use crate::CryptoError;

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(into = "ByteBuf", try_from = "ByteBuf")]
pub struct SigningKey(ed25519::SecretKey);

crate::macros::impl_key_debug!(SigningKey);

impl SigningKey {
    pub const ALGORITHM: &'static str = "ed25519";
    // SEEDBYTES is the size of the private key alone where SECRETKEYBYTES also contains the public part
    pub const SIZE: usize = SEEDBYTES;

    pub fn verify_key(&self) -> VerifyKey {
        VerifyKey::try_from(self.0.public_key().0).unwrap()
    }

    pub fn generate() -> Self {
        Self(gen_keypair().1)
    }

    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        sign(data, &self.0)
    }
}

impl TryFrom<&[u8]> for SigningKey {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // if you wonder, `try_into` will also fail if data is too small
        let arr: [u8; Self::SIZE] = data.try_into().map_err(|_| CryptoError::DataSize)?;
        let (_, sk) = ed25519::keypair_from_seed(&ed25519::Seed(arr));
        Ok(Self(sk))
    }
}

impl From<[u8; Self::SIZE]> for SigningKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        // TODO: zerocopy
        let (_, sk) = ed25519::keypair_from_seed(&ed25519::Seed(key));
        Self(sk)
    }
}

impl AsRef<[u8]> for SigningKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        // SecretKey is composed of Seed then PublicKey, we export only seed here
        &self.0 .0[..Self::SIZE]
    }
}

impl TryFrom<ByteBuf> for SigningKey {
    type Error = CryptoError;
    fn try_from(data: ByteBuf) -> Result<Self, Self::Error> {
        // TODO: zerocopy
        Self::try_from(data.into_vec().as_ref())
    }
}

impl From<SigningKey> for ByteBuf {
    fn from(data: SigningKey) -> Self {
        Self::from(data.as_ref())
    }
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VerifyKey(ed25519::PublicKey);

crate::macros::impl_key_debug!(VerifyKey);

super::utils::impl_try_from!(VerifyKey, ed25519::PublicKey);

impl VerifyKey {
    pub const ALGORITHM: &'static str = "ed25519";
    pub const SIZE: usize = PUBLICKEYBYTES;

    pub fn unsecure_unwrap(signed: &[u8]) -> Option<&[u8]> {
        signed.get(SIGNATUREBYTES..)
    }

    pub fn verify(&self, signed: &[u8]) -> Result<Vec<u8>, CryptoError> {
        verify(signed, &self.0).or(Err(CryptoError::SignatureVerification))
    }
}

impl AsRef<[u8]> for VerifyKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
}
