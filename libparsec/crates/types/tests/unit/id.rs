// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use super::*;

#[test]
fn from_str() {
    let too_long = "a".repeat(33);

    assert!(too_long.parse::<DeviceName>().is_err());
    assert!("pc1".parse::<DeviceName>().is_ok());

    assert!(too_long.parse::<UserID>().is_err());
    assert!("alice".parse::<UserID>().is_ok());

    assert!("dummy".parse::<DeviceID>().is_err());
    assert!(format!("alice@{}", too_long).parse::<DeviceID>().is_err());
    assert!("alice@pc1".parse::<DeviceID>().is_ok());
}

#[rstest]
#[case::valid("john.doe@example.com", "John Doe", true)]
#[case::invalid_email("test", "test", false)]
#[case::invalid_email("a@b..c", "test", false)]
#[case::invalid_email("@b.c", "test", false)]
#[case::parenthesis_allowed("a@b.c", "()", true)]
#[case::invalid_name_with_backslash("a@b", "hell\\o", false)]
#[case::switched("John Doe", "john.doe@example.com", false)]
#[case::empty_email("", "foo", false)]
#[case::max_size_email_and_label(&format!("{}@x.y", "a".repeat(250)), &"x".repeat(254), true)]
#[case::too_long_email(&"x".repeat(255), "foo", false)]
#[case::empty_label("foo@example.com", "", false)]
#[case::too_long_label("foo@example.com", &"x".repeat(255), false)]
#[case::redacted_reserved_domain("john.doe@redacted.invalid", "John Doe", false)]
fn human_handle(#[case] email: &str, #[case] label: &str, #[case] is_ok: bool) {
    p_assert_eq!(HumanHandle::new(email, label).is_ok(), is_ok);
    p_assert_eq!(
        HumanHandle::from_str(&format!("{label} <{email}>")).is_ok(),
        is_ok
    );
    if is_ok {
        let human_handle = HumanHandle::new(email, label).unwrap();
        p_assert_eq!(human_handle.label(), label);
        p_assert_eq!(human_handle.email(), email);
        p_assert_eq!(human_handle.as_ref(), format!("{label} <{email}>"));
    }
}

#[test]
fn device_id_to_user_id_and_device_name() {
    let device_id = "john@pc1".parse::<DeviceID>().unwrap();

    let (user_id, device_name) = device_id.into();

    p_assert_eq!(user_id, "john".parse::<UserID>().unwrap());

    p_assert_eq!(device_name, "pc1".parse::<DeviceName>().unwrap());
}

#[test]
fn display() {
    let user_id = "john".parse::<UserID>().unwrap();
    p_assert_eq!(format!("{}", user_id), "john");

    let device_name = "pc1".parse::<DeviceName>().unwrap();
    p_assert_eq!(format!("{}", device_name), "pc1");

    let device_id = "john@pc1".parse::<DeviceID>().unwrap();
    p_assert_eq!(format!("{}", device_id), "john@pc1");
}

#[rstest]
#[case::empty("", "")]
#[case::single("a", "a")]
#[case::multiple("aaa", "aaa")]
#[case::underscore("_", "__")]
#[case::single_uppercase("A", "_a")]
#[case::multiple_uppercase("AAA", "_a_a_a")]
#[case::uppercase_and_underscore("_A_", "___a__")]
#[case::unicode("Δ", "_δ")]
fn uncaseify_test(#[case] input: &str, #[case] expected: &str) {
    p_assert_eq!(uncaseify(input), expected)
}

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
