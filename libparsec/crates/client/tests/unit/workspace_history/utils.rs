// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use crate::{
    CertificateOps, ClientConfig, EventBus, MountpointMountStrategy, WorkspaceHistoryOps,
    WorkspaceStorageCacheSize,
};
use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_realm_get_keys_bundle,
    test_send_hook_vlob_read_versions, AuthenticatedCmds, ProxyConfig,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

pub(crate) async fn workspace_history_ops_with_server_access_factory(
    env: &TestbedEnv,
    device: &Arc<LocalDevice>,
    realm_id: VlobID,
) -> WorkspaceHistoryOps {
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

    // In server-based mode, `WorkspaceHistoryOps` starts by querying the server to
    // fetch the workspace manifest v1.
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vlob_read_versions!(env, realm_id, (realm_id, 1)),
        test_send_hook_realm_get_keys_bundle!(env, device.user_id, realm_id),
    );

    WorkspaceHistoryOps::start_with_server_access(config.clone(), cmds, certificates_ops, realm_id)
        .await
        .unwrap()
}
