// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_client_connection::test_register_send_hook;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    ClientListWorkspacesInfoError, WorkspaceInfoItem, client::tests::utils::client_factory,
};

#[parsec_test(testbed = "coolorg")]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let realm1_id = VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap();
    let realm2_id = VlobID::from_hex("8fbd7de1e2a04b0f9e1b6c8f3e0a5d42").unwrap();
    let realm1_users_per_role: HashMap<RealmRole, u64> = [
        (RealmRole::Owner, 1),
        (RealmRole::Manager, 2),
        (RealmRole::Contributor, 3),
        (RealmRole::Reader, 4),
    ]
    .into_iter()
    .collect();
    let realm2_users_per_role: HashMap<RealmRole, u64> = [
        (RealmRole::Owner, 1),
        (RealmRole::Manager, 0),
        (RealmRole::Contributor, 0),
        (RealmRole::Reader, 0),
    ]
    .into_iter()
    .collect();

    test_register_send_hook(&env.discriminant_dir, {
        let realm1_users_per_role = realm1_users_per_role.clone();
        let realm2_users_per_role = realm2_users_per_role.clone();
        move |_req: authenticated_cmds::latest::realm_list_info::Req| {
            authenticated_cmds::latest::realm_list_info::Rep::Ok {
                realms: vec![
                    authenticated_cmds::latest::realm_list_info::RealmInfoItem {
                        realm_id: realm1_id,
                        metadata_size: 1024,
                        data_size: 2048,
                        users_per_role: realm1_users_per_role,
                    },
                    authenticated_cmds::latest::realm_list_info::RealmInfoItem {
                        realm_id: realm2_id,
                        metadata_size: 0,
                        data_size: 0,
                        users_per_role: realm2_users_per_role,
                    },
                ],
            }
        }
    });

    let workspaces = client.list_workspaces_info().await.unwrap();

    p_assert_eq!(
        workspaces,
        vec![
            WorkspaceInfoItem {
                realm_id: realm1_id,
                metadata_size: 1024,
                data_size: 2048,
                users_per_role: realm1_users_per_role,
            },
            WorkspaceInfoItem {
                realm_id: realm2_id,
                metadata_size: 0,
                data_size: 0,
                users_per_role: realm2_users_per_role,
            },
        ]
    );
}

#[parsec_test(testbed = "coolorg")]
async fn ok_no_workspaces(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        |_req: authenticated_cmds::latest::realm_list_info::Req| {
            authenticated_cmds::latest::realm_list_info::Rep::Ok { realms: vec![] }
        },
    );

    let workspaces = client.list_workspaces_info().await.unwrap();
    p_assert_eq!(workspaces, vec![]);
}

#[parsec_test(testbed = "coolorg")]
async fn author_not_allowed(env: &TestbedEnv) {
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    test_register_send_hook(
        &env.discriminant_dir,
        |_req: authenticated_cmds::latest::realm_list_info::Req| {
            authenticated_cmds::latest::realm_list_info::Rep::AuthorNotAllowed
        },
    );

    let err = client.list_workspaces_info().await.unwrap_err();
    p_assert_matches!(err, ClientListWorkspacesInfoError::AuthorNotAllowed);
}

#[parsec_test(testbed = "coolorg")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client.list_workspaces_info().await.unwrap_err();
    p_assert_matches!(err, ClientListWorkspacesInfoError::Offline(_));
}

#[parsec_test(testbed = "coolorg")]
async fn unknown_server_response(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        |_req: authenticated_cmds::latest::realm_list_info::Req| {
            authenticated_cmds::latest::realm_list_info::Rep::UnknownStatus {
                unknown_status: "".into(),
                reason: None,
            }
        },
    );

    let err = client.list_workspaces_info().await.unwrap_err();
    p_assert_matches!(err, ClientListWorkspacesInfoError::Internal(_));
}
