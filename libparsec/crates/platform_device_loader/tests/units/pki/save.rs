// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::save_pki_local_pending;
use crate::tests::pki::create_pki_local_pending_from_device;
use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::prelude::*;
use libparsec_tests_lite::parsec_test;

#[parsec_test(testbed = "minimal")]
async fn ok_simple(tmp_path: TmpPath, env: &TestbedEnv) {
    let path = tmp_path.join("local_pki_enrol.keys");

    let alice_device = env.local_device("alice@dev1");
    let local_pending = create_pki_local_pending_from_device(alice_device);
    save_pki_local_pending(local_pending, path).await.unwrap();
}
