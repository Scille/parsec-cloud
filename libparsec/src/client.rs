// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;
pub use libparsec_types::DeviceAccessStrategy;

use crate::{
    handle::{register_handle, take_and_close_handle, Handle, HandleItem},
    ClientConfig, ClientEvent, OnEventCallbackPlugged,
};

#[derive(Debug, thiserror::Error)]
pub enum ClientStartError {
    #[error(transparent)]
    LoadDeviceInvalidPath(anyhow::Error),
    #[error("Cannot deserialize file content")]
    LoadDeviceInvalidData,
    #[error("Failed to decrypt file content")]
    LoadDeviceDecryptionFailed,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<libparsec_platform_device_loader::LoadDeviceError> for ClientStartError {
    fn from(value: libparsec_platform_device_loader::LoadDeviceError) -> Self {
        match value {
            libparsec_platform_device_loader::LoadDeviceError::InvalidPath(err) => {
                Self::LoadDeviceInvalidPath(err)
            }
            libparsec_platform_device_loader::LoadDeviceError::InvalidData => {
                Self::LoadDeviceInvalidData
            }
            libparsec_platform_device_loader::LoadDeviceError::DecryptionFailed => {
                Self::LoadDeviceDecryptionFailed
            }
        }
    }
}

pub async fn client_start(
    config: ClientConfig,

    // Access to the event bus is done through this callback.
    // Ad-hoc code should be added to the binding system to handle this (hence
    // why this is passed as a parameter instead of as part of `ClientConfig`:
    // we can have a simple `if func_name == "client_login"` that does a special
    // cooking of it last param.
    #[cfg(not(target_arch = "wasm32"))] on_event_callback: Arc<dyn Fn(ClientEvent) + Send + Sync>,
    // On web we run on the JS runtime which is monothreaded, hence everything is !Send
    #[cfg(target_arch = "wasm32")] on_event_callback: Arc<dyn Fn(ClientEvent)>,

    access: DeviceAccessStrategy,
) -> Result<Handle, ClientStartError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();
    // TODO
    let events_plugged = OnEventCallbackPlugged::new(on_event_callback);

    // 1) Load the device

    let device = libparsec_platform_device_loader::load_device(&config.config_dir, &access).await?;

    // 2) Actually start the client

    let client = libparsec_client::Client::start(config, events_plugged.event_bus.clone(), device)
        .await
        .map_err(ClientStartError::Internal)?;

    let handle = register_handle(HandleItem::Client((client, events_plugged)));

    Ok(handle)
}

#[derive(Debug, thiserror::Error)]
pub enum ClientStopError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_stop(handle: Handle) -> Result<(), ClientStopError> {
    let (client, events_plugged) = take_and_close_handle(handle, |x| match x {
        crate::handle::HandleItem::Client(client) => Some(client),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    client.stop().await;

    // Wait until after the client is close to disconnect to the event bus to ensure
    // we don't miss events fired during client teardown
    drop(events_plugged);

    Ok(())
}
