// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_history_ops_with_server_access_factory;
use crate::workspace_history::{
    WorkspaceHistoryEntryStat, WorkspaceHistoryStatFolderChildrenError,
};

// Note `open_folder_reader` is not directly tested since it is internally used by `stat_folder_children`

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
    let ops =
        workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id.to_owned()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back `/` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, wksp1_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );
    ops.set_timestamp_of_interest(wksp1_foo_v2_children_available_timestamp)
        .await
        .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/foo` path: workspace manifest is already in cache
        // Note workspace key bundle has already been loaded at workspace history ops startup
        // 2) Resolve `/foo` path: get back the `foo` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, wksp1_foo_id),
        // 3) Get `/foo/egg.txt` & `/foo/spam` manifest (order of fetch is not guaranteed)
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
    );

    let mut children_stats = ops
        .stat_folder_children(&"/foo".parse().unwrap())
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
async fn entry_available_but_not_referenced_in_parent(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_v1_timestamp: DateTime = *env.template.get_stuff("wksp1_foo_v1_timestamp");
    let alice = env.local_device("alice@dev1");
    let ops =
        workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id.to_owned()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back `/` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v1_timestamp, wksp1_id, wksp1_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );
    ops.set_timestamp_of_interest(wksp1_foo_v1_timestamp)
        .await
        .unwrap();

    p_assert_eq!(
        ops.stat_folder_children(&"/".parse().unwrap())
            .await
            .unwrap(),
        vec![]
    );
    p_assert_matches!(
        ops.stat_folder_children(&"/foo".parse().unwrap()).await,
        Err(WorkspaceHistoryStatFolderChildrenError::EntryNotFound)
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
    let ops =
        workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id.to_owned()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back `/` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_timestamp, wksp1_id, wksp1_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );
    ops.set_timestamp_of_interest(wksp1_foo_v2_timestamp)
        .await
        .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/foo` path: workspace manifest is already in cache
        // Note workspace key bundle has already been loaded at workspace history ops startup
        // 2) Resolve `/foo` path: get back the `foo` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_timestamp, wksp1_id, wksp1_foo_id),
        // 3) Get `/foo/egg.txt` and `/foo/spam` manifest, which didn't exist at that time.
        // Note there is no guarantee on the order of those requests.
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
    );

    p_assert_eq!(
        ops.stat_folder_children(&"/foo".parse().unwrap())
            .await
            .unwrap(),
        vec![]
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    p_assert_matches!(
        ops.stat_folder_children(&"/".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::Offline
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
#[ignore] // TODO: how to stop certificates ops ? Can the export flavored stop ?
async fn stopped(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    // ops.certificates_ops.stop().await.unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/` path: get back the workspace manifest...
        test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_id),
        // ...should be checking the workspace manifest, but the certificate ops
        // is stopped so nothing more happened !
    );

    p_assert_matches!(
        ops.stat_folder_children(&"/".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::Stopped
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_realm_access(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Resolve `/` path: get back the workspace manifest, but fail !
        move |_req: authenticated_cmds::latest::vlob_read_batch::Req| {
            authenticated_cmds::latest::vlob_read_batch::Rep::AuthorNotAllowed
        }
    );

    p_assert_matches!(
        ops.stat_folder_children(&"/".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::NoRealmAccess
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_not_found(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    // Note no query to the server needs to be mocked here:
    // - Workspace manifest is always guaranteed to be in cache
    // - Workspace key bundle has already been loaded at workspace history ops startup

    p_assert_matches!(
        ops.stat_folder_children(&"/dummy".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::EntryNotFound
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_is_file(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Workspace manifest is always guaranteed to be in cache
        // Workspace key bundle has already been loaded at workspace history ops startup
        // 1) Resolve `/bar.txt` path: get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_bar_txt_id),
    );

    p_assert_matches!(
        ops.stat_folder_children(&"/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatFolderChildrenError::EntryIsFile
    );
}
