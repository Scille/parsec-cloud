// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec_tests_fixtures::prelude::*;
use libparsec_tests_lite::p_assert_matches;
use libparsec_types::prelude::*;

use crate::{AvailableDevice, AvailableDeviceType, DeviceAccessStrategy, DeviceSaveStrategy};

#[test]
fn available_device() {
    let org: OrganizationID = "CoolOrg".parse().unwrap();

    let available = AvailableDevice {
        key_file_path: "/foo/bar".into(),
        created_on: "2010-01-01T00:00:00Z".parse().unwrap(),
        protected_on: "2010-01-10T00:00:00Z".parse().unwrap(),
        server_addr: "parsec3://parsec.invalid".parse().unwrap(),
        organization_id: org.clone(),
        user_id: "alice".parse().unwrap(),
        device_id: "alice@dev1".parse().unwrap(),
        human_handle: HumanHandle::from_raw("john@example.com", "John Doe").unwrap(),
        device_label: "MyPc".parse().unwrap(),
        ty: AvailableDeviceType::Password,
    };
    p_assert_eq!(
        available.device_id.hex(),
        "de10a11cec0010000000000000000000"
    );
}

#[test]
fn conversions() {
    let password: Password = "P@ssw0rd.".to_string().into();
    let key_file = PathBuf::from("/foo/bar");
    let save_strategy = DeviceSaveStrategy::Password {
        password: password.clone(),
    };
    let access_strategy = save_strategy.clone().into_access(key_file.clone());

    p_assert_matches!(save_strategy.ty(), AvailableDeviceType::Password);

    p_assert_matches!(
        &access_strategy,
        DeviceAccessStrategy::Password { key_file: c_kf, password: c_p }
        if *c_kf == key_file && *c_p == password
    );

    p_assert_eq!(access_strategy.key_file(), key_file);
    p_assert_matches!(
        access_strategy.clone().into_save_strategy(AvailableDeviceType::Password),
        Some(DeviceSaveStrategy::Password { password: c_p })
        if c_p == password
    );

    p_assert_matches!(
        access_strategy.into_save_strategy(AvailableDeviceType::Keyring),
        None,
    );
}
