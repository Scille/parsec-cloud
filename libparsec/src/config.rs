// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub use libparsec_client::{MountpointMountStrategy, ProxyConfig, WorkspaceStorageCacheSize};
use libparsec_client_connection::{AnonymousServerCmds, ConnectionError};
pub use libparsec_platform_device_loader::{
    PARSEC_BASE_CONFIG_DIR, PARSEC_BASE_DATA_DIR, PARSEC_BASE_HOME_DIR, get_default_config_dir,
    get_default_data_base_dir, get_default_mountpoint_base_dir,
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
    AccountConfig, OpenBaoSecretConfig, OrganizationBootstrapConfig,
};

pub struct OpenBaoAuthConfig {
    pub id: String,
    pub mount_path: String,
    pub name: String,
    pub help_link: Option<String>,
    pub icon: Option<String>,
}

pub struct OpenBaoConfig {
    pub server_url: String,
    pub secret: OpenBaoSecretConfig,
    pub transit_mount_path: String,
    pub auths: Vec<OpenBaoAuthConfig>,
}
pub struct CryptPadConfig {
    pub server_url: String,
}
pub struct ServerConfig {
    pub account: AccountConfig,
    pub organization_bootstrap: OrganizationBootstrapConfig,
    pub openbao: Option<OpenBaoConfig>,
    pub cryptpad: Option<CryptPadConfig>,
    pub advisory_device_file_protection: Vec<AdvisoryDeviceFileProtection>,
    pub server_version: Option<String>,
}

pub async fn get_server_config(
    config_dir: &Path,
    addr: ParsecAddr,
) -> Result<ServerConfig, GetServerConfigError> {
    let cmds = AnonymousServerCmds::new(config_dir, addr, ProxyConfig::default())?;

    use libparsec_client_connection::protocol::anonymous_server_cmds::latest::server_config::{
        Rep, Req,
    };

    let (rep, server_version) = cmds.send_verbose(Req).await?;

    match  rep{
        Rep::Ok {
            account,
            organization_bootstrap,
            openbao,
            cryptpad,
            advisory_device_file_protection,
        } => Ok(ServerConfig {
            account,
            organization_bootstrap,
            // This field has been introduced in Parsec 3.9
            cryptpad: match cryptpad {
                Maybe::Absent => None,
                Maybe::Present(cryptpad) => match cryptpad {
                   libparsec_protocol::anonymous_server_cmds::v5::server_config::CryptPadConfig::Disabled => None,
                   libparsec_protocol::anonymous_server_cmds::v5::server_config::CryptPadConfig::Enabled { server_url } => Some(CryptPadConfig{ server_url }),
                }
            },
            // This field has been introduced in Parsec 3.9
            advisory_device_file_protection: match advisory_device_file_protection {
                Maybe::Absent => vec![],
                Maybe::Present(advisory_device_file_protection) => advisory_device_file_protection.into_iter().filter_map(|raw| {
                    // Just skip unrecognized items for forward-compatibility
                    raw.parse().ok()
                }).collect(),
            },
            openbao: match openbao {
                libparsec_protocol::anonymous_server_cmds::v5::server_config::OpenBaoConfig::Disabled => None,
                libparsec_protocol::anonymous_server_cmds::v5::server_config::OpenBaoConfig::Enabled {
                    auths, secret, transit_mount_path, server_url
                    } => Some(
                    OpenBaoConfig {
                        server_url,
                        secret,
                        // This field has been introduced in Parsec 3.8, for older server version
                        // we just assume the mount path is `transit`.
                        transit_mount_path: match transit_mount_path {
                            Maybe::Absent => "transit".to_string(),
                            Maybe::Present(transit_mount_path) => transit_mount_path
                        },
                        auths: auths
                            .into_iter()
                            .filter_map(|auth| {
                                if auth.id.is_empty() || auth.mount_path.is_empty() {
                                    log::warn!("Server config has returned an invalid OpenBaoAuthConfig: {:?}", auth);
                                    return None;
                                }
                                let name = match auth.name {
                                    Maybe::Present(name) if !name.is_empty() => name,
                                    // Fallback for older servers
                                    _ => match auth.id.as_str() {
                                        "PRO_CONNECT" => String::from("ProConnect"),
                                        "HEXAGONE" => String::from("Hexagone"),
                                        _ => auth.id.clone(),
                                    },
                                };
                                Some(OpenBaoAuthConfig {
                                    id: auth.id,
                                    mount_path: auth.mount_path,
                                    name,
                                    help_link: auth.help_link.unwrap_or(None),
                                    icon: auth.icon.unwrap_or(None),
                                })
                        }).collect()
                    }
                ),
            },
            server_version
        }),
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
