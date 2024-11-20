// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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

    let threshold = 2;
    let data_key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));
    let reveal_token = InvitationToken::from_hex("a927333c187d46f28bce5892563da708").unwrap();

    let expected = ShamirRecoverySecret {
        data_key,
        reveal_token,
    };

    let data = ShamirRecoverySecret::decrypt_and_load_from_shares(threshold, &encrypted).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt_into_shares(threshold, encrypted.len());

    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = ShamirRecoverySecret::decrypt_and_load_from_shares(threshold, &encrypted2).unwrap();
    p_assert_eq!(data2, expected);
}

#[rstest]
fn serde_shamir_recovery_share_data(alice: &Device) {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   type: 'shamir_recovery_share_data'
    //   weighted_share: [ 0x3132, 0x6162, ]
    let encrypted: &[u8] = hex!(
        "ffccff01540b6a1b776f0799e4131c1649cb2fa263cce086bd15fab3a0d03b465febec"
        "4ed1008ed3de2bdbf53bf9fdcdb4cfc5ce70057c9bef8a014ece7e50f37867c86ea2dd"
        "ce9fdcfec2869ded44ec0299d9688b85f15d06d35e47eb29754aa51fc6fff018947cf0"
        "5f4847e6d6e63ab74482"
    )
    .as_ref();

    let expected = ShamirRecoveryShareData {
        weighted_share: vec![
            ShamirShare::load(b"12").unwrap(),
            ShamirShare::load(b"ab").unwrap(),
        ],
    };

    let data =
        ShamirRecoveryShareData::decrypt_and_load_for(encrypted, &alice.private_key).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt_for(&alice.public_key());
    // Note we cannot just compare with `data` due to signature and keys order
    let data2 =
        ShamirRecoveryShareData::decrypt_and_load_for(&encrypted2, &alice.private_key).unwrap();
    p_assert_eq!(data2, expected);
}
