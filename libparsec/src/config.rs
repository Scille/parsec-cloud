// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

pub use libparsec_client::{
    MountpointMountStrategy, ProxyConfig, ServerConfig, WorkspaceStorageCacheSize,
};
pub use libparsec_platform_device_loader::{
    get_default_config_dir, get_default_data_base_dir, get_default_mountpoint_base_dir,
    PARSEC_BASE_CONFIG_DIR, PARSEC_BASE_DATA_DIR, PARSEC_BASE_HOME_DIR,
};
pub use libparsec_types::prelude::*;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub config_dir: PathBuf,
    pub data_base_dir: PathBuf,
    pub mountpoint_mount_strategy: MountpointMountStrategy,
    pub workspace_storage_cache_size: WorkspaceStorageCacheSize,
    pub with_monitors: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            config_dir: get_default_config_dir(),
            data_base_dir: get_default_data_base_dir(),
            mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
            workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
            with_monitors: false,
        }
    }
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
            mountpoint_mount_strategy: config.mountpoint_mount_strategy,
            workspace_storage_cache_size: config.workspace_storage_cache_size,
            proxy: ProxyConfig::default(),
            with_monitors: config.with_monitors,
        }
    }
}
