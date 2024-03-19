// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client::ServerConfig;
pub use libparsec_client::{
    ClientCreateWorkspaceError, ClientGetCurrentSelfProfileError, ClientGetUserDeviceError,
    ClientListUserDevicesError, ClientListUsersError, ClientListWorkspaceUsersError,
    ClientRenameWorkspaceError, ClientRevokeUserError, ClientShareWorkspaceError, DeviceInfo,
    UserInfo, WorkspaceInfo, WorkspaceUserAccessInfo,
};
use libparsec_platform_async::event::{Event, EventListener};
use libparsec_platform_device_loader::ChangeAuthentificationError;
use libparsec_types::prelude::*;
pub use libparsec_types::{DeviceAccessStrategy, RealmRole};

use crate::{
    handle::{
        borrow_from_handle, filter_close_handles, register_handle_with_init, take_and_close_handle,
        FilterCloseHandle, Handle, HandleItem,
    },
    ClientConfig, ClientEvent, DeviceSaveStrategy, OnEventCallbackPlugged,
};

fn borrow_client(client: Handle) -> anyhow::Result<Arc<libparsec_client::Client>> {
    borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })
}

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
            libparsec_platform_device_loader::LoadDeviceError::Internal(e) => Self::Internal(e),
        }
    }
}

