// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

pub use libparsec_client_connection::ProxyConfig;
use libparsec_types::prelude::*;

pub const DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE: u64 = 512 * 1024 * 1024;

#[derive(Debug, Clone, Copy)]
pub enum WorkspaceStorageCacheSize {
    Default,
    // TODO: support arbitrary int size in bindings
    Custom { size: u32 },
}

impl WorkspaceStorageCacheSize {
    pub fn cache_size(&self) -> u64 {
        match &self {
            Self::Default => DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
            Self::Custom { size } => *size as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MountpointMountStrategy {
    Directory {
        base_dir: PathBuf,
    },
    /// Only allowed on Windows
    DriveLetter,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    // On web, `config_dir`&`data_base_dir` are converted into String and
    // used as database name when using IndexedDB API
    pub config_dir: PathBuf,
    pub data_base_dir: PathBuf,
    /// Ignored on web platform
    pub mountpoint_mount_strategy: MountpointMountStrategy,
    pub workspace_storage_cache_size: WorkspaceStorageCacheSize,
    // pub prevent_sync_pattern: Option<PathBuf>,
    pub proxy: ProxyConfig,
    /// If `false`, nothing runs & react in the background, useful for tests
    /// or CLI where the client is started to only perform a single operation.
    pub with_monitors: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    pub user_profile_outsider_allowed: bool,
    pub active_users_limit: ActiveUsersLimit,
}

// It's easy to provide "good enough" default value so that the config is guaranteed
// to be always available \o/
// Note in practice this default config should be overwritten as soon as the client
// connects to the server (server always starts by sending a SSE event about its config).
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            user_profile_outsider_allowed: false,
            active_users_limit: ActiveUsersLimit::NoLimit,
        }
    }
}
