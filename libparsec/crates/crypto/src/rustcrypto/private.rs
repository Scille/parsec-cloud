// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: This should be part of crypto_box crate, provide a PR ?
// Taken verbatim from https://github.com/Karrq/sealed_box/blob/master/src/lib.rs
mod sealed_box {

    use blake2::{
        digest::{Update, VariableOutput},
        Blake2bVar,
    };
    use crypto_box::{
        aead::{self, Aead},
        SalsaBox,
    };

    //re-export keys
    pub use crypto_box::{PublicKey, SecretKey};
    use rand::rngs::OsRng;

    const BOX_NONCE_LENGTH: usize = 24;
    const BOX_OVERHEAD: usize = 16;

    //32 = PublicKey length
    const SEALED_OVERHEAD: usize = 32 + BOX_OVERHEAD;

    ///generate the nonce for the given public keys
    ///
    /// nonce = Blake2b(ephemeral_pk||target_pk)
    /// nonce_length = 24
    fn get_nonce(ephemeral_pk: &PublicKey, target_pk: &PublicKey) -> [u8; BOX_NONCE_LENGTH] {
        let mut hasher = Blake2bVar::new(BOX_NONCE_LENGTH).expect(
            "Should not fail because `BOX_NONCE_LENGTH` is smaller than `Blake2bVar::MAX_OUTPUT_SIZE`",
        );

        hasher.update(ephemeral_pk.as_bytes());
        hasher.update(target_pk.as_bytes());

        let out = hasher.finalize_boxed();

        let mut array = [0u8; BOX_NONCE_LENGTH];
        array.copy_from_slice(&out);

        array
    }

    ///encrypts the given buffer for the given public key
    ///
    /// overhead = 48 = (32 ephemeral_pk||16 box_overhead)
    pub fn seal(data: &[u8], pk: &PublicKey) -> Vec<u8> {
        let mut out = Vec::with_capacity(SEALED_OVERHEAD + data.len());

        let ep_sk = SecretKey::generate(&mut OsRng);
        let ep_pk = ep_sk.public_key();
        out.extend_from_slice(ep_pk.as_bytes());

        let nonce = get_nonce(&ep_pk, pk);
        let nonce = aead::generic_array::GenericArray::from_slice(&nonce);

        let salsabox = SalsaBox::new(pk, &ep_sk);
        let encrypted = salsabox
            .encrypt(nonce, data)
            .expect("Should not fail as long as the associated data is not provided");

        out.extend_from_slice(&encrypted);
        out
    }

    ///attempt to decrypt the given ciphertext with the given secret key
    /// will fail if the secret key doesn't match the public key used to encrypt the payload
    /// or if the ciphertext is not long enough
    pub fn open(ciphertext: &[u8], sk: &SecretKey) -> Option<Vec<u8>> {
        if ciphertext.len() <= 32 {
            //not long enough
            return None;
        }

        let pk = sk.public_key();

        let ephemeral_pk: PublicKey = {
            let bytes = &ciphertext[..32];
            let mut array = [0u8; 32];
            array.copy_from_slice(bytes);
            array.into()
        };

        // Libsodium's sealed box uses `crypto_scalarmult_curve25519` that always rejects
        // small-order point for the public key (as it may lead to an all-zeros result no
        // matter what the other peer's private key is).
        //
        // see:
        // - https://github.com/jedisct1/libsodium/blob/95a428090fb7229567244649d901a2fe94c199bc/src/libsodium/crypto_box/curve25519xsalsa20poly1305/box_curve25519xsalsa20poly1305.c#L42)
        // - https://github.com/jedisct1/libsodium/blob/95a428090fb7229567244649d901a2fe94c199bc/src/libsodium/crypto_scalarmult/curve25519/ref10/x25519_ref10.c#L85
        if has_small_order(ephemeral_pk.as_bytes()) {
            return None;
        }

        let nonce = get_nonce(&ephemeral_pk, &pk);
        let nonce = aead::generic_array::GenericArray::from_slice(&nonce);

        let encrypted = &ciphertext[32..];
        let salsabox = SalsaBox::new(&ephemeral_pk, sk);

        salsabox.decrypt(nonce, encrypted).ok()
    }

