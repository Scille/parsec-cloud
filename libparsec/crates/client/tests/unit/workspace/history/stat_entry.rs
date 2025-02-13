// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_realm_get_keys_bundle, test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::workspace_ops_factory;
use crate::workspace::{WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError};

#[parsec_test(testbed = "workspace_history")]
async fn ok_folder(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_v2_children_available_timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/` path: get back the workspace manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, wksp1_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    p_assert_matches!(
        ops.history.stat_entry(wksp1_foo_v2_children_available_timestamp, &"/".parse().unwrap()).await,
        Ok(WorkspaceHistoryEntryStat::Folder{
            id,
            parent,
            created,
            updated,
            version,
        })
        if {
            p_assert_eq!(id, wksp1_id);
            p_assert_eq!(parent, wksp1_id);
            p_assert_eq!(created, "2000-01-12T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-12T00:00:00Z".parse().unwrap());
            p_assert_eq!(version, 2);
            true
        }
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn ok_file(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_v2_children_available_timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/bar.txt` path: get back the workspace manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, wksp1_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 3) Resolve `/bar.txt` path: get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, wksp1_bar_txt_id),
    );

    p_assert_matches!(
        ops.history.stat_entry(wksp1_foo_v2_children_available_timestamp, &"/bar.txt".parse().unwrap()).await,
        Ok(WorkspaceHistoryEntryStat::File{
            id,
            parent,
            created,
            updated,
            version,
            size,
        })
        if {
            p_assert_eq!(id, wksp1_bar_txt_id);
            p_assert_eq!(parent, wksp1_id);
            p_assert_eq!(created, "2000-01-18T00:00:00Z".parse().unwrap());
            p_assert_eq!(updated, "2000-01-18T00:00:00Z".parse().unwrap());
            p_assert_eq!(version, 2);
            p_assert_eq!(size, 14);
            true
        }
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
        // 1) Resolve `/` path: get back the workspace manifest, but at this time
        // the realm does not exist yet !
        test_send_hook_vlob_read_batch!(env, at: at_before, wksp1_id, wksp1_id),
        // No need to fetch workspace keys bundle since there is no vlob to decrypt !
    );

    p_assert_matches!(
        ops.history
            .stat_entry(at_before, &"/".parse().unwrap())
            .await,
        Err(WorkspaceHistoryStatEntryError::EntryNotFound)
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
            .stat_entry(wksp1_bootstrapped_timestamp, &"/".parse().unwrap())
            .await,
        Err(WorkspaceHistoryStatEntryError::EntryNotFound)
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
            .stat_entry(at, &"/".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatEntryError::Offline(_)
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
        // 1) Resolve `/` path: get back the workspace manifest...
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_id),
        // ...should be checking the workspace manifest, but the certificate ops
        // is stopped so nothing more happened !
    );

    p_assert_matches!(
        ops.history
            .stat_entry(at, &"/".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatEntryError::Stopped
    );
}

#[parsec_test(testbed = "coolorg")]
async fn no_realm_access(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let mallory = env.local_device("mallory@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &mallory, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/` path: get back the workspace manifest, but fail !
        move |req: authenticated_cmds::latest::vlob_read_batch::Req| {
            p_assert_eq!(req.realm_id, wksp1_id);
            p_assert_eq!(req.at, Some(at));
            p_assert_eq!(req.vlobs, [wksp1_id]);
            authenticated_cmds::latest::vlob_read_batch::Rep::AuthorNotAllowed
        }
    );

    p_assert_matches!(
        ops.history
            .stat_entry(at, &"/".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatEntryError::NoRealmAccess
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_not_found(env: &TestbedEnv) {
    let at = DateTime::now();
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/dummy` path: get back the workspace manifest
        test_send_hook_vlob_read_batch!(env, at: at, wksp1_id, wksp1_id),
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // Nothing more since `/dummy` is not in the workspace manifest
    );

    p_assert_matches!(
        ops.history
            .stat_entry(at, &"/dummy".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatEntryError::EntryNotFound
    );
}
