// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use super::utils::{MockedAccountVaultOperations, MockedOpenBaoOperations};
use crate::{list_available_devices, save_device, AvailableDevice, DeviceSaveStrategy};
use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
#[case("password")]
#[cfg_attr(not(target_arch = "wasm32"), case("keyring"))]
#[case("account_vault")]
#[case("openbao")]
// TODO #11269
// #[cfg_attr(target_os = "windows", case("smartcard"))]
async fn save_list(#[case] kind: &str, tmp_path: TmpPath) {
    use crate::tests::utils::key_present_in_system;

    let devices_dir = crate::get_devices_dir(&tmp_path);
    let key_file = devices_dir.join("keyring_file.keys");
    let url = ParsecOrganizationAddr::from_any(
        // cspell:disable-next-line
        "parsec3://test.invalid/Org?no_ssl=true&p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA",
    )
    .unwrap();
    let device = LocalDevice::generate_new_device(
        url,
        UserProfile::Admin,
        HumanHandle::from_raw("alice@dev1", "alice").unwrap(),
        "alice label".parse().unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    let (save_strategy, expected_available_device) = match kind {
        "keyring" => {
            let save_strategy = DeviceSaveStrategy::Keyring;
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
                ty: save_strategy.ty(),
            };
            (save_strategy, expected_available_device)
        }

        "password" => {
            let save_strategy = DeviceSaveStrategy::Password {
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
                ty: save_strategy.ty(),
            };
            (save_strategy, expected_available_device)
        }

        "account_vault" => {
            let save_strategy = DeviceSaveStrategy::AccountVault {
                operations: Arc::new(MockedAccountVaultOperations::new(
                    device.human_handle.email().to_owned(),
                )),
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
                ty: save_strategy.ty(),
            };
            (save_strategy, expected_available_device)
        }

        "openbao" => {
            let save_strategy = DeviceSaveStrategy::OpenBao {
                operations: Arc::new(MockedOpenBaoOperations::new(
                    device.human_handle.email().to_owned(),
                )),
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
                ty: save_strategy.ty(),
            };
            (save_strategy, expected_available_device)
        }

        "smartcard" => todo!(),

        unknown => panic!("Unknown kind: {unknown:?}"),
    };

    assert!(!key_present_in_system(&key_file).await);

    device
        .time_provider
        .mock_time_frozen("2000-01-01T00:00:00Z".parse().unwrap());
    let available_device = save_device(&tmp_path, &save_strategy, &device, key_file.clone())
        .await
        .unwrap();
    device.time_provider.unmock_time();

    p_assert_eq!(available_device, expected_available_device);
    assert!(key_present_in_system(&key_file).await);

    let devices = list_available_devices(&tmp_path).await.unwrap();

    p_assert_eq!(devices, [expected_available_device]);
}
