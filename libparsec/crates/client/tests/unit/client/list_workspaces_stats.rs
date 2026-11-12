// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::test_register_send_hook;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::{ClientListWorkspacesStatsError, WorkspaceStats};

#[parsec_test(testbed = "coolorg")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp2_id = VlobID::default();

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::realm_list_stats::Req| {
            authenticated_cmds::latest::realm_list_stats::Rep::Ok {
                realms: vec![
                    authenticated_cmds::latest::realm_list_stats::RealmStats {
                        realm_id: wksp1_id,
                        metadata_size: 663,
                        data_size: 4096,
                        owners: 1,
                        managers: 2,
                        contributors: 3,
                        readers: 4,
                    },
                    authenticated_cmds::latest::realm_list_stats::RealmStats {
                        realm_id: wksp2_id,
                        metadata_size: 0,
                        data_size: 0,
                        owners: 0,
                        managers: 0,
                        contributors: 0,
                        readers: 0,
                    },
                ],
            }
        },
    );

    p_assert_eq!(
        client.list_workspaces_stats().await.unwrap(),
        vec![
            WorkspaceStats {
                workspace_id: wksp1_id,
                metadata_size: 663,
                data_size: 4096,
                owners: 1,
                managers: 2,
                contributors: 3,
                readers: 4,
            },
            WorkspaceStats {
                workspace_id: wksp2_id,
                metadata_size: 0,
                data_size: 0,
                owners: 0,
                managers: 0,
                contributors: 0,
                readers: 0,
            },
        ]
    );
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok_with_server(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let stats = client.list_workspaces_stats().await.unwrap();

    // Alice's and Bob's user realms are not listed, only `wksp1` is
    p_assert_eq!(stats.len(), 1);
    let wksp1_stats = &stats[0];
    p_assert_eq!(wksp1_stats.workspace_id, wksp1_id);
    // `wksp1` contains its workspace manifest
    p_assert_matches!(wksp1_stats.metadata_size, size if size > 0);
    p_assert_eq!(wksp1_stats.data_size, 0);
    p_assert_eq!(wksp1_stats.owners, 1); // Alice
    p_assert_eq!(wksp1_stats.managers, 0);
    p_assert_eq!(wksp1_stats.contributors, 0);
    p_assert_eq!(wksp1_stats.readers, 1); // Bob
}

#[parsec_test(testbed = "coolorg")]
async fn author_not_allowed(env: &TestbedEnv) {
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    test_register_send_hook(
        &env.discriminant_dir,
        |_req: authenticated_cmds::latest::realm_list_stats::Req| {
            authenticated_cmds::latest::realm_list_stats::Rep::AuthorNotAllowed
        },
    );

    p_assert_matches!(
        client.list_workspaces_stats().await.unwrap_err(),
        ClientListWorkspacesStatsError::AuthorNotAllowed
    );
}

#[parsec_test(testbed = "coolorg")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // No send hook registered: the server is not reachable

    p_assert_matches!(
        client.list_workspaces_stats().await.unwrap_err(),
        ClientListWorkspacesStatsError::Offline(_)
    );
}

#[parsec_test(testbed = "coolorg")]
async fn unknown_status(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        |_req: authenticated_cmds::latest::realm_list_stats::Req| {
            authenticated_cmds::latest::realm_list_stats::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
            }
        },
    );

    p_assert_matches!(
        client.list_workspaces_stats().await.unwrap_err(),
        ClientListWorkspacesStatsError::Internal(_)
    );
}
