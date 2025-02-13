// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_realm_get_keys_bundle, test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::workspace_ops_factory;
use crate::workspace::{WorkspaceHistoryEntryStat, WorkspaceHistoryStatFolderChildrenError};

// Note `open_folder_reader_by_id` is not directly tested since it is internally used by `stat_folder_children_by_id`

#[parsec_test(testbed = "workspace_history")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_foo_v2_children_available_timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back the `foo` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, wksp1_foo_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 3) Get `/foo/egg.txt` & `/foo/spam` manifest (order of fetch is not guaranteed)
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
    );

    let mut children_stats = ops
        .history
        .stat_folder_children_by_id(wksp1_foo_v2_children_available_timestamp, wksp1_foo_id)
        .await
        .unwrap();
    children_stats.sort_by(|a, b| a.0.cmp(&b.0));
    p_assert_eq!(
        children_stats,
        vec![
            (
                "egg.txt".parse().unwrap(),
                WorkspaceHistoryEntryStat::File {
                    id: wksp1_foo_egg_txt_id,
                    parent: wksp1_foo_id,
                    created: "2000-01-20T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-20T00:00:00Z".parse().unwrap(),
                    version: 1,
                    size: 0,
                }
            ),
            (
                "spam".parse().unwrap(),
                WorkspaceHistoryEntryStat::Folder {
                    id: wksp1_foo_spam_id,
                    parent: wksp1_foo_id,
                    created: "2000-01-21T00:00:00Z".parse().unwrap(),
                    updated: "2000-01-21T00:00:00Z".parse().unwrap(),
                    version: 1,
                }
            ),
        ]
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn before_realm_exists(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_created_timestamp: DateTime = *env.template.get_stuff("wksp1_created_timestamp");
    let at_before = wksp1_created_timestamp.add_us(-1000);

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back the workspace manifest, but at this time the realm does not exist yet !
        test_send_hook_vlob_read_batch!(env, at: at_before, wksp1_id, wksp1_id),
        // No need to fetch workspace keys bundle since there is no vlob to decrypt !
    );

    p_assert_matches!(
        ops.history
            .stat_folder_children_by_id(at_before, wksp1_id)
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::EntryNotFound
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn before_workspace_manifest_v1(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bootstrapped_timestamp: DateTime =
        *env.template.get_stuff("wksp1_bootstrapped_timestamp");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Try to get back the workspace manifest... but fail since it doesn't exist yet !
        test_send_hook_vlob_read_batch!(env, at: wksp1_bootstrapped_timestamp, wksp1_id, wksp1_id),
    );

    p_assert_matches!(
        ops.history
            .stat_folder_children_by_id(wksp1_bootstrapped_timestamp, wksp1_id)
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::EntryNotFound
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn entry_available_but_not_referenced_in_parent(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_v1_timestamp: DateTime = *env.template.get_stuff("wksp1_foo_v1_timestamp");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back the `foo` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v1_timestamp, wksp1_id, wksp1_foo_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    p_assert_eq!(
        ops.history
            .stat_folder_children_by_id(wksp1_foo_v1_timestamp, wksp1_foo_id)
            .await
            .unwrap(),
        vec![]
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn entry_available_but_not_its_children(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_foo_v2_timestamp");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/foo` path: get back the `foo` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_timestamp, wksp1_id, wksp1_foo_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 3) Get `/foo/egg.txt` and `/foo/spam` manifest, which didn't exist at that time.
        // Note there is no guarantee on the order of those requests.
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
    );

    p_assert_eq!(
        ops.history
            .stat_folder_children_by_id(wksp1_foo_v2_timestamp, wksp1_foo_id)
            .await
            .unwrap(),
        vec![]
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    p_assert_matches!(
        ops.history
            .stat_folder_children_by_id(at, wksp1_id)
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::Offline(_)
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    ops.certificates_ops.stop().await.unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back the workspace manifest...
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_id),
        // ...should be checking the workspace manifest, but the certificate ops
        // is stopped so nothing more happened !
    );

    p_assert_matches!(
        ops.history
            .stat_folder_children_by_id(at, wksp1_id)
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::Stopped
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_realm_access(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back the workspace manifest, but fail !
        move |req: authenticated_cmds::latest::vlob_read_batch::Req| {
            p_assert_eq!(req.realm_id, wksp1_id);
            p_assert_eq!(req.at, Some(at));
            p_assert_eq!(req.vlobs, [wksp1_id]);
            authenticated_cmds::latest::vlob_read_batch::Rep::AuthorNotAllowed
        }
    );

    p_assert_matches!(
        ops.history
            .stat_folder_children_by_id(at, wksp1_id)
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::NoRealmAccess
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_not_found(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let dummy_id = VlobID::default();

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Try to get back the dummy manifest... and fail since it doesn't exist !
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, dummy_id),
    );

    p_assert_matches!(
        ops.history
            .stat_folder_children_by_id(at, dummy_id)
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::EntryNotFound
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_is_file(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_bar_txt_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    p_assert_matches!(
        ops.history
            .stat_folder_children_by_id(at, wksp1_bar_txt_id)
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::EntryIsFile
    );
}
