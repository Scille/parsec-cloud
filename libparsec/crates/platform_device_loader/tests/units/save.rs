// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use std::sync::Arc;

use crate::{
    load_device, save_device, tests::utils::MockedAccountVaultOperations,
    AccountVaultOperationsUploadOpaqueKeyError, AvailableDevice, AvailableDeviceType,
    DeviceAccessStrategy, DeviceSaveStrategy, LoadDeviceError, SaveDeviceError,
};
use libparsec_client_connection::ConnectionError;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
enum BadPathKind {
    PathIsDir,
    TmpPathIsDir,
    PathIsRoot,
    PathHasNoName,
}

// Wasm impl do not use a filesystem, so we do not have invalidPath error except if the path
// contain invalid utf-8 char.
#[cfg(not(target_arch = "wasm32"))]
#[parsec_test(testbed = "minimal")]
#[case::path_is_dir(BadPathKind::PathIsDir)]
#[case::path_is_dir(BadPathKind::TmpPathIsDir)]
#[case::path_is_root(BadPathKind::PathIsRoot)]
#[case::path_has_no_name(BadPathKind::PathHasNoName)]
async fn bad_path(tmp_path: TmpPath, #[case] kind: BadPathKind, env: &TestbedEnv) {
    let key_file = match kind {
        BadPathKind::PathIsDir => {
            let path = tmp_path.join("devices/my_device.keys");
            std::fs::create_dir_all(&path).unwrap();
            path
        }
        BadPathKind::TmpPathIsDir => {
            let path = tmp_path.join("devices/my_device.keys");
            let tmp_path = tmp_path.join("devices/my_device.keys.tmp");
            std::fs::create_dir_all(tmp_path).unwrap();
            path
        }
        BadPathKind::PathIsRoot => PathBuf::from("/"),
        BadPathKind::PathHasNoName => tmp_path.join("devices/my_device.keys/.."),
    };

    let device = env.local_device("alice@dev1");
    let save_strategy = DeviceSaveStrategy::Password {
        password: "P@ssw0rd.".to_owned().into(),
    };
    let outcome = save_device(&tmp_path, &save_strategy, &device, key_file).await;
    p_assert_matches!(outcome, Err(SaveDeviceError::InvalidPath(_)));
}

#[parsec_test(testbed = "minimal")]
async fn ok_simple(tmp_path: TmpPath, env: &TestbedEnv) {
    let key_file = tmp_path.join("a_devices.keys");

    let save_strategy = DeviceSaveStrategy::Password {
        password: "P@ssw0rd.".to_owned().into(),
    };
    let access_strategy = save_strategy.clone().into_access(key_file.clone());
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
            server_url: format!("https://{}/", alice_device.organization_addr.hostname()),
            organization_id: alice_device.organization_id().clone(),
            user_id: alice_device.user_id,
            device_id: alice_device.device_id,
            human_handle: alice_device.human_handle.clone(),
            device_label: alice_device.device_label.clone(),
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

    let save_strategy = DeviceSaveStrategy::Password {
        password: "P@ssw0rd.".to_owned().into(),
    };
    let access_strategy = save_strategy.clone().into_access(key_file.clone());
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
            server_url: format!("https://{}/", alice_device.organization_addr.hostname()),
            organization_id: alice_device.organization_id().clone(),
            user_id: alice_device.user_id,
            device_id: alice_device.device_id,
            human_handle: alice_device.human_handle.clone(),
            device_label: alice_device.device_label.clone(),
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
    let save_strategy = DeviceSaveStrategy::AccountVault {
        operations: account_vault_operations.clone(),
    };

    match kind {
        "remote_opaque_key_upload_offline" => {
            account_vault_operations.inject_next_error_upload_opaque_key(
                AccountVaultOperationsUploadOpaqueKeyError::Offline(ConnectionError::NoResponse(
                    None,
                )),
            );

            p_assert_matches!(
                save_device(&tmp_path, &save_strategy, &alice_device, key_file.clone()).await,
                Err(SaveDeviceError::RemoteOpaqueKeyUploadOffline(
                    ConnectionError::NoResponse(None)
                ))
            );
        }

        "remote_opaque_key_upload_failed" => {
            account_vault_operations.inject_next_error_upload_opaque_key(
                AccountVaultOperationsUploadOpaqueKeyError::BadVaultKeyAccess(
                    DataError::Decryption,
                ),
            );

            p_assert_matches!(
                save_device(&tmp_path, &save_strategy, &alice_device, key_file.clone()).await,
                Err(SaveDeviceError::RemoteOpaqueKeyUploadFailed(err))
                if err.to_string() == "Cannot decrypt the vault key access return by the server: Invalid encryption"
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
    let old_access = DeviceAccessStrategy::Password {
        key_file: key_file.clone(),
        password: "P@ssw0rd.".to_owned().into(),
    };
    let device = load_device(&env.discriminant_dir, &old_access)
        .await
        .unwrap();

    // Now overwrite the key file in testbed
    let new_save_strategy = DeviceSaveStrategy::Password {
        password: "N3wP@ssw0rd.".to_owned().into(),
    };
    let new_access_strategy = new_save_strategy.clone().into_access(key_file.clone());
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
