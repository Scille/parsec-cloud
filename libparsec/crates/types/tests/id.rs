// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::str::FromStr;

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

#[test]
fn device_label_bad_size() {
    DeviceLabel::from_str("").unwrap_err();
}

#[rstest]
#[case::text_32_long("This_one_is_exactly_32_char_long")]
#[case::alphanum("01239AbcdDEFA")]
#[case::with_underscore("hello_World")]
#[case::with_hyphen("hello-World")]
// cspell:disable-next-line
#[case::with_unicode_1("ªµºÖØøˆˠˬˮ\u{301}ͶͽͿΆΊ")]
#[case::with_unicode_2("ΌΎΣҁ\u{484}Աՙֈ\u{5c7}\u{5bf}\u{5c1}\u{5af}\u{5c4}")]
#[case::with_unicode_3("ת\u{5ef}\u{610}٩ۓە")]
#[case::with_unicode_4("\u{6ea}\u{6df}ۿܐݍ߀ߺ\u{7fd}ࠀࡀ")]
// cspell:disable-next-line
#[case::hello("𝔅𝔬𝔫𝔧𝔬𝔲𝔯")]
fn organization_id_user_id_and_device_name(#[case] raw: &str) {
    use unicode_normalization::UnicodeNormalization;

    let nfc_raw = raw.nfc().collect::<String>();
    p_assert_eq!(raw, nfc_raw, "raw != nfc_raw (expected `{:#?}`)", nfc_raw);

    let organization_id = OrganizationID::from_str(raw).unwrap();
    p_assert_eq!(organization_id.to_string(), raw);
    p_assert_eq!(organization_id.as_ref(), raw);
    p_assert_eq!(organization_id, OrganizationID::from_str(raw).unwrap());

    let user_id = UserID::from_str(raw).unwrap();
    p_assert_eq!(user_id.to_string(), raw);
    p_assert_eq!(user_id.as_ref(), raw);
    p_assert_eq!(user_id, UserID::from_str(raw).unwrap());

    let device_name = DeviceName::from_str(raw).unwrap();
    p_assert_eq!(device_name.to_string(), raw);
    p_assert_eq!(device_name.as_ref(), raw);
    p_assert_eq!(device_name, DeviceName::from_str(raw).unwrap());
}

#[rstest]
#[case::text_33_long("This_text_is_exactly_33_char_long")]
#[case::empty("")]
#[case::invalid_tilde("F~o")]
#[case::invalid_space("f o")]
fn bad_organization_id_user_id_and_device_name(#[case] raw: &str) {
    OrganizationID::from_str(raw).unwrap_err();
    UserID::from_str(raw).unwrap_err();
    DeviceName::from_str(raw).unwrap_err();
}

#[rstest]
#[case("ali-c_e@d-e_v")]
#[case("ALICE@DEV")]
#[case("a@x")]
#[case(&("a".repeat(32) + "@" + &"b".repeat(32)))]
#[case("关羽@三国")]
fn device_id(#[case] raw: &str) {
    let (user_id, device_name) = raw.split_once('@').unwrap();
    let device_id = DeviceID::from_str(raw).unwrap();

    p_assert_eq!(device_id, DeviceID::from_str(raw).unwrap());
    p_assert_eq!(device_id.user_id(), &UserID::from_str(user_id).unwrap());
    p_assert_eq!(
        device_id.device_name(),
        &DeviceName::from_str(device_name).unwrap()
    );
}

#[rstest]
#[case("a")]
#[case(&("a".repeat(33) + "@" + &"x".repeat(32)))]
#[case(&("a".repeat(32) + "@" + &"x".repeat(33)))]
#[case("a@@x")]
#[case("a@1@x")]
fn bad_device_id(#[case] raw: &str) {
    DeviceID::from_str(raw).unwrap_err();
}
