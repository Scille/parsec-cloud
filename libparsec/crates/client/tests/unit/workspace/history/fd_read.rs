// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks, test_send_hook_block_read,
    test_send_hook_realm_get_keys_bundle, test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::workspace_ops_factory;
use crate::workspace::WorkspaceHistoryFdReadError;

#[parsec_test(testbed = "workspace_history")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_v1_timestamp: DateTime =
        *env.template.get_stuff("wksp1_bar_txt_v1_timestamp");
    let wksp1_bar_txt_v2_timestamp: DateTime =
        *env.template.get_stuff("wksp1_bar_txt_v2_timestamp");
    let wksp1_bar_txt_v1_block_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_v1_block_access");
    let wksp1_bar_txt_v2_block1_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_v2_block1_access");
    let wksp1_bar_txt_v2_block2_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_v2_block2_access");
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

    macro_rules! assert_fd_read {
        ($fd:expr, $offset:expr, $size:expr, $expected:expr) => {{
            let mut buf = Vec::new();
            ops.history
                .fd_read($fd, $offset, $size, &mut buf)
                .await
                .unwrap();
            p_assert_eq!(buf, $expected);
        }};
    }

    // 1) Access bar.txt's v1

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_v1_block_access.id),
    );

    assert_fd_read!(fd_v1, 0, 100, b"Hello v1");
    assert_fd_read!(fd_v1, 0, 1, b"H");
    assert_fd_read!(fd_v1, 4, 3, b"o v");
    assert_fd_read!(fd_v1, 10, 1, b"");

    // 2) Access bar.txt's v2

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_v2_block1_access.id),
        test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_v2_block2_access.id),
    );

    assert_fd_read!(fd_v2, 0, 100, b"Hello v2 world");
    assert_fd_read!(fd_v2, 100, 1, b"");
    assert_fd_read!(fd_v2, 10, 2, b"or");

    ops.history.fd_close(fd_v1).unwrap();
    ops.history.fd_close(fd_v2).unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn bad_file_descriptor(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let mut buff = vec![];
    p_assert_matches!(
        ops.history
            .fd_read(FileDescriptor(42), 0, 1, &mut buff)
            .await
            .unwrap_err(),
        WorkspaceHistoryFdReadError::BadFileDescriptor
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

    let mut buff = vec![];
    p_assert_matches!(
        ops.history.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::BadFileDescriptor
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(env: &TestbedEnv) {
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

    let mut buff = vec![];
    p_assert_matches!(
        ops.history.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::Offline(_)
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_block_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_block_access");
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

    ops.certificates_ops.stop().await.unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Fetch the block...
        test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_block_access.id),
        // ...the certificate ops is stopped so nothing more happened !
    );

    let mut buff = vec![];
    p_assert_matches!(
        ops.history.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::Stopped
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_realm_access(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_block_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_block_access");
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

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::block_read::Req| {
            p_assert_eq!(req.block_id, wksp1_bar_txt_block_access.id);
            authenticated_cmds::latest::block_read::Rep::AuthorNotAllowed
        }
    );

    let mut buff = vec![];
    p_assert_matches!(
        ops.history.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::NoRealmAccess
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn block_not_found(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_block_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_block_access");
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

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::block_read::Req| {
            p_assert_eq!(req.block_id, wksp1_bar_txt_block_access.id);
            authenticated_cmds::latest::block_read::Rep::BlockNotFound
        }
    );

    let mut buff = vec![];
    p_assert_matches!(
        ops.history.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::BlockNotFound
    );
}
