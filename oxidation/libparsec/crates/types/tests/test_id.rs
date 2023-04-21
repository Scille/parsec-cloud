// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use rstest::rstest;
use std::str::FromStr;

use libparsec_types::prelude::*;

#[rstest]
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

#[rstest]
#[case(&"x".repeat(33))]
#[case("F~o")]
#[case("f o")]
fn test_bad_organization_id_user_id_and_device_name(#[case] raw: &str) {
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

#[rstest]
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
