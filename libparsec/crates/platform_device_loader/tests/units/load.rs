// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use super::utils::MockedAccountVaultOperations;
use crate::{
    load_device, save_device, AccountVaultOperationsFetchOpaqueKeyError, DeviceAccessStrategy,
    DeviceSaveStrategy, LoadDeviceError,
};

use libparsec_client_connection::ConnectionError;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

// TODO: Additional tests to write !
// - load ok
// - load ok (relative path in access, hence config_dir is used)
// - bad password

#[cfg(not(target_arch = "wasm32"))]
enum BadPathKind {
    UnknownPath,
    ExistingParent,
    NotAFile,
}

// Wasm impl do not use a filesystem, so we do not have invalidPath error except if the path
// contain invalid utf-8 char.
#[cfg(not(target_arch = "wasm32"))]
#[parsec_test]
#[case::unknown_path(BadPathKind::UnknownPath)]
#[case::existing_parent(BadPathKind::ExistingParent)]
#[case::not_a_file(BadPathKind::NotAFile)]
async fn bad_path(tmp_path: TmpPath, #[case] kind: BadPathKind) {
    let key_file = tmp_path.join("devices/my_device.keys");
    match kind {
        BadPathKind::UnknownPath => (),
        BadPathKind::ExistingParent => {
            std::fs::create_dir_all(key_file.parent().unwrap()).unwrap();
        }
        BadPathKind::NotAFile => {
            std::fs::create_dir_all(&key_file).unwrap();
        }
    };

    let access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };
    let outcome = load_device(&tmp_path, &access).await;
    p_assert_matches!(outcome, Err(LoadDeviceError::InvalidPath(_)));
}

#[parsec_test]
async fn bad_file_content(tmp_path: TmpPath) {
    let key_file = tmp_path.join("devices/my_device.keys");
    crate::tests::utils::create_device_file(&key_file, b"dummy").await;

    let access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };
    let outcome = load_device(&tmp_path, &access).await;
    p_assert_matches!(outcome, Err(LoadDeviceError::InvalidData));
}

#[parsec_test]
async fn invalid_salt_size(tmp_path: TmpPath) {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   type: "password"
    //   salt: b"salt"  <== Salt is too small
    //   ciphertext: b"ciphertext"  <== Dummy data never considered given salt is invalid
    //   human_handle: ["alice@example.com", "Alicey McAliceFace"]
    //   device_label: "My dev1 machine"
    //   device_id: "alice@dev1"
    //   organization_id: "CoolOrg"
    //   slug: "f78292422e#CoolOrg#alice@dev1"
    let content = hex!(
        "88a474797065a870617373776f7264aa63697068657274657874c40a63697068657274"
        "657874ac68756d616e5f68616e646c6592b1616c696365406578616d706c652e636f6d"
        "b2416c69636579204d63416c69636546616365ac6465766963655f6c6162656caf4d79"
        "2064657631206d616368696e65a96465766963655f6964aa616c6963654064657631af"
        "6f7267616e697a6174696f6e5f6964a7436f6f6c4f7267a4736c7567bd663738323932"
        "3432326523436f6f6c4f726723616c6963654064657631a473616c74c40473616c74"
    )
    .as_ref();

    // Store it in a path compatible with the legacy format
    let key_file =
        tmp_path.join("devices/c17fc4c8bf#corp#alice@laptop/c17fc4c8bf#corp#alice@laptop.keys");
    crate::tests::utils::create_device_file(&key_file, content).await;

    let access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };
    let outcome = load_device(&tmp_path, &access).await;
    p_assert_matches!(outcome, Err(LoadDeviceError::InvalidData));
}

