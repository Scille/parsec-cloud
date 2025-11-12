// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

pub use libparsec_client::{MountpointMountStrategy, ProxyConfig, WorkspaceStorageCacheSize};
pub use libparsec_platform_device_loader::{
    get_default_config_dir, get_default_data_base_dir, get_default_mountpoint_base_dir,
    PARSEC_BASE_CONFIG_DIR, PARSEC_BASE_DATA_DIR, PARSEC_BASE_HOME_DIR,
};
use libparsec_types::prelude::*;

pub use log::Level as LogLevel;

/// Partial client configuration.
/// This configuration should be configured by the user before converting it to
/// the proper configuration [`libparsec_client::ClientConfig`] by using default values as fallback.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub config_dir: PathBuf,
    pub data_base_dir: PathBuf,
    pub mountpoint_mount_strategy: MountpointMountStrategy,
    pub workspace_storage_cache_size: WorkspaceStorageCacheSize,
    pub with_monitors: bool,
    /// The prevent sync pattern, if not provided will use a default pattern.
    /// The pattern is formatted like a `.gitignore` file.
    pub prevent_sync_pattern: Option<String>,
    pub log_level: Option<LogLevel>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            config_dir: get_default_config_dir(),
            data_base_dir: get_default_data_base_dir(),
            mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
            workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
            with_monitors: false,
            prevent_sync_pattern: None,
            log_level: None,
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
            prevent_sync_pattern: match config.prevent_sync_pattern {
                Some(custom_glob_ignore) => PreventSyncPattern::from_glob_ignore_file(
                    &custom_glob_ignore,
                )
                .unwrap_or_else(|err| {
                    // Fall back to default pattern if the custom pattern is invalid
                    log::warn!(
                        "Invalid custom prevent sync pattern, falling back to default: {err}"
                    );
                    PreventSyncPattern::default()
                }),
                None => PreventSyncPattern::default(),
            },
        }
    }
}
