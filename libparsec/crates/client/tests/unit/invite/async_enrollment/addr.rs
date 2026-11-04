// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;

use super::super::utils::client_factory;

#[parsec_test(testbed = "minimal")]
async fn get_async_enrollment_addr(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    p_assert_eq!(
        client.get_async_enrollment_addr().to_string(),
        "parsec3://noserver.example.com/OfflineOrg?a=async_enrollment"
    );
}
