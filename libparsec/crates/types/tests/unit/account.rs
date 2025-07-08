// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use libparsec_tests_lite::prelude::*;

use crate::prelude::*;

#[test]
fn validation_code_base() {
    let c = ValidationCode::default();
    let c2 = ValidationCode::default();
    assert_ne!(c, c2);
    // Round-trip check
    let c_raw = std::str::from_utf8(c.0.as_ref()).unwrap();
    p_assert_eq!(c_raw.parse::<ValidationCode>().unwrap(), c);

    let j: ValidationCode = "AD3FXJ".parse().unwrap();
    let j2: ValidationCode = "AD3FXJ".parse().unwrap();
    let h: ValidationCode = "AD3FXH".parse().unwrap();
    p_assert_eq!(j, j2);
    assert_ne!(j, h);

    p_assert_eq!(j.as_ref(), "AD3FXJ");
    p_assert_eq!(format!("{:?}", j), "ValidationCode(\"AD3FXJ\")");

    // Bad size
    p_assert_matches!(
        "I2345".parse::<ValidationCode>(),
        Err(ValidationCodeParseError::BadSize)
    );

    p_assert_matches!(
        "I234567".parse::<ValidationCode>(),
        Err(ValidationCodeParseError::BadSize)
    );

    // Not base32
    p_assert_matches!(
        "123456".parse::<ValidationCode>(), // 1 is not a valid character
        Err(ValidationCodeParseError::NotBase32)
    );
    p_assert_matches!(
        "ABC+EF".parse::<ValidationCode>(),
        Err(ValidationCodeParseError::NotBase32)
    );
}

#[test]
fn serde_validation_code() {
    let code: ValidationCode = "AD3FXJ".parse().unwrap();
    serde_test::assert_tokens(&code, &[serde_test::Token::BorrowedStr("AD3FXJ")]);
}

#[rstest]
fn serde_account_vault_item_opaque_key() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   type: 'account_vault_item_opaque_key'
    //   key_id: ext(2, 0xac42ef607148434a94cc502fd5e61bad)
    //   encrypted_data: 0x3c656e637279707465645f646174613e
    let raw: &[u8] = hex!(
        "0028b52ffd0058c50200040583a474797065bd6163636f756e745f7661756c745f6974"
        "656d5f6f70617175655f6b6579a66b65795f6964d802ac42ef607148434a94cc502fd5"
        "e61badae656e637279707465645f64617461c4103c3e01000fc9330a"
    )
    .as_ref();

    let expected = AccountVaultItemOpaqueKey {
        key_id: AccountVaultItemOpaqueKeyID::from_hex("ac42ef607148434a94cc502fd5e61bad").unwrap(),
        encrypted_data: Bytes::from_static(b"<encrypted_data>"),
    };
    println!("***expected: {:?}", expected.dump());

    let data = match AccountVaultItem::load(raw).unwrap() {
        AccountVaultItem::OpaqueKey(data) => {
            p_assert_eq!(data, expected);
            data
        }
        AccountVaultItem::RegistrationDevice(_) => unreachable!(),
    };

    // Also test serialization round trip
    let raw2 = data.dump();
    match AccountVaultItem::load(&raw2).unwrap() {
        AccountVaultItem::OpaqueKey(data2) => {
            p_assert_eq!(data2, expected);
        }
        AccountVaultItem::RegistrationDevice(_) => unreachable!(),
    }

    // Finally test the fingerprint
    p_assert_eq!(
        expected.fingerprint(),
        HashDigest::from(hex!(
            "16b5e0feafc39019374541cafd328924d664e9be0aadad82b406cfbed48619f2"
        )),
    );
    p_assert_eq!(
        AccountVaultItem::OpaqueKey(expected).fingerprint(),
        HashDigest::from(hex!(
            "16b5e0feafc39019374541cafd328924d664e9be0aadad82b406cfbed48619f2"
        )),
    );
}

