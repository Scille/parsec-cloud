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

use crate::{CryptoError, CryptoResult, SecretKey};

trait EnforceSerialize {
    const ALGORITHM: &'static [u8];
    const SIZE_IN_BITS: usize;
    const SIZE_IN_BYTES: usize = Self::SIZE_IN_BITS / 8;

    /// Here we avoid uncessary allocation & enforce output has `key_size`
    fn serialize(&self, output: &[u8], data: &[u8]) -> Vec<u8> {
        assert!(output.len() <= Self::SIZE_IN_BYTES);
        let mut res = vec![0; Self::ALGORITHM.len() + 1 + Self::SIZE_IN_BYTES + data.len()];

        let (algorithm_part, others) = res.split_at_mut(Self::ALGORITHM.len());
        let (colon, others) = others.split_at_mut(1);
        // Here we enforce output has key size with zeros
        let (_zeros, others) = others.split_at_mut(Self::SIZE_IN_BYTES - output.len());
        let (output_part, data_part) = others.split_at_mut(output.len());

        algorithm_part.copy_from_slice(Self::ALGORITHM);
        colon[0] = b':';
        output_part.copy_from_slice(output);
        data_part.copy_from_slice(data);

        res
    }
}

trait EnforceDeserialize {
    const ALGORITHM: &'static [u8];
    const SIZE_IN_BITS: usize;
    const SIZE_IN_BYTES: usize = Self::SIZE_IN_BITS / 8;

    fn deserialize(data: &[u8]) -> CryptoResult<(&[u8], &[u8])> {
        let index = data
            .iter()
            .position(|x| *x == b':')
            .ok_or(CryptoError::Decryption)?;
        let (algo, output_and_data) = data.split_at(index + 1);

        if &algo[..index] != Self::ALGORITHM {
            return Err(CryptoError::Algorithm(
                String::from_utf8_lossy(&algo[..index]).into(),
            ));
        }

        Ok(output_and_data.split_at(Self::SIZE_IN_BYTES))
    }
}

/*
 * PrivateKey
 */

#[derive(Clone)]
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
            .map_err(CryptoError::SequesterPrivateKeyDer)
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
            .map_err(CryptoError::SequesterPrivateKeyDer)
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
            .map_err(CryptoError::SequesterPublicKeyDer)
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
            .map_err(CryptoError::SequesterPublicKeyDer)
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
            .map_err(CryptoError::SequesterPrivateKeyDer)
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

#[derive(Clone)]
pub struct SequesterVerifyKeyDer(VerifyingKey<Sha256>);

crate::impl_key_debug!(SequesterVerifyKeyDer);

impl From<SequesterPublicKeyDer> for SequesterVerifyKeyDer {
    fn from(pub_key: SequesterPublicKeyDer) -> Self {
        Self(VerifyingKey::from(pub_key.0))
    }
}

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
            .map_err(CryptoError::SequesterPublicKeyDer)
    }

    pub fn verify(&self, data: &[u8]) -> CryptoResult<Vec<u8>> {
        let (signature, data) = Self::deserialize(data)?;

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
            .map_err(CryptoError::SequesterPublicKeyDer)
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

#[test]
fn test_keys() {
    use hex_literal::hex;
    assert_eq!(SequesterPrivateKeyDer::SIZE_IN_BITS, 1024);

    let priv_key = SequesterPrivateKeyDer::try_from(
        &hex!(
            "30820276020100300d06092a864886f70d0101010500048202603082025c0201"
            "0002818100a0b7653b73670b364cd39727a3843502d1ba0402e38f197d4b6ab2"
            "ea31da8b23cfef82fdc7bf723d0ae6093767aa92274dde3ab0f959f0624d4d5f"
            "4c7601a561a646e2f323e60f69e52f53158654f0a592ba0f19cebb3f7b2cbc6b"
            "7920670c6fe1b23c13a77597dc50d34dc1c454c25ee3348cfae7e089bcf50a26"
            "6b4033f5f502030100010281804c2b1edb32325fe9f8373fa815a30eccab4111"
            "ad9cd3b12ce49548fe1d6a8a0f0af964878a277da8d8857550c0dce22fa683f0"
            "24f7c85c58fa71f4fc73e10bd39f8129a2508870f07715c7945038766236face"
            "3b591bbb5ac4311c807c063ff43b50997152b54a54a4d3fb6ea87ba802ef3831"
            "c8029642072c045785d9debb01024100ced120f598dde6472aa257d71d7dd7ee"
            "cfb392694f510babe42620d79374d609e4917f980fca8a5905e8cccdca107364"
            "9c1bc327c05a7ab271645e12cced3445024100c6efb2dcb8bc84d8ae6ecb17b4"
            "cb0aa5b4b5685cf60afc560ff2c3268a7be38faabb9da43555b05c6e05b3570c"
            "69d290fb9b340c275f83794de9bbff4ac94df1024039588b870e08195e0a5851"
            "7af8567895634a2b82bfd77d210076020d4479d50f912d36eff710f623911be8"
            "0df7c56ff9a9bf98f160c8b5d4dcd433b18ad90af102403b493609b7785f32e0"
            "111eaf6aeed3b67c7b4fa5dca17b7ffe72bf9bddcb7c0ed5b7e20c0ce5039118"
            "2cd4bc8d7380103b1b8ed04c6f9793f017473296cc155102410098b85ecb215a"
            "60e90d5d8f3e3f70044b47e4e082aa3a5873a5ac0a16ec109d724a44ccb5541e"
            "8bfe26726bada8c43f344bb953d05fe86afb81c4166f0f2ee425"
        )[..],
    )
    .unwrap();

    let expected_public_key = hex!(
        "30819f300d06092a864886f70d010101050003818d0030818902818100a0b7653b73670b364cd397"
        "27a3843502d1ba0402e38f197d4b6ab2ea31da8b23cfef82fdc7bf723d0ae6093767aa92274dde3a"
        "b0f959f0624d4d5f4c7601a561a646e2f323e60f69e52f53158654f0a592ba0f19cebb3f7b2cbc6b"
        "7920670c6fe1b23c13a77597dc50d34dc1c454c25ee3348cfae7e089bcf50a266b4033f5f5020301"
        "0001");

    let pub_key = SequesterPublicKeyDer::from(priv_key.clone());

    assert_eq!(pub_key.dump(), expected_public_key);

    let data = b"Hello world";

    let encrypted = pub_key.encrypt(data);

    assert_eq!(priv_key.decrypt(&encrypted).unwrap(), data);

    let signing_key = SequesterSigningKeyDer::from(priv_key);
    let verify_key = SequesterVerifyKeyDer::from(pub_key);

    let signed = signing_key.sign(data);

    assert_eq!(verify_key.verify(&signed).unwrap(), data);
}
