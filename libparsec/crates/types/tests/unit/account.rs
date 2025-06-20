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
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   type: 'account_vault_item_web_local_device_key'
    //   organization_id: 'CoolOrg'
    //   device_id: ext(2, 0xac42ef607148434a94cc502fd5e61bad)
    //   encrypted_data: 0x3c656e637279707465645f646174613e
    let raw: &[u8] = hex!(
        "07aa7e85fab172da57ff6fec2addc3c505f3d0ff7920f27ae37c3d1d99417c0bac885e"
        "0940b991fdc587da7198865ef95f85268fd8de8432c987042702e5844cbb920003e0c7"
        "862867e92cde0da3fad206bedee4478c264c42db0832bef2b05c0097c094727040da93"
        "f0c00630801a86741c0cd1768691190304a9ed0437e25fa4de59ca38aaca3beb49e2f0"
        "97cfaa7059d5eb900561321cb8f2f77b43199d4bdbfdfc5f2b0050793198e4a4"
    )
    .as_ref();

    let expected = AccountVaultItemWebLocalDeviceKey {
        organization_id: "CoolOrg".parse().unwrap(),
        device_id: DeviceID::from_hex("ac42ef607148434a94cc502fd5e61bad").unwrap(),
        encrypted_data: Bytes::from_static(b"<encrypted_data>"),
    };
    println!("***expected: {:?}", expected.dump_and_encrypt(&key));

    let data = AccountVaultItemWebLocalDeviceKey::decrypt_and_load(raw, &key).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump_and_encrypt(&key);
    let data2 = AccountVaultItemWebLocalDeviceKey::decrypt_and_load(&raw2, &key).unwrap();
    p_assert_eq!(data2, expected);
}

#[rstest]
fn serde_account_vault_item_auto_enrollment_device() {
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   type: 'account_vault_item_auto_enrollment_device'
    //   organization_id: 'CoolOrg'
    //   user_id: ext(2, 0xac42ef607148434a94cc502fd5e61bad)
    //   encrypted_data: 0x3c656e637279707465645f646174613e
    let raw: &[u8] = hex!(
        "116aa8ab4985b55bb699e9fe2f644e7679a1f2be36448b400146b31355028c326900f8"
        "b1ecc83b2c5d13dad3d4afe2dd0529d9050dec75c9c3da9d2f676e4034b4714d3f719b"
        "644c75f9ccb32e2d573bedc163bbbcdc60ae0d1045ee21f77f01823bc8c41783229828"
        "badab1df9538849df3b8489bfbbe2d95ae80a7536015b4ec7be6799517201a3fe578cf"
        "7c2874b2896aa290dfd1918c83f4a5dca4bdaa7c7bbb58b4962aa223688a25fe"
    )
    .as_ref();

    let expected = AccountVaultItemRegistrationDevice {
        organization_id: "CoolOrg".parse().unwrap(),
        user_id: UserID::from_hex("ac42ef607148434a94cc502fd5e61bad").unwrap(),
        encrypted_data: Bytes::from_static(b"<encrypted_data>"),
    };
    println!("***expected: {:?}", expected.dump_and_encrypt(&key));

    let data = AccountVaultItemRegistrationDevice::decrypt_and_load(raw, &key).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump_and_encrypt(&key);
    let data2 = AccountVaultItemRegistrationDevice::decrypt_and_load(&raw2, &key).unwrap();
    p_assert_eq!(data2, expected);
}
