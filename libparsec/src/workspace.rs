// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

pub use libparsec_client::workspace::{
    EntryStat, OpenOptions, WorkspaceCreateFileError, WorkspaceCreateFolderError,
    WorkspaceFdCloseError, WorkspaceFdFlushError, WorkspaceFdReadError, WorkspaceFdResizeError,
    WorkspaceFdWriteError, WorkspaceOpenFileError, WorkspaceRemoveEntryError,
    WorkspaceRenameEntryError, WorkspaceStatEntryError,
};
use libparsec_platform_async::event::{Event, EventListener};
use libparsec_types::prelude::*;

use crate::handle::{
    borrow_from_handle, filter_close_handles, register_handle_with_init, take_and_close_handle,
    FilterCloseHandle, Handle, HandleItem,
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
    let workspace_handle = workspace;

    let (client_handle, realm_id) = take_and_close_handle(workspace_handle, |x| match x {
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

    // Finally cleanup the handles related to the workspace's mountpoints
    // (Note the mountpoints are automatically unmounted when the handle item is dropped).
    loop {
        let mut maybe_wait = None;
        filter_close_handles(client_handle, |x| match x {
            HandleItem::Mountpoint {
                workspace: x_workspace,
                ..
            } if *x_workspace == workspace_handle => FilterCloseHandle::Close,
            // If something is still starting, it will most likely won't go very far
            // (all workspace ops now are stopped), but we have to wait for it anyway
            HandleItem::StartingMountpoint {
                workspace: x_workspace,
                to_wake_on_done,
                ..
            } if *x_workspace == workspace_handle => {
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
        // loop here: the workspace's handle is closed so no new workspace mount
        // commands can be issued and we only process the remaining ones.
        match maybe_wait {
            Some(listener) => listener.await,
            None => break,
        }
    }

    Ok(())
}

/*
 * Info on workspace
 */

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceInfoError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct StartedWorkspaceInfo {
    pub client: Handle,
    pub id: VlobID,
    pub current_name: EntryName,
    pub current_self_role: RealmRole,
    pub mountpoints: Vec<(Handle, std::path::PathBuf)>,
}

pub async fn workspace_info(workspace: Handle) -> Result<StartedWorkspaceInfo, WorkspaceInfoError> {
    let workspace_handle = workspace;
    let (client_handle, workspace) = borrow_from_handle(workspace_handle, |x| match x {
        HandleItem::Workspace {
            client,
            workspace_ops,
            ..
        } => Some((*client, workspace_ops.clone())),
        _ => None,
    })?;

    let (current_name, current_self_role) = workspace.get_current_name_and_self_role();

    #[cfg(target_arch = "wasm32")]
    let mountpoints = vec![];
    #[cfg(not(target_arch = "wasm32"))]
    let mountpoints = {
        let mut mountpoints = vec![];
        crate::handle::iter_opened_handles(|mountpoint_handle, x| match x {
            HandleItem::Mountpoint {
                workspace: x_workspace,
                mountpoint,
                ..
            } if *x_workspace == workspace_handle => {
                mountpoints.push((mountpoint_handle, mountpoint.path().to_owned()));
            }
            _ => (),
        });
        mountpoints
    };

    Ok(StartedWorkspaceInfo {
        client: client_handle,
        id: workspace.realm_id(),
        current_name,
        current_self_role,
        mountpoints,
    })
}

/*
 * Mount workspace
 */

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceMountError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[cfg(target_arch = "wasm32")]
pub async fn workspace_mount(
    _workspace: Handle,
) -> Result<(Handle, std::path::PathBuf), WorkspaceMountError> {
    Err(WorkspaceMountError::Internal(anyhow::anyhow!(
        "Not available in web"
    )))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn workspace_mount(
    workspace: Handle,
) -> Result<(Handle, std::path::PathBuf), WorkspaceMountError> {
    let workspace_handle = workspace;
    let (client_handle, workspace) = borrow_from_handle(workspace_handle, |x| match x {
        HandleItem::Workspace {
            client,
            workspace_ops,
            ..
        } => Some((*client, workspace_ops.clone())),
        _ => None,
    })?;

    // 1. Check if the mountpoint isn't already started (or starting)

    enum RegisterFailed {
        AlreadyRegistered((Handle, std::path::PathBuf)),
        ConcurrentRegister(EventListener),
    }
    let initializing = loop {
        let outcome = register_handle_with_init(
            HandleItem::StartingMountpoint {
                client: client_handle,
                workspace: workspace_handle,
                to_wake_on_done: vec![],
            },
            // We don't check if the mountpoint path is already used in the precondition,
            // this is because if that's the case the mount operation will simply fail
            |handle, item| match item {
                HandleItem::Mountpoint {
                    workspace: x_workspace,
                    mountpoint: x_mountpoint,
                    ..
                } if *x_workspace == workspace_handle => Err(RegisterFailed::AlreadyRegistered((
                    handle,
                    x_mountpoint.path().to_owned(),
                ))),
                HandleItem::StartingMountpoint {
                    workspace: x_workspace,
                    to_wake_on_done,
                    ..
                } if *x_workspace == workspace_handle => {
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
            Err(RegisterFailed::AlreadyRegistered((handle, mountpoint_path))) => {
                // Go idempotent here
                return Ok((handle, mountpoint_path));
            }
            // Wait for concurrent operation to finish before retrying
            Err(RegisterFailed::ConcurrentRegister(listener)) => listener.await,
        }
    };

    // 2. Actually do the mount

    let mountpoint = libparsec_platform_mountpoint::Mountpoint::mount(workspace).await?;

    // 3. Finally register the mountpoint to get a handle

    let mountpoint_path = mountpoint.path().to_owned();
    let handle = initializing.initialized(HandleItem::Mountpoint {
        client: client_handle,
        workspace: workspace_handle,
        mountpoint,
    });

    Ok((handle, mountpoint_path))
}

/*
 * Unmount workspace
 */

#[derive(Debug, thiserror::Error)]
pub enum MountpointUnmountError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[cfg(target_arch = "wasm32")]
pub async fn mountpoint_unmount(_mountpoint: Handle) -> Result<(), MountpointUnmountError> {
    Err(MountpointUnmountError::Internal(anyhow::anyhow!(
        "Not available in web"
    )))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn mountpoint_unmount(mountpoint: Handle) -> Result<(), MountpointUnmountError> {
    let mountpoint = take_and_close_handle(mountpoint, |x| match x {
        HandleItem::Mountpoint { mountpoint, .. } => Ok(mountpoint),
        // Note we consider an error if the handle is in `HandleItem::StartingMountpoint` state
        // this is because at that point this is not a legit use of the handle given it
        // has never been yet provided to the caller in the first place !
        // On top of that it simplifies the start logic (given it guarantees nothing will
        // concurrently close the handle)
        invalid => Err(invalid),
    })?;

    mountpoint
        .unmount()
        .await
        .map_err(MountpointUnmountError::Internal)
}

/*
 * Get file path in mountpoint
 */

#[derive(Debug, thiserror::Error)]
pub enum MountpointToOsPathError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[cfg(target_arch = "wasm32")]
pub async fn mountpoint_to_os_path(
    _mountpoint: Handle,
    _parsec_path: FsPath,
) -> Result<std::path::PathBuf, MountpointToOsPathError> {
    Err(MountpointToOsPathError::Internal(anyhow::anyhow!(
        "Not available in web"
    )))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn mountpoint_to_os_path(
    mountpoint: Handle,
    parsec_path: FsPath,
) -> Result<std::path::PathBuf, MountpointToOsPathError> {
    borrow_from_handle(mountpoint, |x| match x {
        HandleItem::Mountpoint { mountpoint, .. } => Some(mountpoint.to_os_path(&parsec_path)),
        _ => None,
    })
    .map_err(MountpointToOsPathError::Internal)
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

pub async fn workspace_open_file(
    workspace: Handle,
    path: FsPath,
    mode: OpenOptions,
) -> Result<FileDescriptor, WorkspaceOpenFileError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.open_file(path, mode).await
}

pub async fn fd_close(workspace: Handle, fd: FileDescriptor) -> Result<(), WorkspaceFdCloseError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.fd_close(fd).await
}

pub async fn fd_flush(workspace: Handle, fd: FileDescriptor) -> Result<(), WorkspaceFdFlushError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.fd_flush(fd).await
}

pub async fn fd_read(
    workspace: Handle,
    fd: FileDescriptor,
    offset: u64,
    size: u64,
) -> Result<Vec<u8>, WorkspaceFdReadError> {
    let workspace = borrow_workspace(workspace)?;

    let mut buf = Vec::with_capacity(size as usize);
    workspace.fd_read(fd, offset, size, &mut buf).await?;
    Ok(buf)
}

pub async fn fd_resize(
    workspace: Handle,
    fd: FileDescriptor,
    length: u64,
    truncate_only: bool,
) -> Result<(), WorkspaceFdResizeError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.fd_resize(fd, length, truncate_only).await
}

pub async fn fd_write(
    workspace: Handle,
    fd: FileDescriptor,
    offset: u64,
    data: Vec<u8>,
) -> Result<u64, WorkspaceFdWriteError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.fd_write(fd, offset, &data).await
}

pub async fn fd_write_with_constrained_io(
    workspace: Handle,
    fd: FileDescriptor,
    offset: u64,
    data: Vec<u8>,
) -> Result<u64, WorkspaceFdWriteError> {
    let workspace = borrow_workspace(workspace)?;

    workspace
        .fd_write_with_constrained_io(fd, offset, &data)
        .await
}
