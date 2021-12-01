// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use ed25519_dalek::{
    Keypair, Signature, Signer, Verifier, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH,
};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

/*
 * SigningKey
 */

#[derive(Debug, Deserialize, Serialize)]
#[serde(into = "ByteBuf", try_from = "ByteBuf")]
pub struct SigningKey(Keypair);

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
    pub const SIZE: usize = SECRET_KEY_LENGTH;

    pub fn verify_key(&self) -> VerifyKey {
        VerifyKey(self.0.public)
    }

    pub fn generate() -> Self {
        Self(Keypair::generate(&mut rand_07::thread_rng()))
    }

    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let mut signed = Vec::with_capacity(SIGNATURE_LENGTH + data.len());
        signed.extend(self.0.sign(data).to_bytes());
        signed.extend_from_slice(data);
        signed
    }
}

impl AsRef<[u8]> for SigningKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.secret.as_bytes()
    }
}

impl TryFrom<&[u8]> for SigningKey {
    type Error = &'static str;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let sk = ed25519_dalek::SecretKey::from_bytes(data).map_err(|_| ("Invalid data size"))?;
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

impl TryFrom<ByteBuf> for SigningKey {
    type Error = &'static str;
    fn try_from(data: ByteBuf) -> Result<Self, Self::Error> {
        // TODO: zerocopy
        Self::try_from(data.into_vec().as_ref())
    }
}

impl From<SigningKey> for ByteBuf {
    fn from(data: SigningKey) -> Self {
        Self::from(data.0.secret.to_bytes())
    }
}

/*
 * VerifyKey
 */

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(into = "ByteBuf", try_from = "ByteBuf")]
pub struct VerifyKey(ed25519_dalek::PublicKey);

impl VerifyKey {
    pub const ALGORITHM: &'static str = "ed25519";
    pub const SIZE: usize = PUBLIC_KEY_LENGTH;

    pub fn unsecure_unwrap<'a>(signed: &'a [u8]) -> Option<&'a [u8]> {
        signed.get(SIGNATURE_LENGTH..)
    }

    pub fn verify(&self, signed: &[u8]) -> Result<Vec<u8>, &'static str> {
        // TODO: handle wrong signed size
        let signature = Signature::try_from(&signed[..SIGNATURE_LENGTH]).unwrap();
        let message = &signed[SIGNATURE_LENGTH..];
        self.0
            .verify(message, &signature)
            .map_err(|_| "Signature verification error")?;
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
    type Error = &'static str;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let pk = ed25519_dalek::PublicKey::from_bytes(data).map_err(|_| ("Invalid data size"))?;
        Ok(Self(pk))
    }
}

impl From<[u8; Self::SIZE]> for VerifyKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        // TODO: zerocopy
        Self::try_from(key.as_ref()).unwrap()
    }
}

impl TryFrom<ByteBuf> for VerifyKey {
    type Error = &'static str;
    fn try_from(data: ByteBuf) -> Result<Self, Self::Error> {
        // TODO: zerocopy
        Self::try_from(data.into_vec().as_ref())
    }
}

impl From<VerifyKey> for ByteBuf {
    fn from(data: VerifyKey) -> Self {
        Self::from(data.0.to_bytes())
    }
}
