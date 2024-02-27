// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_client_connection::{AuthenticatedCmds, ProxyConfig};
use libparsec_types::prelude::*;

use crate::{
    certif::CertifOps,
    workspace::{LocalUserManifestWorkspaceEntry, WorkspaceOps},
    ClientConfig, EventBus, WorkspaceStorageCacheSize,
};

pub(crate) async fn workspace_ops_factory(
    discriminant_dir: &Path,
    device: &Arc<LocalDevice>,
    realm_id: VlobID,
) -> WorkspaceOps {
    let config = Arc::new(ClientConfig {
        config_dir: discriminant_dir.to_owned(),
        data_base_dir: discriminant_dir.to_owned(),
        mountpoint_base_dir: discriminant_dir.to_owned(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        with_monitors: false,
    });
    let event_bus = EventBus::default();
    let cmds = Arc::new(
        AuthenticatedCmds::new(&config.config_dir, device.clone(), config.proxy.clone()).unwrap(),
    );
    let certificates_ops = Arc::new(
        CertifOps::start(
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
        LocalUserManifestWorkspaceEntry {
            id: realm_id,
            name: "wksp1".parse().unwrap(),
            name_origin: CertificateBasedInfoOrigin::Placeholder,
            role: RealmRole::Owner,
            role_origin: CertificateBasedInfoOrigin::Placeholder,
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
        LocalUserManifestWorkspaceEntry {
            id: realm_id,
            name: "wksp1".parse().unwrap(),
            name_origin: CertificateBasedInfoOrigin::Placeholder,
            role: RealmRole::Owner,
            role_origin: CertificateBasedInfoOrigin::Placeholder,
        },
    )
    .await
    .unwrap()
}

macro_rules! ls {
    ($ops:expr, $path:expr) => {
        async {
            let path = $path.parse().unwrap();
            let info = $ops.stat_entry(&path).await.unwrap();
            let children = match info {
                crate::workspace::EntryStat::Folder { children, .. } => children,
                x => panic!("Bad info type: {:?}", x),
            };
            children
                .iter()
                .map(|(entry_name, _)| entry_name.to_string())
                .collect::<Vec<_>>()
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
pub(super) use assert_ls;
pub(super) use ls;
