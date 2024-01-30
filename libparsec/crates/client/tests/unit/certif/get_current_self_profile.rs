// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let res = ops.get_current_self_profile().await.unwrap();

    p_assert_eq!(res, UserProfile::Admin);
}
