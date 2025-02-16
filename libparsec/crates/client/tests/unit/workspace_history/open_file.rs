// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{workspace_history_ops_with_server_access_factory, DataAccessStrategy};
use crate::workspace_history::WorkspaceHistoryOpenFileError;

#[parsec_test(testbed = "workspace_history")]
async fn ok(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    #[values("open", "open_and_get_id")] kind: &str,
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

    match kind {
        "open" => {
            let outcome = ops.open_file("/bar.txt".parse().unwrap()).await;
            p_assert_matches!(
                outcome,
                Ok(fd) if fd.0 == 1
            );
        }
        "open_and_get_id" => {
            let outcome = ops.open_file_and_get_id("/bar.txt".parse().unwrap()).await;
            p_assert_matches!(
                outcome,
                Ok((fd, entry_id)) if fd.0 == 1 && entry_id == wksp1_bar_txt_id
            )
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn server_only_offline(#[values("open", "open_and_get_id")] kind: &str, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    let outcome = match kind {
        "open" => ops
            .open_file("/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        "open_and_get_id" => ops
            .open_file_and_get_id("/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        unknown => panic!("Unknown kind: {}", unknown),
    };
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::Offline(_));
}

#[parsec_test(testbed = "minimal_client_ready")]
#[ignore] // TODO: how to stop certificates ops ? Can the export flavored stop ?
async fn stopped(#[values("open", "open_and_get_id")] kind: &str, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    // ops.certificates_ops.stop().await.unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Resolve `/bar.txt` path: get back the workspace manifest...
        test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_id),
        // ...should be checking the workspace manifest, but the certificate ops
        // is stopped so nothing more happened !
    );

    let outcome = match kind {
        "open" => ops
            .open_file("/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        "open_and_get_id" => ops
            .open_file_and_get_id("/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        unknown => panic!("Unknown kind: {}", unknown),
    };
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::Stopped);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn server_only_no_realm_access(
    #[values("open", "open_and_get_id")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Resolve `/bar.txt` path: get back the workspace manifest, but fail !
        move |_req: authenticated_cmds::latest::vlob_read_batch::Req| {
            authenticated_cmds::latest::vlob_read_batch::Rep::AuthorNotAllowed
        }
    );

    let outcome = match kind {
        "open" => ops
            .open_file("/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        "open_and_get_id" => ops
            .open_file_and_get_id("/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        unknown => panic!("Unknown kind: {}", unknown),
    };
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::NoRealmAccess);
}

#[parsec_test(testbed = "workspace_history")]
async fn entry_not_found(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    #[values("open", "open_and_get_id")] kind: &str,
    env: &TestbedEnv,
) {
    let ops = strategy.start_workspace_history_ops(env).await;

    // Note no query to the server needs to be mocked here:
    // - Workspace manifest is always guaranteed to be in cache
    // - Workspace key bundle has already been loaded at workspace history ops startup

    let outcome = match kind {
        "open" => ops
            .open_file("/dummy.txt".parse().unwrap())
            .await
            .unwrap_err(),
        "open_and_get_id" => ops
            .open_file_and_get_id("/dummy.txt".parse().unwrap())
            .await
            .unwrap_err(),
        unknown => panic!("Unknown kind: {}", unknown),
    };
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::EntryNotFound);
}

#[parsec_test(testbed = "workspace_history")]
async fn entry_not_a_file(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    #[values("open", "open_and_get_id")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");
    let ops = strategy
        .start_workspace_history_ops_at(env, wksp1_v2_timestamp)
        .await;

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // Workspace manifest is always guaranteed to be in cache
            // Workspace key bundle has already been loaded at workspace history ops startup
            // 1) Resolve `/foo` path: get back the `foo` manifest
            test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_foo_id),
        );
    }

    let outcome = match kind {
        "open" => ops.open_file("/foo".parse().unwrap()).await.unwrap_err(),
        "open_and_get_id" => ops
            .open_file_and_get_id("/foo".parse().unwrap())
            .await
            .unwrap_err(),
        unknown => panic!("Unknown kind: {}", unknown),
    };
    p_assert_matches!(
        outcome,
        WorkspaceHistoryOpenFileError::EntryNotAFile { entry_id } if entry_id == wksp1_foo_id
    );
}
