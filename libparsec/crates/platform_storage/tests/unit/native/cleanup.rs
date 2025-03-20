// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::{parsec_test, tmp_path, TestbedEnv, TmpPath};
use libparsec_types::VlobID;

use crate::{
    native::model::get_workspace_storage_db_relative_path, remove_device_data,
    workspace::workspace_storage_non_speculative_init,
};

#[parsec_test(testbed = "minimal")]
async fn cleanup_device_data(tmp_path: TmpPath, env: &TestbedEnv) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ff").unwrap();
    let alice = env.local_device("alice@dev1");

    // 1) Create some data associated with the device.
    workspace_storage_non_speculative_init(&tmp_path, &alice, realm_id)
        .await
        .unwrap();

    // 2) Check some data are written to the filesystem.
    let wksp_file = get_workspace_storage_db_relative_path(&alice, realm_id);
    let wksp_path = tmp_path.join(&wksp_file);
    assert!(wksp_path.exists(), "wksp file does not exist");
    let device_dir = tmp_path.join(alice.device_id.hex());
    assert!(device_dir.exists(), "device dir does not exist");

    // 3) Cleanup the device data.
    remove_device_data(&tmp_path, alice.device_id)
        .await
        .unwrap();
    assert!(!wksp_file.exists(), "wksp file still exists");
    assert!(!device_dir.exists(), "device dir still exists");
}

// TODO: Convert this test be compatible with web
// see https://github.com/Scille/parsec-cloud/issues/10007
