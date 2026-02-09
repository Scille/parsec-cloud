// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::tests::pki::create_pki_local_pending_from_device;
use crate::{
    get_default_local_pending_file, get_local_pending_dir, list_pki_local_pending, remove_device,
    save_pki_local_pending,
};
use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::prelude::*;
use libparsec_tests_lite::parsec_test;

#[parsec_test(testbed = "coolorg")]
async fn ok_simple(tmp_path: TmpPath, env: &TestbedEnv) {
    let path = get_local_pending_dir(&tmp_path);
    let mut expected = Vec::new();
    // No pending enrollments
    assert_eq!(list_pki_local_pending(&path).await.unwrap(), expected);

    // Some pending enrollments
    for device in ["alice@dev1", "bob@dev1", "mallory@dev1"] {
        let device = env.local_device(device);
        let local_pending = create_pki_local_pending_from_device(device);
        save_pki_local_pending(
            local_pending.clone(),
            get_default_local_pending_file(&path, local_pending.enrollment_id),
        )
        .await
        .unwrap();
        expected.push(local_pending)
    }
    expected.sort_by_key(|e| e.enrollment_id);
    assert_eq!(list_pki_local_pending(&path).await.unwrap(), expected);

    // Less pending enrollments
    remove_device(
        &path,
        &get_default_local_pending_file(&path, expected[2].enrollment_id),
    )
    .await
    .unwrap();
    expected.pop();
    assert_eq!(list_pki_local_pending(&path).await.unwrap(), expected);
}
