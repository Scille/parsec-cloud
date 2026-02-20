// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use crate::{
    load_device, save_device, tests::utils::MockedAccountVaultOperations,
    AccountVaultOperationsUploadOpaqueKeyError, AvailableDevice, AvailableDeviceType,
    DeviceAccessStrategy, DevicePrimaryProtectionStrategy, DeviceSaveStrategy, LoadDeviceError,
    SaveDeviceError,
};
use libparsec_client_connection::ConnectionError;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test(testbed = "minimal")]
async fn ok_simple(tmp_path: TmpPath, env: &TestbedEnv) {
    let key_file = tmp_path.join("a_devices.keys");

    let access_strategy =
        DeviceAccessStrategy::new_password(key_file.clone(), "P@ssw0rd.".to_owned().into());
    let save_strategy = access_strategy.clone().into();
    let alice_device = env.local_device("alice@dev1");
    alice_device
        .time_provider
        .mock_time_frozen("2000-01-01T00:00:00Z".parse().unwrap());
    let outcome = save_device(&tmp_path, &save_strategy, &alice_device, key_file.clone())
        .await
        .unwrap();
    p_assert_eq!(
        outcome,
        AvailableDevice {
            key_file_path: key_file,
            created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
            protected_on: "2000-01-01T00:00:00Z".parse().unwrap(),
            server_addr: alice_device.organization_addr.clone().into(),
            organization_id: alice_device.organization_id().clone(),
            user_id: alice_device.user_id,
            device_id: alice_device.device_id,
            human_handle: alice_device.human_handle.clone(),
            device_label: alice_device.device_label.clone(),
            totp_opaque_key_id: None,
            ty: AvailableDeviceType::Password,
        }
    );

    // Roundtrip check
    let loaded = load_device(&tmp_path, &access_strategy).await.unwrap();
    p_assert_eq!(*loaded, *alice_device);
}

#[cfg(not(target_arch = "wasm32"))]
enum OkKind {
    ExistingParentDir,
    MissingParentDir,
    OverwritingFile,
}

// Wasm impl do not use a filesystem, so we do not have invalidPath error except if the path
// contain invalid utf-8 char.
#[cfg(not(target_arch = "wasm32"))]
#[parsec_test(testbed = "minimal")]
#[case::existing_parent_dir(OkKind::ExistingParentDir)]
#[case::missing_parent_dir(OkKind::MissingParentDir)]
#[case::overwriting_file(OkKind::OverwritingFile)]
async fn ok(tmp_path: TmpPath, #[case] kind: OkKind, env: &TestbedEnv) {
    let key_file = match kind {
        OkKind::ExistingParentDir => {
            let path = tmp_path.join("devices/my_device.keys");
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            path
        }
        OkKind::MissingParentDir => {
            // Both parent and grand parent dirs are missing
            tmp_path.join("devices/sub/my_device.keys")
        }
        OkKind::OverwritingFile => {
            let path = tmp_path.join("devices/my_device.keys");
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, b"<dummy>").unwrap();
            path
        }
    };

    let access_strategy =
        DeviceAccessStrategy::new_password(key_file.clone(), "P@ssw0rd.".to_owned().into());
    let save_strategy = access_strategy.clone().into();
    let alice_device = env.local_device("alice@dev1");
    alice_device
        .time_provider
        .mock_time_frozen("2000-01-01T00:00:00Z".parse().unwrap());
    let outcome = save_device(&tmp_path, &save_strategy, &alice_device, key_file.clone())
        .await
        .unwrap();
    p_assert_eq!(
        outcome,
        AvailableDevice {
            key_file_path: key_file,
            created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
            protected_on: "2000-01-01T00:00:00Z".parse().unwrap(),
            server_addr: alice_device.organization_addr.clone().into(),
            organization_id: alice_device.organization_id().clone(),
            user_id: alice_device.user_id,
            device_id: alice_device.device_id,
            human_handle: alice_device.human_handle.clone(),
            device_label: alice_device.device_label.clone(),
            totp_opaque_key_id: None,
            ty: AvailableDeviceType::Password,
        }
    );

    // Roundtrip check
    let loaded = load_device(&tmp_path, &access_strategy).await.unwrap();
    p_assert_eq!(*loaded, *alice_device);
}

