// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use libparsec_platform_device_loader::{
    load_device, save_device, LoadDeviceError, SaveDeviceError,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

enum BadPathKind {
    PathIsDir,
    TmpPathIsDir,
    PathIsRoot,
    PathHasNoName,
}

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
    let access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };
    let outcome = save_device(&tmp_path, &access, &device).await;
    p_assert_matches!(outcome, Err(SaveDeviceError::InvalidPath(_)));
}

enum OkKind {
    ExistingParentDir,
    MissingParentDir,
    OverwrittingFile,
}

#[parsec_test(testbed = "minimal")]
#[case::existing_parent_dir(OkKind::ExistingParentDir)]
#[case::missing_parent_dir(OkKind::MissingParentDir)]
#[case::overwritting_file(OkKind::OverwrittingFile)]
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
        OkKind::OverwrittingFile => {
            let path = tmp_path.join("devices/my_device.keys");
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, b"<dummy>").unwrap();
            path
        }
    };

    let access = DeviceAccessStrategy::Password {
        key_file,
        password: "P@ssw0rd.".to_owned().into(),
    };
    let alice_device = env.local_device("alice@dev1");
    let outcome = save_device(&tmp_path, &access, &alice_device).await;
    p_assert_matches!(outcome, Ok(()));

    // Roundtrip check
    let loaded = load_device(&tmp_path, &access).await.unwrap();
    p_assert_eq!(*loaded, *alice_device);
}

#[parsec_test(testbed = "empty")]
async fn testbed(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.bootstrap_organization("alice"); // alice@dev1
    });
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
    let new_access = DeviceAccessStrategy::Password {
        key_file: key_file.clone(),
        password: "N3wP@ssw0rd.".to_owned().into(),
    };
    save_device(&env.discriminant_dir, &new_access, &device)
        .await
        .unwrap();

    // Finally roundtrip check
    p_assert_matches!(
        load_device(&env.discriminant_dir, &old_access).await,
        Err(LoadDeviceError::DecryptionFailed)
    );
    let reloaded_device = load_device(&env.discriminant_dir, &new_access)
        .await
        .unwrap();
    p_assert_eq!(reloaded_device, device);
}
