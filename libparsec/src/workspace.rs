// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub use libparsec_client::{
    user_ops::{
        ClientInfoError, RenameWorkspaceError as ClientWorkspaceRenameError,
        ShareWorkspaceError as ClientWorkspaceShareError,
    },
    workspace_ops::{EntryInfo, EntryInfoError as WorkspaceEntryInfoError},
};
use libparsec_platform_async::event::{Event, EventListener};
use libparsec_types::prelude::*;
pub use libparsec_types::{DeviceAccessStrategy, RealmRole};

use crate::handle::{
    borrow_from_handle, register_handle_with_init, take_and_close_handle, Handle, HandleItem,
};

/*
 * Client start workspace
 */

#[derive(Debug, thiserror::Error)]
pub enum ClientStartWorkspaceError {
    #[error("Cannot start workspace: no access")]
    NoAccess,
    // We cannot just be idempotent and return the existing handle here: this is because
    // the routine that originally started the workspace can arbitrary decide to stop it,
    // at which point it becomes illegal to use the handle (resulting in an internal error)
    #[error("Workspace already started")]
    AlreadyStarted,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<libparsec_client::ClientStartWorkspaceError> for ClientStartWorkspaceError {
    fn from(value: libparsec_client::ClientStartWorkspaceError) -> Self {
        match value {
            libparsec_client::ClientStartWorkspaceError::NoAccess => {
                ClientStartWorkspaceError::NoAccess
            }
            libparsec_client::ClientStartWorkspaceError::Internal(e) => {
                ClientStartWorkspaceError::Internal(e)
            }
        }
    }
}

pub async fn client_start_workspace(
    client: Handle,
    realm_id: VlobID,
) -> Result<Handle, ClientStartWorkspaceError> {
    // 1. Check if the workspace isn't already started (or starting)

    enum RegisterFailed {
        AlreadyRegistered,
        ConcurrentRegister(EventListener),
    }
    let initializing = loop {
        let outcome = register_handle_with_init(
            HandleItem::StartingWorkspace {
                client,
                realm_id,
                to_wake_on_done: vec![],
            },
            |item| match item {
                HandleItem::Workspace {
                    client: x_client,
                    workspace_ops: x_workspace_ops,
                } if *x_client == client && x_workspace_ops.realm_id() == realm_id => {
                    Err(RegisterFailed::AlreadyRegistered)
                }
                HandleItem::StartingWorkspace {
                    client: x_client,
                    realm_id: x_realm_id,
                    to_wake_on_done,
                } if *x_client == client && *x_realm_id == realm_id => {
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
            Err(RegisterFailed::AlreadyRegistered) => {
                return Err(ClientStartWorkspaceError::AlreadyStarted)
            }
            // Wait for concurrent operation to finish before retrying
            Err(RegisterFailed::ConcurrentRegister(listener)) => listener.await,
        }
    };

    // 2. Actually start the workspace

    let client_handle = client;
    let client = borrow_from_handle(client_handle, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    let workspace_ops = client.start_workspace(realm_id).await?;

    // 3. Finally register the workspace to get a handle

    let workspace_handle = initializing.initialized(HandleItem::Workspace {
        client: client_handle,
        workspace_ops,
    });

    Ok(workspace_handle)
}

/*
 * Client stop workspace
 */

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceStopError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn workspace_stop(workspace: Handle) -> Result<(), WorkspaceStopError> {
    let (client_handle, realm_id) = take_and_close_handle(workspace, |x| match x {
        HandleItem::Workspace {
            client,
            workspace_ops,
        } => Ok((client, workspace_ops.realm_id())),
        // Note we consider an error if the handle is in `HandleItem::StartingWorkspace` state
        // this is because at that point this is not a legit use of the handle given it
        // hasn't been yet provided to the caller !
        // On top of that is simplify the start logic (given it guarantees nothing will
        // concurrently close the handle)
        invalid => Err(invalid),
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    let client = borrow_from_handle(client_handle, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })
    .ok_or_else(|| {
        // TODO: log error here, this is not expected to happen !
        anyhow::anyhow!("Invalid handle")
    })?;

    client
        .stop_workspace(realm_id)
        .await
        .map_err(WorkspaceStopError::Internal)
}

/*
 * Client workspace entry info
 */

pub async fn workspace_entry_info(
    workspace: Handle,
    path: &FsPath,
) -> Result<EntryInfo, WorkspaceEntryInfoError> {
    let workspace = borrow_from_handle(workspace, |x| match x {
        HandleItem::Workspace { workspace_ops, .. } => Some(workspace_ops.clone()),
        _ => None,
    })
    .ok_or_else(|| anyhow::anyhow!("Invalid handle"))?;

    workspace.entry_info(path).await
}
