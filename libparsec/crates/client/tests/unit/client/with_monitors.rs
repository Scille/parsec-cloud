// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Client, ClientConfig, EventBus, WorkspaceInfo, WorkspaceStorageCacheSize};

// TODO: enable once the monitors are implemented
#[ignore]
#[parsec_test(testbed = "coolorg", with_server)]
async fn sharing(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");

    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: libparsec_client_connection::ProxyConfig::default(),
        with_monitors: true,
    });
    let alice_event_bus = EventBus::default();
    let bob_event_bus = EventBus::default();

    // Start Alice & Bob clients

    let alice_client = Client::start(config.clone(), alice_event_bus, alice)
        .await
        .unwrap();
    let bob_client = Client::start(config, bob_event_bus, bob.clone())
        .await
        .unwrap();

    // Alice creates and share a workspace with Bob...

    let wid = alice_client
        .create_workspace("new workspace".parse().unwrap())
        .await
        .unwrap();
    alice_client
        .share_workspace(
            wid,
            bob.device_id.user_id().to_owned(),
            Some(RealmRole::Manager),
        )
        .await
        .unwrap();

    // ...then Bob eventually gets notified about it

    // TODO: use event instead of this ugly polling loop !
    loop {
        let workspaces = bob_client.list_workspaces().await;
        let found = workspaces.into_iter().find(|entry| entry.id == wid);
        if let Some(entry) = found {
            let WorkspaceInfo {
                id,
                name,
                self_current_role,
                is_started,
                is_bootstrapped,
            } = entry;
            p_assert_eq!(id, wid);
            p_assert_eq!(name, "new workspace".parse().unwrap());
            p_assert_eq!(self_current_role, RealmRole::Manager);
            p_assert_eq!(is_started, false);
            p_assert_eq!(is_bootstrapped, true);
            break;
        }
        // Not found, wait a bit and retry
        libparsec_platform_async::sleep(std::time::Duration::from_millis(100)).await;
    }

    alice_client.stop().await;
    bob_client.stop().await;
}
