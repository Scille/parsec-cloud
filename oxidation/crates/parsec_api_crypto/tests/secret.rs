// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use pretty_assertions::assert_eq;
use serde_test::{assert_tokens, Token};

use parsec_api_crypto::{CryptoError, SecretKey};

#[macro_use]
mod common;

#[test]
fn consts() {
    assert_eq!(SecretKey::ALGORITHM, "xsalsa20poly1305");
    assert_eq!(SecretKey::SIZE, 32);
}

test_serde!(serde, SecretKey);

#[test]
fn round_trip() {
    let sk = SecretKey::generate();

    let data = b"Hello, world !";
    let ciphered = sk.encrypt(data);

    let data2 = sk.decrypt(&ciphered).unwrap();
    assert_eq!(data2, data);
}

#[test]
fn bad_decrypt() {
    let sk = SecretKey::generate();

    assert_eq!(sk.decrypt(b""), Err(CryptoError::Nonce));

    assert_eq!(sk.decrypt(&[0; 64]), Err(CryptoError::Decryption));
}

#[test]
fn data_decrypt_spec() {
    let sk = SecretKey::from(hex!(
        "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
    ));
    // Ciphertext generated with base python implementation
    let ciphertext = hex!("5705b4386acf746e64aca52767d7fdd66bff3e82d87be3346c6d8e9c0ca0db43afe622c04473551737c2a0c65200bf64580c40f639ad6c622286ba13a92612ea2358d78f3b96");
    let cleartext = sk.decrypt(&ciphertext).unwrap();
    assert_eq!(cleartext, b"all your base are belong to us");
}

test_msgpack_serialization!(
    secretkey_serialization_spec,
    SecretKey,
    hex!("856785fb1f72d3e2fdace29f02fbf8da9161cc84baec9669870f5c69fa5dc7e6"),
    hex!("c420856785fb1f72d3e2fdace29f02fbf8da9161cc84baec9669870f5c69fa5dc7e6")
);

#[test]
fn hmac() {
    let sk = SecretKey::from(hex!(
        "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
    ));
    let data = b"all your base are belong to us";
    let hmac = sk.hmac(data, 5);
    assert_eq!(hmac, hex!("a0f507f4be"));
}
