// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use crate::{
    CertificateOps, ClientConfig, EventBus, MountpointMountStrategy, WorkspaceHistoryOps,
    WorkspaceHistoryRealmExportDecryptor, WorkspaceStorageCacheSize,
};
use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_realm_get_keys_bundle,
    test_send_hook_vlob_read_batch, test_send_hook_vlob_read_versions, AuthenticatedCmds,
    ProxyConfig,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

fn config_factory(env: &TestbedEnv) -> Arc<ClientConfig> {
    Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::empty(),
    })
}

pub(crate) async fn workspace_history_ops_with_server_access_factory(
    env: &TestbedEnv,
    device: &Arc<LocalDevice>,
    realm_id: VlobID,
) -> WorkspaceHistoryOps {
    let config = config_factory(env);
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

    WorkspaceHistoryOps::start_with_server_access(
        config.clone(),
        cmds,
        certificates_ops,
        device.organization_id().to_owned(),
        realm_id,
    )
    .await
    .unwrap()
}

pub(crate) async fn workspace_history_ops_with_realm_export_access_factory(
    env: &TestbedEnv,
    decryptors: &[&str],
    relative_original_export_db_path: &str,
) -> (WorkspaceHistoryOps, TmpPath) {
    // Retrieve the realm export database

    let exe = std::env::current_exe().unwrap();
    let mut path = exe.as_path();
    let original_export_db_path = loop {
        path = match path.parent() {
            None => panic!(
                "Cannot find `{:?}` while crawling up from `{:?}`",
                relative_original_export_db_path, exe
            ),
            Some(path) => path,
        };
        let candidate = path.join(relative_original_export_db_path);
        if candidate.exists() {
            break candidate;
        }
    };

    // Copy the realm export database since SQLite modifies the file in place
    // even when only doing read operations.

    let tmp_path = TmpPath::create();
    let export_db_path = tmp_path.join(original_export_db_path.file_name().unwrap());
    std::fs::copy(original_export_db_path, &export_db_path).unwrap();

    let decryptors = decryptors
        .iter()
        .map(|decryptor| {
            if let Ok(device_id) = decryptor.parse::<DeviceID>() {
                let local_device = env.local_device(device_id);
                return WorkspaceHistoryRealmExportDecryptor::User {
                    user_id: local_device.user_id,
                    private_key: Box::new(local_device.private_key.clone()),
                };
            }

            let maybe_sequester_service_id = match *decryptor {
                "sequester_service_1" => Some(*env.template.get_stuff("sequester_service_1_id")),
                "sequester_service_2" => Some(*env.template.get_stuff("sequester_service_2_id")),
                _ => None,
            };

            if let Some(sequester_service_id) = maybe_sequester_service_id {
                let private_key = env
                    .template
                    .events
                    .iter()
                    .find_map(|e| match e {
                        TestbedEvent::NewSequesterService(e) if e.id == sequester_service_id => {
                            Some(Box::new(e.encryption_private_key.clone()))
                        }
                        _ => None,
                    })
                    .unwrap();
                return WorkspaceHistoryRealmExportDecryptor::SequesterService {
                    sequester_service_id,
                    private_key,
                };
            }

            panic!(
                "Invalid decryptor `{}` (expected e.g. `alice@dev1` or `sequester_service_1`)",
                decryptor
            );
        })
        .collect();

    let config = config_factory(env);
    let ops = WorkspaceHistoryOps::start_with_realm_export(config, &export_db_path, decryptors)
        .await
        .unwrap();

    (ops, tmp_path)
}

pub enum DataAccessStrategy {
    RealmExport,
    Server,
}

pub struct WorkspaceHistoryOpsWithMaybeTmpPath {
    ops: WorkspaceHistoryOps,
    /// Must be kept here since the temporary path is removed on drop
    _tmp_path: Option<TmpPath>,
}

impl WorkspaceHistoryOpsWithMaybeTmpPath {
    pub fn tmp_path(&self) -> Option<&Path> {
        self._tmp_path.as_ref().map(|x| x.as_path())
    }
}

impl std::ops::Deref for WorkspaceHistoryOpsWithMaybeTmpPath {
    type Target = WorkspaceHistoryOps;
    fn deref(&self) -> &Self::Target {
        &self.ops
    }
}

impl std::ops::DerefMut for WorkspaceHistoryOpsWithMaybeTmpPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ops
    }
}

impl DataAccessStrategy {
    pub async fn start_workspace_history_ops_at(
        &self,
        env: &TestbedEnv,
        timestamp: DateTime,
    ) -> WorkspaceHistoryOpsWithMaybeTmpPath {
        let ops = self.start_workspace_history_ops(env).await;

        if matches!(self, DataAccessStrategy::Server) {
            let realm_id = ops.realm_id();
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                // Get back `/` manifest
                test_send_hook_vlob_read_batch!(env, at: timestamp, realm_id, realm_id),
                // Note workspace key bundle has already been loaded at workspace history ops startup
            );
        }
        ops.set_timestamp_of_interest(timestamp).await.unwrap();

        ops
    }

    pub async fn start_workspace_history_ops(
        &self,
        env: &TestbedEnv,
    ) -> WorkspaceHistoryOpsWithMaybeTmpPath {
        match self {
            DataAccessStrategy::RealmExport => {
                let (ops, tmp_path) = workspace_history_ops_with_realm_export_access_factory(
                    env,
                    &["alice@dev1"],
                    "server/tests/realm_export/workspace_history_export.sqlite",
                )
                .await;
                WorkspaceHistoryOpsWithMaybeTmpPath {
                    ops,
                    _tmp_path: Some(tmp_path),
                }
            }

            DataAccessStrategy::Server => {
                let alice = env.local_device("alice@dev1");
                let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
                let ops =
                    workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;
                WorkspaceHistoryOpsWithMaybeTmpPath {
                    ops,
                    _tmp_path: None,
                }
            }
        }
    }
}
