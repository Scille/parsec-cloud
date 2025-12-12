// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use hex_literal::hex;
use pretty_assertions::{assert_eq, assert_matches};
use serde_test::{assert_tokens, Token};

use super::{
    platform,
    utils::{test_msgpack_serialization, test_serde},
};
use crate::{CryptoError, PrivateKey, PublicKey, SecretKey, SharedSecretKeyRole};

#[platform::test]
fn consts() {
    assert_eq!(PrivateKey::ALGORITHM, "curve25519blake2bxsalsa20poly1305");
    assert_eq!(PublicKey::ALGORITHM, "curve25519blake2bxsalsa20poly1305");

    assert_eq!(PrivateKey::SIZE, 32);
    assert_eq!(PublicKey::SIZE, 32);
}

test_serde!(private_serde, PrivateKey);
test_serde!(public_serde, PublicKey);

#[platform::test]
fn round_trip() {
    let privkey = PrivateKey::generate();
    let pubkey = privkey.public_key();

    let data = b"Hello, world !";
    let ciphered = pubkey.encrypt_for_self(data);

    let data2 = privkey.decrypt_from_self(&ciphered).unwrap();
    assert_eq!(data2, data);
}

#[platform::test]
fn generate_shared_secret_key_ok() {
    let privkey1 = PrivateKey::from(hex!(
        "7d406ac1e4df6c16d656290350499345432e8f75260c2809302ba8c4500548c2"
    ));
    let privkey2 = PrivateKey::from(hex!(
        "1e41b6450e2862452c2c40f5b1b6298d8a2433b3131ac8291a4840e5057c313b"
    ));
    let expected_shared_key = SecretKey::from(hex!(
        "4883290bad8fb20dee068de4967e5df149bda8c3702864bc7cb5d59c098a642f"
    ));

    let claimer_shared_key = privkey1
        .generate_shared_secret_key(&privkey2.public_key(), SharedSecretKeyRole::Claimer)
        .unwrap();
    assert_eq!(claimer_shared_key, expected_shared_key);

    let greeter_shared_key = privkey2
        .generate_shared_secret_key(&privkey1.public_key(), SharedSecretKeyRole::Greeter)
        .unwrap();
    assert_eq!(greeter_shared_key, expected_shared_key);

    // Make sure the shared key works as intended
    let encrypted = claimer_shared_key.encrypt(b"Hello, world !");
    let decrypted = greeter_shared_key.decrypt(&encrypted).unwrap();
    assert_eq!(&decrypted, b"Hello, world !");
}

#[platform::test]
fn generate_shared_secret_key_non_contributory() {
    // Shared secret key generation is based on libsodium's `crypto_scalarmult`,
    // which itself uses `x25519` Diffie-Hellman, in which an all-zero key cause
    // issue with "contributory" behavior (aka that each party contributed a public
    // value which increased the security of the resulting shared secret).
    //
    // See https://vnhacker.blogspot.com/2015/09/why-not-validating-curve25519-public.html
    //
    // Last but not least, there is multiple ways to end up with a all-zero, and we test
    // all of them using the `EIGHT_TORSION` list.
    for small_order_ed_point in curve25519_dalek::constants::EIGHT_TORSION {
        let montgomery_point = small_order_ed_point.to_montgomery();
        let raw_point = montgomery_point.to_bytes();
        let invalid_pk = PublicKey::from(raw_point);
        let sk = PrivateKey::generate();

        assert_matches!(
            sk.generate_shared_secret_key(&invalid_pk, SharedSecretKeyRole::Claimer),
            Err(CryptoError::SharedSecretKey(_)),
        );
    }
}

#[platform::test]
fn decrypt_existing() {
    let privkey = PrivateKey::from(hex!(
        "7e771d03da8ed86aaea5b82f2b3754984cb49023e9c51297b99c5c4fd0d2dc54"
    ));
    // Ciphertext generated with base python implementation
    let ciphertext = hex!(
        "e38919105de15295805e7dd8d3c03c79185278b7fd27abac2f9147e06afab065b1a639"
        "db2c1191b930bf172327b0a9b2ab4acad57895e787e1b2dd1dfec9"
    );
    let cleartext = privkey.decrypt_from_self(&ciphertext).unwrap();
    assert_eq!(cleartext, b"Hello, World !");
}

