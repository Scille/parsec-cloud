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
fn serde_account_vault_item_web_local_device_key() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   type: 'account_vault_item_web_local_device_key'
    //   organization_id: 'CoolOrg'
    //   device_id: ext(2, 0xac42ef607148434a94cc502fd5e61bad)
    //   encrypted_data: 0x3c656e637279707465645f646174613e
    let raw: &[u8] = hex!(
        "0028b52ffd0058d50300f40684a474797065d9276163636f756e745f7661756c745f69"
        "74656d5f7765625f6c6f63616c5f6465766963655f6b6579af6f7267616e697a617469"
        "6f6e5f6964a7436f6f6c4f7267a96964d802ac42ef607148434a94cc502fd5e61badae"
        "656e637279707465645f64617461c4103c3e0200a6291f31ae3203"
    )
    .as_ref();

    let expected = AccountVaultItemWebLocalDeviceKey {
        organization_id: "CoolOrg".parse().unwrap(),
        device_id: DeviceID::from_hex("ac42ef607148434a94cc502fd5e61bad").unwrap(),
        encrypted_data: Bytes::from_static(b"<encrypted_data>"),
    };
    println!("***expected: {:?}", expected.dump());

    let data = match AccountVaultItem::load(raw).unwrap() {
        AccountVaultItem::WebLocalDeviceKey(data) => {
            p_assert_eq!(data, expected);
            data
        }
        AccountVaultItem::RegistrationDevice(_) => unreachable!(),
    };

    // Also test serialization round trip
    let raw2 = data.dump();
    match AccountVaultItem::load(&raw2).unwrap() {
        AccountVaultItem::WebLocalDeviceKey(data2) => {
            p_assert_eq!(data2, expected);
        }
        AccountVaultItem::RegistrationDevice(_) => unreachable!(),
    }

    // Finally test the fingerprint
    p_assert_eq!(
        expected.fingerprint(),
        HashDigest::from(hex!(
            "0d97bbe805b808a1c53b1f0a117349d014c47a26ee5a513c3956fdb420699595"
        )),
    );
    p_assert_eq!(
        AccountVaultItem::WebLocalDeviceKey(expected).fingerprint(),
        HashDigest::from(hex!(
            "0d97bbe805b808a1c53b1f0a117349d014c47a26ee5a513c3956fdb420699595"
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
        AccountVaultItem::WebLocalDeviceKey(_) => unreachable!(),
    };

    // Also test serialization round trip
    let raw2 = data.dump();
    match AccountVaultItem::load(&raw2).unwrap() {
        AccountVaultItem::RegistrationDevice(data2) => {
            p_assert_eq!(data2, expected);
        }
        AccountVaultItem::WebLocalDeviceKey(_) => unreachable!(),
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
    //   type: 'vault_key_access'
    //   vault_key: 0x114413b514a2197e083c49b8b3637dbc330bdf7c0e7e8b2a9a9dc6236885485f
    let raw: &[u8] = hex!(
        "19d3b8ec723939ad27eb82790a1e83db0d4ff8e67843b67d7ab8ef2631d630080ddc41"
        "5c4aac2959ec5b34f67f1d4f55ecb718275449b7dc1d00418931e02569e0cafe013aee"
        "29ae50d3a4d2a61d9bcafed90912500834c243dece2d8f221683a9bddc2306de70f2f5"
        "3dfd00dd126b5b1b2c182613"
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
