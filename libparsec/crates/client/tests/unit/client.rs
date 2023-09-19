// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_client_connection::{protocol, test_register_send_hook, ProxyConfig};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Client, ClientConfig, EventBus, WorkspaceStorageCacheSize};

async fn client_factory(discriminant_dir: &Path, device: Arc<LocalDevice>) -> Client {
    let event_bus = EventBus::default();
    let config = Arc::new(ClientConfig {
        config_dir: discriminant_dir.to_owned(),
        data_base_dir: discriminant_dir.to_owned(),
        mountpoint_base_dir: discriminant_dir.to_owned(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
    });
    Client::start(config, event_bus, device).await.unwrap()
}

#[parsec_test(testbed = "minimal")]
async fn basic_info(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_eq!(client.device_id(), &"alice@dev1".parse().unwrap());
    p_assert_eq!(
        client.device_slug().as_str(),
        "9ff2284ce2#OfflineOrg#alice@dev1"
    );
    p_assert_eq!(
        client.device_slughash().as_str(),
        "7542104c696b8f7c3472f6ae46c63162208e102b36cf2e19d710f37bbfe8bcbb"
    );
    p_assert_eq!(
        client.device_label(),
        Some(&"My dev1 machine".parse().unwrap())
    );
    // Cannot compare between `HumanHandle` as it ignores `label` field
    p_assert_matches!(client.human_handle(), Some(handle) if handle.label() == "Alicey McAliceFace" && handle.email() == "alice@example.com");
    p_assert_eq!(client.organization_id(), &"OfflineOrg".parse().unwrap());
    p_assert_matches!(client.profile().await.unwrap(), UserProfile::Admin);

    client.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn base(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        |_req: protocol::authenticated_cmds::latest::realm_create::Req| {
            protocol::authenticated_cmds::latest::realm_create::Rep::Ok
        },
    );

    let wid = client
        .user_ops
        .workspace_create("wksp1".parse().unwrap())
        .await
        .unwrap();

    client
        .user_ops
        .workspace_rename(wid, "wksp1'".parse().unwrap())
        .await
        .unwrap();

    // let bob = env.local_device("bob@dev1".parse().unwrap());
    // client.user_ops.workspace_share(&wid, &bob, Some(RealmRole::Reader), None).await.unwrap();

    // client.user_ops.sync().await.unwrap();

    client.stop().await;
}