test_msgpack_serialization!(
    privatekey_serialization_spec,
    PrivateKey,
    hex!("8cf4c04041c083fbda69d34d972428e40d1cee3f6ee1dd8268b000bcda39d7f6"),
    hex!("c4208cf4c04041c083fbda69d34d972428e40d1cee3f6ee1dd8268b000bcda39d7f6")
);

test_msgpack_serialization!(
    publickey_serialization_spec,
    PublicKey,
    hex!("397120b8638d42d15c2280d580010b010546d5f4c9d2d6ea8bf80baed7f28f11"),
    hex!("c420397120b8638d42d15c2280d580010b010546d5f4c9d2d6ea8bf80baed7f28f11")
);

#[platform::test]
fn private_key_should_verify_length_when_deserialize() {
    let data = hex!("c40564756d6d79");
    assert_eq!(
        rmp_serde::from_slice::<PrivateKey>(&data)
            .unwrap_err()
            .to_string(),
        "Invalid data size"
    );
}

#[platform::test]
fn public_key_should_verify_length_when_deserialize() {
    let data = hex!("c40564756d6d79");
    assert_eq!(
        rmp_serde::from_slice::<PublicKey>(&data)
            .unwrap_err()
            .to_string(),
        "Invalid data size"
    );
}

#[platform::test]
fn privkey_from_too_small_data() {
    assert!(matches!(
        PrivateKey::try_from(b"dummy".as_ref()),
        Err(CryptoError::DataSize)
    ))
}

#[platform::test]
fn pubkey_from_too_small_data() {
    assert!(matches!(
        PublicKey::try_from(b"dummy".as_ref()),
        Err(CryptoError::DataSize)
    ))
}

#[platform::test]
fn encrypted_too_small() {
    let too_small = b"dummy";

    let privkey = PrivateKey::from(hex!(
        "7e771d03da8ed86aaea5b82f2b3754984cb49023e9c51297b99c5c4fd0d2dc54"
    ));
    assert!(matches!(
        privkey.decrypt_from_self(too_small),
        Err(CryptoError::Decryption)
    ));
}

#[platform::test]
fn pubkey_hash() {
    let vk1 = PublicKey::from(hex!(
        "78958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537"
    ));
    let vk2 = PublicKey::from(hex!(
        "7e771d03da8ed86aaea5b82f2b3754984cb49023e9c51297b99c5c4fd0d2dc54"
    ));

    let hash = |x: &PublicKey| {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        hasher.finish()
    };

    assert_eq!(hash(&vk1), hash(&vk1));
    assert_ne!(hash(&vk1), hash(&vk2));
}

#[platform::test]
fn private_key_from() {
    let raw = hex!("78958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537");
    let key = PrivateKey::from(raw);

    assert_eq!(*key.to_bytes(), raw);
}

#[platform::test]
fn private_key_try_from_array_of_u8() {
    let raw = hex!("78958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537");
    let key = PrivateKey::try_from(raw.as_ref()).unwrap();

    assert_eq!(*key.to_bytes(), raw);

    let outcome = PrivateKey::try_from(b"<too_small>".as_ref());

    assert_eq!(outcome, Err(CryptoError::DataSize));
}

#[platform::test]
fn private_key_try_from_serde_bytes() {
    let raw = hex!("78958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537");
    let key = PrivateKey::try_from(serde_bytes::Bytes::new(&raw)).unwrap();

    assert_eq!(*key.to_bytes(), raw);

    let outcome = PrivateKey::try_from(serde_bytes::Bytes::new(b"<too_small>"));

    assert_eq!(outcome, Err(CryptoError::DataSize));
}

#[platform::test]
fn publickey_key_try_from_static_array_of_u8() {
    let raw = hex!("78958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537");
    let key = PublicKey::from(raw);

    assert_eq!(key.as_ref(), raw);
}

#[platform::test]
fn publickey_key_try_from_array_of_u8() {
    let raw = hex!("78958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537");
    let key = PublicKey::try_from(raw.as_ref()).unwrap();

    assert_eq!(key.as_ref(), raw);

    let outcome = PublicKey::try_from(b"<too_small>".as_ref());

    assert_eq!(outcome, Err(CryptoError::DataSize));
}

#[platform::test]
fn publickey_key_try_from_serde_bytes() {
    let raw = hex!("78958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537");
    let key = PublicKey::try_from(serde_bytes::Bytes::new(&raw)).unwrap();

    assert_eq!(key.as_ref(), raw);

    let outcome = PublicKey::try_from(serde_bytes::Bytes::new(b"<too_small>"));

    assert_eq!(outcome, Err(CryptoError::DataSize));
}
