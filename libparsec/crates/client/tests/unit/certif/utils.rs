// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{AuthenticatedCmds, ProxyConfig};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    certif::{store::CertificatesStore, CertifOps},
    ClientConfig, EventBus, MountpointMountStrategy, WorkspaceStorageCacheSize,
};

pub(crate) async fn certificates_ops_factory(
    env: &TestbedEnv,
    device: &Arc<LocalDevice>,
) -> CertifOps {
    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        with_monitors: false,
    });
    let event_bus = EventBus::default();
    let cmds = Arc::new(
        AuthenticatedCmds::new(&config.config_dir, device.clone(), config.proxy.clone()).unwrap(),
    );
    CertifOps::start(config.clone(), device.clone(), event_bus, cmds)
        .await
        .unwrap()
}

pub(super) async fn certificates_store_factory(
    env: &TestbedEnv,
    device: &Arc<LocalDevice>,
) -> CertificatesStore {
    CertificatesStore::start(&env.discriminant_dir, device.clone())
        .await
        .unwrap()
}
