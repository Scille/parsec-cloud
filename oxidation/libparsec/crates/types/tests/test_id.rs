// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::str::FromStr;

use libparsec_tests_types::rstest;
use libparsec_types::{DeviceID, DeviceLabel, DeviceName, HumanHandle, OrganizationID, UserID};

#[rstest::rstest]
#[case("foo42")]
#[case("FOO")]
#[case("f")]
#[case("f-o-o")]
#[case("f_o_o")]
#[case(&"x".repeat(32))]
#[case("三国")]
fn test_organization_id_user_id_and_device_name(#[case] raw: &str) {
    let organization_id = OrganizationID::from_str(raw).unwrap();
    assert_eq!(organization_id.to_string(), raw);
    assert_eq!(organization_id, OrganizationID::from_str(raw).unwrap());

    let user_id = UserID::from_str(raw).unwrap();
    assert_eq!(user_id.to_string(), raw);
    assert_eq!(user_id, UserID::from_str(raw).unwrap());

    let device_name = DeviceName::from_str(raw).unwrap();
    assert_eq!(device_name.to_string(), raw);
    assert_eq!(device_name, DeviceName::from_str(raw).unwrap());
}

#[rstest::rstest]
#[case(&"x".repeat(33))]
#[case("F~o")]
#[case("f o")]
fn test_bad_organization_id_user_id_and_device_name(#[case] raw: &str) {
    OrganizationID::from_str(raw).unwrap_err();
    UserID::from_str(raw).unwrap_err();
    DeviceName::from_str(raw).unwrap_err();
}

#[rstest::rstest]
#[case("ali-c_e@d-e_v")]
#[case("ALICE@DEV")]
#[case("a@x")]
#[case(&("a".repeat(32) + "@" + &"b".repeat(32)))]
#[case("关羽@三国")]
fn test_device_id(#[case] raw: &str) {
    let (user_id, device_name) = raw.split_once('@').unwrap();
    let device_id = DeviceID::from_str(raw).unwrap();

    assert_eq!(device_id, DeviceID::from_str(raw).unwrap());
    assert_eq!(device_id.user_id(), &UserID::from_str(user_id).unwrap());
    assert_eq!(
        device_id.device_name(),
        &DeviceName::from_str(device_name).unwrap()
    );
}

#[rstest::rstest]
#[case("a")]
#[case(&("a".repeat(33) + "@" + &"x".repeat(32)))]
#[case(&("a".repeat(32) + "@" + &"x".repeat(33)))]
#[case("a@@x")]
#[case("a@1@x")]
fn test_bad_device_id(#[case] raw: &str) {
    DeviceID::from_str(raw).unwrap_err();
}

#[test]
fn test_device_label_bad_size() {
    DeviceLabel::from_str("").unwrap_err();
}

#[test]
fn test_from_str() {
    let too_long = "a".repeat(33);

    assert!(too_long.parse::<DeviceName>().is_err());
    assert!("pc1".parse::<DeviceName>().is_ok());

    assert!(too_long.parse::<UserID>().is_err());
    assert!("alice".parse::<UserID>().is_ok());

    assert!("dummy".parse::<DeviceID>().is_err());
    assert!(format!("alice@{}", too_long).parse::<DeviceID>().is_err());
    assert!("alice@pc1".parse::<DeviceID>().is_ok());
}

#[rstest::rstest]
#[case::valid("john.doe@example.com", "John Doe", true)]
#[case::invalid_email("test", "test", false)]
#[case::invalid_email("a@b..c", "test", false)]
#[case::invalid_email("@b.c", "test", false)]
#[case::parenthesis_allowed("a@b.c", "()", true)]
#[case::invalid_name_with_backslash("a@b", "hell\\o", false)]
#[case::switched("John Doe", "john.doe@example.com", false)]
#[case::empty_email("", "foo", false)]
#[case::too_long_email(&"x".repeat(256), "foo", false)]
#[case::empty_label("foo@example.com", "", false)]
#[case::too_long_label("foo@example.com", &"x".repeat(256), false)]
fn test_human_handle(#[case] email: &str, #[case] label: &str, #[case] is_ok: bool) {
    assert_eq!(HumanHandle::new(email, label).is_ok(), is_ok);
    assert_eq!(
        HumanHandle::from_str(&format!("{label} <{email}>")).is_ok(),
        is_ok
    );
}
