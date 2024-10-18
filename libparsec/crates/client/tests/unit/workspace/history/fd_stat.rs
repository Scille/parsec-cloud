// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_realm_get_keys_bundle,
    test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::workspace_ops_factory;
use crate::workspace::{WorkspaceHistoryFdStatError, WorkspaceHistoryFileStat};

#[parsec_test(testbed = "workspace_history")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_v1_timestamp: DateTime =
        *env.template.get_stuff("wksp1_bar_txt_v1_timestamp");
    let wksp1_bar_txt_v2_timestamp: DateTime =
        *env.template.get_stuff("wksp1_bar_txt_v2_timestamp");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    // 0) Open `bar.txt` v1 and v2

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v1_timestamp, wksp1_id, wksp1_bar_txt_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    let fd_v1 = ops
        .history
        .open_file_by_id(wksp1_bar_txt_v1_timestamp, wksp1_bar_txt_id)
        .await
        .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v2_timestamp, wksp1_id, wksp1_bar_txt_id),
        // (workspace keys bundle already fetched)
    );

    let fd_v2 = ops
        .history
        .open_file_by_id(wksp1_bar_txt_v2_timestamp, wksp1_bar_txt_id)
        .await
        .unwrap();

    // 1) Access bar.txt's v1

    p_assert_matches!(
        ops.history.fd_stat(fd_v1).await.unwrap(),
        WorkspaceHistoryFileStat {
            id,
            created,
            updated,
            version,
            size,
        } if {
            p_assert_eq!(id, wksp1_bar_txt_id);
            p_assert_eq!(created, "2000-01-10T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-10T00:00:00Z".parse().unwrap());
            p_assert_eq!(version, 1);
            p_assert_eq!(size, 8);
            true
        }
    );

    // 2) Access bar.txt's v2

    p_assert_matches!(
        ops.history.fd_stat(fd_v2).await.unwrap(),
        WorkspaceHistoryFileStat {
            id,
            created,
            updated,
            version,
            size,
        } if {
            p_assert_eq!(id, wksp1_bar_txt_id);
            p_assert_eq!(created, "2000-01-18T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-18T00:00:00Z".parse().unwrap());
            p_assert_eq!(version, 2);
            p_assert_eq!(size, 14);
            true
        }
    );

    ops.history.fd_close(fd_v1).unwrap();
    ops.history.fd_close(fd_v2).unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn bad_file_descriptor(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    p_assert_matches!(
        ops.history.fd_stat(FileDescriptor(42)).await.unwrap_err(),
        WorkspaceHistoryFdStatError::BadFileDescriptor
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
        // 1) Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_bar_txt_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    let fd = ops
        .history
        .open_file_by_id(at, wksp1_bar_txt_id)
        .await
        .unwrap();
    ops.history.fd_close(fd).unwrap();

    p_assert_matches!(
        ops.history.fd_stat(fd).await.unwrap_err(),
        WorkspaceHistoryFdStatError::BadFileDescriptor
    );
}
