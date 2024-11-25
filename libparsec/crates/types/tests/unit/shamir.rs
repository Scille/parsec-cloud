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
    let reveal_token = InvitationToken::from_hex("a927333c187d46f28bce5892563da708").unwrap();

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

#[rstest]
fn serde_shamir_recovery_share_data_weighted_max_value(alice: &Device) {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   type: 'shamir_recovery_share_data'
    //   weighted_share: [ <256 times 0x3132> ]
    let raw: &[u8] = hex!(
        "35b8e9925e37e1c91b08a25f601e225faf82feb8e5ffa744a9b7a59578f5c50bf0c431"
        "26d5b10b59078a6469861216c5c0cc4bb57edf11c01c6ac2478fd14fb304833a94a3e4"
        "ce66b4ac432d4948aa681df05149f8f8c0ce7f172c8e100f2d4bb0e2c5ad56420520ab"
        "bbfddd4b444cd1760c4280fd14"
    )
    .as_ref();

    let data = ShamirRecoveryShareData::decrypt_and_load_for(raw, &alice.private_key).unwrap();

    p_assert_eq!(data.weighted_share.len(), 255)
}

#[rstest]
fn serde_shamir_recovery_share_data_weighted_share_too_big(alice: &Device) {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   type: 'shamir_recovery_share_data'
    //   weighted_share: [ <257 times 0x3132> ]
    let raw: &[u8] = hex!(
        "40cfe3ab5bd66a3ff24d1d63006846db5d8b731d2efecd56421ace6070ec1e656c3f51"
        "abb94669363396941bb3940af3cf557851a624972c51f4af11aee50a5e591c58e4625f"
        "7a0b177e79ee96c5049dc5cc243c045dba40d4dd0326643c308216424cedb555527681"
        "27320de3459d484e4afedacfa1"
    )
    .as_ref();

    let err = ShamirRecoveryShareData::decrypt_and_load_for(raw, &alice.private_key).unwrap_err();

    p_assert_matches!(err, DataError::DataIntegrity{
            data_type,
            invariant
        } if data_type == "libparsec_types::shamir::ShamirRecoveryShareData" && invariant == "weighted_share <= 255"
    );
}
