// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client::{
    Client, ClientConfig, EventBus, MountpointMountStrategy, ProxyConfig, WorkspaceStorageCacheSize,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::Mountpoint;

#[parsec_test(testbed = "minimal_client_ready")]
async fn mount_on_drive(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");

    let event_bus = EventBus::default();
    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_mount_strategy: MountpointMountStrategy::DriveLetter,
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        with_monitors: false,
    });
    let client = Client::start(config, event_bus, alice).await.unwrap();

    let wksp1_ops = client.start_workspace(wksp1_id).await.unwrap();

    let mountpoint_path = {
        // 1) Mount the workspace

        let mountpoint = Mountpoint::mount(wksp1_ops.clone()).await.unwrap();
        let mountpoint_path = mountpoint.path().to_owned();

        // 2) Ensure the mount is on a drive letter

        let mut mountpoint_path_components = mountpoint_path.components();
        p_assert_matches!(
            mountpoint_path_components.next(),
            Some(std::path::Component::Prefix(prefix))
            if matches!(prefix.kind(), std::path::Prefix::Disk(_))
        );
        p_assert_matches!(mountpoint_path_components.next(), None);

        // 3) Ensure workspace data can be accessed

        tokio::fs::try_exists(&mountpoint_path).await.unwrap();
        tokio::fs::try_exists(&mountpoint_path.join("foo/egg.txt"))
            .await
            .unwrap();

        mountpoint.unmount().await.unwrap();

        mountpoint_path
    };

    // 3) Ensure mountpoint disappeared once unmounted

    tokio::fs::try_exists(&mountpoint_path).await.unwrap();

    client.stop().await;
}
