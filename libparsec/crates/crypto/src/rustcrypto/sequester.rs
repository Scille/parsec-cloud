// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use rand::rngs::SysRng;
use rsa::{
    RsaPublicKey,
    oaep::Oaep,
    pkcs8::{DecodePublicKey, EncodePrivateKey, EncodePublicKey, der::zeroize::Zeroizing},
    pss::{Signature, SigningKey, VerifyingKey},
    rand_core::UnwrapErr,
    sha2::{Digest, Sha256},
    signature::{RandomizedSigner, SignatureEncoding, Verifier},
    traits::PublicKeyParts,
};
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;

use crate::{
    CryptoError, CryptoResult, RsaPrivateKey, SecretKey, SequesterKeySize, deserialize_with_armor,
    serialize_with_armor,
};

/*
 * PrivateKey
 */

#[derive(Clone, PartialEq, Eq)]
pub struct SequesterPrivateKeyDer(RsaPrivateKey);

crate::impl_key_debug!(SequesterPrivateKeyDer);

impl TryFrom<&[u8]> for SequesterPrivateKeyDer {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        RsaPrivateKey::try_from(bytes).map(Self)
    }
}

impl SequesterPrivateKeyDer {
    const ALGORITHM: &'static str = "RSAES-OAEP-SHA256-XSALSA20-POLY1305";

    pub fn generate_pair(size_in_bits: SequesterKeySize) -> (Self, SequesterPublicKeyDer) {
        let (priv_key, pub_key) = RsaPrivateKey::gen_keypair(size_in_bits as usize);

        (Self(priv_key), SequesterPublicKeyDer(pub_key.0))
    }

    pub fn size_in_bytes(&self) -> usize {
        self.0.size_in_bytes()
    }

    pub fn dump(&self) -> Zeroizing<Vec<u8>> {
        self.0.to_pkcs8_der()
    }

    pub fn dump_pem(&self) -> Zeroizing<String> {
        self.0.to_pkcs8_pem()
    }

    pub fn load_pem(s: &str) -> CryptoResult<Self> {
        RsaPrivateKey::load_pkcs8_pem(s).map(Self)
    }

    pub fn decrypt(&self, data: &[u8]) -> CryptoResult<Vec<u8>> {
        let (cipherkey, ciphertext) =
            deserialize_with_armor(data, self.size_in_bytes(), Self::ALGORITHM)?;
        let padding = Oaep::<rsa::sha2::Sha256>::new();

        let clearkey = SecretKey::try_from(
            &self
                .0
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

#[derive(Clone, PartialEq, Eq, Deserialize, Hash)]
#[serde(try_from = "&Bytes")]
pub struct SequesterPublicKeyDer(RsaPublicKey);

crate::impl_key_debug!(SequesterPublicKeyDer);

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

impl SequesterPublicKeyDer {
    const ALGORITHM: &'static str = "RSAES-OAEP-SHA256-XSALSA20-POLY1305";

    pub fn size_in_bytes(&self) -> usize {
        self.0.n().bits() as usize / 8
    }

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
        let padding = Oaep::<rsa::sha2::Sha256>::new();
        let secret_key = SecretKey::generate();
        let secret_key_encrypted = self
            .0
            .encrypt(&mut UnwrapErr(SysRng), padding, secret_key.as_ref())
            .expect("Unreachable");

        // RSAES-OAEP uses 42 bytes for padding, hence even with an insecure
        // 1024 bits RSA key there is still 86 bytes available for payload
        // which is plenty to store the 32 bytes XSalsa20 key
        serialize_with_armor(
            &secret_key_encrypted,
            &secret_key.encrypt(data),
            self.size_in_bytes(),
            Self::ALGORITHM,
        )
    }
}

/*
 * SigningKey
 */

#[derive(Clone, PartialEq)]
pub struct SequesterSigningKeyDer(SigningKey<Sha256>);

crate::impl_key_debug!(SequesterSigningKeyDer);

impl Eq for SequesterSigningKeyDer {}

impl TryFrom<&[u8]> for SequesterSigningKeyDer {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        RsaPrivateKey::try_from(bytes)
            .map(rsa::RsaPrivateKey::from)
            .map(SigningKey::from)
            .map(Self)
    }
}

impl SequesterSigningKeyDer {
    const ALGORITHM: &'static str = "RSASSA-PSS-SHA256";

    pub fn generate_pair(size_in_bits: SequesterKeySize) -> (Self, SequesterVerifyKeyDer) {
        let (priv_key, pub_key) = SequesterPrivateKeyDer::generate_pair(size_in_bits);
        let signing_key = SigningKey::from(rsa::RsaPrivateKey::from(priv_key.0));

        (Self(signing_key), SequesterVerifyKeyDer::from(pub_key.0))
    }

    pub fn size_in_bytes(&self) -> usize {
        self.0.as_ref().n().bits() as usize / 8
    }

    pub fn salt_len(&self) -> usize {
        self.0.salt_len()
    }

    pub fn hash_len() -> usize {
        Sha256::output_size()
    }

    pub fn dump(&self) -> Zeroizing<Vec<u8>> {
        self.0.to_pkcs8_der().expect("Unreachable").to_bytes()
    }

    pub fn dump_pem(&self) -> Zeroizing<String> {
        self.0
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::default())
            .expect("Unreachable")
    }

    pub fn load_pem(s: &str) -> CryptoResult<Self> {
        RsaPrivateKey::load_pkcs8_pem(s)
            .map(rsa::RsaPrivateKey::from)
            .map(SigningKey::from)
            .map(Self)
    }

    // Signature format:
    //   <algorithm name>:<signature><data>
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let signature = self
            .0
            .try_sign_with_rng(&mut SysRng, data)
            .expect("Failed to generate signature");

        serialize_with_armor(
            signature.to_bytes().as_ref(),
            data,
            self.size_in_bytes(),
            Self::ALGORITHM,
        )
    }
}

/*
 * VerifyKey
 */

#[derive(Clone, Deserialize, PartialEq)]
#[serde(try_from = "&Bytes")]
pub struct SequesterVerifyKeyDer(VerifyingKey<Sha256>);

crate::impl_key_debug!(SequesterVerifyKeyDer);

impl Eq for SequesterVerifyKeyDer {}

impl From<rsa::RsaPublicKey> for SequesterVerifyKeyDer {
    fn from(value: rsa::RsaPublicKey) -> Self {
        Self(VerifyingKey::new_with_auto_salt_len(value))
    }
}

impl TryFrom<&[u8]> for SequesterVerifyKeyDer {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        RsaPublicKey::from_public_key_der(bytes)
            .map(Self::from)
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

impl SequesterVerifyKeyDer {
    const ALGORITHM: &'static str = "RSASSA-PSS-SHA256";

    pub fn size_in_bytes(&self) -> usize {
        self.0.as_ref().n().bits() as usize / 8
    }

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
            .map(Self::from)
            .map_err(|err| CryptoError::SequesterPublicKeyDer(err.to_string()))
    }

    pub fn verify(&self, data: &[u8]) -> CryptoResult<Vec<u8>> {
        let (signature, data) =
            deserialize_with_armor(data, self.size_in_bytes(), Self::ALGORITHM)?;

        let signature = Signature::try_from(signature).expect(concat!(
            "The conversion can fail if the raw signature is too big,",
            " here it is not possible because the signature will be `size_in_bytes()` long"
        ));
        self.0
            .verify(data, &signature)
            .map_err(|_| CryptoError::SignatureVerification)?;

        Ok(data.to_vec())
    }
}
