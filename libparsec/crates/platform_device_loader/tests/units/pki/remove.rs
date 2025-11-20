// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::parsec_test;
use libparsec_types::PKIEnrollmentID;

use crate::{
    get_default_local_pending_file, remove_device, save_pki_local_pending,
    tests::pki::save::create_pki_local_pending_from_device, RemoveDeviceError,
};

#[parsec_test(testbed = "minimal")]
async fn remove_ok(tmp_path: TmpPath, env: &TestbedEnv) {
    let alice_device = env.local_device("alice@dev1");
    let local_pending = create_pki_local_pending_from_device(alice_device);
    let enrollment_id = local_pending.enrollment_id;

    let path = get_default_local_pending_file(&tmp_path, enrollment_id);
    save_pki_local_pending(local_pending, path.clone())
        .await
        .unwrap();
    remove_device(&tmp_path, &path).await.unwrap();
}

#[parsec_test]
async fn not_found(tmp_path: TmpPath) {
    let path = get_default_local_pending_file(&tmp_path, PKIEnrollmentID::default());
    assert!(matches!(
        remove_device(&tmp_path, &path).await.unwrap_err(),
        RemoveDeviceError::NotFound
    ));
}
