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
            // Get back `/bar.txt` manifest
            test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_bar_txt_id),
            // Note workspace key bundle has already been loaded at workspace history ops startup
        );
    }

    p_assert_matches!(
        ops.open_file_by_id(wksp1_bar_txt_id).await,
        Ok(fd) if fd.0 == 1
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn server_only_offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    let outcome = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap_err();
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::Offline(_));
}

#[parsec_test(testbed = "minimal_client_ready")]
#[ignore] // TODO: how to stop certificates ops ? Can the export flavored stop ?
async fn stopped(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    // ops.certificates_ops.stop().await.unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back `/bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_bar_txt_id),
        // ...should be checking the manifest, but the certificate ops is stopped !
    );

    let outcome = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap_err();
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::Stopped);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn server_only_no_realm_access(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Try to get back `/bar.txt` manifest, but fail !
        move |_req: authenticated_cmds::latest::vlob_read_batch::Req| {
            authenticated_cmds::latest::vlob_read_batch::Rep::AuthorNotAllowed
        }
    );

    let outcome = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap_err();
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::NoRealmAccess);
}

#[parsec_test(testbed = "workspace_history")]
async fn entry_not_found(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let dummy_id = VlobID::default();
    let ops = strategy.start_workspace_history_ops(env).await;

    let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
    let last_realm_certificate_timestamp = env.get_last_realm_certificate_timestamp(wksp1_id);

    let at = ops.timestamp_of_interest();
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

    let outcome = ops.open_file_by_id(dummy_id).await.unwrap_err();
    p_assert_matches!(outcome, WorkspaceHistoryOpenFileError::EntryNotFound);
}

#[parsec_test(testbed = "workspace_history")]
async fn entry_not_a_file(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
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
            // Get back `/foo` manifest
            test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_foo_id),
            // Note workspace key bundle has already been loaded at workspace history ops startup
        );
    }

    let outcome = ops.open_file_by_id(wksp1_foo_id).await.unwrap_err();
    p_assert_matches!(
        outcome,
        WorkspaceHistoryOpenFileError::EntryNotAFile { entry_id } if entry_id == wksp1_foo_id
    );
}
