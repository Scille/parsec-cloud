// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/* use ed25519_dalek::Verifier;
use rsa::{
    pkcs8::{
        der::zeroize::Zeroizing, DecodePrivateKey, DecodePublicKey, EncodePrivateKey,
        EncodePublicKey,
    },
    pss::{Signature, SigningKey, VerifyingKey},
    signature::RandomizedSigner,
    PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey,
}; */

use zeroize::Zeroizing;

use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::{Padding, Rsa};
use openssl::sign::{Signer, Verifier};

use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;
/* use sha2::Sha256; */

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
pub struct SequesterPrivateKeyDer(PKey<Private>);

crate::impl_key_debug!(SequesterPrivateKeyDer);

impl Default for SequesterPrivateKeyDer {
    fn default() -> Self {
        Self(
            PKey::from_rsa(Rsa::generate(Self::SIZE_IN_BITS.try_into().unwrap()).unwrap())
                .expect("Unreachable"),
        )
    }
}

impl TryFrom<&[u8]> for SequesterPrivateKeyDer {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        PKey::private_key_from_der(bytes)
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
        Zeroizing::new(self.0.private_key_to_der().expect("Unreachable"))
    }

    pub fn dump_pem(&self) -> Zeroizing<String> {
        let pkey_pem = self.0.private_key_to_pem_pkcs8().expect("Unreachable");

        Zeroizing::new(
            std::str::from_utf8(&pkey_pem)
                .expect("Unreachable")
                .to_string(),
        )
    }

    pub fn load_pem(s: &str) -> CryptoResult<Self> {
        PKey::private_key_from_pem(s.as_bytes())
            .map(Self)
            .map_err(|err| CryptoError::SequesterPrivateKeyDer(err.to_string()))
    }

    pub fn decrypt(&self, data: &[u8]) -> CryptoResult<Vec<u8>> {
        let (cipherkey, ciphertext) = Self::deserialize(data)?;

        let mut decrypted_key_der: Vec<u8> = vec![0; cipherkey.len()];
        let decrypted_key_bytecount = self
            .0
            .rsa()
            .expect("Unreachable")
            .private_decrypt(cipherkey, &mut decrypted_key_der, Padding::PKCS1_OAEP)
            .map_err(|_| CryptoError::Decryption)?;

        let clearkey = SecretKey::try_from(&decrypted_key_der[0..decrypted_key_bytecount])?;

        clearkey.decrypt(ciphertext)
    }
}

/*
 * PublicKey
 */

#[derive(Clone, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct SequesterPublicKeyDer(Rsa<Public>);

crate::impl_key_debug!(SequesterPublicKeyDer);

impl From<SequesterPrivateKeyDer> for SequesterPublicKeyDer {
    fn from(priv_key: SequesterPrivateKeyDer) -> Self {
        let private_key = priv_key.0.rsa().expect("Unreachable");

        Self(
            Rsa::from_public_components(
                private_key.n().to_owned().expect("?"),
                private_key.e().to_owned().expect("?"),
            )
            .expect("Unreachable"),
        )
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
        self.0.public_key_to_der().expect("Unreachable")
    }

    pub fn dump_pem(&self) -> String {
        let pkey_pem = self.0.public_key_to_pem().expect("Unreachable");

        std::str::from_utf8(&pkey_pem)
            .expect("Unreachable")
            .to_string()
    }

    pub fn load_pem(s: &str) -> CryptoResult<Self> {
        Rsa::public_key_from_pem(s.as_bytes())
            .map(Self)
            .map_err(|err| CryptoError::SequesterPublicKeyDer(err.to_string()))
    }

    // Encryption format:
    //   <algorithm name>:<encrypted secret key with RSA key><encrypted data with secret key>
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let secret_key = SecretKey::generate();

        let mut encrypted_secret_key: Vec<u8> = vec![0; self.0.size() as usize];

        let encrypted_key_bytes = self
            .0
            .public_encrypt(
                secret_key.as_ref(),
                &mut encrypted_secret_key,
                Padding::PKCS1_OAEP,
            )
            .unwrap();

        EnforceSerialize::serialize(
            self,
            &encrypted_secret_key[0..encrypted_key_bytes],
            &secret_key.encrypt(data),
        )
    }
}