pub async fn client_start(
    config: ClientConfig,

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

    enum RegisterFailed {
        AlreadyRegistered(Handle),
        ConcurrentRegister(EventListener),
    }

    let initializing = loop {
        let outcome = register_handle_with_init(
            HandleItem::StartingClient {
                slug: slug.clone(),
                to_wake_on_done: vec![],
            },
            |handle, item| match item {
                HandleItem::Client { client, .. } if client.device_slug() == slug => {
                    Err(RegisterFailed::AlreadyRegistered(handle))
                }
                HandleItem::StartingClient {
                    slug: x_slug,
                    to_wake_on_done,
                } if *x_slug == slug => {
                    let event = Event::new();
                    let listener = event.listen();
                    to_wake_on_done.push(event);
                    Err(RegisterFailed::ConcurrentRegister(listener))
                }
                _ => Ok(()),
            },
        );

        match outcome {
            Ok(initializing) => break initializing,
            Err(RegisterFailed::AlreadyRegistered(handle)) => {
                // Go idempotent here
                return Ok(handle);
            }
            // Wait for concurrent operation to finish before retrying
            Err(RegisterFailed::ConcurrentRegister(listener)) => listener.await,
        }
    };

    // 3) Actually start the client

    let on_event = OnEventCallbackPlugged::new(on_event_callback);
    let client = libparsec_client::Client::start(config, on_event.event_bus.clone(), device)
        .await
        .map_err(ClientStartError::Internal)?;

    let handle = initializing.initialized(move |item: &mut HandleItem| {
        let starting = std::mem::replace(item, HandleItem::Client { client, on_event });
        match starting {
            HandleItem::StartingClient {
                to_wake_on_done, ..
            } => {
                for event in to_wake_on_done {
                    event.notify(usize::MAX);
                }
            }
            _ => unreachable!(),
        }
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
    let client_handle = client;
    let (client, on_event) = take_and_close_handle(client_handle, |x| match x {
        HandleItem::Client { client, on_event } => Ok((client, on_event)),
        // Note we consider an error if the handle is in `HandleItem::StartingClient` state
        // this is because at that point this is not a legit use of the handle given it
        // has never been yet provided to the caller in the first place !
        // On top of that it simplifies the start logic (given it guarantees nothing will
        // concurrently close the handle)
        invalid => Err(invalid),
    })?;

    // Note stopping the client also stop all it related workspace ops
    client.stop().await;

    // Wait until after the client is closed to disconnect the event bus to ensure
    // we don't miss any event fired during client teardown
    drop(on_event);

    // Finally cleanup the handles related to the client's workspaces and mountpoints
    // (Note the mountpoints are automatically unmounted when the handle item is dropped).
    loop {
        let mut maybe_wait = None;
        filter_close_handles(client_handle, |x| match x {
            HandleItem::Workspace {
                client: x_client, ..
            }
            | HandleItem::Mountpoint {
                client: x_client, ..
            } if *x_client == client_handle => FilterCloseHandle::Close,
            // If something is still starting, it will most likely won't go very far
            // (all workspace ops now are stopped), but we have to wait for it anyway
            HandleItem::StartingMountpoint {
                client: x_client,
                to_wake_on_done,
                ..
            }
            | HandleItem::StartingWorkspace {
                client: x_client,
                to_wake_on_done,
                ..
            } if *x_client == client_handle => {
                if maybe_wait.is_none() {
                    let event = Event::new();
                    let listener = event.listen();
                    to_wake_on_done.push(event);
                    maybe_wait = Some(listener);
                }
                FilterCloseHandle::Keep
            }
            _ => FilterCloseHandle::Keep,
        });
        // Note there is no risk (in theory it least !) to end up in an infinite
        // loop here: the client's handle is closed so no new workspace start
        // commands can be issued and we only process the remaining ones.
        match maybe_wait {
            Some(listener) => listener.await,
            None => break,
        }
    }

    Ok(())
}

/*
 * Client info
 */

pub struct ClientInfo {
    pub organization_addr: ParsecOrganizationAddr,
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    pub user_id: UserID,
    pub device_label: DeviceLabel,
    pub human_handle: HumanHandle,
    pub current_profile: UserProfile,
    pub server_config: ServerConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientInfoError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_info(client: Handle) -> Result<ClientInfo, ClientInfoError> {
    let client = borrow_client(client)?;

    Ok(ClientInfo {
        organization_addr: client.organization_addr().clone(),
        organization_id: client.organization_id().clone(),
        device_id: client.device_id().clone(),
        user_id: client.device_id().user_id().clone(),
        device_label: client.device_label().to_owned(),
        human_handle: client.human_handle().to_owned(),
        current_profile: client
            .get_current_self_profile()
            .await
            .map_err(|e| match e {
                ClientGetCurrentSelfProfileError::Stopped => ClientInfoError::Stopped,
                ClientGetCurrentSelfProfileError::Internal(e) => {
                    e.context("Cannot retrieve profile").into()
                }
            })?,
        server_config: client.server_config(),
    })
}

/*
 * Change access
 */

#[derive(Debug, thiserror::Error)]
pub enum ClientChangeAuthenticationError {
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error("Cannot deserialize file content")]
    InvalidData,
    #[error("Failed to decrypt file content")]
    DecryptionFailed,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ChangeAuthentificationError> for ClientChangeAuthenticationError {
    fn from(value: ChangeAuthentificationError) -> Self {
        match value {
            ChangeAuthentificationError::InvalidPath(e) => Self::InvalidPath(e),
            ChangeAuthentificationError::InvalidData => Self::InvalidData,
            ChangeAuthentificationError::DecryptionFailed => Self::DecryptionFailed,
            ChangeAuthentificationError::CannotRemoveOldDevice => {
                Self::Internal(anyhow::anyhow!(value))
            }
            ChangeAuthentificationError::Internal(e) => Self::Internal(e),
        }
    }
}

pub async fn client_change_authentication(
    config: ClientConfig,
    current_auth: DeviceAccessStrategy,
    new_auth: DeviceSaveStrategy,
) -> Result<(), ClientChangeAuthenticationError> {
    let key_file = current_auth.key_file().to_owned();

    libparsec_platform_device_loader::change_authentication(
        &config.config_dir,
        &current_auth,
        &new_auth.into_access(key_file),
    )
    .await?;
    Ok(())
}

/*
 * Revoke user
 */

pub async fn client_revoke_user(client: Handle, user: UserID) -> Result<(), ClientRevokeUserError> {
    let client = borrow_client(client)?;

    client.revoke_user(user).await
}

/*
 * List users
 */

// TODO: order by user name (asc/desc), offset/limit
pub async fn client_list_users(
    client: Handle,
    skip_revoked: bool,
    // offset: Option<usize>,
    // limit: Option<usize>,
) -> Result<Vec<UserInfo>, ClientListUsersError> {
    let client = borrow_client(client)?;

    client.list_users(skip_revoked, None, None).await
}

/*
 * List user devices
 */

pub async fn client_list_user_devices(
    client: Handle,
    user: UserID,
) -> Result<Vec<DeviceInfo>, ClientListUserDevicesError> {
    let client = borrow_client(client)?;

    client.list_user_devices(user).await
}

/*
 * Get user device
 */

pub async fn client_get_user_device(
    client: Handle,
    device: DeviceID,
) -> Result<(UserInfo, DeviceInfo), ClientGetUserDeviceError> {
    let client = borrow_client(client)?;

    client.get_user_device(device).await
}

/*
 * List workspace's users
 */

pub async fn client_list_workspace_users(
    client: Handle,
    realm_id: VlobID,
) -> Result<Vec<WorkspaceUserAccessInfo>, ClientListWorkspaceUsersError> {
    let client = borrow_client(client)?;

    client.list_workspace_users(realm_id).await
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
) -> Result<Vec<WorkspaceInfo>, ClientListWorkspacesError> {
    let client = borrow_client(client)?;

    let workspaces = client.list_workspaces().await;

    Ok(workspaces)
}

/*
 * Create workspace
 */

pub async fn client_create_workspace(
    client: Handle,
    name: EntryName,
) -> Result<VlobID, ClientCreateWorkspaceError> {
    let client = borrow_client(client)?;

    client.create_workspace(name).await
}

/*
 * Client rename workspace
 */

pub async fn client_rename_workspace(
    client: Handle,
    realm_id: VlobID,
    new_name: EntryName,
) -> Result<(), ClientRenameWorkspaceError> {
    let client = borrow_client(client)?;

    client.rename_workspace(realm_id, new_name).await
}

/*
 * Client share workspace
 */

pub async fn client_share_workspace(
    client: Handle,
    realm_id: VlobID,
    recipient: UserID,
    role: Option<RealmRole>,
) -> Result<(), ClientShareWorkspaceError> {
    let client = borrow_client(client)?;

    client.share_workspace(realm_id, recipient, role).await
}
