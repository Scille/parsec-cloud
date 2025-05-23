// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use super::*;

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
fn organization_id_ok(#[case] raw: &str) {
    use unicode_normalization::UnicodeNormalization;

    let nfc_raw = raw.nfc().collect::<String>();
    p_assert_eq!(raw, nfc_raw, "raw != nfc_raw (expected `{:#?}`)", nfc_raw);

    let organization_id = OrganizationID::from_str(raw).unwrap();
    p_assert_eq!(organization_id.to_string(), raw);
    p_assert_eq!(organization_id.as_ref(), raw);
    p_assert_eq!(organization_id, OrganizationID::from_str(raw).unwrap());
}

#[rstest]
#[case::text_33_long("This_text_is_exactly_33_char_long")]
#[case::empty("")]
#[case::invalid_tilde("F~o")]
#[case::invalid_space("f o")]
fn organization_id_ko(#[case] raw: &str) {
    p_assert_matches!(OrganizationID::from_str(raw), Err(InvalidOrganizationID));
}

#[rstest]
#[case::valid("john.doe@example.com", "John Doe")]
#[case::parenthesis_allowed("a@b.c", "()")]
#[case::max_size_email_and_label(&format!("{}@x.y", "a".repeat(250)), &"x".repeat(254))]
fn human_handle_ok(#[case] email: &str, #[case] label: &str) {
    let human_handle = HumanHandle::from_raw(email, label).unwrap();
    p_assert_eq!(human_handle.label(), label);
    p_assert_eq!(human_handle.email().to_string(), email);
    p_assert_eq!(human_handle.as_ref(), format!("{label} <{email}>"));

    let human_handle2 = HumanHandle::from_str(&format!("{label} <{email}>")).unwrap();
    p_assert_eq!(human_handle, human_handle2);
}

#[rstest]
#[case::invalid_email("test", "test", HumanHandleParseError::InvalidEmail)]
#[case::invalid_email("a@b..c", "test", HumanHandleParseError::InvalidEmail)]
#[case::invalid_email("@b.c", "test", HumanHandleParseError::InvalidEmail)]
#[case::invalid_name_with_backslash("a@b", "hell\\o", HumanHandleParseError::InvalidLabel)]
#[case::switched(
    "John Doe",
    "john.doe@example.com",
    HumanHandleParseError::InvalidEmail
)]
#[case::empty_email("", "foo", HumanHandleParseError::InvalidEmail)]
#[case::too_long_email(&"x".repeat(255), "foo", HumanHandleParseError::InvalidEmail)]
#[case::empty_label("foo@example.com", "", HumanHandleParseError::InvalidLabel)]
#[case::too_long_label("foo@example.com", &"x".repeat(255), HumanHandleParseError::InvalidLabel)]
#[case::redacted_reserved_domain(
    "john.doe@redacted.invalid",
    "John Doe",
    HumanHandleParseError::InvalidEmail
)]
fn human_handle_ko(
    #[case] email: &str,
    #[case] label: &str,
    #[case] expected_error: HumanHandleParseError,
) {
    let assert_match_against_expected_error = |error| match expected_error {
        HumanHandleParseError::MissingEmail => {
            p_assert_matches!(error, Err(HumanHandleParseError::MissingEmail))
        }
        HumanHandleParseError::InvalidEmail => {
            p_assert_matches!(error, Err(HumanHandleParseError::InvalidEmail))
        }
        HumanHandleParseError::InvalidLabel => {
            p_assert_matches!(error, Err(HumanHandleParseError::InvalidLabel))
        }
    };

    assert_match_against_expected_error(HumanHandle::from_raw(email, label));
    assert_match_against_expected_error(HumanHandle::from_str(&format!("{label} <{email}>")));
}

#[test]
fn human_handle_from_string_with_missing_email() {
    p_assert_matches!(
        HumanHandle::from_str("John Doe"),
        Err(HumanHandleParseError::MissingEmail)
    );
}

#[rstest]
#[case::simple("PC1")]
#[case::minimal("1")]
#[case::text_255_long(&"x".repeat(255))]
#[case::with_unicode("三国")]
fn device_label_ok(#[case] raw: &str) {
    let label = DeviceLabel::from_str(raw).unwrap();
    p_assert_eq!(label.as_ref(), raw);
}

#[rstest]
#[case::empty("")]
#[case::text_256_long(&"x".repeat(256))]
#[case::text_256_long_with_wide_codepoint(&("x".repeat(254) + "国"))]
fn device_label_ko(#[case] raw: &str) {
    p_assert_matches!(DeviceLabel::from_str(raw), Err(InvalidDeviceLabel));
}
