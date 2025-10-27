// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{list_available_devices, save_device};
use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
#[case("password")]
#[cfg_attr(not(target_arch = "wasm32"), case("keyring"))]
#[case("account_vault")]
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
            let ciphertext_key_id =
                AccountVaultItemOpaqueKeyID::from_hex("4ce154500ce340bcaa4d44dcb9b841a1").unwrap();
            let save_strategy = DeviceSaveStrategy::AccountVault {
                ciphertext_key_id,
                ciphertext_key: hex!(
                    "2f64db9fae22c574ade28c44cd206c00f8119e9b49dacc131dad31c043ebe766"
                )
                .into(),
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
