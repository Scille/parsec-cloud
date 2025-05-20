// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{
    DataAccessStrategy, StartWorkspaceHistoryOpsError,
    workspace_history_ops_with_server_access_factory,
};
use crate::workspace_history::{WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError};

#[parsec_test(testbed = "workspace_history")]
async fn ok_folder(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
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

    // `/` manifest is already in cache, no need for `test_register_sequence_of_send_hooks` here
    p_assert_matches!(
        ops.stat_entry(&"/".parse().unwrap()).await,
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
async fn ok_file(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_v2_children_available_timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");
    let ops = match strategy
        .start_workspace_history_ops_at(env, wksp1_foo_v2_children_available_timestamp)
        .await
    {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // 1) Resolve `/bar.txt` path: workspace manifest is already in cache
            // Note workspace key bundle has already been loaded at workspace history ops startup
            // 2) Resolve `/bar.txt` path: get back the `bar.txt` manifest
            test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, wksp1_bar_txt_id),
        );
    }

    p_assert_matches!(
        ops.stat_entry(&"/bar.txt".parse().unwrap()).await,
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

#[parsec_test(testbed = "minimal_client_ready")]
async fn server_only_offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    p_assert_matches!(
        ops.stat_entry(&"/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatEntryError::Offline(_)
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
        // Resolve `/` path: get back the workspace manifest...
        test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_id),
        // ...should be checking the workspace manifest, but the certificate ops
        // is stopped so nothing more happened !
    );

    p_assert_matches!(
        ops.stat_entry(&"/".parse().unwrap()).await.unwrap_err(),
        WorkspaceHistoryStatEntryError::Stopped
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn server_only_no_realm_access(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Resolve `/bar.txt` path: get back `bar.txt` manifest, but fail !
        move |_req: authenticated_cmds::latest::vlob_read_batch::Req| {
            authenticated_cmds::latest::vlob_read_batch::Rep::AuthorNotAllowed
        }
    );

    p_assert_matches!(
        ops.stat_entry(&"/bar.txt".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatEntryError::NoRealmAccess
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn entry_not_found(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let ops = match strategy.start_workspace_history_ops(env).await {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    // Note no query to the server needs to be mocked here:
    // - Workspace manifest is always guaranteed to be in cache
    // - Workspace key bundle has already been loaded at workspace history ops startup

    p_assert_matches!(
        ops.stat_entry(&"/dummy".parse().unwrap())
            .await
            .unwrap_err(),
        WorkspaceHistoryStatEntryError::EntryNotFound
    );
}
