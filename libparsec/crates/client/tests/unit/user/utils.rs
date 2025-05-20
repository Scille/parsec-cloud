// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{AuthenticatedCmds, ProxyConfig};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    ClientConfig, EventBus, MountpointMountStrategy, WorkspaceStorageCacheSize,
    certif::CertificateOps, user::UserOps,
};

pub(crate) async fn user_ops_factory(env: &TestbedEnv, device: &Arc<LocalDevice>) -> UserOps {
    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::empty(),
    });
    let event_bus = EventBus::default();
    let cmds = Arc::new(
        AuthenticatedCmds::new(&config.config_dir, device.clone(), config.proxy.clone()).unwrap(),
    );
    let certificates_ops = Arc::new(
        CertificateOps::start(
            config.clone(),
            device.clone(),
            event_bus.clone(),
            cmds.clone(),
        )
        .await
        .unwrap(),
    );
    UserOps::start(&config, device.clone(), cmds, certificates_ops, event_bus)
        .await
        .unwrap()
}
