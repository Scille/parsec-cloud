// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client::{
    certificates_ops::CertificatesOps, user_ops::UserOps, ClientConfig, EventBus,
    WorkspaceStorageCacheSize,
};
use libparsec_client_connection::{AuthenticatedCmds, ProxyConfig};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

pub(crate) async fn user_ops_factory(env: &TestbedEnv, device: &Arc<LocalDevice>) -> UserOps {
    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_base_dir: env.discriminant_dir.clone(),
        preferred_org_creation_backend_addr: "parsec://example.com".parse().unwrap(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
    });
    let event_bus = EventBus::default();
    let cmds = Arc::new(
        AuthenticatedCmds::new(&config.config_dir, device.clone(), config.proxy.clone()).unwrap(),
    );
    let certificates_ops = Arc::new(
        CertificatesOps::start(
            config.clone(),
            device.clone(),
            event_bus.clone(),
            cmds.clone(),
        )
        .await
        .unwrap(),
    );
    UserOps::start(
        config.clone(),
        device.clone(),
        cmds,
        certificates_ops,
        event_bus,
    )
    .await
    .unwrap()
}
