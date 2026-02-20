// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use super::utils::{MockedAccountVaultOperations, MockedOpenBaoOperations};
use crate::{
    list_available_devices, save_device, AvailableDevice, DevicePrimaryProtectionStrategy,
    DeviceSaveStrategy,
};
use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
#[case("password")]
#[cfg_attr(not(target_arch = "wasm32"), case("keyring"))]
#[case("account_vault")]
#[case("openbao")]
// TODO #11269
// #[cfg_attr(target_os = "windows", case("pki"))]
async fn save_list(#[case] kind: &str, #[values(false, true)] with_totp: bool, tmp_path: TmpPath) {
    use crate::tests::utils::key_present_in_system;

    let devices_dir = crate::get_devices_dir(&tmp_path);
    let key_file = devices_dir.join("keyring_file.keys");
    let url = ParsecOrganizationAddr::from_any(
        // cspell:disable-next-line
        "parsec3://test.invalid/Org?p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA",
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

    let (mut save_strategy, mut expected_available_device) = match kind {
        "keyring" => {
            let save_strategy = DeviceSaveStrategy {
                totp_protection: None,
                primary_protection: DevicePrimaryProtectionStrategy::Keyring,
            };
            let expected_available_device = AvailableDevice {
                key_file_path: key_file.clone(),
                created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                protected_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                server_addr: "parsec3://test.invalid".parse().unwrap(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.clone(),
                device_label: device.device_label.clone(),
                totp_opaque_key_id: None,
                ty: save_strategy.primary_protection.ty(),
            };
            (save_strategy, expected_available_device)
        }

        "password" => {
            let save_strategy = DeviceSaveStrategy {
                totp_protection: None,
                primary_protection: DevicePrimaryProtectionStrategy::Password {
                    password: "P@ssw0rd.".to_string().into(),
                },
            };
            let expected_available_device = AvailableDevice {
                key_file_path: key_file.clone(),
                created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                protected_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                server_addr: "parsec3://test.invalid".parse().unwrap(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.clone(),
                device_label: device.device_label.clone(),
                totp_opaque_key_id: None,
                ty: save_strategy.primary_protection.ty(),
            };
            (save_strategy, expected_available_device)
        }

        "account_vault" => {
            let save_strategy = DeviceSaveStrategy {
                totp_protection: None,
                primary_protection: DevicePrimaryProtectionStrategy::AccountVault {
                    operations: Arc::new(MockedAccountVaultOperations::new(
                        device.human_handle.email().to_owned(),
                    )),
                },
            };
            let expected_available_device = AvailableDevice {
                key_file_path: key_file.clone(),
                created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                protected_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                server_addr: "parsec3://test.invalid".parse().unwrap(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.clone(),
                device_label: device.device_label.clone(),
                totp_opaque_key_id: None,
                ty: save_strategy.primary_protection.ty(),
            };
            (save_strategy, expected_available_device)
        }

        "openbao" => {
            let save_strategy = DeviceSaveStrategy {
                totp_protection: None,
                primary_protection: DevicePrimaryProtectionStrategy::OpenBao {
                    operations: Arc::new(MockedOpenBaoOperations::new(
                        device.human_handle.email().to_owned(),
                    )),
                },
            };
            let expected_available_device = AvailableDevice {
                key_file_path: key_file.clone(),
                created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                protected_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                server_addr: "parsec3://test.invalid".parse().unwrap(),
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                human_handle: device.human_handle.clone(),
                device_label: device.device_label.clone(),
                totp_opaque_key_id: None,
                ty: save_strategy.primary_protection.ty(),
            };
            (save_strategy, expected_available_device)
        }

        "pki" => todo!(),

        unknown => panic!("Unknown kind: {unknown:?}"),
    };

    if with_totp {
        let totp_opaque_key_id =
            TOTPOpaqueKeyID::from_hex("8fdb73524fdd495194e877a5fafbe0a1").unwrap();
        let totp_opaque_key = SecretKey::generate();

        save_strategy.totp_protection = Some((totp_opaque_key_id, totp_opaque_key));
        expected_available_device.totp_opaque_key_id = Some(totp_opaque_key_id);
    }

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
