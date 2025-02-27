// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: implement web
#![cfg(not(target_arch = "wasm32"))]

use std::path::Path;

use crate::{load_device, save_device};
use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
async fn save_load(#[values("keyring", "password")] kind: &str, tmp_path: TmpPath) {
    let key_file = tmp_path.join("keyring_file");
    let url = ParsecOrganizationAddr::from_any(
        // cspell:disable-next-line
        "parsec3://test.invalid/Org?no_ssl=true&p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA",
    )
    .unwrap();
    let device = LocalDevice::generate_new_device(
        url,
        UserProfile::Admin,
        HumanHandle::new("alice@dev1", "alice").unwrap(),
        "alice label".parse().unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    let (access, expected_available_device) = match kind {
        "keyring" => {
            let access = DeviceAccessStrategy::Keyring {
                key_file: key_file.clone(),
            };
            let expected_available_device = AvailableDevice {
                key_file_path: key_file.clone(),
                created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                protected_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                server_url: "http://test.invalid/".to_string(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.clone(),
                device_label: device.device_label.clone(),
                ty: DeviceFileType::Keyring,
            };
            (access, expected_available_device)
        }
        "password" => {
            let access = DeviceAccessStrategy::Password {
                key_file: key_file.clone(),
                password: "P@ssw0rd.".to_string().into(),
            };
            let expected_available_device = AvailableDevice {
                key_file_path: key_file.clone(),
                created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                protected_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                server_url: "http://test.invalid/".to_string(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.clone(),
                device_label: device.device_label.clone(),
                ty: DeviceFileType::Password,
            };
            (access, expected_available_device)
        }
        unknown => panic!("Unknown kind: {}", unknown),
    };

    assert!(!key_file.exists());

    device
        .time_provider
        .mock_time_frozen("2000-01-01T00:00:00Z".parse().unwrap());
    let available_device = save_device(&tmp_path, &access, &device).await.unwrap();
    device.time_provider.unmock_time();

    p_assert_eq!(available_device, expected_available_device);
    assert!(key_file.exists());

    let res = load_device(Path::new(""), &access).await.unwrap();

    p_assert_eq!(*res, device);
}
