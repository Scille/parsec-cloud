// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{CryptoError, CryptoResult};
use openssl::{
    pkey::{PKey, Private, Public},
    rsa::{Padding, Rsa},
    sign::Signer,
};
use zeroize::Zeroizing;

#[derive(Clone)]
pub struct RsaPrivateKey(pub(crate) PKey<Private>);

crate::impl_key_debug!(RsaPrivateKey);

impl TryFrom<&[u8]> for RsaPrivateKey {
    type Error = CryptoError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        PKey::private_key_from_der(bytes)
            .map_err(|e| CryptoError::RsaPrivateKeyDER(e.to_string()))
            .and_then(Self::try_from)
    }
}

impl TryFrom<PKey<Private>> for RsaPrivateKey {
    type Error = CryptoError;

    fn try_from(key: PKey<Private>) -> Result<Self, Self::Error> {
        if let Err(e) = key.rsa() {
            return Err(CryptoError::RsaPrivateKeyDER(e.to_string()));
        } else {
            Ok(Self(key))
        }
    }
}

impl PartialEq for RsaPrivateKey {
    fn eq(&self, other: &Self) -> bool {
        let privkey = self.0.rsa().expect("Unreachable");
        let other_privkey = other.0.rsa().expect("Unreachable");
        privkey.n() == other_privkey.n()
            && privkey.e() == other_privkey.e()
            && privkey.d() == other_privkey.d()
            && privkey.p() == other_privkey.p()
            && privkey.q() == other_privkey.q()
            && privkey.dmp1() == other_privkey.dmp1()
            && privkey.dmq1() == other_privkey.dmq1()
            && privkey.iqmp() == other_privkey.iqmp()
    }
}

impl Eq for RsaPrivateKey {}

impl RsaPrivateKey {
    pub fn gen_keypair(size: usize) -> (Self, RsaPublicKey) {
        let pkey = Rsa::generate(size as u32).expect("Cannot generate the RSA key");
        let pubkey = Rsa::from_public_components(
            pkey.n().to_owned().expect("Unreachable"),
            pkey.e().to_owned().expect("Unreachable"),
        )
        .and_then(PKey::from_rsa)
        .expect("Unreachable");
        let pkey = PKey::from_rsa(pkey).expect("Unreachable");
        (Self(pkey), RsaPublicKey(pubkey))
    }

    pub fn load_pkcs8_pem(pem: &str) -> CryptoResult<Self> {
        PKey::private_key_from_pem(pem.as_bytes())
            .map_err(|e| CryptoError::RsaPrivateKeyDER(e.to_string()))
            .and_then(Self::try_from)
    }

    pub fn size_in_bytes(&self) -> usize {
        self.0.size() as usize
    }

    pub fn to_pkcs8_der(&self) -> zeroize::Zeroizing<Vec<u8>> {
        self.0
            .private_key_to_pkcs8()
            .map(Zeroizing::new)
            .expect("Unreachable")
    }

    pub fn to_pkcs8_pem(&self) -> zeroize::Zeroizing<String> {
        self.0
            .private_key_to_pem_pkcs8()
            .map(|v| String::from_utf8(v).expect("Unreachable"))
            .map(Zeroizing::new)
            .expect("Unreachable")
    }

    pub fn sign_pkcs1v15_unprefixed(&self, data: &[u8]) -> CryptoResult<Vec<u8>> {
        let mut signer = Signer::new_without_digest(&self.0).expect("Unable to build a signer");
        signer
            .set_rsa_padding(Padding::PKCS1)
            .expect("OpenSSL error");

        signer.update(data).expect("Unreachable");
        signer.sign_to_vec().map_err(|_| CryptoError::Signature)
    }
}

pub struct RsaPublicKey(pub(crate) PKey<Public>);
