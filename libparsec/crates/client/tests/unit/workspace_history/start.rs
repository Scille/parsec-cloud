// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_history_ops_with_server_access_factory;

#[parsec_test(testbed = "workspace_history")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    p_assert_eq!(ops.realm_id(), wksp1_id,);

    p_assert_eq!(ops.organization_id(), alice.organization_id(),);

    let timestamp_lower_bound = ops.timestamp_lower_bound();
    let timestamp_higher_bound = ops.timestamp_higher_bound();

    p_assert_eq!(
        timestamp_lower_bound,
        // Wksp1's workspace manifest was created on 2001-01-02
        "2001-01-02T00:00:00Z".parse().unwrap(),
    );

    assert!(
        timestamp_higher_bound >= "2001-01-30T00:00:00Z".parse().unwrap(),
        "{:?}",
        timestamp_higher_bound
    );

    p_assert_eq!(ops.timestamp_of_interest(), timestamp_lower_bound,);
}
