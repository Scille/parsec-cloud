// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;

#[parsec_test(testbed = "coolorg")]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let bob_user_id: UserID = "bob".parse().unwrap();
    let share_recipients = HashMap::from([(bob_user_id, 2)]);
    client
        .shamir_setup_create(share_recipients, 1)
        .await
        .unwrap();
}
