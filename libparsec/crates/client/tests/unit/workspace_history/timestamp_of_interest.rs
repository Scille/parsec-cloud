// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::WorkspaceHistorySetTimestampOfInterestError;

use super::utils::{
    DataAccessStrategy, StartWorkspaceHistoryOpsError,
    workspace_history_ops_with_server_access_factory,
};

#[parsec_test(testbed = "workspace_history")]
async fn ok(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let t0 = DateTime::now();
    let ops = match strategy.start_workspace_history_ops(env).await {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    // Lower timestamp bound is the upload of workspace manifest v1

    let timestamp_lower_bound = ops.timestamp_lower_bound();
    p_assert_eq!(
        timestamp_lower_bound,
        "2001-01-02T00:00:00Z".parse().unwrap()
    );

    // Higher timestamp bound is current time for server-based workspace history

    let timestamp_higher_bound = ops.timestamp_higher_bound();
    match strategy {
        DataAccessStrategy::Server => {
            assert!(timestamp_higher_bound >= t0);
        }
        DataAccessStrategy::RealmExport => {
            p_assert_eq!(
                timestamp_higher_bound,
                "2025-02-15T22:16:26.601990Z".parse().unwrap()
            );
        }
    }

    let default_timestamp_of_interest = ops.timestamp_of_interest();
    p_assert_eq!(default_timestamp_of_interest, timestamp_lower_bound);

    // Set valid timestamp
    let wksp1_foo_v2_children_available_timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, wksp1_id),
    );

    ops.set_timestamp_of_interest(wksp1_foo_v2_children_available_timestamp)
        .await
        .unwrap();
    p_assert_eq!(
        ops.timestamp_of_interest(),
        wksp1_foo_v2_children_available_timestamp
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn older_than_lower_bound(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let ops = match strategy.start_workspace_history_ops(env).await {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    let timestamp_lower_bound = ops.timestamp_lower_bound();

    // Set timestamp older than realm creation
    p_assert_matches!(
        ops.set_timestamp_of_interest(timestamp_lower_bound.add_us(-1))
            .await,
        Err(WorkspaceHistorySetTimestampOfInterestError::OlderThanLowerBound)
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn newer_than_higher_bound(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let ops = match strategy.start_workspace_history_ops(env).await {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    let timestamp_higher_bound = ops.timestamp_higher_bound();

    // Set timestamp too far in the future
    p_assert_matches!(
        ops.set_timestamp_of_interest(timestamp_higher_bound.add_us(1))
            .await,
        Err(WorkspaceHistorySetTimestampOfInterestError::NewerThanHigherBound)
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn server_only_offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_v2_children_available_timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");
    let alice = env.local_device("alice@dev1");
    let ops =
        workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id.to_owned()).await;

    p_assert_matches!(
        ops.set_timestamp_of_interest(wksp1_foo_v2_children_available_timestamp)
            .await,
        Err(WorkspaceHistorySetTimestampOfInterestError::Offline(_))
    );
}
