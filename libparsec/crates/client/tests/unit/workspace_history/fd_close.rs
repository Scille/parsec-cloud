// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::DataAccessStrategy;
use crate::workspace_history::WorkspaceHistoryFdCloseError;

#[parsec_test(testbed = "workspace_history")]
async fn ok(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");
    let ops = strategy
        .start_workspace_history_ops_at(env, wksp1_v2_timestamp)
        .await;

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // Workspace manifest is always guaranteed to be in cache
            // Workspace key bundle has already been loaded at workspace history ops startup
            // 1) Resolve `/bar.txt` path: get back the `bar.txt` manifest
            test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_bar_txt_id),
        );
    }

    let fd = ops.open_file("/bar.txt".parse().unwrap()).await.unwrap();
    ops.fd_close(fd).unwrap();
}

#[parsec_test(testbed = "workspace_history")]
async fn bad_file_descriptor(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let ops = strategy.start_workspace_history_ops(env).await;

    p_assert_matches!(
        ops.fd_close(FileDescriptor(42)).unwrap_err(),
        WorkspaceHistoryFdCloseError::BadFileDescriptor
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn reuse_after_close(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");
    let ops = strategy
        .start_workspace_history_ops_at(env, wksp1_v2_timestamp)
        .await;

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // Workspace manifest is always guaranteed to be in cache
            // Workspace key bundle has already been loaded at workspace history ops startup
            // 1) Resolve `/bar.txt` path: get back the `bar.txt` manifest
            test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_bar_txt_id),
        );
    }

    let fd = ops.open_file("/bar.txt".parse().unwrap()).await.unwrap();
    ops.fd_close(fd).unwrap();

    p_assert_matches!(
        ops.fd_close(fd).unwrap_err(),
        WorkspaceHistoryFdCloseError::BadFileDescriptor
    );
}
