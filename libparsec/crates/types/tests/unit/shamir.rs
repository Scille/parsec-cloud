// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::num::NonZeroU8;

use libparsec_tests_lite::prelude::*;

use crate::{
    fixtures::{alice, Device},
    prelude::*,
};

#[rstest]
fn serde_shamir_share_invalid_key_size() {
    p_assert_eq!(
        ShamirShare::load(b""),
        Err(CryptoError::KeySize {
            expected: 2,
            got: 0
        })
    )
}

#[rstest]
fn serde_shamir_recovery_secret() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   type: 'shamir_recovery_secret'
    //   data_key: 0xb1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3
    //   reveal_token: 0xa927333c187d46f28bce5892563da708
    let encrypted: Vec<ShamirShare> = [
        &hex!(
            "016126310fa3549a308621f53cdca2b14a25b4b94e214a91d90bc4dc76414d1ef4f7c0"
            "d6d107adbab597d74f9c2bccd4273154ef95961ee4fd0da9042730e62ee2e44c9a555b"
            "f4ac6d6208c6ad81cc1b44e0fd6105158ee2dbfd85d7c144d6a63cf4e0122f4445411b"
            "56c2a892f3820e55cb"
        ),
        &hex!(
            "02c234a06f41a8c12b14426f8939d2ef3b8de0d73ff52fa94e803a005d1835aa7e1208"
            "1e1a98e8f5929f100286b7381ac533c80df543068b26a4815d0bbc28ee42be2cac2b94"
            "4f66e4288d9d09700c6c913def2b9c859b760853f62f2e351ee329c53b4d0bcca205fc"
            "a71902d150e35b5e93"
        ),
        &hex!(
            "03a33a244f1ffc03229163191191092e141e270610b90c4ac8f99bbf442f1dc6f3babb"
            "ada8ed203b8f6ca6397bc39fab9bc6bc53d5fb0eae6fc3996ae43399ae22880cbe01d1"
            "262068e5055f9ed44c412976e1e6ebf563f1b2c22c8c801aad2bd121727817b4ff39a1"
            "0350641b313768ac50"
        ),
        &hex!(
            "0499109faf984d771d2d8446feee3253d9c0480bdd40e5d97d8bdba50baac5df77c585"
            "9391bb626bdc8f8398b292cd9b1c37edd435f436558debd1ef53b9a9731f0aecc0d717"
            "24efebbc9a2b5c8f9182269acbbfb3b8b143b31210c2edd7936903a790f343c1718d2f"
            "58b24b570b21f14823"
        ),
    ]
    .into_iter()
    .map(|raw| ShamirShare::load(raw).unwrap())
    .collect();

    let threshold = 2.try_into().unwrap();
    let data_key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));
    let reveal_token = AccessToken::from_hex("a927333c187d46f28bce5892563da708").unwrap();

    let expected = ShamirRecoverySecret {
        data_key,
        reveal_token,
    };

    let data = ShamirRecoverySecret::decrypt_and_load_from_shares(threshold, &encrypted).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt_into_shares(
        threshold,
        NonZeroU8::try_from(u8::try_from(encrypted.len()).unwrap()).unwrap(),
    );

    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = ShamirRecoverySecret::decrypt_and_load_from_shares(threshold, &encrypted2).unwrap();
    p_assert_eq!(data2, expected);
}

#[rstest]
fn serde_shamir_recovery_share_data(alice: &Device) {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   type: 'shamir_recovery_share_data'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   weighted_share: [ 0x3132, 0x6162, ]
    let encrypted: &[u8] = hex!(
        "63aee06749879cab0e65621ac823d9978039dadbce89445336ab3b3d06374b3cd88c7e"
        "3670d669161d00c73bef9ce8372281a53771477d593525ba3ae5ce48b8d5d020785746"
        "bf80aa66e13406238840ef577ff443187bfbf0f5d51414bf119629634f66ca0eb4d6c5"
        "f746d14be78c53db137e76ef06865d891a441cd981d4d3b190a6b8eb3eeceaf983e7ed"
        "320222a144a52f25ce09a7a05b380375c6f0b1723860a4dec0ec491de08c3a813f7472"
        "203ac6ac5d9a42e6d9290b889123cf9450ec254e59363c11708d89df3cf73e6a2339ae"
        "41ef209ce609cb3a044d"
    )
    .as_ref();

    let expected = ShamirRecoveryShareData {
        author: alice.device_id,
        timestamp: "2020-01-01T00:00:00Z".parse().unwrap(),
        weighted_share: vec![
            ShamirShare::load(b"12").unwrap(),
            ShamirShare::load(b"ab").unwrap(),
        ],
    };

    let data = ShamirRecoveryShareData::decrypt_verify_and_load_for(
        encrypted,
        &alice.private_key,
        &alice.verify_key(),
        alice.device_id,
        "2020-01-01T00:00:00Z".parse().unwrap(),
    )
    .unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_sign_and_encrypt_for(&alice.signing_key, &alice.public_key());
    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = ShamirRecoveryShareData::decrypt_verify_and_load_for(
        &encrypted2,
        &alice.private_key,
        &alice.verify_key(),
        alice.device_id,
        "2020-01-01T00:00:00Z".parse().unwrap(),
    )
    .unwrap();
    p_assert_eq!(data2, expected);
}

