// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

pub use libparsec_client::{ProxyConfig, WorkspaceStorageCacheSize};
pub use libparsec_types::prelude::*;

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
            workspace_storage_cache_size: config.workspace_storage_cache_size,
            proxy: ProxyConfig::default(),
        }
    }
}

pub fn get_default_data_base_dir() -> PathBuf {
    #[cfg(target_arch = "wasm32")]
    {
        PathBuf::from("")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut path = dirs::data_dir().expect("Could not determine base data directory");
        // TODO: temporary name to avoid clashing with stable parsec
        path.push("parsec-v3-alpha");
        path
    }
}

pub fn get_default_config_dir() -> PathBuf {
    #[cfg(target_arch = "wasm32")]
    {
        PathBuf::from("")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // TODO: check MacOS path correspond to Parsec v2
        let mut path = dirs::config_dir().expect("Could not determine base config directory");
        // TODO: temporary name to avoid clashing with stable parsec
        path.push("parsec-v3-alpha");
        path
    }
}

pub fn get_default_mountpoint_base_dir() -> PathBuf {
    #[cfg(target_arch = "wasm32")]
    {
        PathBuf::from("")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut path = dirs::home_dir().expect("Could not determine home directory");
        // TODO: temporary name to avoid clashing with stable parsec
        path.push("Parsec v3 Alpha");
        path
    }
}