    // Rust conversion of libsodium's `has_small_order`.
    // see: https://github.com/jedisct1/libsodium/blob/95a428090fb7229567244649d901a2fe94c199bc/src/libsodium/crypto_scalarmult/curve25519/ref10/x25519_ref10.c#L11-L70
    #[expect(clippy::needless_range_loop)]
    fn has_small_order(s: &[u8; 32]) -> bool {
        const BLOCKLIST: [[u8; 32]; 7] = [
            /* 0 (order 4) */
            [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
            /* 1 (order 1) */
            [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
            /* 325606250916557431795983626356110631294008115727848805560023387167927233504
            (order 8) */
            [
                0xe0, 0xeb, 0x7a, 0x7c, 0x3b, 0x41, 0xb8, 0xae, 0x16, 0x56, 0xe3, 0xfa, 0xf1, 0x9f,
                0xc4, 0x6a, 0xda, 0x09, 0x8d, 0xeb, 0x9c, 0x32, 0xb1, 0xfd, 0x86, 0x62, 0x05, 0x16,
                0x5f, 0x49, 0xb8, 0x00,
            ],
            /* 39382357235489614581723060781553021112529911719440698176882885853963445705823
            (order 8) */
            [
                0x5f, 0x9c, 0x95, 0xbc, 0xa3, 0x50, 0x8c, 0x24, 0xb1, 0xd0, 0xb1, 0x55, 0x9c, 0x83,
                0xef, 0x5b, 0x04, 0x44, 0x5c, 0xc4, 0x58, 0x1c, 0x8e, 0x86, 0xd8, 0x22, 0x4e, 0xdd,
                0xd0, 0x9f, 0x11, 0x57,
            ],
            /* p-1 (order 2) */
            [
                0xec, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0x7f,
            ],
            /* p (=0, order 4) */
            [
                0xed, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0x7f,
            ],
            /* p+1 (=1, order 1) */
            [
                0xee, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0x7f,
            ],
        ];

        let mut c = [0u8; 7];

        for j in 0..31 {
            for i in 0..7 {
                c[i] |= s[j] ^ BLOCKLIST[i][j];
            }
        }
        for i in 0..7 {
            c[i] |= (s[31] & 0x7f) ^ BLOCKLIST[i][31];
        }

        let mut k: usize = 0;
        for i in 0..7 {
            k |= (c[i] as usize).wrapping_sub(1);
        }

        ((k >> 8) & 1) == 1
    }
}

use blake2::Blake2b512;
use crypto_box::KEY_SIZE;
use digest::Digest;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_bytes::Bytes;
use zeroize::Zeroizing;

use crate::{CryptoError, SecretKey};

/*
 * PrivateKey
 */

#[derive(Debug)]
pub enum SharedSecretKeyRole {
    Claimer,
    Greeter,
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct PrivateKey(crypto_box::SecretKey);

crate::impl_key_debug!(PrivateKey);

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for PrivateKey {}

impl PrivateKey {
    pub const ALGORITHM: &'static str = "curve25519blake2bxsalsa20poly1305";
    pub const SIZE: usize = KEY_SIZE;

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public_key())
    }

    pub fn generate() -> Self {
        Self(crypto_box::SecretKey::generate(&mut OsRng))
    }

    pub fn decrypt_from_self(&self, ciphered: &[u8]) -> Result<Vec<u8>, CryptoError> {
        sealed_box::open(ciphered, &self.0).ok_or(CryptoError::Decryption)
    }

    pub fn generate_shared_secret_key(
        &self,
        peer_public_key: &PublicKey,
        role: SharedSecretKeyRole,
    ) -> Result<SecretKey, CryptoError> {
        // Re-implementation of libsodium's `crypto_kx_XXX_session_keys`:
        // 1. x25519 Diffie-Hellman.
        // 2. Blake2b hash (with a 512 bits output) of the output of step 1.
        // 3. Output of step 2 can be splitted into two 256 bits keys.
        // see https://github.com/jedisct1/libsodium/blob/85ddc5c2c6c7b8f7c99f9af6039e18f1f2ca0daa/src/libsodium/crypto_kx/crypto_kx.c#L54-L63

        let self_public_key: x25519_dalek::PublicKey = self.public_key().0.to_bytes().into();
        let peer_public_key: x25519_dalek::PublicKey = peer_public_key.0.to_bytes().into();

        let self_static_secret: x25519_dalek::StaticSecret = self.0.to_bytes().into();
        let shared_secret = self_static_secret.diffie_hellman(&peer_public_key);
        if !shared_secret.was_contributory() {
            // Error message taken from libsodium-rs
            return Err(CryptoError::SharedSecretKey(
                "curve25519 scalarmult failed (result may be all zeros)".to_string(),
            ));
        }
        let mut hasher = Blake2b512::new();
        hasher.update(shared_secret.as_bytes());

        // Always hash claimer first then greeter second
        match role {
            SharedSecretKeyRole::Claimer => {
                hasher.update(self_public_key);
                hasher.update(peer_public_key);
            }
            SharedSecretKeyRole::Greeter => {
                hasher.update(peer_public_key);
                hasher.update(self_public_key);
            }
        }

        let raw_512 = Zeroizing::new(hasher.finalize_reset());

        // Under the hood, `crypto_kx` splits a 512 bits hash into two
        // 256 bits keys.
        // The idea is to have each peer doing encryption with a different
        // key so that:
        // 1. Each peer can use a counter as nonce without the need for
        //   synchronization with the other peer.
        // 2. To avoid reflection attacks.
        //
        // However 1 is not needed since we use XSalsa20 for encryption (that
        // uses a random nonce) and we are safe from 2 given we never use the
        // shared secret key for mutual authentication (but only for transmitting
        // data between clients).
        //
        // Hence why we only keep a single key here.
        Ok(SecretKey::try_from(&raw_512[..Self::SIZE]).expect("valid size"))
    }

    pub fn to_bytes(&self) -> zeroize::Zeroizing<[u8; KEY_SIZE]> {
        self.0.to_bytes().into()
    }
}

impl TryFrom<&[u8]> for PrivateKey {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        <[u8; Self::SIZE]>::try_from(data)
            .map(Self::from)
            .map_err(|_| CryptoError::DataSize)
    }
}

