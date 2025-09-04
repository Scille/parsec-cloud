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
fn consts() {
    assert_eq!(HashDigest::ALGORITHM, "sha256");
    assert_eq!(HashDigest::SIZE, 32);
}

test_serde!(hash_serde, HashDigest);

#[platform::test]
fn hash_spec() {
    let digest = HashDigest::from_data(b"Hello, World !");
    assert_eq!(
        digest.as_ref(),
        &[
            0x7d, 0x48, 0x69, 0x15, 0xb9, 0x14, 0x33, 0x2b, 0xb5, 0x73, 0x0f, 0xd7, 0x72, 0x22,
            0x3e, 0x8b, 0x27, 0x69, 0x19, 0xe5, 0x1e, 0xdc, 0xa2, 0xde, 0x0f, 0x82, 0xc5, 0xfc,
            0x1b, 0xce, 0x7e, 0xb5
        ]
    );
    assert_eq!(
        digest.hexdigest(),
        "7d486915b914332bb5730fd772223e8b276919e51edca2de0f82c5fc1bce7eb5".to_string()
    );
}

test_msgpack_serialization!(
    hash_serialization_spec,
    HashDigest,
    hex!("7d486915b914332bb5730fd772223e8b276919e51edca2de0f82c5fc1bce7eb5"),
    hex!("c4207d486915b914332bb5730fd772223e8b276919e51edca2de0f82c5fc1bce7eb5")
);

#[platform::test]
fn hashdigest_should_verify_length_when_deserialize() {
    let data = hex!("c40564756d6d79");
    assert_eq!(
        rmp_serde::from_slice::<HashDigest>(&data)
            .unwrap_err()
            .to_string(),
        "Invalid data size"
    );
}

#[platform::test]
fn from() {
    let raw = hex!("7d486915b914332bb5730fd772223e8b276919e51edca2de0f82c5fc1bce7eb5");
    let digest = HashDigest::from(raw);

    assert_eq!(digest.as_ref(), raw,);
}

#[platform::test]
fn try_from() {
    let raw = hex!("7d486915b914332bb5730fd772223e8b276919e51edca2de0f82c5fc1bce7eb5");
    let digest = HashDigest::try_from(raw.as_ref()).unwrap();

    assert_eq!(digest.as_ref(), raw,);

    let err = HashDigest::try_from(b"<too_small>".as_ref());
    assert_matches!(err, Err(CryptoError::DataSize))
}

#[platform::test]
fn debug() {
    let raw = hex!("7d486915b914332bb5730fd772223e8b276919e51edca2de0f82c5fc1bce7eb5");
    let digest = HashDigest::try_from(raw.as_ref()).unwrap();

    assert_eq!(
        format!("{:?}", digest),
        "HashDigest(7d486915b914332bb5730fd772223e8b276919e51edca2de0f82c5fc1bce7eb5)"
    );
}
