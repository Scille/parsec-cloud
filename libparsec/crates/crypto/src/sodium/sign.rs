// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libsodium_rs::crypto_sign::{
    sign, sign_detached, verify_detached, KeyPair, PublicKey, SecretKey, BYTES as SIGNATUREBYTES,
    PUBLICKEYBYTES, SECRETKEYBYTES, SEEDBYTES,
};
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;

use crate::CryptoError;

/*
 * SigningKey
 */

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct SigningKey(SecretKey);

crate::impl_key_debug!(SigningKey);

impl SigningKey {
    pub const ALGORITHM: &'static str = "ed25519";
    /// Size of the secret key.
    ///
    /// The secret key is compose of secret + public part, the size correspond to the secret part
    pub const SIZE: usize = SECRETKEYBYTES - PUBLICKEYBYTES;
    pub const SIGNATURE_SIZE: usize = SIGNATUREBYTES;

    pub fn verify_key(&self) -> VerifyKey {
        VerifyKey(
            PublicKey::from_secret_key(&self.0).expect("Cannot get public key from secret key"),
        )
    }

    pub fn generate() -> Self {
        Self(
            KeyPair::generate()
                .expect("Cannot generate signing key")
                .secret_key,
        )
    }

    /// Sign the message and prefix it with the signature.
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        sign(data, &self.0).expect("Cannot sign data")
    }

    /// Sign the message and return only the signature.
    pub fn sign_only_signature(&self, data: &[u8]) -> [u8; Self::SIGNATURE_SIZE] {
        sign_detached(data, &self.0).expect("Cannot sign data")
    }

    pub fn to_bytes(&self) -> zeroize::Zeroizing<[u8; Self::SIZE]> {
        // SecretKey is composed of Seed then PublicKey, we export only seed here
        <[u8; Self::SIZE]>::try_from(&self.0.as_bytes()[..Self::SIZE])
            .expect("The internal array is > Self::SIZE")
            .into()
    }
}

impl TryFrom<&[u8]> for SigningKey {
    type Error = CryptoError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        <[u8; SEEDBYTES]>::try_from(data)
            .map_err(|_| CryptoError::DataSize)
            .and_then(|raw| KeyPair::from_seed(raw.as_slice()).map_err(|_| CryptoError::DataSize))
            .map(|kp| Self(kp.secret_key))
    }
}

impl From<[u8; Self::SIZE]> for SigningKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        KeyPair::from_seed(key.as_slice())
            .map(|kp| Self(kp.secret_key))
            .expect("Cannot generate keypair from seed")
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
        serializer.serialize_bytes(self.to_bytes().as_ref())
    }
}

/*
 * VerifyKey
 */

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct VerifyKey(PublicKey);

crate::impl_key_debug!(VerifyKey);

impl std::hash::Hash for VerifyKey {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write(self.as_ref())
    }
}

impl TryFrom<&[u8]> for VerifyKey {
    type Error = CryptoError;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        <[u8; Self::SIZE]>::try_from(v)
            .map(PublicKey::from)
            .map(Self)
            .map_err(|_| CryptoError::DataSize)
    }
}

impl From<[u8; Self::SIZE]> for VerifyKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        Self(PublicKey::from(key))
    }
}

impl AsRef<[u8]> for VerifyKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

super::impl_serde_traits!(VerifyKey);

impl VerifyKey {
    pub const ALGORITHM: &'static str = "ed25519";
    /// Size of the public key.
    pub const SIZE: usize = PUBLICKEYBYTES;

    /// Unwrap a signed blob that use the following format: `SIGNATURE | DATA`
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
        match verify_detached(raw_signature, message, &self.0) {
            true => Ok(()),
            false => Err(CryptoError::SignatureVerification),
        }
    }
}
