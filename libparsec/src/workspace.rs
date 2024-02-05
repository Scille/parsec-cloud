// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

pub use libparsec_client::workspace::{
    EntryStat, WorkspaceCreateFileError, WorkspaceCreateFolderError, WorkspaceRemoveEntryError,
    WorkspaceRenameEntryError, WorkspaceStatEntryError,
};
use libparsec_platform_async::event::{Event, EventListener};
use libparsec_types::prelude::*;

use crate::handle::{
    borrow_from_handle, register_handle_with_init, take_and_close_handle, Handle, HandleItem,
};

fn borrow_workspace(workspace: Handle) -> anyhow::Result<Arc<libparsec_client::WorkspaceOps>> {
    borrow_from_handle(workspace, |x| match x {
        HandleItem::Workspace { workspace_ops, .. } => Some(workspace_ops.clone()),
        _ => None,
    })
}

/*
 * Client start workspace
 */

#[derive(Debug, thiserror::Error)]
pub enum ClientStartWorkspaceError {
    #[error("Workspace not found")]
    WorkspaceNotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<libparsec_client::ClientStartWorkspaceError> for ClientStartWorkspaceError {
    fn from(value: libparsec_client::ClientStartWorkspaceError) -> Self {
        match value {
            libparsec_client::ClientStartWorkspaceError::WorkspaceNotFound => {
                ClientStartWorkspaceError::WorkspaceNotFound
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
        AlreadyRegistered(Handle),
        ConcurrentRegister(EventListener),
    }
    let initializing = loop {
        let outcome = register_handle_with_init(
            HandleItem::StartingWorkspace {
                client,
                realm_id,
                to_wake_on_done: vec![],
            },
            |handle, item| match item {
                HandleItem::Workspace {
                    client: x_client,
                    workspace_ops: x_workspace_ops,
                } if *x_client == client && x_workspace_ops.realm_id() == realm_id => {
                    Err(RegisterFailed::AlreadyRegistered(handle))
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
            Err(RegisterFailed::AlreadyRegistered(handle)) => {
                // Go idempotent here
                return Ok(handle);
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
    })?;

    let workspace_ops = client.start_workspace(realm_id).await?;

    // 3. Finally register the workspace to get a handle

    let workspace_handle = initializing.initialized(HandleItem::Workspace {
        client: client_handle,
        workspace_ops,
    });

    Ok(workspace_handle)
}

/*
 * Stop workspace
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
        // has never been yet provided to the caller in the first place !
        // On top of that it simplifies the start logic (given it guarantees nothing will
        // concurrently close the handle)
        invalid => Err(invalid),
    })?;

    let client = borrow_from_handle(client_handle, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    client.stop_workspace(realm_id).await;

    Ok(())
}

/*
 * Workspace FS operations
 */

pub async fn workspace_stat_entry(
    workspace: Handle,
    path: &FsPath,
) -> Result<EntryStat, WorkspaceStatEntryError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.stat_entry(path).await
}

pub async fn workspace_rename_entry(
    workspace: Handle,
    path: FsPath,
    new_name: EntryName,
    overwrite: bool,
) -> Result<(), WorkspaceRenameEntryError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.rename_entry(path, new_name, overwrite).await
}

pub async fn workspace_create_folder(
    workspace: Handle,
    path: FsPath,
) -> Result<VlobID, WorkspaceCreateFolderError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.create_folder(path).await
}

pub async fn workspace_create_folder_all(
    workspace: Handle,
    path: FsPath,
) -> Result<VlobID, WorkspaceCreateFolderError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.create_folder_all(path).await
}

pub async fn workspace_create_file(
    workspace: Handle,
    path: FsPath,
) -> Result<VlobID, WorkspaceCreateFileError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.create_file(path).await
}

pub async fn workspace_remove_entry(
    workspace: Handle,
    path: FsPath,
) -> Result<(), WorkspaceRemoveEntryError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.remove_entry(path).await
}

pub async fn workspace_remove_file(
    workspace: Handle,
    path: FsPath,
) -> Result<(), WorkspaceRemoveEntryError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.remove_file(path).await
}

pub async fn workspace_remove_folder(
    workspace: Handle,
    path: FsPath,
) -> Result<(), WorkspaceRemoveEntryError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.remove_folder(path).await
}

pub async fn workspace_remove_folder_all(
    workspace: Handle,
    path: FsPath,
) -> Result<(), WorkspaceRemoveEntryError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.remove_folder_all(path).await
}
