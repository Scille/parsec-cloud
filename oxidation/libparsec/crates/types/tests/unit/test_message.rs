// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;

use crate::{
    fixtures::{alice, bob, Device},
    EntryID, MessageContent,
};
use libparsec_crypto::*;

#[rstest]
fn serde_sharing_granted_message(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "sharing.granted"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   name: "wksp1"
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   encryption_revision: 3
    //   encrypted_on: ext(1, 1638618643.208821)
    //   key: hex!("b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3")
    let data = hex!(
        "6889d8e705000bbae41ec97c41b65120b049dd79cff35254f22cccfe5ca693434a61ce5e820e8a"
        "5507dc0bfad00953bd260657cf887ecde0df8788b23b757fca98fcfd0e0a7e83d4687e443b6d5c"
        "06bd56d58065eb081ace97f953cdaba770826f4dbc0a39d397f47624bec096db6d672796d216e8"
        "ff6dc0330663e1be35cf3a67923ed91da5c44ab7f32e4bdb0f545f69989bee813be91539495b98"
        "1521165b875f234bc9f0f5591cda817118a28ff7efb4b5648b5984be5b6bf4321dde9641474540"
        "d74fb313fc7d1d7063f41af9a929b60665037abbcd7b1704a6c97395c83c4c5aade84d1376d56c"
        "e3972fa90b12566cc9e7913b3a86be8c2d56a74a4215ee11843f0048cc13e4fc573b9ff2b97473"
        "686dcc4a88699d156ba17e"
    );
    let timestamp = "2021-12-04T11:50:43.208821Z".parse().unwrap();

    let expected = MessageContent::SharingGranted {
        author: alice.device_id.to_owned(),
        timestamp,
        name: "wksp1".parse().unwrap(),
        id: EntryID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        encryption_revision: 3,
        encrypted_on: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        key: SecretKey::from(hex!(
            "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
        )),
    };

    let message = MessageContent::decrypt_verify_and_load_for(
        &data,
        &bob.private_key,
        &alice.verify_key(),
        &alice.device_id,
        timestamp,
    )
    .unwrap();

    assert_eq!(message, expected);

    // Also test serialization round trip
    let data2 = message.dump_sign_and_encrypt_for(&alice.signing_key, &bob.public_key());
    // Note we cannot just compare with `data` due to signature and keys order
    let message2 = MessageContent::decrypt_verify_and_load_for(
        &data2,
        &bob.private_key,
        &alice.verify_key(),
        &alice.device_id,
        timestamp,
    )
    .unwrap();
    assert_eq!(message2, expected);
}

#[rstest]
fn serde_sharing_reencrypted_message(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "sharing.reencrypted"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   name: "wksp1"
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   encryption_revision: 3
    //   encrypted_on: ext(1, 1638618643.208821)
    //   key: hex!("b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3")
    let data = hex!(
        "cdd07aae900833f4afea2081eebee25b91d5d52ea28e2aebc667225d6964797bb5647009b5c670"
        "9ce6622684faad75ae55bbcf387337f02664464013f7b1dbff6018efe99adc9d8338244031afc1"
        "8afcfa484931be5888483d984eecfc83cf630aa8597f83f22cb4af6cba37c1af080f136b5d8e3e"
        "b9434441087e598844be61d1d458ea667156e97b789240e9159a5619ef71d27da0e5b5eebbe725"
        "15ea77fdd9ee782608e2527eff5bfc5a3a3e4c8b95bce6cf24492d97ec7dfe3fe75f810b4c3335"
        "d7b1cbc045aabaed0c51ee974af50b9a5d75f9eac59599bf6fa0cb96ed6729fb8f5f6ef0a0684d"
        "71cf79a00b82dab088a73a16e9c0deeaa79800ef8089851b689a678b45b404e79a05f43e12586b"
        "ac45feee5177af0487"
    );
    let timestamp = "2021-12-04T11:50:43.208821Z".parse().unwrap();

    let expected = MessageContent::SharingReencrypted {
        author: alice.device_id.to_owned(),
        timestamp,
        name: "wksp1".parse().unwrap(),
        id: EntryID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        encryption_revision: 3,
        encrypted_on: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        key: SecretKey::from(hex!(
            "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
        )),
    };

    let message = MessageContent::decrypt_verify_and_load_for(
        &data,
        &bob.private_key,
        &alice.verify_key(),
        &alice.device_id,
        timestamp,
    )
    .unwrap();

    assert_eq!(message, expected);

    // Also test serialization round trip
    let data2 = message.dump_sign_and_encrypt_for(&alice.signing_key, &bob.public_key());
    // Note we cannot just compare with `data` due to signature and keys order
    let message2 = MessageContent::decrypt_verify_and_load_for(
        &data2,
        &bob.private_key,
        &alice.verify_key(),
        &alice.device_id,
        timestamp,
    )
    .unwrap();
    assert_eq!(message2, expected);
}

