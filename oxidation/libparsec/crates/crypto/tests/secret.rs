// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use pretty_assertions::assert_eq;
use rstest::rstest;
use serde_test::{assert_tokens, Token};

use libparsec_crypto::{CryptoError, SecretKey};

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

#[test]
fn secret_key_should_verify_length_when_deserialize() {
    let data = hex!("c40564756d6d79");
    assert_eq!(
        rmp_serde::from_slice::<SecretKey>(&data)
            .unwrap_err()
            .to_string(),
        "Invalid data size"
    );
}

#[test]
fn test_recovery_passphrase() {
    let (passphrase, key) = SecretKey::generate_recovery_passphrase();

    let key2 = SecretKey::from_recovery_passphrase(&passphrase).unwrap();
    assert_eq!(key2, key);

    // Add dummy stuff to the passphrase should not cause issues
    let altered_passphrase = passphrase.to_lowercase().replace('-', "@  白");
    let key3 = SecretKey::from_recovery_passphrase(&altered_passphrase).unwrap();
    assert_eq!(key3, key);
}

#[rstest]
#[case::empty("")]
#[case::only_invalid_characters("-@//白")]
#[case::too_short("D5VR-53YO-QYJW-VJ4A-4DQR-4LVC-W425-3CXN-F3AQ-J6X2-YVPZ-XBAO")]
#[case::too_long("D5VR-53YO-QYJW-VJ4A-4DQR-4LVC-W425-3CXN-F3AQ-J6X2-YVPZ-XBAO-NU4Q-NU4Q")]
fn test_invalid_passphrase(#[case] bad_passphrase: &str) {
    assert_eq!(
        SecretKey::from_recovery_passphrase(bad_passphrase).unwrap_err(),
        CryptoError::DataSize
    );
}

#[test]
fn test_from_password() {
    let password = "P@ssw0rd.";
    let salt = hex!("cffcc16d78cbc0e773aa5ee7b2210159");

    let expected = SecretKey::from(hex!(
        "8f46e610b307443ec4ac81a4d799cbe1b97987901d4f681b82dacf3b59cad0a1"
    ));

    assert_eq!(SecretKey::from_password(password, &salt), expected);
}
