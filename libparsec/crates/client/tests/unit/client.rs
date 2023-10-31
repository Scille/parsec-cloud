// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_client_connection::{protocol, test_register_send_hook, ProxyConfig};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    workspace_ops::{EntryStat, FsOperationError},
    Client, ClientConfig, EventBus, WorkspaceStorageCacheSize,
};

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
    p_assert_eq!(client.device_label(), &"My dev1 machine".parse().unwrap());
    // Cannot compare between `HumanHandle` as it ignores `label` field
    p_assert_matches!(client.human_handle(), handle if handle.label() == "Alicey McAliceFace" && handle.email() == "alice@example.com");
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
        .create_workspace("wksp1".parse().unwrap())
        .await
        .unwrap();

    client
        .user_ops
        .rename_workspace(wid, "wksp1'".parse().unwrap())
        .await
        .unwrap();

    // let bob = env.local_device("bob@dev1".parse().unwrap());
    // client.user_ops.workspace_share(&wid, &bob, Some(RealmRole::Reader), None).await.unwrap();

    // client.user_ops.sync().await.unwrap();

    client.stop().await;
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn inbound_outbound_sync(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    // TODO: this test cannot mock time provider given it communicates with the server
    // (which has it own non-mocked clock !), we should be able to disable server
    // ballpark check for this kind of tests.
    // TODO: use env global time provider instead of mocking each device independently
    // alice.time_provider.mock_time_faster(1000.);
    // bob.time_provider.mock_time_faster(1000.);
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice_client = client_factory(&env.discriminant_dir, alice).await;
    let bob_client = client_factory(&env.discriminant_dir, bob).await;

    let alice_wksp1_ops = alice_client.start_workspace(wksp1_id).await.unwrap();
    let bob_wksp1_ops = bob_client.start_workspace(wksp1_id).await.unwrap();

    alice_wksp1_ops
        .create_folder(&"/hello".parse().unwrap())
        .await
        .unwrap();

    macro_rules! timeout_after_1s {
        ($x:expr) => {{
            let mut attempts = 0;
            loop {
                if $x {
                    break;
                }
                attempts += 1;
                if attempts > 1000 {
                    panic!("Timeout after 1s !");
                }
                libparsec_platform_async::sleep(std::time::Duration::from_millis(10)).await;
            }
        }};
    }

    // Wait for alice to do the outbound sync
    timeout_after_1s!({
        let stat_root = alice_wksp1_ops
            .stat_entry(&"/".parse().unwrap())
            .await
            .unwrap();
        let stat_child = alice_wksp1_ops
            .stat_entry(&"/hello".parse().unwrap())
            .await
            .unwrap();
        let mut need_sync = match stat_root {
            EntryStat::Folder { need_sync, .. } => need_sync,
            stat => panic!("Invalid entry type: {:?}", stat),
        };
        need_sync |= match stat_child {
            EntryStat::Folder { need_sync, .. } => need_sync,
            stat => panic!("Invalid entry type: {:?}", stat),
        };
        !need_sync
    });

    // Wait for bob to do the inbound sync
    timeout_after_1s!({
        match bob_wksp1_ops.stat_entry(&"/hello".parse().unwrap()).await {
            Err(FsOperationError::EntryNotFound) => false,
            err @ Err(_) => panic!("Unexpected error: {:?}", err),
            Ok(stat_child) => {
                let mut need_sync = match stat_child {
                    EntryStat::Folder { need_sync, .. } => need_sync,
                    stat => panic!("Invalid entry type: {:?}", stat),
                };

                let stat_root = bob_wksp1_ops
                    .stat_entry(&"/".parse().unwrap())
                    .await
                    .unwrap();
                need_sync |= match stat_root {
                    EntryStat::Folder {
                        need_sync,
                        children,
                        ..
                    } => {
                        if !need_sync {
                            assert!(children.contains(&"hello".parse().unwrap()));
                        }
                        need_sync
                    }
                    stat => panic!("Invalid entry type: {:?}", stat),
                };

                !need_sync
            }
        }
    });

    alice_client.stop().await;
    bob_client.stop().await;
}
