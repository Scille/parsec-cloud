// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use hex_literal::hex;
use pretty_assertions::assert_eq;
use serde_test::{assert_tokens, Token};

use libparsec_crypto::{KeyDerivation, SecretKey};

#[macro_use]
mod common;

#[test]
fn consts() {
    assert_eq!(KeyDerivation::ALGORITHM, "blake2b");
    assert_eq!(KeyDerivation::SIZE, 32);
}

test_serde!(serde, KeyDerivation);

#[test]
fn derive_uuid_from_uuid_stability() {
    let id = uuid::uuid!("cadc3f583b8647f2a3227400fc02e096");

    let kd1 = KeyDerivation::generate();
    let gen_id = kd1.derive_uuid_from_uuid(id);

    let kd2 = KeyDerivation::generate();
    let gen_id2 = kd2.derive_uuid_from_uuid(id);
    assert_ne!(gen_id, gen_id2);

    let kd1_bis = KeyDerivation::try_from(kd1.as_ref()).unwrap();
    let gen_id_bis = kd1_bis.derive_uuid_from_uuid(id);
    assert_eq!(gen_id_bis, gen_id);
}

#[test]
fn derive_uuid_from_uuid_spec() {
    let kd = KeyDerivation::from(hex!(
        "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
    ));
    let id = uuid::uuid!("cadc3f583b8647f2a3227400fc02e096");
    let gen_id = kd.derive_uuid_from_uuid(id);

    println!("gen_id: {:X?}", gen_id);

    let expected_gen_id = uuid::uuid!("6e4e1d47d0eb03670695f081d678f0da");
    assert_eq!(gen_id, expected_gen_id);
}

#[test]
fn derive_secret_key_from_uuid_stability() {
    let id = uuid::uuid!("cadc3f583b8647f2a3227400fc02e096");

    let kd1 = KeyDerivation::generate();
    let sk1 = kd1.derive_secret_key_from_uuid(id);

    let kd2 = KeyDerivation::generate();
    let sk2 = kd2.derive_secret_key_from_uuid(id);
    assert_ne!(sk1, sk2);

    let kd1_bis = KeyDerivation::try_from(kd1.as_ref()).unwrap();
    let sk1_bis = kd1_bis.derive_secret_key_from_uuid(id);
    assert_eq!(sk1_bis, sk1);
}

#[test]
fn derive_secret_key_from_uuid_spec() {
    let kd = KeyDerivation::from(hex!(
        "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
    ));
    let id = uuid::uuid!("cadc3f583b8647f2a3227400fc02e096");
    let sk = kd.derive_secret_key_from_uuid(id);

    println!("sk: {:X?}", sk.as_ref());

    let expected_secret_key = SecretKey::from(hex!(
        "37e3cb9389fbc55d17daff9aeedf0ebaf18bc8be733e8a991f56b83bd379f3b8"
    ));
    assert_eq!(sk, expected_secret_key);
}

test_msgpack_serialization!(
    key_derivation_serialization_spec,
    KeyDerivation,
    hex!("856785fb1f72d3e2fdace29f02fbf8da9161cc84baec9669870f5c69fa5dc7e6"),
    hex!("c420856785fb1f72d3e2fdace29f02fbf8da9161cc84baec9669870f5c69fa5dc7e6")
);

#[test]
fn hash() {
    let kd1 = KeyDerivation::from(hex!(
        "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
    ));
    let kd2 = KeyDerivation::from(hex!(
        "8f46e610b307443ec4ac81a4d799cbe1b97987901d4f681b82dacf3b59cad0a1"
    ));

    let hash = |x: &KeyDerivation| {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        hasher.finish()
    };

    assert_eq!(hash(&kd1), hash(&kd1));
    assert_ne!(hash(&kd1), hash(&kd2));
}