impl From<[u8; Self::SIZE]> for PrivateKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        Self(key.into())
    }
}

impl TryFrom<&Bytes> for PrivateKey {
    type Error = CryptoError;
    fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
    }
}

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_bytes())
    }
}

/*
 * PublicKey
 */

#[derive(Clone, Deserialize)]
#[serde(try_from = "&Bytes")]
pub struct PublicKey(crypto_box::PublicKey);

crate::impl_key_debug!(PublicKey);

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for PublicKey {}

impl std::hash::Hash for PublicKey {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.as_ref().hash(state)
    }
}

impl PublicKey {
    pub const ALGORITHM: &'static str = "curve25519blake2bxsalsa20poly1305";
    pub const SIZE: usize = KEY_SIZE;

    pub fn encrypt_for_self(&self, data: &[u8]) -> Vec<u8> {
        sealed_box::seal(data, &self.0)
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = CryptoError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        <[u8; Self::SIZE]>::try_from(data)
            .map(Self::from)
            .map_err(|_| CryptoError::DataSize)
    }
}

impl From<[u8; Self::SIZE]> for PublicKey {
    fn from(key: [u8; Self::SIZE]) -> Self {
        Self(key.into())
    }
}

// impl TryFrom<[u8; Self::SIZE]> for PublicKey {
//     type Error = CryptoError;
//     fn try_from(data: [u8; Self::SIZE]) -> Result<Self, Self::Error> {
//         Self::try_from(data.as_ref())
//     }
// }

impl TryFrom<&Bytes> for PublicKey {
    type Error = CryptoError;
    fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
        Self::try_from(data.as_ref())
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.as_ref())
    }
}
