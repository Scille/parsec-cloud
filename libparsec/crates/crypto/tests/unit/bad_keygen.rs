// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use hex_literal::hex;
use pretty_assertions::{assert_eq, assert_matches};
use serde_test::{assert_tokens, Token};

use super::{
    platform,
    utils::{test_msgpack_serialization, test_serde},
};
use crate::{CryptoError, HashDigest};

#[platform::test]
fn bad_keygen() {
    #[cfg(not(feature = "use-libsodium"))]
    {
        panic!("Must run with `--features use-libsodium` !")
    }

    #[cfg(feature = "use-libsodium")]
    {
        const TOTAL: usize = 10_000_000;
        for i in 0..TOTAL {
            let sodium_privkey = libsodium_rs::crypto_sign::KeyPair::generate()
                .expect("Cannot generate libsodium signing key")
                .secret_key;

            let sodium_pubkey =
                libsodium_rs::crypto_sign::PublicKey::from_secret_key(&sodium_privkey)
                    .expect("Cannot get public key from secret key");

            let outcome_privkey =
                ed25519_dalek::SigningKey::try_from(&sodium_privkey.as_bytes()[..32]);
            let outcome_pubkey =
                ed25519_dalek::VerifyingKey::try_from(sodium_pubkey.as_bytes().as_ref());
            if outcome_privkey.is_err() || outcome_pubkey.is_err() {
                panic!("Invalid key detected !privkey outcome: {:?}\npubkey_outcome: {:?}\nprivate: {}\npublic: {}\n", outcome_privkey, outcome_pubkey, &sodium_privkey, &sodium_pubkey);
            }

            if i % 10_000 == 0 {
                println!("{}/{}", i, TOTAL)
            }
        }
    }
}
