// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::*;

use libparsec_tests_lite::prelude::*;

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
