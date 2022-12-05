// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use zeroize::Zeroizing;

use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::{Padding, Rsa};
use openssl::sign::{RsaPssSaltlen, Signer, Verifier};

use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;

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
            PKey::from_rsa(Rsa::generate(Self::SIZE_IN_BITS as u32).expect("Unreachable"))
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
            String::from_utf8(pkey_pem)
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

        let mut decrypted_key_der = vec![0; cipherkey.len()];
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

        String::from_utf8(pkey_pem)
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

        let mut encrypted_secret_key = vec![0; self.0.size() as usize];

        let encrypted_key_bytes = self
            .0
            .public_encrypt(
                secret_key.as_ref(),
                &mut encrypted_secret_key,
                Padding::PKCS1_OAEP,
            )
            .expect("Unable to decrypt a secret key");

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
        let public_key = PKey::from_rsa(self.0.to_owned()).expect("Unreachable");
        public_key.public_eq(&PKey::from_rsa(other.0.to_owned()).expect("Unreachable"))
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
            String::from_utf8(pkey_pem)
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
        let mut signer =
            Signer::new(MessageDigest::sha256(), &self.0).expect("Unable to build a Signer");

        signer
            .set_rsa_padding(Padding::PKCS1_PSS)
            .expect("OpenSSL error");
        signer
            .set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)
            .expect("OpenSSL error");

        signer.update(data).expect("Unreachable");
        let signed_data = signer.sign_to_vec().expect("Unable to sign a message");

        self.serialize(&signed_data, data)
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
            String::from_utf8(pkey_pem)
                .expect("Unable to get UTF-8 String from public key PEM")
                .to_string(),
        )
    }

    pub fn verify(&self, data: &[u8]) -> CryptoResult<Vec<u8>> {
        let (signature, contents) = Self::deserialize(data)?;

        let mut verifier = Verifier::new(MessageDigest::sha256(), &self.0)
            .map_err(|_| CryptoError::SignatureVerification)?;

        verifier
            .set_rsa_padding(Padding::PKCS1_PSS)
            .expect("OpenSSL error");
        verifier
            .set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)
            .expect("OpenSSL error");

        verifier
            .update(contents)
            .map_err(|_| CryptoError::SignatureVerification)?;

        match verifier.verify(signature) {
            Ok(signature_is_valid) => match signature_is_valid {
                true => Ok(contents.to_vec()),
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

#[test]
fn test_pub_signature_verification() {
    use hex_literal::hex;

    let pub_key_verifier = SequesterVerifyKeyDer::try_from(
        &hex!(
            "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689"
            "b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a"
            "3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62c"
            "b38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195"
            "f3bcc72ab57207ebfd0203010001"
        )[..],
    )
    .unwrap();

    let signed_data = hex!(
        "5253415353412d5053532d5348413235363a0afc141b03789ac3f2c69bd1e577e279d4570b"
        "f3fe387f389fe52c2b4ac08a9cacbd3c5c1b080cb39969cf3ff7a375619b4b5adc4aef2aec"
        "2800f6ead1d78019f8e37d036880b71e6ba4e89562e14ab2d6b35d0db4db48d818f8d4395f"
        "8d692be38fcdfa8d526a352bb811393dd987ed5a8257b7583d145099037178456baf3c4865"
        "6c6c6f20776f726c64"
    );

    let verification = pub_key_verifier.verify(&signed_data);

    assert!(verification.is_ok());
}

#[test]
fn test_priv_decipher_verification() {
    use hex_literal::hex;

    let priv_key = SequesterPrivateKeyDer::try_from(
        &hex!(
            "30820276020100300d06092a864886f70d0101010500048202603082025c
            02010002818100b2dc00a3c3b5c689b069f3f40c494d2a5be313b1034fbf
            1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045d75b584c1943
            bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb3
            8582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66
            f7ee18303195f3bcc72ab57207ebfd0203010001028180417d044ef20dd0
            9001a409cac5e4e0f82d84cb64f8cd6e30d1212e9df703647fde7eff7eb4
            813e5b4218cccef93e0b947ac1adbb626da9622a6f89afd55c8ac8bb0a0f
            1832b2520fc1d92ac320180b9657a8b1598e6701119d8f77f5285ac5703f
            4c0a93e1e5ebe6ec179bccff62495e7734d5899d86d2dcbdeb648923d29b
            11024100eb2b5d5ad5bc28cae1654505c16a2b0d82cfa237f8f10f70e5e7
            a3333028aade5de4f9e2b3e8a9f924a0538f7119e088f3c11f1258319c59
            da03d637a324c663024100c2b3c49145dfd7930ac7f5681afa43b7b18f69
            7163469c7ab9a6ca8e12168eab1a33cc4e73b53f4509c48052a31ff289e7
            357a54f88e28ee543040009976621f024026b608b3ff22ee04177e38126e
            782f8615d65ff99ebcefb1c1e69372c5a6ac19d692ee9f66c611d4b536bf
            0a89af9cca6e7587cbd940b160090740a7ffeef9c902410093566a77ecc2
            9965e290b2bb173f2fa380b0a0007839e50c52154fcef70d2ee5782c9e7c
            f7bebea445e1f7a1916409ac25d5283fc8dffb456f5c1bf2d82ee7cd0240
            18b8c579f32bbd74cc1f10c786e1e0e192526c9e4134c65bfc76799b8290
            0adf467a5c7fb3164eb775650abaae08500bc6691e60c8575b5a5abf4f21
            56911c4e"
        )[..],
    )
    .unwrap();

    let data = priv_key
        .decrypt(&hex!(
            "52534145532d4f4145502d5853414c534132302d504f4c59313330353a278b9346743cc609"
            "258a4a82023059411d78c29aed9cf9893dc36b1f1f0055e3db8fa3624b4e3ced4fa1d3683b"
            "97cff2694ddbebeb9a59d1533e9b4dba005958f70a1b7b4b54fef420fb200146a73ac0f457"
            "168e71decb50d98af9332da36b5143e3470f7858a1a43f0f7ffff6e98e2487579d96a3791d"
            "69d48ba307e9984dad42781a567fa27e74e9ee88fd945736968855588eec48f43faa396464"
            "d9a5e8cfd5326ea0193ff5732b51423146d683b74870ed"
        ))
        .unwrap();

    assert_eq!(data, "Hello world".as_bytes());
}
