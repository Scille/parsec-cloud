// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

pub use libparsec_client::user_ops::{
    ClientInfoError, WorkspaceRenameError as ClientWorkspaceRenameError,
    WorkspaceShareError as ClientWorkspaceShareError,
};
use libparsec_types::prelude::*;
pub use libparsec_types::{DeviceAccessStrategy, RealmRole};

use crate::{
    handle::{
        borrow_from_handle, register_handle_with_init, take_and_close_handle, Handle, HandleItem,
    },
    ClientConfig, ClientEvent, OnEventCallbackPlugged,
};

/*
 * Start
 */

#[derive(Debug, thiserror::Error)]
pub enum ClientStartError {
    #[error("This client is already running")]
    DeviceAlreadyRunning,
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
    // On web we run on the JS runtime which is mono-threaded, hence everything is !Send
    #[cfg(target_arch = "wasm32")] on_event_callback: Arc<dyn Fn(ClientEvent)>,

    access: DeviceAccessStrategy,
) -> Result<Handle, ClientStartError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();

    // 1) Load the device

    let device = libparsec_platform_device_loader::load_device(&config.config_dir, &access).await?;

    // 2) Make sure another client is not running this device

    let slug = device.slug();
    let initializing =
        register_handle_with_init(HandleItem::StartingClient { slug: slug.clone() }, |item| {
            match item {
                HandleItem::Client { client, .. } if client.device_slug() == slug => false,
                HandleItem::StartingClient { slug: candidate } if *candidate == slug => false,
                _ => true,
            }
        })
        .map_err(|_| ClientStartError::DeviceAlreadyRunning)?;

    // 3) Actually start the client

    let on_event = OnEventCallbackPlugged::new(on_event_callback);
    let client = libparsec_client::Client::start(config, on_event.event_bus.clone(), device)
        .await
        .map_err(ClientStartError::Internal)?;

    let handle = initializing.initialized(HandleItem::Client {
        client: Arc::new(client),
        on_event,
    });

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
    let (client, on_event) = take_and_close_handle(client, |x| match x {
        HandleItem::Client { client, on_event } => Some((client, on_event)),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    client.stop().await;

    // Wait until after the client is close to disconnect to the event bus to ensure
    // we don't miss events fired during client teardown
    drop(on_event);

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
) -> Result<Vec<(VlobID, EntryName)>, ClientListWorkspacesError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
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
) -> Result<VlobID, ClientWorkspaceCreateError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
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
    realm_id: VlobID,
    new_name: EntryName,
) -> Result<(), ClientWorkspaceRenameError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    client.user_ops.workspace_rename(realm_id, new_name).await
}

/*
 * Client share workspace
 */

pub async fn client_workspace_share(
    client: Handle,
    realm_id: VlobID,
    recipient: UserID,
    role: Option<RealmRole>,
) -> Result<(), ClientWorkspaceShareError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    client
        .user_ops
        .workspace_share(realm_id, &recipient, role)
        .await
}

pub struct ClientInfo {
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    pub device_label: Option<DeviceLabel>,
    pub user_id: UserID,
    pub profile: UserProfile,
    pub human_handle: Option<HumanHandle>,
}

pub async fn client_info(client: Handle) -> Result<ClientInfo, ClientInfoError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    Ok(ClientInfo {
        organization_id: client.organization_id().clone(),
        device_id: client.device_id().clone(),
        device_label: client.device_label().cloned(),
        user_id: client.device_id().user_id().clone(),
        profile: client.profile().await?,
        human_handle: client.human_handle().cloned(),
    })
}