#[parsec_test(testbed = "minimal")]
async fn remote_error(
    #[values("remote_opaque_key_upload_offline", "remote_opaque_key_upload_failed")] kind: &str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    let alice_device = env.local_device("alice@dev1");
    let key_file = tmp_path.join("devices/keyring_file.keys");

    let account_vault_operations = Arc::new(MockedAccountVaultOperations::new(
        alice_device.human_handle.email().to_owned(),
    ));
    let save_strategy = DeviceSaveStrategy {
        totp_protection: None,
        primary_protection: DevicePrimaryProtectionStrategy::AccountVault {
            operations: account_vault_operations.clone(),
        },
    };

    match kind {
        "remote_opaque_key_upload_offline" => {
            account_vault_operations.inject_next_error_upload_opaque_key(
                AccountVaultOperationsUploadOpaqueKeyError::Offline(ConnectionError::NoResponse(
                    None,
                )),
            );

            p_assert_matches!(
                save_device(&tmp_path, &save_strategy, &alice_device, key_file.clone()).await.unwrap_err(),
                err @ SaveDeviceError::RemoteOpaqueKeyUploadOffline { .. }
                if err.to_string() == "No response from Parsec account server: Cannot communicate with the server: Failed to retrieve the response: Server unavailable"
            );
        }

        "remote_opaque_key_upload_failed" => {
            account_vault_operations.inject_next_error_upload_opaque_key(
                AccountVaultOperationsUploadOpaqueKeyError::BadVaultKeyAccess(
                    DataError::Decryption,
                ),
            );

            p_assert_matches!(
                save_device(&tmp_path, &save_strategy, &alice_device, key_file.clone()).await.unwrap_err(),
                err @ SaveDeviceError::RemoteOpaqueKeyUploadFailed { .. }
                if err.to_string() == "Parsec account server opaque key upload failed: Cannot decrypt the vault key access returned by the server: Invalid encryption"
            );
        }

        unknown => panic!("Unknown kind: {unknown}"),
    }
}

#[parsec_test(testbed = "empty")]
async fn testbed(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.bootstrap_organization("alice"); // alice@dev1
    })
    .await;

    // Note the key file must be the device nickname, otherwise the testbed won't
    // understand which device should be loaded.
    let key_file = env.discriminant_dir.join("devices/alice@dev1.keys");

    // Sanity check to ensure the key file is the one present in testbed
    let old_access =
        DeviceAccessStrategy::new_password(key_file.clone(), "P@ssw0rd.".to_owned().into());
    let device = load_device(&env.discriminant_dir, &old_access)
        .await
        .unwrap();

    // Now overwrite the key file in testbed
    let new_access_strategy = DeviceAccessStrategy {
        key_file: key_file.clone(),
        totp_protection: Some((TOTPOpaqueKeyID::default(), SecretKey::generate())),
        primary_protection: DevicePrimaryProtectionStrategy::Password {
            password: "N3wP@ssw0rd.".to_owned().into(),
        },
    };
    let new_save_strategy = new_access_strategy.clone().into();
    save_device(&env.discriminant_dir, &new_save_strategy, &device, key_file)
        .await
        .unwrap();

    // Finally roundtrip check
    p_assert_matches!(
        load_device(&env.discriminant_dir, &old_access).await,
        Err(LoadDeviceError::DecryptionFailed)
    );
    let reloaded_device = load_device(&env.discriminant_dir, &new_access_strategy)
        .await
        .unwrap();
    p_assert_eq!(reloaded_device, device);
}
