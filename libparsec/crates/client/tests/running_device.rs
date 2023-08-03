// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use libparsec_client::{Client, ClientConfig, EventBus, WorkspaceStorageCacheSize};
use libparsec_client_connection::{protocol, test_register_send_hook, ProxyConfig};
use libparsec_tests_fixtures::prelude::*;

#[parsec_test(testbed = "minimal")]
async fn base(env: &TestbedEnv) {
    let event_bus = EventBus::default();
    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
    });
    let alice = env.local_device("alice@dev1");
    let client = Client::start(config, event_bus, alice).await.unwrap();

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
