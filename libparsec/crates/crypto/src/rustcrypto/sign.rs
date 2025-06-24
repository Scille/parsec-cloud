// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use ed25519_dalek::{Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;

use crate::CryptoError;

/*
 * SigningKey
 */

#[derive(Deserialize, Clone)]
#[serde(try_from = "&Bytes")]
pub struct SigningKey(ed25519_dalek::SigningKey);

crate::impl_key_debug!(SigningKey);

impl PartialEq for SigningKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bytes() == other.0.to_bytes()
    }
}

impl Eq for SigningKey {}

impl SigningKey {
    pub const ALGORITHM: &'static str = "ed25519";
    /// Size of the secret key.
    pub const SIZE: usize = ed25519_dalek::SECRET_KEY_LENGTH;
    /// Size of the signature generate by the signing key.
    pub const SIGNATURE_SIZE: usize = ed25519_dalek::SIGNATURE_LENGTH;

    pub fn verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.verifying_key())
    }

    pub fn generate() -> Self {
        Self(ed25519_dalek::SigningKey::generate(&mut OsRng))
    }

    /// Sign the message and prefix it with the signature.
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let mut signed = Vec::with_capacity(Self::SIGNATURE_SIZE + data.len());
        signed.extend(self.0.sign(data).to_bytes());
        signed.extend_from_slice(data);
        signed
    }

    /// Sign the message and return only the signature.
    pub fn sign_only_signature(&self, data: &[u8]) -> [u8; Self::SIGNATURE_SIZE] {
        self.0.sign(data).to_bytes()
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        self.0.to_bytes()
    }
}

impl TryFrom<&[u8]> for SigningKey {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        ed25519_dalek::SigningKey::try_from(data)
            .map(Self)
            .map_err(|_| Self::Error::DataSize)
    }
}

impl From<[u8; Self::SIZE]> for SigningKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        // TODO: zero copy
        Self(ed25519_dalek::SigningKey::from(key))
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
        serializer.serialize_bytes(&self.to_bytes())
    }
}

/*
 * VerifyKey
 */

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct VerifyKey(ed25519_dalek::VerifyingKey);

crate::impl_key_debug!(VerifyKey);

impl std::hash::Hash for VerifyKey {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.as_ref().hash(state)
    }
}

impl VerifyKey {
    pub const ALGORITHM: &'static str = "ed25519";
    /// Size of the public key.
    pub const SIZE: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;

    pub fn unsecure_unwrap(
        signed: &[u8],
    ) -> Result<(&[u8; SigningKey::SIGNATURE_SIZE], &[u8]), CryptoError> {
        if signed.len() < SigningKey::SIGNATURE_SIZE {
            return Err(CryptoError::Signature);
        }
        let signature = signed[..SigningKey::SIGNATURE_SIZE]
            .try_into()
            .map_err(|_e| CryptoError::Signature)?;
        let message = &signed[SigningKey::SIGNATURE_SIZE..];
        Ok((signature, message))
    }

    /// Verify a message using the given [VerifyKey].
    /// `signed` value is the concatenation of the `signature` + the signed `data`
    pub fn verify<'a>(&self, signed: &'a [u8]) -> Result<&'a [u8], CryptoError> {
        let (signature, message) = Self::unsecure_unwrap(signed)?;
        self.verify_with_signature(signature, message)?;
        Ok(message)
    }

    /// Verify a signature using the given [VerifyKey], `signature` and `message`
    pub fn verify_with_signature(
        &self,
        raw_signature: &[u8; SigningKey::SIGNATURE_SIZE],
        message: &[u8],
    ) -> Result<(), CryptoError> {
        let signature = Signature::from_bytes(raw_signature);
        self.0
            .verify(message, &signature)
            .map_err(|_| CryptoError::SignatureVerification)?;
        Ok(())
    }
}

impl AsRef<[u8]> for VerifyKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl TryFrom<&[u8]> for VerifyKey {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        ed25519_dalek::VerifyingKey::try_from(data)
            .map(Self)
            .map_err(|_| Self::Error::DataSize)
    }
}

impl TryFrom<[u8; Self::SIZE]> for VerifyKey {
    type Error = CryptoError;
    fn try_from(data: [u8; Self::SIZE]) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
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
