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
async fn ok(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back `/bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_bar_txt_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    p_assert_matches!(
        ops.history.open_file_by_id(at, wksp1_bar_txt_id).await,
        Ok(fd) if fd.0 == 1
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let outcome = ops
        .history
        .open_file_by_id(DateTime::now(), wksp1_bar_txt_id)
        .await
        .unwrap_err();
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::Offline(_));
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    ops.certificates_ops.stop().await.unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back `/bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_bar_txt_id),
        // ...should be checking the manifest, but the certificate ops is stopped !
    );

    let outcome = ops
        .history
        .open_file_by_id(at, wksp1_bar_txt_id)
        .await
        .unwrap_err();
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::Stopped);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_realm_access(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Try to get back `/bar.txt` manifest, but fail !
        move |req: authenticated_cmds::latest::vlob_read_batch::Req| {
            p_assert_eq!(req.realm_id, wksp1_id);
            p_assert_eq!(req.at, Some(at));
            p_assert_eq!(req.vlobs, [wksp1_bar_txt_id]);
            authenticated_cmds::latest::vlob_read_batch::Rep::AuthorNotAllowed
        }
    );

    let outcome = ops
        .history
        .open_file_by_id(at, wksp1_bar_txt_id)
        .await
        .unwrap_err();
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::NoRealmAccess);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_not_found(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let dummy_id = VlobID::default();
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
    let last_realm_certificate_timestamp = env.get_last_realm_certificate_timestamp(wksp1_id);

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Try to get dummy ID from the server
        move |req: authenticated_cmds::latest::vlob_read_batch::Req| {
            p_assert_eq!(req.realm_id, wksp1_id);
            p_assert_eq!(req.at, Some(at));
            p_assert_eq!(req.vlobs, [dummy_id]);
            authenticated_cmds::latest::vlob_read_batch::Rep::Ok {
                items: vec![],
                needed_common_certificate_timestamp: last_common_certificate_timestamp,
                needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
            }
        }
    );

    let outcome = ops.history.open_file_by_id(at, dummy_id).await.unwrap_err();
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::EntryNotFound);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_not_a_file(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back `/foo` manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_foo_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    let outcome = ops
        .history
        .open_file_by_id(at, wksp1_foo_id)
        .await
        .unwrap_err();
    p_assert_matches!(
        outcome,
        WorkspaceHistoryOpenFileError::EntryNotAFile { entry_id } if entry_id == wksp1_foo_id
    );
}
