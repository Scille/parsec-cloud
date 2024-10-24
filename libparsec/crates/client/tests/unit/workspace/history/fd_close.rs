// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_realm_get_keys_bundle,
    test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::workspace_ops_factory;
use crate::workspace::WorkspaceHistoryFdCloseError;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let at = DateTime::now();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/bar.txt` path: get back the workspace manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 3) Resolve `/bar.txt` path: get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_bar_txt_id),
    );

    let fd = ops
        .history
        .open_file(at, "/bar.txt".parse().unwrap())
        .await
        .unwrap();
    ops.history.fd_close(fd).unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn bad_file_descriptor(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    p_assert_matches!(
        ops.history.fd_close(FileDescriptor(42)).unwrap_err(),
        WorkspaceHistoryFdCloseError::BadFileDescriptor
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn reuse_after_close(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let at = DateTime::now();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/bar.txt` path: get back the workspace manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 3) Resolve `/bar.txt` path: get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_bar_txt_id),
    );

    let fd = ops
        .history
        .open_file(at, "/bar.txt".parse().unwrap())
        .await
        .unwrap();
    ops.history.fd_close(fd).unwrap();

    p_assert_matches!(
        ops.history.fd_close(fd).unwrap_err(),
        WorkspaceHistoryFdCloseError::BadFileDescriptor
    );
}