#[rstest]
fn serde_sharing_revoked_message(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "sharing.revoked"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    let data = hex!(
        "a7a5f09f3591b79c6ff9afcb58dc116edea24134083d1306f057ac6fdbcc641588e4d749318f10"
        "0fbd218c7e90b33f3b45389588593014dd8fab332849a3efc59abc1c27cca39de6766789a4984a"
        "afa44b95cc09ec70373cd94c2158258628c4ac388240e97d74759b35393405e4e29c353c92751f"
        "9839d500aa4762b0ff9c2dd2e5756fe9ffffa14e7d3d8b1cd7c58c4ad224f0a7ff1d0b7465bc00"
        "d9cc0f8eb5965858e5291fc59b7e63a518303a6ae7b23a819aac032422f21fc404f003665e4776"
        "69de94e7305ee83ead"
    );
    let timestamp = "2021-12-04T11:50:43.208821Z".parse().unwrap();

    let expected = MessageContent::SharingRevoked {
        author: alice.device_id.to_owned(),
        timestamp,
        id: EntryID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
    };

    let message = MessageContent::decrypt_verify_and_load_for(
        &data,
        &bob.private_key,
        &alice.verify_key(),
        &alice.device_id,
        timestamp,
    )
    .unwrap();

    assert_eq!(message, expected);

    // Also test serialization round trip
    let data2 = message.dump_sign_and_encrypt_for(&alice.signing_key, &bob.public_key());
    // Note we cannot just compare with `data` due to signature and keys order
    let message2 = MessageContent::decrypt_verify_and_load_for(
        &data2,
        &bob.private_key,
        &alice.verify_key(),
        &alice.device_id,
        timestamp,
    )
    .unwrap();
    assert_eq!(message2, expected);
}

#[rstest]
fn serde_ping_message(alice: &Device, bob: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "ping"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   ping: "foo"
    let data = hex!(
        "5f27e2d92463f8f4efa6925cf58c69c0dbbaf816ef9d998f7e2c68b9a9136e40c7d22aa586dded"
        "c5f140e28c1bc12956c2a4789eeeef30e994f2c6b48db20435f9253ccbdec2a450b9a9fb247962"
        "45ad4351639e1a2e090f413b00c572375093c62348e346e23eb19cc1e42b56aa402e30ab6bc24b"
        "4e17745404ed85b67e8246aa3f5ba1e8cf8c8f1f0beb50da6ed53dbc08b478b479138607ed319f"
        "8a22f1a239d3bcf431a667d9d42f755916cdefaa"
    );
    let timestamp = "2021-12-04T11:50:43.208821Z".parse().unwrap();

    let expected = MessageContent::Ping {
        author: alice.device_id.to_owned(),
        timestamp,
        ping: "foo".to_owned(),
    };

    let message = MessageContent::decrypt_verify_and_load_for(
        &data,
        &bob.private_key,
        &alice.verify_key(),
        &alice.device_id,
        timestamp,
    )
    .unwrap();

    assert_eq!(message, expected);

    // Also test serialization round trip
    let data2 = message.dump_sign_and_encrypt_for(&alice.signing_key, &bob.public_key());
    // Note we cannot just compare with `data` due to signature and keys order
    let message2 = MessageContent::decrypt_verify_and_load_for(
        &data2,
        &bob.private_key,
        &alice.verify_key(),
        &alice.device_id,
        timestamp,
    )
    .unwrap();
    assert_eq!(message2, expected);
}
