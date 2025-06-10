// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use hex_literal::hex;
use pretty_assertions::{assert_eq as p_assert_eq, assert_matches as p_assert_matches};
use rstest::rstest;

use libparsec_crypto::{CryptoError, KeyDerivation, PasswordAlgorithm, SecretKey};

#[test]
fn argon2id() {
    let algo = PasswordAlgorithm::generate_argon2id();
    p_assert_matches!(algo, PasswordAlgorithm::Argon2id { .. });
}

#[test]
fn generate_fake_from_seed() {
    let seed1 = SecretKey::from([1u8; 32]);
    let seed2 = SecretKey::from([2u8; 32]);

    let alice_algo1 = PasswordAlgorithm::generate_fake_from_seed("alice@example.com", &seed1);
    let alice_algo1_retried =
        PasswordAlgorithm::generate_fake_from_seed("alice@example.com", &seed1);
    let alice_algo2 = PasswordAlgorithm::generate_fake_from_seed("alice@example.com", &seed2);
    let bob_algo1 = PasswordAlgorithm::generate_fake_from_seed("bob@example.com", &seed1);

    assert_eq!(alice_algo1, alice_algo1_retried);
    assert_ne!(alice_algo1, alice_algo2);
    assert_ne!(alice_algo1, bob_algo1);

    let legit = PasswordAlgorithm::generate_argon2id();
    match (&alice_algo1, &legit) {
        (
            PasswordAlgorithm::Argon2id {
                salt,
                opslimit,
                memlimit_kb,
                parallelism,
            },
            PasswordAlgorithm::Argon2id {
                salt: expected_salt,
                opslimit: expected_opslimit,
                memlimit_kb: expected_memlimit_kb,
                parallelism: expected_parallelism,
            },
        ) => {
            assert_ne!(salt, expected_salt);
            assert_eq!(salt.len(), expected_salt.len());
            assert_eq!(opslimit, expected_opslimit);
            assert_eq!(memlimit_kb, expected_memlimit_kb);
            assert_eq!(parallelism, expected_parallelism);
        }
    }
}

#[test]
fn serialization() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   type: 'ARGON2ID'
    //   memlimit_kb: 3
    //   opslimit: 65536
    //   parallelism: 1
    //   salt: 0x3c64756d6d795f73616c743e
    let raw: &[u8] = hex!(
        "85a474797065a84152474f4e324944ab6d656d6c696d69745f6b6203a86f70736c696d"
        "6974ce00010000ab706172616c6c656c69736d01a473616c74c40c3c64756d6d795f73"
        "616c743e"
    )
    .as_ref();

    let expected = PasswordAlgorithm::Argon2id {
        salt: b"<dummy_salt>".to_vec(),
        opslimit: 65536,
        memlimit_kb: 3,
        parallelism: 1,
    };

    let data: PasswordAlgorithm = rmp_serde::from_slice(raw).unwrap();
    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = rmp_serde::to_vec_named(&data).unwrap();
    println!("***expected: {:?}", &raw2);
    assert_eq!(raw2, raw);
    let data2: PasswordAlgorithm = rmp_serde::from_slice(&raw2).unwrap();
    assert_eq!(data2, expected);
}

macro_rules! compute_test {
    ($name:ident, $ty:ident) => {
        mod $name {
            use super::*;

            #[test]
            fn ok() {
                let algo = PasswordAlgorithm::Argon2id {
                    salt: hex!("cffcc16d78cbc0e773aa5ee7b2210159").to_vec(),
                    opslimit: 1,
                    memlimit_kb: 8,
                    parallelism: 1,
                };
                let password = "P@ssw0rd.".to_owned().into();

                let expected = $ty::from(hex!(
                    "a2754dba7066a49f487737790388548c2af0ddfbed609805184ca5023afe1983"
                ));

                p_assert_eq!(algo.$name(&password).unwrap(), expected);
            }

            #[rstest]
            #[case::salt_too_small(PasswordAlgorithm::Argon2id {
                                            salt: b"dummy".to_vec(),
                                            opslimit: 1,
                                            memlimit_kb: 8,
                                            parallelism: 1,
                                    })]
            #[case::opslimit_too_small(PasswordAlgorithm::Argon2id {
                                        salt: hex!("cffcc16d78cbc0e773aa5ee7b2210159").to_vec(),
                                        opslimit: 0,
                                        memlimit_kb: 8,
                                        parallelism: 1,
                                    })]
            #[case::memlimit_too_small(PasswordAlgorithm::Argon2id {
                                        salt: hex!("cffcc16d78cbc0e773aa5ee7b2210159").to_vec(),
                                        opslimit: 1,
                                        memlimit_kb: 7,
                                        parallelism: 1,
                                    })]
            #[case::parallelism_too_small(PasswordAlgorithm::Argon2id {
                                        salt: hex!("cffcc16d78cbc0e773aa5ee7b2210159").to_vec(),
                                        opslimit: 1,
                                        memlimit_kb: 8,
                                        parallelism: 0,
                                    })]
            fn ko(#[case] algo: PasswordAlgorithm) {
                p_assert_matches!(
                    algo.$name(&"Passw0rd".to_owned().into()),
                    Err(CryptoError::DataSize)
                );
            }
        }
    };
}

compute_test!(compute_secret_key, SecretKey);
compute_test!(compute_key_derivation, KeyDerivation);
