// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

pub use libparsec_types::prelude::*;

use crate::{ClientConfig, ClientEvent, OnEventCallbackPlugged};

#[derive(Debug, Clone)]
pub enum DeviceSaveStrategy {
    Password { password: Password },
    Smartcard,
}

/*
 * Bootstrap organization
 */

#[derive(Debug, thiserror::Error)]
pub enum BootstrapOrganizationError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Invalid bootstrap token")]
    InvalidToken,
    #[error("Bootstrap token already used")]
    AlreadyUsedToken,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    BadTimestamp {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error("Cannot save device: {0}")]
    SaveDeviceError(anyhow::Error),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<libparsec_client::BootstrapOrganizationError> for BootstrapOrganizationError {
    fn from(value: libparsec_client::BootstrapOrganizationError) -> Self {
        match value {
            libparsec_client::BootstrapOrganizationError::Offline => {
                BootstrapOrganizationError::Offline
            }
            libparsec_client::BootstrapOrganizationError::InvalidToken => {
                BootstrapOrganizationError::InvalidToken
            }
            libparsec_client::BootstrapOrganizationError::AlreadyUsedToken => {
                BootstrapOrganizationError::AlreadyUsedToken
            }
            libparsec_client::BootstrapOrganizationError::BadTimestamp {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => BootstrapOrganizationError::BadTimestamp {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
            libparsec_client::BootstrapOrganizationError::Internal(e) => {
                BootstrapOrganizationError::Internal(e)
            }
        }
    }
}

pub async fn bootstrap_organization(
    config: ClientConfig,

    // Access to the event bus is done through this callback.
    // Ad-hoc code should be added to the binding system to handle this (hence
    // why this is passed as a parameter instead of as part of `ClientConfig`:
    // we can have a simple `if func_name == "client_login"` that does a special
    // cooking of it last param.
    #[cfg(not(target_arch = "wasm32"))] on_event_callback: Arc<dyn Fn(ClientEvent) + Send + Sync>,
    // On web we run on the JS runtime which is monothreaded, hence everything is !Send
    #[cfg(target_arch = "wasm32")] on_event_callback: Arc<dyn Fn(ClientEvent)>,

    bootstrap_organization_addr: BackendOrganizationBootstrapAddr,
    save_strategy: DeviceSaveStrategy,
    human_handle: Option<HumanHandle>,
    device_label: Option<DeviceLabel>,
    sequester_authority_verify_key: Option<SequesterVerifyKeyDer>,
) -> Result<AvailableDevice, BootstrapOrganizationError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();
    let events_plugged = OnEventCallbackPlugged::new(on_event_callback);
    // TODO: connect event_bus to on_event_callback

    let finalize_ctx = libparsec_client::bootstrap_organization(
        config.clone(),
        events_plugged.event_bus,
        bootstrap_organization_addr,
        human_handle,
        device_label,
        sequester_authority_verify_key,
    )
    .await?;

    let access = {
        let key_file = libparsec_platform_device_loader::get_default_key_file(
            &config.config_dir,
            &finalize_ctx.new_local_device,
        );
        match save_strategy {
            DeviceSaveStrategy::Password { password } => {
                DeviceAccessStrategy::Password { key_file, password }
            }
            DeviceSaveStrategy::Smartcard => DeviceAccessStrategy::Smartcard { key_file },
        }
    };

    let available_device = finalize_ctx
        .save_local_device(&access)
        .await
        .map_err(BootstrapOrganizationError::SaveDeviceError)?;

    Ok(available_device)
}
