// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use crate::{archive_device, save_device, tests::utils::key_is_archived};
use libparsec_tests_fixtures::{parsec_test, tmp_path, TestbedEnv, TmpPath};
use libparsec_types::DeviceAccessStrategy;

#[parsec_test(testbed = "minimal")]
async fn archive_ok(tmp_path: TmpPath, env: &TestbedEnv) {
    // 1. Save device to filesystem.
    let device = env.local_device("alice@dev1");
    let key_file = tmp_path.join("alice.device");
    let access = DeviceAccessStrategy::Password {
        key_file: key_file.clone(),
        password: "FooBar".to_owned().into(),
    };
    save_device(&tmp_path, &access, &device).await.unwrap();

    // 2. Archive the device.
    archive_device(&key_file).await.unwrap();

    // 3. Check that the device as been archived.
    assert!(key_is_archived(&key_file).await);
}
