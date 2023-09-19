// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod entry_transactions;
mod fetch;

pub use entry_transactions::*;

use std::sync::Arc;

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_storage::workspace::{WorkspaceCacheStorage, WorkspaceDataStorage};
use libparsec_types::prelude::*;

use crate::{certificates_ops::CertificatesOps, event_bus::EventBus, ClientConfig};

// enum WorkspaceOpsState {
//     /// Default mode: read & write operations are allowed
//     ReadWrite,
//     /// User has a reduced role in this workspace: only read operations are allowed
//     ReadOnly,
//     /// The workspace is under reencryption: only read operations are allowed
//     WaitForReencryption,
//     HelpWithReencryption,
// }

#[derive(Debug)]
pub struct WorkspaceOps {
    #[allow(unused)]
    config: Arc<ClientConfig>,
    #[allow(unused)]
    device: Arc<LocalDevice>,
    data_storage: WorkspaceDataStorage,
    cache_storage: WorkspaceCacheStorage,
    #[allow(unused)]
    cmds: Arc<AuthenticatedCmds>,
    #[allow(unused)]
    certificates_ops: Arc<CertificatesOps>,
    #[allow(unused)]
    event_bus: EventBus,
    realm_id: VlobID,
    realm_key: SecretKey,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceOpsError {
    #[error("Unknown workspace `{0}`")]
    UnknownWorkspace(VlobID),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct ReencryptionJob {}

impl WorkspaceOps {
    pub async fn start(
        config: Arc<ClientConfig>,
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertificatesOps>,
        event_bus: EventBus,
        realm_id: VlobID,
        realm_key: SecretKey,
    ) -> Result<Self, anyhow::Error> {
        // TODO: handle errors
        let data_storage =
            WorkspaceDataStorage::start(&config.data_base_dir, device.clone(), realm_id).await?;
        let cache_storage = WorkspaceCacheStorage::start(
            &config.data_base_dir,
            config.workspace_storage_cache_size.cache_size(),
            device.clone(),
            realm_id,
        )
        .await?;
        Ok(Self {
            config,
            device,
            data_storage,
            cache_storage,
            cmds,
            certificates_ops,
            event_bus,
            realm_id,
            realm_key,
        })
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        // TODO: is the storages teardown order important ?
        self.data_storage
            .stop()
            .await
            .context("Cannot stop data storage")?;
        self.cache_storage.stop().await;
        Ok(())
    }

    pub async fn entry_info(&self, path: &FsPath) -> anyhow::Result<EntryInfo> {
        entry_transactions::entry_info(self, path).await
    }
}

#[cfg(test)]
#[path = "../../tests/unit/workspace_ops/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
