// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_client_connection::ProxyConfig;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Client, ClientConfig, EventBus, MountpointMountStrategy, WorkspaceStorageCacheSize};

/// Create a client for the given device WITHOUT monitors (i.e. the client has
/// no background task reacting to events, which is pretty useful for testing
/// given we don't want concurrency operations)
pub(crate) async fn client_factory(
    discriminant_dir: &Path,
    device: Arc<LocalDevice>,
) -> Arc<Client> {
    let event_bus = EventBus::default();
    let config = Arc::new(ClientConfig {
        config_dir: discriminant_dir.to_owned(),
        data_base_dir: discriminant_dir.to_owned(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::empty(),
    });
    Client::start(config, event_bus, device).await.unwrap()
}

pub(crate) fn make_config(env: &TestbedEnv) -> Arc<ClientConfig> {
    Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::from_regex(r"\.tmp$").unwrap(),
    })
}
