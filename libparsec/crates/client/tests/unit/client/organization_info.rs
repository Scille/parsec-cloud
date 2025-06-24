// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::TestbedEnv;
use libparsec_tests_lite::{p_assert_eq, parsec_test};

use crate::{client::tests::utils::client_factory, OrganizationInfo};

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok(env: &TestbedEnv) {
    let alice_device = env.local_device("alice@dev1");
    let alice_client = client_factory(&env.discriminant_dir, alice_device).await;

    p_assert_eq!(
        alice_client.organization_info().await.unwrap(),
        OrganizationInfo {
            total_block_bytes: 0,
            total_metadata_bytes: 663
        }
    );
}
