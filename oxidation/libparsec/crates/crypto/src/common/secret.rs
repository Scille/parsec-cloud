// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use base32::Alphabet;
use serde::Serialize;
use serde_bytes::Bytes;
use std::fmt::Debug;

use crate::{CryptoError, RECOVERY_PASSPHRASE_SYMBOLS};

pub trait SecretKeyTrait:
    AsRef<[u8]>
    + for<'a> TryFrom<&'a [u8], Error = CryptoError>
    + for<'a> TryFrom<&'a Bytes, Error = CryptoError>
    + From<[u8; 32]>
    + Serialize
    + Debug
{
    const ALGORITHM: &'static str = "xsalsa20poly1305";
    const SIZE: usize = 32;

    fn generate() -> Self;
    fn encrypt(&self, data: &[u8]) -> Vec<u8>;
    fn decrypt(&self, ciphered: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn hmac(&self, data: &[u8], digest_size: usize) -> Vec<u8>;

    /// passphrase is typically something like:
    /// D5VR-53YO-QYJW-VJ4A-4DQR-4LVC-W425-3CXN-F3AQ-J6X2-YVPZ-XBAO-NU4Q
    ///
    /// Yes, it looks like a good old CD key *insert keygen music*
    fn generate_recovery_passphrase() -> (String, Self) {
        let key = Self::generate();
        let b32 = base32::encode(Alphabet::RFC4648 { padding: false }, key.as_ref());
        let passphrase = b32
            .as_bytes()
            .chunks(4)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .expect("Unreachable because variable b32 is a utf8 string")
            .join("-");
        (passphrase, key)
    }

    fn from_recovery_passphrase(passphrase: &str) -> Result<Self, CryptoError> {
        // Lowercase is not allowed in theory, but it's too tempting to fix this here ;-)
        let passphrase = passphrase.to_uppercase();
        // Filter out any unknown characters, this is typically useful to remove
        // the `-` and whitespaces.
        // Note we also discard possible typos from the user (for instance if he types
        // a `8` or a `9`), but this is no big deal given 1) it should not happen
        // because GUI should use `RECOVERY_PASSPHRASE_SYMBOLS` to prevent user
        // from being able to provide invalid characters and 2) it will most likely
        // lead to a bad password anyway
        let mut b32 = passphrase
            .chars()
            .filter(|c| RECOVERY_PASSPHRASE_SYMBOLS.contains(c))
            .map(|c| match c {
                '0' => 'O',
                '1' => 'I',
                _ => c,
            })
            .collect::<String>();
        // Add padding
        for _ in -(passphrase.len() as i32) % 8..0 {
            b32.push('=');
        }

        let rawkey = base32::decode(Alphabet::RFC4648 { padding: true }, &b32)
            .expect("Unreachable due to construction of variable b32");

        Self::try_from(&rawkey[..])
    }
}
