// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use rand::rngs::OsRng;
use rsa::{
    pkcs8::{DecodePrivateKey, EncodePrivateKey},
    PublicKeyParts,
};

use crate::{CryptoError, CryptoResult};

#[derive(Clone, PartialEq, Eq)]
pub struct RsaPrivateKey(pub(crate) rsa::RsaPrivateKey);

crate::impl_key_debug!(RsaPrivateKey);

impl TryFrom<&[u8]> for RsaPrivateKey {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        rsa::RsaPrivateKey::from_pkcs8_der(bytes)
            .map(Self)
            .map_err(|e| CryptoError::RsaPrivateKeyDER(e.to_string()))
    }
}

impl From<RsaPrivateKey> for rsa::RsaPrivateKey {
    fn from(value: RsaPrivateKey) -> Self {
        value.0
    }
}

impl RsaPrivateKey {
    pub fn gen_keypair(size: usize) -> (Self, RsaPublicKey) {
        let pkey = rsa::RsaPrivateKey::new(&mut OsRng, size).expect("Cannot generate the RSA key");
        let pubkey = pkey.to_public_key();
        (Self(pkey), RsaPublicKey(pubkey))
    }

    pub fn load_pkcs8_pem(pem: &str) -> CryptoResult<Self> {
        rsa::RsaPrivateKey::from_pkcs8_pem(pem)
            .map(Self)
            .map_err(|e| CryptoError::RsaPrivateKeyDER(e.to_string()))
    }

    pub fn size_in_bytes(&self) -> usize {
        self.0.n().bits() / 8
    }

    pub fn to_pkcs8_der(&self) -> zeroize::Zeroizing<Vec<u8>> {
        self.0.to_pkcs8_der().expect("Unreachable").to_bytes()
    }

    pub fn to_pkcs8_pem(&self) -> zeroize::Zeroizing<String> {
        self.0
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::default())
            .expect("Unreachable")
    }

    pub fn sign_pkcs1v15_unprefixed(&self, data: &[u8]) -> CryptoResult<Vec<u8>> {
        self.0
            .sign(rsa::pkcs1v15::Pkcs1v15Sign::new_raw(), data)
            .map_err(|_| CryptoError::Signature)
    }
}

pub struct RsaPublicKey(pub(crate) rsa::RsaPublicKey);
