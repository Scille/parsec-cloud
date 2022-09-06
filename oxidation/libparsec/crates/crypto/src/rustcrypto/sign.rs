// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;

use crate::CryptoError;

/*
 * SigningKey
 */

#[derive(Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct SigningKey(Keypair);

crate::macros::impl_key_debug!(SigningKey);

impl Clone for SigningKey {
    fn clone(&self) -> Self {
        Self::try_from(self.as_ref()).unwrap()
    }
}

impl PartialEq for SigningKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
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
        VerifyKey(self.0.public)
    }

    pub fn generate() -> Self {
        Self(Keypair::generate(&mut rand_07::thread_rng()))
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
}

impl AsRef<[u8]> for SigningKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.secret.as_bytes()
    }
}

impl TryFrom<&[u8]> for SigningKey {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let sk = ed25519_dalek::SecretKey::from_bytes(data).map_err(|_| Self::Error::DataSize)?;
        let pk = ed25519_dalek::PublicKey::from(&sk);
        Ok(Self(Keypair {
            secret: sk,
            public: pk,
        }))
    }
}

impl From<[u8; Self::SIZE]> for SigningKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        // TODO: zerocopy
        Self::try_from(key.as_ref()).unwrap()
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
pub struct VerifyKey(ed25519_dalek::PublicKey);

crate::macros::impl_key_debug!(VerifyKey);

impl VerifyKey {
    pub const ALGORITHM: &'static str = "ed25519";
    /// Size of the public key.
    pub const SIZE: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;

    pub fn unsecure_unwrap(signed: &[u8]) -> Option<&[u8]> {
        signed.get(SigningKey::SIGNATURE_SIZE..)
    }

    /// Verify a message using the given [VerifyKey].
    /// `signed` value is the concatenation of the `signature` + the signed `data`
    pub fn verify(&self, signed: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // Signature::try_from expects a [u8;64] and I have no idea how to get
        // one except by slicing, so we make sure the array is large enough before slicing.
        if signed.len() < Signature::BYTE_SIZE {
            return Err(CryptoError::Signature);
        }
        self.verify_with_signature(
            &signed[..Signature::BYTE_SIZE],
            &signed[Signature::BYTE_SIZE..],
        )
    }

    /// Verify a signature using the given [VerifyKey], `signature` and `message`
    pub fn verify_with_signature(
        &self,
        raw_signature: &[u8],
        message: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        if raw_signature.len() != Signature::BYTE_SIZE {
            return Err(CryptoError::Signature);
        }
        let signature = Signature::try_from(raw_signature)
            .expect("Precondition already checked for the signature size");
        self.0
            .verify(message, &signature)
            .map_err(|_| CryptoError::SignatureVerification)?;
        Ok(message.into())
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
        ed25519_dalek::PublicKey::from_bytes(data)
            .map(Self)
            .map_err(|_| Self::Error::DataSize)
    }
}

impl From<[u8; Self::SIZE]> for VerifyKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        // TODO: zerocopy
        Self::try_from(key.as_ref()).unwrap()
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