impl TryFrom<&[u8]> for SequesterPublicKeyDer {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Rsa::public_key_from_der(bytes)
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

impl PartialEq for SequesterPublicKeyDer {
    fn eq(&self, other: &Self) -> bool {
        let public_key = PKey::from_rsa(self.0.to_owned()).unwrap();
        public_key.public_eq(&PKey::from_rsa(other.0.to_owned()).unwrap())
    }
}

impl Eq for SequesterPublicKeyDer {}

/*
 * SigningKey
 */

#[derive(Clone)]
pub struct SequesterSigningKeyDer(PKey<Private>);

crate::impl_key_debug!(SequesterSigningKeyDer);

impl EnforceSerialize for SequesterSigningKeyDer {
    const SIZE_IN_BITS: usize = Self::SIZE_IN_BITS;
    const ALGORITHM: &'static [u8] = b"RSASSA-PSS-SHA256";
}

impl From<SequesterPrivateKeyDer> for SequesterSigningKeyDer {
    fn from(priv_key: SequesterPrivateKeyDer) -> Self {
        Self(priv_key.0)
    }
}

impl SequesterSigningKeyDer {
    #[cfg(test)]
    pub const SIZE_IN_BITS: usize = 1024;
    #[cfg(not(test))]
    pub const SIZE_IN_BITS: usize = 4096;

    pub fn dump(&self) -> Zeroizing<Vec<u8>> {
        Zeroizing::new(self.0.private_key_to_der().expect("Unreachable"))
    }

    pub fn dump_pem(&self) -> Zeroizing<String> {
        let pkey_pem = self
            .0
            .rsa()
            .expect("Unreachable")
            .private_key_to_pem()
            .expect("Unreachable");

        Zeroizing::new(
            std::str::from_utf8(&pkey_pem)
                .expect("Unreachable")
                .to_string(),
        )
    }

    pub fn load_pem(s: &str) -> CryptoResult<Self> {
        PKey::private_key_from_pkcs8(s.as_bytes())
            .map(Self)
            .map_err(|err| CryptoError::SequesterPrivateKeyDer(err.to_string()))
    }

    // Signature format:
    //   <algorithm name>:<signature><data>
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.0).expect("What");
        signer.update(data).expect("Unable to sign");

        self.serialize(&signer.sign_to_vec().unwrap(), data)
    }
}

/*
 * VerifyKey
 */

#[derive(Clone)]
pub struct SequesterVerifyKeyDer(PKey<Public>);

crate::impl_key_debug!(SequesterVerifyKeyDer);

impl From<SequesterPublicKeyDer> for SequesterVerifyKeyDer {
    fn from(pub_key: SequesterPublicKeyDer) -> Self {
        Self(PKey::from_rsa(pub_key.0).expect("Unreachable"))
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

    pub fn dump(&self) -> Zeroizing<Vec<u8>> {
        Zeroizing::new(self.0.public_key_to_der().expect("Unreachable"))
    }

    pub fn dump_pem(&self) -> Zeroizing<String> {
        let pkey_pem = self
            .0
            .rsa()
            .expect("Unreachable")
            .public_key_to_pem()
            .expect("Unreachable");

        Zeroizing::new(
            std::str::from_utf8(&pkey_pem)
                .expect("Unable to get UTF-8 String from public key PEM")
                .to_string(),
        )
    }

    pub fn verify(&self, data: &[u8]) -> CryptoResult<Vec<u8>> {
        let (signature, data) = Self::deserialize(data)?;

        let mut verifier = Verifier::new(MessageDigest::sha256(), &self.0)
            .map_err(|_| CryptoError::SignatureVerification)?;

        verifier
            .update(data)
            .map_err(|_| CryptoError::SignatureVerification)?;

        match verifier.verify(signature) {
            Ok(signature_is_valid) => match signature_is_valid {
                true => Ok(data.to_vec()),
                false => Err(CryptoError::Signature),
            },
            Err(_) => Err(CryptoError::SignatureVerification),
        }
    }
}

impl TryFrom<&[u8]> for SequesterVerifyKeyDer {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        PKey::public_key_from_der(bytes)
            .map(Self)
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
