// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::{ClientCreateWorkspaceError, EventWorkspaceLocallyCreated, WorkspaceInfo};

#[parsec_test(testbed = "minimal")]
async fn ok(#[values(false, true)] restart_client: bool, env: &TestbedEnv) {
    let env = &env.customize(|builder| {
        builder.certificates_storage_fetch_certificates("alice@dev1");
    });

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // 1) Create the workspace in local

    let mut spy = client.event_bus.spy.start_expecting();

    let wid = client
        .create_workspace("wksp1".parse().unwrap())
        .await
        .unwrap();

    spy.wait_and_assert_next(|event: &EventWorkspaceLocallyCreated| {
        p_assert_eq!(event.realm_id, wid);
        p_assert_eq!(event.name, "wksp1".parse().unwrap());
    })
    .await;

    // 2) Ensure the workspace is present and not yet synchronized

    let maybe_restart_client = |client: Arc<crate::Client>| async move {
        if restart_client {
            client.stop().await;
            let alice = env.local_device("alice@dev1");
            client_factory(&env.discriminant_dir, alice).await
        } else {
            client
        }
    };

    macro_rules! check_list_workspace {
        ($client:expr, $expected_is_bootstrapped:expr) => {
            async {
                let mut workspaces = $client.list_workspaces().await;
                p_assert_eq!(workspaces.len(), 1);
                let WorkspaceInfo {
                    id,
                    name,
                    self_current_role,
                    is_started,
                    is_bootstrapped,
                } = workspaces.pop().unwrap();
                p_assert_eq!(id, wid);
                p_assert_eq!(name, "wksp1".parse().unwrap());
                p_assert_eq!(self_current_role, RealmRole::Owner);
                p_assert_eq!(is_started, false);
                p_assert_eq!(is_bootstrapped, $expected_is_bootstrapped);
            }
        };
    }

    let client = maybe_restart_client(client).await;
    check_list_workspace!(client, false).await;
}

#[parsec_test(testbed = "minimal")]
async fn duplicated_name_is_allowed(
    #[values(false, true)] previous_is_bootstrapped: bool,
    env: &TestbedEnv,
) {
    let common_name: EntryName = "common_wksp_name".parse().unwrap();
    let (env, wksp1_id) = env.customize_with_map(|builder| {
        let wksp1_id = if previous_is_bootstrapped {
            let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);
            builder.rotate_key_realm(wksp1_id);
            builder.rename_realm(wksp1_id, common_name.clone());
            wksp1_id
        } else {
            VlobID::default()
        };

        builder.certificates_storage_fetch_certificates("alice@dev1");
        if previous_is_bootstrapped {
            builder
                .user_storage_local_update("alice@dev1")
                .update_local_workspaces_with_fetched_certificates();
        } else {
            builder
                .user_storage_local_update("alice@dev1")
                .add_or_update_placeholder(wksp1_id, common_name.clone());
        }

        wksp1_id
    });

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // 1) Create the workspace in local

    let mut spy = client.event_bus.spy.start_expecting();

    let wksp2_id = client.create_workspace(common_name.clone()).await.unwrap();

    spy.wait_and_assert_next(|event: &EventWorkspaceLocallyCreated| {
        p_assert_eq!(event.realm_id, wksp2_id);
        p_assert_eq!(event.name, common_name.clone());
    })
    .await;

    // 2) Check the list of workspaces

    let workspaces = client.list_workspaces().await;
    p_assert_eq!(workspaces.len(), 2);

    // wksp1
    {
        let WorkspaceInfo {
            id,
            name,
            self_current_role,
            is_started,
            is_bootstrapped,
        } = &workspaces[0];
        p_assert_eq!(*id, wksp1_id);
        p_assert_eq!(name, &common_name);
        p_assert_eq!(*self_current_role, RealmRole::Owner);
        p_assert_eq!(*is_started, false);
        p_assert_eq!(*is_bootstrapped, previous_is_bootstrapped);
    }

    // wksp1
    {
        let WorkspaceInfo {
            id,
            name,
            self_current_role,
            is_started,
            is_bootstrapped,
        } = &workspaces[1];
        p_assert_eq!(*id, wksp2_id);
        p_assert_eq!(name, &common_name);
        p_assert_eq!(*self_current_role, RealmRole::Owner);
        p_assert_eq!(*is_started, false);
        p_assert_eq!(*is_bootstrapped, false);
    }
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.user_ops.stop().await.unwrap();

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .create_workspace("wksp".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientCreateWorkspaceError::Stopped);
    spy.assert_no_events();
}
