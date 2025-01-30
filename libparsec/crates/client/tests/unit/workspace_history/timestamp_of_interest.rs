// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_history_ops_with_server_access_factory;

#[parsec_test(testbed = "workspace_history")]
async fn timestamps(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let t0 = DateTime::now();
    let ops =
        workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id.to_owned()).await;

    let test_set_timestamp_of_interest = |timestamp: DateTime, expected: DateTime| {
        let got = ops.set_timestamp_of_interest(timestamp);
        p_assert_eq!(got, expected);
        p_assert_eq!(ops.timestamp_of_interest(), expected);
    };

    // Lower timestamp bound is the upload of workspace manifest v1

    let timestamp_lower_bound = ops.timestamp_lower_bound();
    p_assert_eq!(
        timestamp_lower_bound,
        "2001-01-02T00:00:00Z".parse().unwrap()
    );

    // Higher timestamp bound is current time for server-based workspace history

    let timestamp_higher_bound = ops.timestamp_higher_bound();
    p_assert_matches!(timestamp_higher_bound, timestamp_higher_bound if timestamp_higher_bound > t0);

    // Set valid timestamp
    let wksp1_foo_v2_children_available_timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");
    test_set_timestamp_of_interest(
        wksp1_foo_v2_children_available_timestamp,
        wksp1_foo_v2_children_available_timestamp,
    );

    // Set timestamp older than realm creation
    test_set_timestamp_of_interest(timestamp_lower_bound.add_us(-1), timestamp_lower_bound);

    // Set timestamp too far in the future
    test_set_timestamp_of_interest(timestamp_higher_bound.add_us(1), timestamp_higher_bound);
}
