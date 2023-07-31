// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

pub use libparsec_client::{ProxyConfig, WorkspaceStorageCacheSize};
pub use libparsec_types::BackendAddr;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub config_dir: PathBuf,
    pub data_base_dir: PathBuf,
    pub mountpoint_base_dir: PathBuf, // Ignored on web
    pub workspace_storage_cache_size: WorkspaceStorageCacheSize,
}

impl From<ClientConfig> for Arc<libparsec_client::ClientConfig> {
    fn from(config: ClientConfig) -> Self {
        Arc::new(config.into())
    }
}

impl From<ClientConfig> for libparsec_client::ClientConfig {
    fn from(config: ClientConfig) -> Self {
        Self {
            config_dir: config.config_dir,
            data_base_dir: config.data_base_dir,
            #[cfg(not(target_arch = "wasm32"))]
            mountpoint_base_dir: config.mountpoint_base_dir,
            #[cfg(target_arch = "wasm32")]
            mountpoint_base_dir: PathBuf::default(),
            workspace_storage_cache_size: config.workspace_storage_cache_size,
            proxy: ProxyConfig::default(),
        }
    }
}
