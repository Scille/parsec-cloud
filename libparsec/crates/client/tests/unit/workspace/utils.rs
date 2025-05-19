// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_client_connection::{AuthenticatedCmds, ProxyConfig};
use libparsec_types::prelude::*;

use crate::{
    ClientConfig, EventBus, MountpointMountStrategy, WorkspaceStorageCacheSize,
    certif::CertificateOps,
    workspace::{LocalUserManifestWorkspaceEntry, WorkspaceExternalInfo, WorkspaceOps},
};

pub(crate) async fn workspace_ops_factory(
    discriminant_dir: &Path,
    device: &Arc<LocalDevice>,
    realm_id: VlobID,
) -> WorkspaceOps {
    workspace_ops_with_prevent_sync_pattern_factory(
        discriminant_dir,
        device,
        realm_id,
        PreventSyncPattern::from_regex(r"\.tmp$").unwrap(),
    )
    .await
}

pub(crate) async fn workspace_ops_with_prevent_sync_pattern_factory(
    discriminant_dir: &Path,
    device: &Arc<LocalDevice>,
    realm_id: VlobID,
    prevent_sync_pattern: PreventSyncPattern,
) -> WorkspaceOps {
    let config = Arc::new(ClientConfig {
        config_dir: discriminant_dir.to_owned(),
        data_base_dir: discriminant_dir.to_owned(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        with_monitors: false,
        prevent_sync_pattern,
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
    WorkspaceOps::start(
        config.clone(),
        device.clone(),
        cmds,
        certificates_ops,
        event_bus,
        realm_id,
        WorkspaceExternalInfo {
            entry: LocalUserManifestWorkspaceEntry {
                id: realm_id,
                name: "wksp1".parse().unwrap(),
                name_origin: CertificateBasedInfoOrigin::Placeholder,
                role: RealmRole::Owner,
                role_origin: CertificateBasedInfoOrigin::Placeholder,
            },
            workspace_index: 0,
            total_workspaces: 1,
        },
    )
    .await
    .unwrap()
}

pub(crate) async fn restart_workspace_ops(ops: WorkspaceOps) -> WorkspaceOps {
    let config = ops.config.clone();
    let device = ops.device.clone();
    let cmds = ops.cmds.clone();
    let certificates_ops = ops.certificates_ops.clone();
    let realm_id = ops.realm_id;
    let event_bus = EventBus::default();

    ops.stop().await.unwrap();

    WorkspaceOps::start(
        config,
        device,
        cmds,
        certificates_ops,
        event_bus,
        realm_id,
        WorkspaceExternalInfo {
            entry: LocalUserManifestWorkspaceEntry {
                id: realm_id,
                name: "wksp1".parse().unwrap(),
                name_origin: CertificateBasedInfoOrigin::Placeholder,
                role: RealmRole::Owner,
                role_origin: CertificateBasedInfoOrigin::Placeholder,
            },
            workspace_index: 0,
            total_workspaces: 1,
        },
    )
    .await
    .unwrap()
}

macro_rules! ls {
    ($ops:expr, $path:expr) => {
        async {
            let path = $path.parse().unwrap();
            let children_stats = $ops.stat_folder_children(&path).await.unwrap();
            let mut children_names = children_stats
                .into_iter()
                .map(|(entry_name, _)| entry_name.to_string())
                .collect::<Vec<_>>();
            children_names.sort();
            children_names
        }
    };
}

macro_rules! assert_ls {
    ($ops:expr, $path:expr, $expected:expr) => {
        async {
            let children = ls!($ops, $path).await;
            // Must specify `$expected` type to handle empty array
            let expected: &[&str] = &$expected;
            p_assert_eq!(children, expected);
        }
    };
}

macro_rules! assert_ls_with_id {
    ($ops:expr, $path:expr, $expected:expr) => {
        async {
            let children = {
                let path = $path.parse().unwrap();
                let children_stats = $ops.stat_folder_children(&path).await.unwrap();
                let mut children = children_stats
                    .into_iter()
                    .map(|(entry_name, entry_stat)| (entry_name.to_string(), entry_stat.id()))
                    .collect::<Vec<_>>();
                children.sort_by(|a, b| a.0.cmp(&b.0));
                children
            };
            // Must specify `$expected` type to handle empty array
            let expected: &[(&str, VlobID)] = &$expected;
            let children = children.iter().map(|(name, id)| (name.as_str(), *id)).collect::<Vec<_>>();
            // let expected = expected.into_iter().map(|(name, id)| (name.to_string(), *id)).collect::<Vec<_>>();
            p_assert_eq!(children, expected);
        }
    };
}

pub(super) use assert_ls;
pub(super) use assert_ls_with_id;
pub(super) use ls;