#[rstest]
fn serde_shamir_recovery_share_data_weighted_max_value(alice: &Device) {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   type: 'shamir_recovery_share_data'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   weighted_share: [ <255 times 0x3132> ]
    let raw: &[u8] = hex!(
    "c0a83b1006f49ccf14259f59a9e36f99940967a4db90ec796beb9a9aff202d5524400d"
    "4389fd16bc1e801559d4fc620c6a5f24377214a69eebd0855723ebd303c9de7a74c9f5"
    "df75eb238eb660808c3d8728821290a190865df1f907ec5d5774654e7e3bdb35007710"
    "5d181a917297275731290df6aa17627f68792bcde642cf2721aecf332a896a89804cbf"
    "5ff817fa50363b25fb3c0d95f9c9d5052352333f644fc4514f2a8f1de5921720a3a6f4"
    "5612a5e7da38e85d968cdc69cfcaf558f395d9423bbb440aa38f52b16bdcb0bf4a86fa"
    "046a79e83a037d77b830c7"
    )
    .as_ref();

    let data = ShamirRecoveryShareData::decrypt_verify_and_load_for(
        raw,
        &alice.private_key,
        &alice.verify_key(),
        alice.device_id,
        "2020-01-01T00:00:00Z".parse().unwrap(),
    )
    .unwrap();

    p_assert_eq!(data.weighted_share.len(), 255)
}

#[rstest]
fn serde_shamir_recovery_share_data_weighted_share_too_big(alice: &Device) {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   type: 'shamir_recovery_share_data'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   weighted_share: [ <256 times 0x3132> ]
    let raw: &[u8] = hex!(
        "b274036a3b6aa66eba12a68e3084db0027079e8d972ab9143742f1a43bcf7162a58e8a"
        "0071597bb780f37e0fe5c2b3ae743eea4dbc4a4a9851f55b09427362cbcd230acc4f98"
        "37a4ca085bdf97672b8920ebef60ce4b30303edf06536f40e69e8110c70ab9cc04590f"
        "a00560fef8e6fa588b6dac90046c361054da2ce8652dcf73673d73dba02fd7c5a9a3e4"
        "16ba93909bfe840ab1cb10fa50cf6414be26b4c7dea86cc27cdaa961308e032a7f5204"
        "971287a1c58f0545e7ebf65bb6fd5467f35c5592d1c0c4029534897f049605da758686"
        "5fbd3a0cb24de9106de89e"
    )
    .as_ref();

    let err = ShamirRecoveryShareData::decrypt_verify_and_load_for(
        raw,
        &alice.private_key,
        &alice.verify_key(),
        alice.device_id,
        "2020-01-01T00:00:00Z".parse().unwrap(),
    )
    .unwrap_err();

    p_assert_matches!(err, DataError::DataIntegrity{
            data_type,
            invariant
        } if data_type == "libparsec_types::shamir::ShamirRecoveryShareData" && invariant == "weighted_share <= 255"
    );
}

#[rstest]
fn serde_shamir_recovery_share_data_bad_expected(alice: &Device) {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   type: 'shamir_recovery_share_data'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1577836800000000) i.e. 2020-01-01T01:00:00Z
    //   weighted_share: [ 0x3132, 0x6162, ]
    let raw: &[u8] = hex!(
        "63aee06749879cab0e65621ac823d9978039dadbce89445336ab3b3d06374b3cd88c7e"
        "3670d669161d00c73bef9ce8372281a53771477d593525ba3ae5ce48b8d5d020785746"
        "bf80aa66e13406238840ef577ff443187bfbf0f5d51414bf119629634f66ca0eb4d6c5"
        "f746d14be78c53db137e76ef06865d891a441cd981d4d3b190a6b8eb3eeceaf983e7ed"
        "320222a144a52f25ce09a7a05b380375c6f0b1723860a4dec0ec491de08c3a813f7472"
        "203ac6ac5d9a42e6d9290b889123cf9450ec254e59363c11708d89df3cf73e6a2339ae"
        "41ef209ce609cb3a044d"
    )
    .as_ref();

    // Bad expected author

    let err = ShamirRecoveryShareData::decrypt_verify_and_load_for(
        raw,
        &alice.private_key,
        &alice.verify_key(),
        "bob@dev1".parse().unwrap(),
        "2020-01-01T00:00:00Z".parse().unwrap(),
    )
    .unwrap_err();
    p_assert_matches!(err, DataError::UnexpectedAuthor {
        expected,
        got
    } if expected == "bob@dev1".parse().unwrap() && got == Some(alice.device_id)
    );

    // Bad expected timestamp

    let err = ShamirRecoveryShareData::decrypt_verify_and_load_for(
        raw,
        &alice.private_key,
        &alice.verify_key(),
        alice.device_id,
        "2020-01-02T00:00:00Z".parse().unwrap(),
    )
    .unwrap_err();
    p_assert_matches!(err, DataError::UnexpectedTimestamp {
        expected,
        got,
    } if expected == "2020-01-02T00:00:00Z".parse().unwrap() && got == "2020-01-01T00:00:00Z".parse().unwrap());
}
