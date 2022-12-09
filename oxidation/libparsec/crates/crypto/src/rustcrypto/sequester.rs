// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use ed25519_dalek::Verifier;
use rsa::{
    pkcs8::{
        der::zeroize::Zeroizing, DecodePrivateKey, DecodePublicKey, EncodePrivateKey,
        EncodePublicKey,
    },
    pss::{Signature, SigningKey, VerifyingKey},
    signature::RandomizedSigner,
    PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey,
};
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;
use sha2::Sha256;

use crate::{CryptoError, CryptoResult, EnforceDeserialize, EnforceSerialize, SecretKey};

/*
 * PrivateKey
 */

#[derive(Clone, PartialEq, Eq)]
pub struct SequesterPrivateKeyDer(RsaPrivateKey);

crate::impl_key_debug!(SequesterPrivateKeyDer);

impl Default for SequesterPrivateKeyDer {
    fn default() -> Self {
        let mut rng = rand_08::thread_rng();
        Self(RsaPrivateKey::new(&mut rng, Self::SIZE_IN_BITS).expect("Unreachable"))
    }
}

impl TryFrom<&[u8]> for SequesterPrivateKeyDer {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        RsaPrivateKey::from_pkcs8_der(bytes)
            .map(Self)
            .map_err(|err| CryptoError::SequesterPrivateKeyDer(err.to_string()))
    }
}

impl EnforceDeserialize for SequesterPrivateKeyDer {
    const SIZE_IN_BITS: usize = Self::SIZE_IN_BITS;
    const ALGORITHM: &'static [u8] = b"RSAES-OAEP-XSALSA20-POLY1305";
}

impl SequesterPrivateKeyDer {
    #[cfg(test)]
    pub const SIZE_IN_BITS: usize = 1024;
    #[cfg(not(test))]
    pub const SIZE_IN_BITS: usize = 4096;

    pub fn dump(&self) -> Zeroizing<Vec<u8>> {
        self.0.to_pkcs8_der().expect("Unreachable").to_bytes()
    }

    pub fn dump_pem(&self) -> Zeroizing<String> {
        self.0
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::default())
            .expect("Unreachable")
    }

    pub fn load_pem(s: &str) -> CryptoResult<Self> {
        RsaPrivateKey::from_pkcs8_pem(s)
            .map(Self)
            .map_err(|err| CryptoError::SequesterPrivateKeyDer(err.to_string()))
    }

    pub fn decrypt(&self, data: &[u8]) -> CryptoResult<Vec<u8>> {
        let (cipherkey, ciphertext) = Self::deserialize(data)?;
        let padding = PaddingScheme::new_oaep::<Sha256>();

        let clearkey = SecretKey::try_from(
            &self
                .0
                .decrypt(padding, cipherkey)
                .map_err(|_| CryptoError::Decryption)?[..],
        )?;

        clearkey.decrypt(ciphertext)
    }
}

/*
 * PublicKey
 */

#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct SequesterPublicKeyDer(RsaPublicKey);

crate::impl_key_debug!(SequesterPublicKeyDer);

impl From<SequesterPrivateKeyDer> for SequesterPublicKeyDer {
    fn from(priv_key: SequesterPrivateKeyDer) -> Self {
        Self(RsaPublicKey::from(priv_key.0))
    }
}

impl EnforceSerialize for SequesterPublicKeyDer {
    const SIZE_IN_BITS: usize = Self::SIZE_IN_BITS;
    const ALGORITHM: &'static [u8] = b"RSAES-OAEP-XSALSA20-POLY1305";
}

impl SequesterPublicKeyDer {
    #[cfg(test)]
    pub const SIZE_IN_BITS: usize = 1024;
    #[cfg(not(test))]
    pub const SIZE_IN_BITS: usize = 4096;

    pub fn dump(&self) -> Vec<u8> {
        self.0.to_public_key_der().expect("Unreachable").into_vec()
    }

    pub fn dump_pem(&self) -> String {
        self.0
            .to_public_key_pem(rsa::pkcs8::LineEnding::default())
            .expect("Unreachable")
    }

    pub fn load_pem(s: &str) -> CryptoResult<Self> {
        RsaPublicKey::from_public_key_pem(s)
            .map(Self)
            .map_err(|err| CryptoError::SequesterPublicKeyDer(err.to_string()))
    }

    // Encryption format:
    //   <algorithm name>:<encrypted secret key with RSA key><encrypted data with secret key>
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut rng = rand_08::thread_rng();
        let padding = PaddingScheme::new_oaep::<Sha256>();
        let secret_key = SecretKey::generate();
        let secret_key_encrypted = self
            .0
            .encrypt(&mut rng, padding, secret_key.as_ref())
            .expect("Unreachable");

        EnforceSerialize::serialize(self, &secret_key_encrypted, &secret_key.encrypt(data))
    }
}

