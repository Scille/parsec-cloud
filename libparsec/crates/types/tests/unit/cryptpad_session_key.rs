// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{alice, Device};
use crate::prelude::*;

#[rstest]
#[case::edit_key(
    // Generated from Parsec 3.9.3-a.0+dev
    // Content:
    //   type: 'cryptpad_session_key'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   document_id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b)
    //   can_edit: True
    //   key: 'i4+K2i6M9yrFMK0cLiQvbDgP'
    &hex!(
        "edcba800655cce5ed0cbf2b5effb740f7af60a8e1713536d3180d36145fbf48fd3975c"
        "371fb6e66e60c6b3f0f85ccab7a44957d09ed5134bab2170b86d859a36dc55861be2ce"
        "074af4d88f4688262cbd1a81d46d684826d1348cea7c5410796b6a5cddc87bdecd51cb"
        "81776b75e0c46721e64f6dee531cdc52841bb0650dc901950d8b73b9d5666864774aeb"
        "0669f5eb8abcc69c42950afcb259907b064fc3caca59e78b37edd8bb9bb55e632e4a84"
        "fd5a10f466bedfd66d8bc22a1ffa456c342265fe5b243dcd07599a0fbd94582679bf14"
        "eb1a6a735dd54d8a508130c3f3bc0c3cf0c1b281c71be7cfdbacdaccca4b408945af14"
        "7835d732217a1670f235"
    ),
    CryptpadSessionKey {
        author: "alice@dev1".parse().unwrap(),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        document_id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        can_edit: true,
        key: "i4+K2i6M9yrFMK0cLiQvbDgP".to_string(),
    },
)]
#[case::view_key(
    // Generated from Parsec 3.9.3-a.0+dev
    // Content:
    //   type: 'cryptpad_session_key'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   document_id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b)
    //   can_edit: False
    //   key: 'QY6ubMC/np5PCqxO5oOZPP/A9HD3r/fN1wjT/fiZGtU='
    &hex!(
        "fd2cff260742334a41a46f67c4729797ca60f7e2d6d4b02dd196ec233db7ad2406cf35"
        "54aac2967c88dbb729f375e2203c1cb07b4e0e6fc73f541ade1756cd497912f9ec74e6"
        "7b108a97e75ac1a0946278cb478827e5ad1e0da134c95d5afa473490d14ee1571e17f7"
        "47b7a09c5895b6381be59a993fbc76a0e61b1e8d0c7f694ab66cedccdff6f3df67b609"
        "bd1be55c6a786fd9613cf493227f414e6bdaa3cc3f7d7a116a392a697b4499c17289dd"
        "822cfe0c8122d48f8edb4aa8b9f5db11476993ff4211b5b49834f626971f81c17b543c"
        "7ac89a5c2e2f863958fba42270f84879786c0b1c3ee723076dee2c7964c747df38db68"
        "37968fdd4c1975a19d3727c566af238301821ec98b5502d2c610772544fd99"
    ),
    CryptpadSessionKey {
        author: "alice@dev1".parse().unwrap(),
        timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        document_id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        can_edit: false,
        key: "QY6ubMC/np5PCqxO5oOZPP/A9HD3r/fN1wjT/fiZGtU=".to_string(),
    },
)]
fn serde_cryptpad_session_key_ok(
    alice: &Device,
    #[case] raw: &[u8],
    #[case] expected: CryptpadSessionKey,
) {
    let data_key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    println!(
        "***expected: {:?}",
        expected.dump_sign_and_encrypt(&alice.signing_key, &data_key)
    );

    let data = CryptpadSessionKey::decrypt_verify_and_load(
        raw,
        &data_key,
        &alice.verify_key(),
        alice.device_id,
        "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
    )
    .unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = expected.dump_sign_and_encrypt(&alice.signing_key, &data_key);

    let data2 = CryptpadSessionKey::decrypt_verify_and_load(
        &raw2,
        &data_key,
        &alice.verify_key(),
        alice.device_id,
        "2021-12-04T11:50:43.208821Z".parse().unwrap(),
        VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
    )
    .unwrap();

    p_assert_eq!(data2, expected);
}

#[rstest]
fn serde_cryptpad_session_key_invalid_expected(alice: &Device) {
    // Generated from Parsec 3.9.1-a.0+dev
    // Content:
    //   type: 'cryptpad_session_key'
    //   author: ext(2, 0x677ed052cf2c4d8eaa95d10a037c28dd)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   vlob_id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b)
    //   is_edit: False
    //   key: 'QY6ubMC/np5PCqxO5oOZPP/A9HD3r/fN1wjT/fiZGtU='
    let raw = hex!(
        "0028b52ffd0058e9040086a474797065b463727970747061645f73657373696f6e5f6b"
        "6579a6617574686f72d802677ed052cf2c4d8eaa95d10a037c28dda974696d65737461"
        "6d70d7010005d250a2269a75a7766c6f625f6964d80287c6b5fd3b454c94bab51d6af1"
        "c6930ba769735f65646974c2a36b6579d92c51593675624d432f6e7035504371784f35"
        "6f4f5a50502f4139484433722f664e31776a542f66695a4774553d"
    )
    .as_ref();

    let data_key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    // Bad decryption key

    p_assert_matches!(
        CryptpadSessionKey::decrypt_verify_and_load(
            raw,
            &SecretKey::from(hex!(
                "0000000000000000000000000000000000000000000000000000000000000000"
            )), // Dummy key
            &alice.verify_key(),
            alice.device_id,
            "2021-12-04T11:50:43.208821Z".parse().unwrap(),
            VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        ),
        Err(_)
    );

    // Bad verify key

    p_assert_matches!(
        CryptpadSessionKey::decrypt_verify_and_load(
            raw,
            &data_key,
            &VerifyKey::from(hex!(
                "0000000000000000000000000000000000000000000000000000000000000000"
            )), // Dummy key
            alice.device_id,
            "2021-12-04T11:50:43.208821Z".parse().unwrap(),
            VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        ),
        Err(_)
    );

    // Test invalid expected arguments

    p_assert_matches!(
        CryptpadSessionKey::decrypt_verify_and_load(
            raw,
            &data_key,
            &alice.verify_key(),
            DeviceID::from_hex("00000000000000000000000000000000").unwrap(), // Dummy device ID
            "2021-12-04T11:50:43.208821Z".parse().unwrap(),
            VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        ),
        Err(_)
    );

    p_assert_matches!(
        CryptpadSessionKey::decrypt_verify_and_load(
            raw,
            &data_key,
            &alice.verify_key(),
            alice.device_id,
            "1970-01-01T11:50:43.208821Z".parse().unwrap(), // Dummy timestamp
            VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        ),
        Err(_)
    );

    p_assert_matches!(
        CryptpadSessionKey::decrypt_verify_and_load(
            raw,
            &data_key,
            &alice.verify_key(),
            alice.device_id,
            "2021-12-04T11:50:43.208821Z".parse().unwrap(),
            VlobID::from_hex("00000000000000000000000000000000").unwrap(), // Dummy vlob ID
        ),
        Err(_)
    );
}
