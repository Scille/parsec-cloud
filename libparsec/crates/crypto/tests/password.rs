// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use hex_literal::hex;
use pretty_assertions::{assert_eq as p_assert_eq, assert_matches as p_assert_matches};
use rstest::rstest;

use libparsec_crypto::{
    CryptoError, KeyDerivation, PasswordAlgorithm, SecretKey, TrustedPasswordAlgorithm,
    UntrustedPasswordAlgorithm,
};

/*
 * PasswordAlgorithm
 */

#[test]
fn argon2id_random_salt() {
    let algo = PasswordAlgorithm::generate_argon2id(
        libparsec_crypto::PasswordAlgorithmSaltStrategy::Random,
    );
    p_assert_matches!(algo, PasswordAlgorithm::Argon2id { .. });

    let algo2 = PasswordAlgorithm::generate_argon2id(
        libparsec_crypto::PasswordAlgorithmSaltStrategy::Random,
    );
    match (algo, algo2) {
        (
            PasswordAlgorithm::Argon2id { salt, .. },
            PasswordAlgorithm::Argon2id { salt: salt2, .. },
        ) => assert_ne!(salt, salt2),
    }
}

#[test]
fn argon2id_salt_from_email() {
    let algo = PasswordAlgorithm::generate_argon2id(
        libparsec_crypto::PasswordAlgorithmSaltStrategy::DerivedFromEmail {
            email: "alice@example.com",
        },
    );
    p_assert_matches!(algo, PasswordAlgorithm::Argon2id { .. });

    let algo2 = PasswordAlgorithm::generate_argon2id(
        libparsec_crypto::PasswordAlgorithmSaltStrategy::DerivedFromEmail {
            email: "alice@example.com",
        },
    );
    match (algo.clone(), algo2) {
        (
            PasswordAlgorithm::Argon2id { salt, .. },
            PasswordAlgorithm::Argon2id { salt: salt2, .. },
        ) => p_assert_eq!(salt, salt2),
    }

    let algo3 = PasswordAlgorithm::generate_argon2id(
        libparsec_crypto::PasswordAlgorithmSaltStrategy::DerivedFromEmail {
            email: "bob@example.com",
        },
    );
    match (algo, algo3) {
        (
            PasswordAlgorithm::Argon2id { salt, .. },
            PasswordAlgorithm::Argon2id { salt: salt3, .. },
        ) => assert_ne!(salt, salt3),
    }
}

macro_rules! compute_test {
    ($name:ident, $ty:ident) => {
        mod $name {
            use super::*;

            #[test]
            fn ok() {
                let algo = PasswordAlgorithm::Argon2id {
                    salt: hex!("cffcc16d78cbc0e773aa5ee7b2210159"),
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
            #[case::opslimit_too_small(PasswordAlgorithm::Argon2id {
                                                salt: hex!("cffcc16d78cbc0e773aa5ee7b2210159"),
                                                opslimit: 0,
                                                memlimit_kb: 8,
                                                parallelism: 1,
                                            })]
            #[case::memlimit_too_small(PasswordAlgorithm::Argon2id {
                                                salt: hex!("cffcc16d78cbc0e773aa5ee7b2210159"),
                                                opslimit: 1,
                                                memlimit_kb: 7,
                                                parallelism: 1,
                                            })]
            #[case::parallelism_too_small(PasswordAlgorithm::Argon2id {
                                                salt: hex!("cffcc16d78cbc0e773aa5ee7b2210159"),
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

/*
 * TrustedPasswordAlgorithm
 */

