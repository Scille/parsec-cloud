// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub use libparsec_client::{MountpointMountStrategy, ProxyConfig, WorkspaceStorageCacheSize};
use libparsec_client_connection::{AnonymousServerCmds, ConnectionError};
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

#[derive(Debug, thiserror::Error)]
pub enum GetServerConfigError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub use libparsec_client_connection::protocol::anonymous_server_cmds::latest::server_config::{
    AccountConfig, ClientAgentConfig, OpenBaoSecretConfig, OrganizationBootstrapConfig,
};

pub enum OpenBaoAuthConfig {
    OIDCHexagone { mount_path: String },
    OIDCProConnect { mount_path: String },
}

pub struct OpenBaoConfig {
    pub server_url: String,
    pub secret: OpenBaoSecretConfig,
    pub auths: Vec<OpenBaoAuthConfig>,
}

pub struct ServerConfig {
    pub client_agent: ClientAgentConfig,
    pub account: AccountConfig,
    pub organization_bootstrap: OrganizationBootstrapConfig,
    pub openbao: Option<OpenBaoConfig>,
}

pub async fn get_server_config(
    config_dir: &Path,
    addr: ParsecAddr,
) -> Result<ServerConfig, GetServerConfigError> {
    let cmds = AnonymousServerCmds::new(config_dir, addr, ProxyConfig::default())?;

    use libparsec_client_connection::protocol::anonymous_server_cmds::latest::server_config::{
        Rep, Req,
    };

    match cmds.send(Req).await? {
        Rep::Ok {
            client_agent,
            account,
            organization_bootstrap,
            openbao,
        } => Ok(ServerConfig {
            client_agent,
            account,
            organization_bootstrap,
            openbao: match openbao {
                libparsec_protocol::anonymous_server_cmds::v5::server_config::OpenBaoConfig::Disabled => None,
                libparsec_protocol::anonymous_server_cmds::v5::server_config::OpenBaoConfig::Enabled { auths, secret, server_url } => Some(
                    OpenBaoConfig {
                        server_url,
                        secret,
                        auths: auths.into_iter().filter_map(|auth| match auth.id.as_ref() {
                            "OIDC_HEXAGONE" => Some(OpenBaoAuthConfig::OIDCHexagone { mount_path: auth.mount_path }),
                            "OIDC_PRO_CONNECT" => Some(OpenBaoAuthConfig::OIDCProConnect { mount_path: auth.mount_path }),
                            // Unknown kind of auth, just ignore it
                            _ => None,
                        }).collect()
                    }
                ),
            },
        }),
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
