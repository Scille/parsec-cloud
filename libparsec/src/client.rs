// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

pub use libparsec_client::user_ops::{UserOpsError, UserOpsWorkspaceShareError};
use libparsec_types::prelude::*;
pub use libparsec_types::{DeviceAccessStrategy, RealmRole};

use crate::{
    handle::{borrow_from_handle, register_handle, take_and_close_handle, Handle, HandleItem},
    ClientConfig, ClientEvent, OnEventCallbackPlugged,
};

/*
 * Start
 */

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

    let handle = register_handle(HandleItem::Client((Arc::new(client), events_plugged)));

    Ok(handle)
}

/*
 * Stop
 */

#[derive(Debug, thiserror::Error)]
pub enum ClientStopError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_stop(client: Handle) -> Result<(), ClientStopError> {
    let (client, events_plugged) = take_and_close_handle(client, |x| match x {
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

/*
 * List workspaces
 */

#[derive(Debug, thiserror::Error)]
pub enum ClientListWorkspacesError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_list_workspaces(
    client: Handle,
) -> Result<Vec<(EntryID, EntryName)>, ClientListWorkspacesError> {
    let client = borrow_from_handle(client, |x| match x {
        crate::handle::HandleItem::Client((client, _)) => Some(client.clone()),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    Ok(client.user_ops.list_workspaces())
}

/*
 * Create workspace
 */

#[derive(Debug, thiserror::Error)]
pub enum ClientWorkspaceCreateError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_workspace_create(
    client: Handle,
    name: EntryName,
) -> Result<EntryID, ClientWorkspaceCreateError> {
    let client = borrow_from_handle(client, |x| match x {
        crate::handle::HandleItem::Client((client, _)) => Some(client.clone()),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    client
        .user_ops
        .workspace_create(name)
        .await
        .map_err(|err| err.into())
}

/*
 * Client rename workspace
 */

pub async fn client_workspace_rename(
    client: Handle,
    workspace_id: EntryID,
    new_name: EntryName,
) -> Result<(), UserOpsError> {
    let client = borrow_from_handle(client, |x| match x {
        crate::handle::HandleItem::Client((client, _)) => Some(client.clone()),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    client
        .user_ops
        .workspace_rename(workspace_id, new_name)
        .await
}

/*
 * Client share workspace
 */

pub async fn client_workspace_share(
    client: Handle,
    workspace_id: EntryID,
    recipient: UserID,
    role: Option<RealmRole>,
) -> Result<(), UserOpsWorkspaceShareError> {
    let client = borrow_from_handle(client, |x| match x {
        crate::handle::HandleItem::Client((client, _)) => Some(client.clone()),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    client
        .user_ops
        .workspace_share(workspace_id, &recipient, role)
        .await
}