#[test]
fn trusted_serialization() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   type: 'ARGON2ID'
    //   memlimit_kb: 131072
    //   opslimit: 3
    //   parallelism: 1
    //   salt: 0x31323334353637383930313233343536
    let raw: &[u8] = hex!(
        "85a474797065a84152474f4e324944ab6d656d6c696d69745f6b62ce00020000a86f70"
        "736c696d697403ab706172616c6c656c69736d01a473616c74c4103132333435363738"
        "3930313233343536"
    )
    .as_ref();

    let expected = TrustedPasswordAlgorithm::Argon2id {
        salt: *b"1234567890123456",
        opslimit: 3,
        memlimit_kb: 128 * 1024,
        parallelism: 1,
    };
    println!(
        "***expected: {:?}",
        rmp_serde::to_vec_named(&expected).unwrap()
    );

    let data: TrustedPasswordAlgorithm = rmp_serde::from_slice(raw).unwrap();
    assert_eq!(data, expected,);

    // Also test serialization round trip
    let raw2 = rmp_serde::to_vec_named(&data).unwrap();
    assert_eq!(raw2, raw);
    let data2: TrustedPasswordAlgorithm = rmp_serde::from_slice(&raw2).unwrap();
    assert_eq!(data2, expected);
}

/*
 * UntrustedPasswordAlgorithm
 */

#[test]
fn untrusted_serialization() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   type: 'ARGON2ID'
    //   memlimit_kb: 131072
    //   opslimit: 3
    //   parallelism: 1
    let raw: &[u8] = hex!(
        "84a474797065a84152474f4e324944ab6d656d6c696d69745f6b62ce00020000a86f70"
        "736c696d697403ab706172616c6c656c69736d01"
    )
    .as_ref();

    let expected = UntrustedPasswordAlgorithm::Argon2id {
        opslimit: 3,
        memlimit_kb: 128 * 1024,
        parallelism: 1,
    };
    println!(
        "***expected: {:?}",
        rmp_serde::to_vec_named(&expected).unwrap()
    );

    let data: UntrustedPasswordAlgorithm = rmp_serde::from_slice(raw).unwrap();
    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = rmp_serde::to_vec_named(&data).unwrap();
    assert_eq!(raw2, raw);
    let data2: UntrustedPasswordAlgorithm = rmp_serde::from_slice(&raw2).unwrap();
    assert_eq!(data2, expected);
}

#[test]
fn untrusted_generate_fake_from_seed() {
    let seed1 = SecretKey::from([1u8; 32]);

    let alice_algo1 =
        UntrustedPasswordAlgorithm::generate_fake_from_seed("alice@example.com", &seed1);
    let alice_algo1_retried =
        UntrustedPasswordAlgorithm::generate_fake_from_seed("alice@example.com", &seed1);

    assert_eq!(alice_algo1, alice_algo1_retried);

    let legit = PasswordAlgorithm::generate_argon2id(
        libparsec_crypto::PasswordAlgorithmSaltStrategy::DerivedFromEmail {
            email: "zack@example.com",
        },
    );
    match (&alice_algo1, &legit) {
        (
            UntrustedPasswordAlgorithm::Argon2id {
                opslimit,
                memlimit_kb,
                parallelism,
            },
            PasswordAlgorithm::Argon2id {
                salt: _,
                opslimit: expected_opslimit,
                memlimit_kb: expected_memlimit_kb,
                parallelism: expected_parallelism,
            },
        ) => {
            assert_eq!(opslimit, expected_opslimit);
            assert_eq!(memlimit_kb, expected_memlimit_kb);
            assert_eq!(parallelism, expected_parallelism);
        }
    }
}

#[rstest]
#[case::opslimit(
    UntrustedPasswordAlgorithm::Argon2id {
        opslimit: 3 - 1,
        memlimit_kb: 128 * 1024,
        parallelism: 1,
    }
)]
#[case::memlimit(
    UntrustedPasswordAlgorithm::Argon2id {
        opslimit: 3,
        memlimit_kb: (128 * 1024) - 1,
        parallelism: 1,
    }
)]
#[case::parallelism(
    UntrustedPasswordAlgorithm::Argon2id {
        opslimit: 3,
        memlimit_kb: 128 * 1024,
        parallelism: 1 - 1,
    }
)]
fn untrusted_validate_config_too_weak(#[case] algo: UntrustedPasswordAlgorithm) {
    p_assert_eq!(
        algo.validate("bob@example.com"),
        Err(CryptoError::Algorithm("Config too weak".to_string()))
    );
}
