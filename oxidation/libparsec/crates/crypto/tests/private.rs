// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use pretty_assertions::assert_eq;
use serde_test::{assert_tokens, Token};

use libparsec_crypto::{prelude::*, PrivateKey, PublicKey, SecretKey};

#[macro_use]
mod common;

#[test]
fn consts() {
    assert_eq!(PrivateKey::ALGORITHM, "curve25519blake2bxsalsa20poly1305");
    assert_eq!(PublicKey::ALGORITHM, "curve25519blake2bxsalsa20poly1305");

    assert_eq!(PrivateKey::SIZE, 32);
    assert_eq!(PublicKey::SIZE, 32);
}

test_serde!(private_serde, PrivateKey);
test_serde!(public_serde, PublicKey);

#[test]
fn round_trip() {
    let privkey = PrivateKey::generate();
    let pubkey = privkey.public_key();

    let data = b"Hello, world !";
    let ciphered = pubkey.encrypt_for_self(data);

    let data2 = privkey.decrypt_from_self(&ciphered).unwrap();
    assert_eq!(data2, data);
}

#[test]
fn generate_shared_secret_key() {
    let privkey1 = PrivateKey::from(hex!(
        "7d406ac1e4df6c16d656290350499345432e8f75260c2809302ba8c4500548c2"
    ));
    let privkey2 = PrivateKey::from(hex!(
        "1e41b6450e2862452c2c40f5b1b6298d8a2433b3131ac8291a4840e5057c313b"
    ));
    let expected_shared_key = SecretKey::from(hex!(
        "0cfa61afc1ce40d7e44f512d5c718ec2d4a9caa6aaaa1ea43b96db57ac61612d"
    ));

    let shared_key = privkey1.generate_shared_secret_key(&privkey2.public_key());
    assert_eq!(shared_key, expected_shared_key);

    let inv_shared_key = privkey2.generate_shared_secret_key(&privkey1.public_key());
    assert_eq!(inv_shared_key, expected_shared_key);

    // Make sure the shared key works as intended
    let encrypted = shared_key.encrypt(b"Hello, world !");
    let decrypted = inv_shared_key.decrypt(&encrypted).unwrap();
    assert_eq!(&decrypted, b"Hello, world !");
}

#[test]
fn decrypt_existing() {
    let privkey = PrivateKey::from(hex!(
        "7e771d03da8ed86aaea5b82f2b3754984cb49023e9c51297b99c5c4fd0d2dc54"
    ));
    // Ciphertext generated with base python implementation
    let ciphertext = hex!("e38919105de15295805e7dd8d3c03c79185278b7fd27abac2f9147e06afab065b1a639db2c1191b930bf172327b0a9b2ab4acad57895e787e1b2dd1dfec9");
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

#[test]
fn private_key_should_verify_length_when_deserialize() {
    let data = hex!("c40564756d6d79");
    assert_eq!(
        rmp_serde::from_slice::<PrivateKey>(&data)
            .unwrap_err()
            .to_string(),
        "Invalid data size"
    );
}

#[test]
fn public_key_should_verify_length_when_deserialize() {
    let data = hex!("c40564756d6d79");
    assert_eq!(
        rmp_serde::from_slice::<PublicKey>(&data)
            .unwrap_err()
            .to_string(),
        "Invalid data size"
    );
}
