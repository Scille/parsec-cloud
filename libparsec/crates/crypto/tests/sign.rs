// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::convert::TryFrom;

use hex_literal::hex;
use pretty_assertions::{assert_eq, assert_matches};
use serde_test::{assert_tokens, Token};

use libparsec_crypto::{CryptoError, SigningKey, VerifyKey};

#[macro_use]
mod common;

#[test]
fn consts() {
    assert_eq!(SigningKey::ALGORITHM, "ed25519");
    assert_eq!(VerifyKey::ALGORITHM, "ed25519");

    assert_eq!(SigningKey::SIZE, 32);
    assert_eq!(VerifyKey::SIZE, 32);
}

test_serde!(signing_serde, SigningKey);
test_serde!(verify_serde, VerifyKey);

#[test]
fn round_trip() {
    let sk = SigningKey::generate();

    let data = b"Hello, world !";
    let signed = sk.sign(data);

    let vk = sk.verify_key();
    let verified_data = vk.verify(&signed).unwrap();
    assert_eq!(verified_data, data);

    let expected_signature = &signed[..SigningKey::SIGNATURE_SIZE];
    let expected_message = &signed[SigningKey::SIGNATURE_SIZE..];

    let (signature, message) = VerifyKey::unsecure_unwrap(&signed).unwrap();
    assert_eq!(signature, expected_signature);
    assert_eq!(message, expected_message);
}

#[test]
fn signature_verification_spec() {
    let vk = VerifyKey::try_from(hex!(
        "78958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537"
    ))
    .unwrap();
    // Signed text generated with base python implementation
    let signed_text = hex!(
        "32d26711dc973e8df13bbafddc23fc26efe4aca1b86a4e0e7dad7c03df7ffc25d24b865478d164f8868ad0e087587e"
        "2c45e45d5598c7929b4605699bbab4b109616c6c20796f75722062617365206172652062656c6f6e6720746f207573"
    );

    let text = vk.verify(&signed_text).unwrap();
    assert_eq!(text, b"all your base are belong to us");

    let (signature, message) = VerifyKey::unsecure_unwrap(&signed_text).unwrap();
    assert_eq!(message, b"all your base are belong to us");
    assert_eq!(signature, &signed_text[..64]);
}

#[test]
fn signature_only() {
    let sk = SigningKey::generate();

    let data = b"Hello world, I would like to sign this message!";
    let signed = sk.sign_only_signature(data);
    let expected_signed_message = sk.sign(data);
    let expected_signature = &expected_signed_message[..SigningKey::SIGNATURE_SIZE];

    assert_eq!(signed, expected_signature);

    let vk = sk.verify_key();
    let signed_message = Vec::from_iter(signed.iter().chain(data).copied());
    let res = vk.verify(&signed_message).unwrap();

    assert_eq!(res, data);

    // Also test verify_with_signature

    vk.verify_with_signature(signed.as_ref().try_into().unwrap(), data)
        .unwrap();
}

test_msgpack_serialization!(
    signingkey_serialization_spec,
    SigningKey,
    hex!("bae756e3815f05b1a5877c7d625d51af5805ef217142781948e62215bdf0f21b"),
    hex!("c420bae756e3815f05b1a5877c7d625d51af5805ef217142781948e62215bdf0f21b")
);

test_msgpack_serialization!(
    verifykey_serialization_spec,
    VerifyKey,
    hex!("78958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537"),
    hex!("c42078958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537")
);

#[test]
fn signing_key_should_verify_length_when_deserialize() {
    let data = hex!("c40564756d6d79");
    assert_eq!(
        rmp_serde::from_slice::<SigningKey>(&data)
            .unwrap_err()
            .to_string(),
        "Invalid data size"
    );
}

#[test]
fn verify_key_should_verify_length_when_deserialize() {
    let data = hex!("c40564756d6d79");
    assert_eq!(
        rmp_serde::from_slice::<VerifyKey>(&data)
            .unwrap_err()
            .to_string(),
        "Invalid data size"
    );
}

#[test]
fn verify_key_bad_point_decompression() {
    // This is a weird case :/
    //
    // Rustcrypto ed25519_dalek returns an error if the provided key doesn't
    // correspond to an actual point on the curve (i.e. is not a valid public key).
    //
    // However Sodiumoxide's libsodium doesn't do this check and just return a key
    // object that always fails to verify.
    //
    // So we must accept that both implementations have different behavior on this,
    // and we just make sure no panic occur.
    match VerifyKey::try_from(hex!(
        "7e771d03da8ed86aaea5b82f2b3754984cb49023e9c51297b99c5c4fd0d2dc54"
    )) {
        // ed25519_dalek style
        Err(err) => assert_matches!(err, CryptoError::DataSize),
        // libsodium style
        Ok(key) => {
            let signed_text = hex!(
                "32d26711dc973e8df13bbafddc23fc26efe4aca1b86a4e0e7dad7c03df7ffc25d24b865478d164f8868ad0e087587e"
                "2c45e45d5598c7929b4605699bbab4b109616c6c20796f75722062617365206172652062656c6f6e6720746f207573"
            );
            assert_matches!(
                key.verify(&signed_text),
                Err(CryptoError::SignatureVerification)
            );
        }
    }
}

#[test]
fn signkey_from_too_small_data() {
    assert_matches!(
        SigningKey::try_from(b"dummy".as_ref()),
        Err(CryptoError::DataSize)
    );
}

#[test]
fn verifykey_from_too_small_data() {
    assert_matches!(
        VerifyKey::try_from(b"dummy".as_ref()),
        Err(CryptoError::DataSize)
    );
}

#[test]
fn signed_too_small() {
    let too_small = b"dummy";

    assert_matches!(
        VerifyKey::unsecure_unwrap(too_small),
        Err(CryptoError::Signature)
    );

    let vk = VerifyKey::try_from(hex!(
        "78958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537"
    ))
    .unwrap();
    assert_matches!(vk.verify(too_small), Err(CryptoError::Signature));
}

#[test]
fn verifykey_hash() {
    let vk1 = VerifyKey::try_from(hex!(
        "78958e49abad190be2d51bab73af07f87682cfcd65cceedd27e4b2a94bfd8537"
    ))
    .unwrap();
    let vk2 = VerifyKey::try_from(hex!(
        "56aed4f320064c98390e02b7fc592c0af95b5fc2e37e50145fc4ed789e16d857"
    ))
    .unwrap();

    let hash = |x: &VerifyKey| {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        hasher.finish()
    };

    assert_eq!(hash(&vk1), hash(&vk1));
    assert_ne!(hash(&vk1), hash(&vk2));
}
