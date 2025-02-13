// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_realm_get_keys_bundle, test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::workspace_ops_factory;
use crate::workspace::WorkspaceHistoryOpenFileError;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(#[values("open", "open_and_get_id")] kind: &str, env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/bar.txt` path: get back the workspace manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 3) Resolve `/bar.txt` path: get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_bar_txt_id),
    );

    match kind {
        "open" => {
            let outcome = ops.history.open_file(at, "/bar.txt".parse().unwrap()).await;
            p_assert_matches!(
                outcome,
                Ok(fd) if fd.0 == 1
            );
        }
        "open_and_get_id" => {
            let outcome = ops
                .history
                .open_file_and_get_id(at, "/bar.txt".parse().unwrap())
                .await;
            p_assert_matches!(
                outcome,
                Ok((fd, entry_id)) if fd.0 == 1 && entry_id == wksp1_bar_txt_id
            )
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(#[values("open", "open_and_get_id")] kind: &str, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let outcome = match kind {
        "open" => ops
            .history
            .open_file(DateTime::now(), "/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        "open_and_get_id" => ops
            .history
            .open_file_and_get_id(DateTime::now(), "/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        unknown => panic!("Unknown kind: {}", unknown),
    };
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::Offline(_));
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(#[values("open", "open_and_get_id")] kind: &str, env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    ops.certificates_ops.stop().await.unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/bar.txt` path: get back the workspace manifest...
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_id),
        // ...should be checking the workspace manifest, but the certificate ops
        // is stopped so nothing more happened !
    );

    let outcome = match kind {
        "open" => ops
            .history
            .open_file(at, "/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        "open_and_get_id" => ops
            .history
            .open_file_and_get_id(at, "/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        unknown => panic!("Unknown kind: {}", unknown),
    };
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::Stopped);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_realm_access(#[values("open", "open_and_get_id")] kind: &str, env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/bar.txt` path: get back the workspace manifest, but fail !
        move |req: authenticated_cmds::latest::vlob_read_batch::Req| {
            p_assert_eq!(req.realm_id, wksp1_id);
            p_assert_eq!(req.at, Some(at));
            p_assert_eq!(req.vlobs, [wksp1_id]);
            authenticated_cmds::latest::vlob_read_batch::Rep::AuthorNotAllowed
        }
    );

    let outcome = match kind {
        "open" => ops
            .history
            .open_file(at, "/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        "open_and_get_id" => ops
            .history
            .open_file_and_get_id(at, "/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        unknown => panic!("Unknown kind: {}", unknown),
    };
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::NoRealmAccess);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_not_found(#[values("open", "open_and_get_id")] kind: &str, env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/dummy.txt` path: get back the workspace manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // Nothing more since `/dummy.txt` is not in the workspace manifest
    );

    let outcome = match kind {
        "open" => ops
            .history
            .open_file(at, "/dummy.txt".parse().unwrap())
            .await
            .unwrap_err(),
        "open_and_get_id" => ops
            .history
            .open_file_and_get_id(at, "/dummy.txt".parse().unwrap())
            .await
            .unwrap_err(),
        unknown => panic!("Unknown kind: {}", unknown),
    };
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::EntryNotFound);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_not_a_file(#[values("open", "open_and_get_id")] kind: &str, env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/foo` path: get back the workspace manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 3) Resolve `/foo` path: get back the `foo` manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_foo_id),
    );

    let outcome = match kind {
        "open" => ops
            .history
            .open_file(at, "/foo".parse().unwrap())
            .await
            .unwrap_err(),
        "open_and_get_id" => ops
            .history
            .open_file_and_get_id(at, "/foo".parse().unwrap())
            .await
            .unwrap_err(),
        unknown => panic!("Unknown kind: {}", unknown),
    };
    p_assert_matches!(
        outcome,
        WorkspaceHistoryOpenFileError::EntryNotAFile { entry_id } if entry_id == wksp1_foo_id
    );
}
