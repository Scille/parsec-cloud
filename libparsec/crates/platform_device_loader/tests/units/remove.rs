// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_fixtures::prelude::*;

use crate::{
    load_device, remove_device, save_device, tests::utils::key_present_in_system,
    DeviceAccessStrategy, DeviceSaveStrategy, LoadDeviceError,
};

#[parsec_test(testbed = "minimal")]
async fn remove_ok(tmp_path: TmpPath, env: &TestbedEnv) {
    // 1. Save device to filesystem.
    let device = env.local_device("alice@dev1");
    let key_file = tmp_path.join("alice.device");
    let save_strategy = DeviceSaveStrategy::new_password("FooBar".to_owned().into());
    save_device(&tmp_path, &save_strategy, &device, key_file.clone())
        .await
        .unwrap();

    // 2. Remove the device.
    remove_device(&tmp_path, &key_file).await.unwrap();

    // 3. Check that the device as been removed.
    assert!(
        !key_present_in_system(&key_file).await,
        "Device file should have been removed"
    );
}

#[parsec_test(testbed = "empty")]
async fn testbed(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.bootstrap_organization("alice"); // alice@dev1
    })
    .await;

    // Note the key file must be the device nickname, otherwise the testbed won't
    // understand which device should be removed.
    let key_file = env.discriminant_dir.join("devices/alice@dev1.keys");

    // Sanity check to ensure the key file is the one present in testbed
    let access =
        DeviceAccessStrategy::new_password(key_file.clone(), "P@ssw0rd.".to_owned().into());
    load_device(&env.discriminant_dir, &access).await.unwrap();

    remove_device(&env.discriminant_dir, &key_file)
        .await
        .unwrap();

    p_assert_matches!(
        load_device(&env.discriminant_dir, &access).await,
        Err(LoadDeviceError::InvalidPath(_))
    );
}