#[rstest]
fn serde_account_vault_item_registration_device() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   type: 'account_vault_item_registration_device'
    //   organization_id: 'CoolOrg'
    //   user_id: ext(2, 0xac42ef607148434a94cc502fd5e61bad)
    //   encrypted_data: 0x3c656e637279707465645f646174613e
    let raw: &[u8] = hex!(
        "0028b52ffd0058c50300d40684a474797065d9266163636f756e745f7661756c745f69"
        "74656d5f726567697374726174696f6e5f646576696365af6f7267616e697a6964a743"
        "6f6f6c4f7267a7757365725f6964d802ac42ef607148434a94cc502fd5e61badae656e"
        "637279707465645f64617461c4103c3e02004653423b6c4201"
    )
    .as_ref();

    let expected = AccountVaultItemRegistrationDevice {
        organization_id: "CoolOrg".parse().unwrap(),
        user_id: UserID::from_hex("ac42ef607148434a94cc502fd5e61bad").unwrap(),
        encrypted_data: Bytes::from_static(b"<encrypted_data>"),
    };
    println!("***expected: {:?}", expected.dump());

    let data = match AccountVaultItem::load(raw).unwrap() {
        AccountVaultItem::RegistrationDevice(data) => {
            p_assert_eq!(data, expected);
            data
        }
        AccountVaultItem::OpaqueKey(_) => unreachable!(),
    };

    // Also test serialization round trip
    let raw2 = data.dump();
    match AccountVaultItem::load(&raw2).unwrap() {
        AccountVaultItem::RegistrationDevice(data2) => {
            p_assert_eq!(data2, expected);
        }
        AccountVaultItem::OpaqueKey(_) => unreachable!(),
    };

    // Finally test the fingerprint
    p_assert_eq!(
        expected.fingerprint(),
        HashDigest::from(hex!(
            "d95abc46277602ddd46f79612de18420c8ad72cebe9137e7452c0ea59c4bef79"
        )),
    );
    p_assert_eq!(
        AccountVaultItem::RegistrationDevice(expected).fingerprint(),
        HashDigest::from(hex!(
            "d95abc46277602ddd46f79612de18420c8ad72cebe9137e7452c0ea59c4bef79"
        )),
    );
}

#[rstest]
fn serde_account_vault_key_access() {
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));
    let vault_key = SecretKey::from(hex!(
        "114413b514a2197e083c49b8b3637dbc330bdf7c0e7e8b2a9a9dc6236885485f"
    ));

    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   type: 'account_vault_key_access'
    //   vault_key: 0x114413b514a2197e083c49b8b3637dbc330bdf7c0e7e8b2a9a9dc6236885485f
    let raw: &[u8] = hex!(
        "dcf034d9e316e6d993b8f7f00083be8e0ba54fa5c05a67de6bde294d1a93310b58d7c5"
        "f41c9d1f6eb93cac8a70fc4ba1bc5629e3d1c6aef955aaaa56012906640c9a8fb0dc25"
        "bc0ed018bf6e99f6933b0f0fa0baeb76bfc9ebb03f58d75e829da97f80aadfebad07a6"
        "d80ee03671205a1a50f860497d4502ae2debc7bb"
    )
    .as_ref();

    let expected = AccountVaultKeyAccess { vault_key };
    println!("***expected: {:?}", expected.dump_and_encrypt(&key));

    let data = AccountVaultKeyAccess::decrypt_and_load(raw, &key).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump_and_encrypt(&key);
    let data2 = AccountVaultKeyAccess::decrypt_and_load(&raw2, &key).unwrap();
    p_assert_eq!(data2, expected);
}

#[rstest]
fn serde_account_vault_item_opaque_key_encrypted_data() {
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));
    let opaque_key_id =
        AccountVaultItemOpaqueKeyID::from_hex("ac42ef607148434a94cc502fd5e61bad").unwrap();
    let opaque_key = SecretKey::from(hex!(
        "114413b514a2197e083c49b8b3637dbc330bdf7c0e7e8b2a9a9dc6236885485f"
    ));

    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   type: 'account_vault_item_opaque_key_encrypted_data'
    //   key_id: ext(2, 0xac42ef607148434a94cc502fd5e61bad)
    //   key: 0x114413b514a2197e083c49b8b3637dbc330bdf7c0e7e8b2a9a9dc6236885485f
    let raw: &[u8] = hex!(
        "b30275bbbb0eb119bd1b404afb38f65984aeab8aa0c2134ea373fda42c48a027a93347"
        "7060796473ad3b100462bf2cea11dd7bb3b80ad5a30e3cc680aa72e96c9ed72ddc1bdb"
        "a660aeb060631fcd159eca47428486b15e68fc0cd7ca77d1b071f48be1bffcfcf18c69"
        "40330dfbf040028949dd6aa7a9ceed733cc6930fb7ce0b041dadd96c40f89c3d4eb7d7"
        "75fa7cee4967d17586e8a09047916f5b85aa3fb274776eab11"
    )
    .as_ref();

    let expected = AccountVaultItemOpaqueKeyEncryptedData {
        key_id: opaque_key_id,
        key: opaque_key,
    };
    println!("***expected: {:?}", expected.dump_and_encrypt(&key));

    let data = AccountVaultItemOpaqueKeyEncryptedData::decrypt_and_load(raw, &key).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump_and_encrypt(&key);
    let data2 = AccountVaultItemOpaqueKeyEncryptedData::decrypt_and_load(&raw2, &key).unwrap();
    p_assert_eq!(data2, expected);
}