#[parsec_test(testbed = "empty")]
async fn testbed(env: &TestbedEnv) {
    let alice_email = env
        .customize(|builder| {
            let alice_email = builder
                .bootstrap_organization("alice")
                .map(|u| u.first_user_human_handle.email().to_owned()); // alice@dev1
            builder.new_user("bob"); // bob@dev1
            builder.new_device("alice"); // alice@dev2

            alice_email
        })
        .await;

    // Ok (device created during organization bootstrap)

    let access = DeviceAccessStrategy::Password {
        key_file: env.discriminant_dir.join("devices/alice@dev1.keys"),
        password: "P@ssw0rd.".to_owned().into(),
    };
    let device = load_device(&env.discriminant_dir, &access).await.unwrap();
    p_assert_eq!(device.device_id, "alice@dev1".parse().unwrap());

    // Ok (device created during user creation)

    let access = DeviceAccessStrategy::Password {
        key_file: env.discriminant_dir.join("devices/bob@dev1.keys"),
        password: "P@ssw0rd.".to_owned().into(),
    };
    let device = load_device(&env.discriminant_dir, &access).await.unwrap();
    p_assert_eq!(device.device_id, "bob@dev1".parse().unwrap());

    // Ok (new device for an existing user)

    let access = DeviceAccessStrategy::AccountVault {
        key_file: env.discriminant_dir.join("devices/alice@dev2.keys"),
        operations: Arc::new(MockedAccountVaultOperations::new(alice_email)),
    };
    let device = load_device(&env.discriminant_dir, &access).await.unwrap();
    p_assert_eq!(device.device_id, "alice@dev2".parse().unwrap());

    // Bad password

    let bad_password_access = DeviceAccessStrategy::Password {
        key_file: env.discriminant_dir.join("devices/alice@dev1.keys"),
        password: "dummy".to_owned().into(),
    };
    p_assert_matches!(
        load_device(&env.discriminant_dir, &bad_password_access).await,
        Err(LoadDeviceError::DecryptionFailed)
    );

    // Bad path (key file is missing)

    let bad_path_access = DeviceAccessStrategy::Password {
        key_file: env.discriminant_dir.join("devices/dummy.keys"),
        password: "P@ssw0rd.".to_owned().into(),
    };
    p_assert_matches!(
        load_device(&env.discriminant_dir, &bad_path_access).await,
        Err(LoadDeviceError::InvalidPath(_))
    );

    // Bad account used

    let bad_ciphertext_access = DeviceAccessStrategy::AccountVault {
        key_file: env.discriminant_dir.join("devices/alice@dev2.keys"),
        operations: Arc::new(MockedAccountVaultOperations::new(
            "dummy@example.invalid".parse().unwrap(),
        )),
    };
    p_assert_matches!(
        load_device(&env.discriminant_dir, &bad_ciphertext_access).await,
        Err(LoadDeviceError::DecryptionFailed)
    );
}

#[parsec_test]
async fn remote_error(
    #[values("remote_opaque_key_fetch_offline", "remote_opaque_key_fetch_failed")] kind: &str,
    tmp_path: TmpPath,
) {
    let key_file = tmp_path.join("devices/keyring_file.keys");
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

    let account_vault_operations = Arc::new(MockedAccountVaultOperations::new(
        device.human_handle.email().to_owned(),
    ));
    let save_strategy = DeviceSaveStrategy::AccountVault {
        operations: account_vault_operations.clone(),
    };
    let access_strategy = save_strategy.clone().into_access(key_file.clone());
    save_device(&tmp_path, &save_strategy, &device, key_file.clone())
        .await
        .unwrap();

    match kind {
        "remote_opaque_key_fetch_offline" => {
            account_vault_operations.inject_next_error_fetch_opaque_key(
                AccountVaultOperationsFetchOpaqueKeyError::Offline(ConnectionError::NoResponse(
                    None,
                )),
            );

            p_assert_matches!(
                load_device(&tmp_path, &access_strategy).await,
                Err(LoadDeviceError::RemoteOpaqueKeyFetchOffline(
                    ConnectionError::NoResponse(None)
                ))
            );
        }

        "remote_opaque_key_fetch_failed" => {
            account_vault_operations.inject_next_error_fetch_opaque_key(
                AccountVaultOperationsFetchOpaqueKeyError::UnknownOpaqueKey,
            );

            p_assert_matches!(
                load_device(&tmp_path, &access_strategy).await,
                Err(LoadDeviceError::RemoteOpaqueKeyFetchFailed(err))
                if err.to_string() == "No opaque key with this ID among the vault items"
            );
        }

        unknown => panic!("Unknown kind: {unknown}"),
    }
}