impl TryFrom<&[u8]> for SequesterPublicKeyDer {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        RsaPublicKey::from_public_key_der(bytes)
            .map(Self)
            .map_err(|err| CryptoError::SequesterPublicKeyDer(err.to_string()))
    }
}

impl TryFrom<&Bytes> for SequesterPublicKeyDer {
    type Error = CryptoError;

    fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
    }
}

impl Serialize for SequesterPublicKeyDer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.dump())
    }
}

/*
 * SigningKey
 */

#[derive(Clone)]
pub struct SequesterSigningKeyDer(SigningKey<Sha256>);

crate::impl_key_debug!(SequesterSigningKeyDer);

impl EnforceSerialize for SequesterSigningKeyDer {
    const SIZE_IN_BITS: usize = Self::SIZE_IN_BITS;
    const ALGORITHM: &'static [u8] = b"RSASSA-PSS-SHA256";
}

impl From<SequesterPrivateKeyDer> for SequesterSigningKeyDer {
    fn from(priv_key: SequesterPrivateKeyDer) -> Self {
        Self(SigningKey::from(priv_key.0))
    }
}

impl SequesterSigningKeyDer {
    #[cfg(test)]
    pub const SIZE_IN_BITS: usize = 1024;
    #[cfg(not(test))]
    pub const SIZE_IN_BITS: usize = 4096;

    pub fn dump(&self) -> Zeroizing<Vec<u8>> {
        self.0.to_pkcs8_der().expect("Unreachable").to_bytes()
    }

    pub fn dump_pem(&self) -> Zeroizing<String> {
        self.0
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::default())
            .expect("Unreachable")
    }

    pub fn load_pem(s: &str) -> CryptoResult<Self> {
        RsaPrivateKey::from_pkcs8_pem(s)
            .map(|x| Self(SigningKey::from(x)))
            .map_err(|err| CryptoError::SequesterPrivateKeyDer(err.to_string()))
    }

    // Signature format:
    //   <algorithm name>:<signature><data>
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let mut rng = rand_08::thread_rng();
        let signature = self.0.sign_with_rng(&mut rng, data);

        self.serialize(signature.as_ref(), data)
    }
}

/*
 * VerifyKey
 */

#[derive(Clone, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct SequesterVerifyKeyDer(VerifyingKey<Sha256>);

crate::impl_key_debug!(SequesterVerifyKeyDer);

impl From<SequesterPublicKeyDer> for SequesterVerifyKeyDer {
    fn from(pub_key: SequesterPublicKeyDer) -> Self {
        Self(VerifyingKey::from(pub_key.0))
    }
}

impl PartialEq for SequesterVerifyKeyDer {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref() == other.0.as_ref()
    }
}

impl Eq for SequesterVerifyKeyDer {}

impl EnforceDeserialize for SequesterVerifyKeyDer {
    const SIZE_IN_BITS: usize = Self::SIZE_IN_BITS;
    const ALGORITHM: &'static [u8] = b"RSASSA-PSS-SHA256";
}

impl SequesterVerifyKeyDer {
    #[cfg(test)]
    pub const SIZE_IN_BITS: usize = 1024;
    #[cfg(not(test))]
    pub const SIZE_IN_BITS: usize = 4096;

    pub fn dump(&self) -> Vec<u8> {
        self.0.to_public_key_der().expect("Unreachable").into_vec()
    }

    pub fn dump_pem(&self) -> String {
        self.0
            .to_public_key_pem(rsa::pkcs8::LineEnding::default())
            .expect("Unreachable")
    }

    pub fn load_pem(s: &str) -> CryptoResult<Self> {
        RsaPublicKey::from_public_key_pem(s)
            .map(|x| Self(VerifyingKey::from(x)))
            .map_err(|err| CryptoError::SequesterPublicKeyDer(err.to_string()))
    }

    pub fn verify(&self, data: &[u8]) -> CryptoResult<Vec<u8>> {
        let (signature, data) = <Self as EnforceDeserialize>::deserialize(data)?;

        // TODO: It Seems to be a mistake from RustCrypto/RSA
        // Why should we allocate there ?
        self.0
            .verify(data, &Signature::from(signature.to_vec()))
            .map_err(|_| CryptoError::SignatureVerification)?;

        Ok(data.to_vec())
    }
}

impl TryFrom<&[u8]> for SequesterVerifyKeyDer {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        RsaPublicKey::from_public_key_der(bytes)
            .map(|x| Self(VerifyingKey::from(x)))
            .map_err(|err| CryptoError::SequesterPublicKeyDer(err.to_string()))
    }
}

impl TryFrom<&Bytes> for SequesterVerifyKeyDer {
    type Error = CryptoError;

    fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
    }
}

impl Serialize for SequesterVerifyKeyDer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.dump())
    }
}
