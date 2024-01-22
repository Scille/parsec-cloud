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
    realm_key: SecretKey,
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
