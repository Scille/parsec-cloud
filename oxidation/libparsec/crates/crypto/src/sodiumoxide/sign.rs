// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;
use sodiumoxide::crypto::sign::{ed25519, gen_keypair, sign, verify, verify_detached, Signature};

use crate::CryptoError;

/*
 * SigningKey
 */

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct SigningKey(ed25519::SecretKey);

crate::impl_key_debug!(SigningKey);

impl SigningKey {
    pub const ALGORITHM: &'static str = "ed25519";
    /// Size of the secret key.
    /// `SEEDBYTES` is the size of the private key alone where `SECRETKEYBYTES` also contains the public part
    pub const SIZE: usize = ed25519::SEEDBYTES;
    pub const SIGNATURE_SIZE: usize = ed25519::SIGNATUREBYTES;

    pub fn verify_key(&self) -> VerifyKey {
        VerifyKey::try_from(self.0.public_key().0).unwrap()
    }

    pub fn generate() -> Self {
        Self(gen_keypair().1)
    }

    /// Sign the message and prefix it with the signature.
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        sign(data, &self.0)
    }

    /// Sign the message and return only the signature.
    pub fn sign_only_signature(&self, data: &[u8]) -> [u8; Self::SIGNATURE_SIZE] {
        use sodiumoxide::crypto::sign::Signer;

        self.0.sign(data).to_bytes()
    }
}

impl TryFrom<&[u8]> for SigningKey {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let arr: [u8; Self::SIZE] = data.try_into().map_err(|_| CryptoError::DataSize)?;
        let (_, sk) = ed25519::keypair_from_seed(&ed25519::Seed(arr));
        Ok(Self(sk))
    }
}

impl From<[u8; Self::SIZE]> for SigningKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        // TODO: zero copy
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

impl TryFrom<&Bytes> for SigningKey {
    type Error = CryptoError;
    fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
    }
}

impl Serialize for SigningKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.as_ref())
    }
}

/*
 * VerifyKey
 */

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct VerifyKey(ed25519::PublicKey);

crate::impl_key_debug!(VerifyKey);

super::utils::impl_try_from!(VerifyKey, ed25519::PublicKey);

impl VerifyKey {
    pub const ALGORITHM: &'static str = "ed25519";
    /// Size of the public key.
    pub const SIZE: usize = ed25519::PUBLICKEYBYTES;

    pub fn unsecure_unwrap(signed: &[u8]) -> Option<&[u8]> {
        signed.get(SigningKey::SIGNATURE_SIZE..)
    }

    pub fn verify(&self, signed: &[u8]) -> Result<Vec<u8>, CryptoError> {
        verify(signed, &self.0).or(Err(CryptoError::SignatureVerification))
    }

    /// Verify a signature using the given [VerifyKey], `signature` and `message`
    pub fn verify_with_signature(
        &self,
        raw_signature: [u8; SigningKey::SIGNATURE_SIZE],
        message: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        let signature =
            Signature::from_bytes(&raw_signature).map_err(|_| CryptoError::Signature)?;
        if !verify_detached(&signature, message, &self.0) {
            return Err(CryptoError::SignatureVerification);
        }
        Ok(message.into())
    }
}

impl AsRef<[u8]> for VerifyKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
}

impl TryFrom<&Bytes> for VerifyKey {
    type Error = CryptoError;
    fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
    }
}

impl Serialize for VerifyKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.as_ref())
    }
}
