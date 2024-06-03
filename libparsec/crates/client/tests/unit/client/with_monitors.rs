// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    workspace::EntryStat, Client, ClientConfig, EventBus, MountpointMountStrategy, WorkspaceInfo,
    WorkspaceStorageCacheSize,
};

#[ignore]
#[parsec_test(testbed = "coolorg", with_server)]
async fn multi_devices(env: &TestbedEnv) {
    let alice1 = env.local_device("alice@dev1");
    let alice2 = env.local_device("alice@dev2");

    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: libparsec_client_connection::ProxyConfig::default(),
        with_monitors: true,
    });
    let alice1_event_bus = EventBus::default();
    let alice2_event_bus = EventBus::default();

    // 1) Start Alice clients

    let alice1_client = Client::start(config.clone(), alice1_event_bus, alice1)
        .await
        .unwrap();
    let alice2_client = Client::start(config, alice2_event_bus, alice2.clone())
        .await
        .unwrap();

    // 2a) Alice1 creates a workspace...

    let wid = alice1_client
        .create_workspace("new workspace".parse().unwrap())
        .await
        .unwrap();

    // 2b) ...then Alice2 eventually gets notified about it

    // TODO: use event instead of this ugly polling loop !
    loop {
        let workspaces = alice2_client.list_workspaces().await;
        let found = workspaces.into_iter().find(|entry| entry.id == wid);
        if let Some(entry) = found {
            let WorkspaceInfo {
                id,
                current_name,
                current_self_role,
                is_started,
                is_bootstrapped,
            } = entry;
            p_assert_eq!(id, wid);
            p_assert_eq!(current_name, "new workspace".parse().unwrap());
            p_assert_eq!(current_self_role, RealmRole::Owner);
            p_assert_eq!(is_started, false);
            p_assert_eq!(is_bootstrapped, true);
            break;
        }
        // Not found, wait a bit and retry
        libparsec_platform_async::sleep(std::time::Duration::from_millis(50)).await;
    }

    // 3a) Alice1 creates a new folder in the workspace...

    let alice1_workspace = alice1_client.start_workspace(wid).await.unwrap();
    let foo_id = alice1_workspace
        .create_folder("/foo".parse().unwrap())
        .await
        .unwrap();

    // 3b) ...then Alice can start it workspace and access it too !

    let alice2_workspace = alice2_client.start_workspace(wid).await.unwrap();
    loop {
        let children_stats = alice2_workspace
            .stat_folder_children(&"/".parse().unwrap())
            .await
            .unwrap();
        let children: Vec<_> = children_stats
            .into_iter()
            .map(|(name, stat)| {
                let id = match stat {
                    EntryStat::File { id, .. } => id,
                    EntryStat::Folder { id, .. } => id,
                };
                (name, id)
            })
            .collect();
        if children == [("foo".parse().unwrap(), foo_id)] {
            break;
        }
        // Not found, wait a bit and retry
        libparsec_platform_async::sleep(std::time::Duration::from_millis(50)).await;
    }

    // 4a) Now time for Alice2 to modify the workspace while Alice1 workspace is running...

    alice2_workspace
        .remove_entry("/foo".parse().unwrap())
        .await
        .unwrap();

    // 4b) ...and for Alice1 to receive it !

    loop {
        let children_stats = alice2_workspace
            .stat_folder_children(&"/".parse().unwrap())
            .await
            .unwrap();
        if children_stats.is_empty() {
            break;
        }
        // Not found, wait a bit and retry
        libparsec_platform_async::sleep(std::time::Duration::from_millis(50)).await;
    }

    alice1_client.stop().await;
    alice2_client.stop().await;
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn sharing(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");

    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: libparsec_client_connection::ProxyConfig::default(),
        with_monitors: true,
    });
    let alice_event_bus = EventBus::default();
    let bob_event_bus = EventBus::default();

    // 1) Start Alice & Bob clients

    let alice_client = Client::start(config.clone(), alice_event_bus, alice.clone())
        .await
        .unwrap();
    let bob_client = Client::start(config, bob_event_bus, bob.clone())
        .await
        .unwrap();

    // 2a) Alice creates and share a workspace with Bob...

    let wid = alice_client
        .create_workspace("new workspace".parse().unwrap())
        .await
        .unwrap();
    alice_client
        .share_workspace(wid, bob.user_id, Some(RealmRole::Owner))
        .await
        .unwrap();

    // 2b) ...then Bob eventually gets notified about it

    // TODO: use event instead of this ugly polling loop !
    loop {
        let workspaces = bob_client.list_workspaces().await;
        let found = workspaces.into_iter().find(|entry| entry.id == wid);
        if let Some(entry) = found {
            let WorkspaceInfo {
                id,
                current_name,
                current_self_role,
                is_started,
                is_bootstrapped,
            } = entry;
            p_assert_eq!(id, wid);
            p_assert_eq!(current_name, "new workspace".parse().unwrap());
            p_assert_eq!(current_self_role, RealmRole::Owner);
            p_assert_eq!(is_started, false);
            p_assert_eq!(is_bootstrapped, true);
            break;
        }
        // Not found, wait a bit and retry
        libparsec_platform_async::sleep(std::time::Duration::from_millis(50)).await;
    }

    // 3a) Now Bob betrays Alice and unshare the workspace with her...

    bob_client
        .share_workspace(wid, alice.user_id, None)
        .await
        .unwrap();

    // 3b) ...poor Alice, she is so surprised ϞϞ(๑O ○ O๑)

    loop {
        let workspaces = alice_client.list_workspaces().await;
        let found = workspaces.into_iter().find(|entry| entry.id == wid);
        if found.is_none() {
            break;
        }
        // Workspace still present, wait a bit and retry
        libparsec_platform_async::sleep(std::time::Duration::from_millis(50)).await;
    }

    alice_client.stop().await;
    bob_client.stop().await;
}
