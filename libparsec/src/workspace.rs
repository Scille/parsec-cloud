// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

pub use libparsec_client::workspace::{
    EntryStat, MoveEntryMode, OpenOptions, WorkspaceCreateFileError, WorkspaceCreateFolderError,
    WorkspaceDecryptPathAddrError, WorkspaceFdCloseError, WorkspaceFdFlushError,
    WorkspaceFdReadError, WorkspaceFdResizeError, WorkspaceFdWriteError,
    WorkspaceGeneratePathAddrError, WorkspaceMoveEntryError, WorkspaceOpenFileError,
    WorkspaceRemoveEntryError, WorkspaceStatEntryError, WorkspaceStatFolderChildrenError,
};
use libparsec_client::{
    EventWorkspaceOpsInboundSyncDone, EventWorkspaceOpsOutboundSyncNeeded,
    EventWorkspaceWatchedEntryChanged,
};
use libparsec_platform_async::event::{Event, EventListener};
use libparsec_types::prelude::*;

use crate::handle::{
    borrow_from_handle, filter_close_handles, register_handle_with_init, take_and_close_handle,
    EntryWatcher, FilterCloseHandle, Handle, HandleItem,
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
    #[error("Cannot refresh workspace: {}", .0)]
    CannotRefreshWorkspace(#[from] libparsec_client::workspace::WorkspaceSyncError),
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
                    ..
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
    workspace_ops.refresh_realm_checkpoint().await?;
    loop {
        let entries = workspace_ops
            .get_need_inbound_sync(32)
            .await
            .map_err(anyhow::Error::from)?;
        if entries.is_empty() {
            break;
        }
        for entry in entries {
            workspace_ops
                .inbound_sync(entry)
                .await
                .map_err(anyhow::Error::from)?;
        }
    }

    // 3. Finally register the workspace to get a handle

    let workspace_handle = initializing.initialized(move |item: &mut HandleItem| {
        let starting = std::mem::replace(
            item,
            HandleItem::Workspace {
                client: client_handle,
                workspace_ops,
                entry_watchers: Default::default(),
            },
        );
        match starting {
            HandleItem::StartingWorkspace {
                to_wake_on_done, ..
            } => {
                for event in to_wake_on_done {
                    event.notify(usize::MAX);
                }
            }
            _ => unreachable!(),
        }
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
            ..
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
 * Get notified on changes in a path
 */

pub type WorkspaceWatchError = WorkspaceStatEntryError;

pub async fn workspace_watch_entry_oneshot(
    workspace: Handle,
    path: FsPath,
) -> Result<VlobID, WorkspaceWatchError> {
    let workspace_handle = workspace;
    let (client_handle, workspace, entry_watchers) =
        borrow_from_handle(workspace_handle, |x| match x {
            HandleItem::Workspace {
                client,
                workspace_ops,
                entry_watchers,
                ..
            } => Some((*client, workspace_ops.clone(), entry_watchers.clone())),
            _ => None,
        })?;

    let event_bus = borrow_from_handle(client_handle, |x| match x {
        HandleItem::Client { on_event, .. } => Some(on_event.event_bus.clone()),
        _ => None,
    })?;

    // 1) Resolve the path to get the entry id

    let stat = workspace.stat_entry(&path).await?;
    let realm_id = workspace.realm_id();
    let entry_id = stat.id();

    // 2) Connect the entry watcher to the event bus

    // Note step 1 and 2 are not atomic, hence a modification occurring between the two
    // steps will not be caught by the watcher. This should be okay though, as step 1
    // is only used to get the entry ID from the path, so we can just pretend the
    // modification occured just before step 1 (it would be different if we were
    // returning the stat obtained from step 1 to the caller).

    {
        let mut entry_watchers_guard = entry_watchers.lock().expect("Mutex is poisoned");
        entry_watchers_guard.last_id += 1;
        let entry_watcher_id = entry_watchers_guard.last_id;

        let on_event_triggered = {
            let entry_watchers = entry_watchers.clone();
            let event_bus = event_bus.clone();

            move |candidate_realm_id: VlobID, candidate_entry_id: VlobID| {
                if candidate_realm_id != realm_id || candidate_entry_id != entry_id {
                    return;
                }

                // Disconnect the entry watcher since it is a one-shot

                {
                    let mut entry_watchers_guard =
                        entry_watchers.lock().expect("Mutex is poisoned");
                    let index = entry_watchers_guard
                        .watchers
                        .iter()
                        .position(|x| x.id == entry_watcher_id);
                    match index {
                        // The watcher is already disconnected, we should not have received the event !
                        None => return,
                        Some(index) => {
                            let watcher = entry_watchers_guard.watchers.swap_remove(index);
                            // The watcher object contains an event bus connection lifetime
                            // about the event type we are currently handling.
                            // Given each event type has its own dedicated lock, dropping
                            // the lifetime here would created a deadlock !
                            // So instead we spawn a task to do the drop later.
                            libparsec_platform_async::spawn(async move {
                                drop(watcher);
                            });
                        }
                    }
                }

                // Dispatch the watcher event

                let event = EventWorkspaceWatchedEntryChanged { realm_id, entry_id };
                // Sending a event from within an event handler :/
                // This is fine as long as the event currently handled and
                // the one being send are of a different type (as each event
                // type has it own dedicated lock).
                event_bus.send(&event);
            }
        };

        let on_local_change_lifetime = event_bus.connect({
            let on_event_triggered = on_event_triggered.clone();
            move |e: &EventWorkspaceOpsOutboundSyncNeeded| {
                on_event_triggered(e.realm_id, e.entry_id)
            }
        });

        let on_remote_change_lifetime = event_bus.connect({
            move |e: &EventWorkspaceOpsInboundSyncDone| on_event_triggered(e.realm_id, e.entry_id)
        });

        entry_watchers_guard.watchers.push(EntryWatcher {
            id: entry_watcher_id,
            lifetimes: (on_local_change_lifetime, on_remote_change_lifetime),
        });
    }

    Ok(entry_id)
}

/*
 * Mount workspace
 */

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceMountError {
    #[error("Mountpoint is disabled")]
    Disabled,
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

    if matches!(
        workspace.config().mountpoint_mount_strategy,
        libparsec_client::MountpointMountStrategy::Disabled
    ) {
        return Err(WorkspaceMountError::Disabled);
    }

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
    let mountpoint_handle = initializing.initialized(move |item: &mut HandleItem| {
        let starting = std::mem::replace(
            item,
            HandleItem::Mountpoint {
                client: client_handle,
                workspace: workspace_handle,
                mountpoint,
            },
        );
        match starting {
            HandleItem::StartingMountpoint {
                to_wake_on_done, ..
            } => {
                for event in to_wake_on_done {
                    event.notify(usize::MAX);
                }
            }
            _ => unreachable!(),
        }
    });

    Ok((mountpoint_handle, mountpoint_path))
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

pub async fn workspace_stat_entry_by_id(
    workspace: Handle,
    entry_id: VlobID,
) -> Result<EntryStat, WorkspaceStatEntryError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.stat_entry_by_id(entry_id).await
}

pub async fn workspace_stat_folder_children(
    workspace: Handle,
    path: &FsPath,
) -> Result<Vec<(EntryName, EntryStat)>, WorkspaceStatFolderChildrenError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.stat_folder_children(path).await
}

pub async fn workspace_stat_folder_children_by_id(
    workspace: Handle,
    entry_id: VlobID,
) -> Result<Vec<(EntryName, EntryStat)>, WorkspaceStatFolderChildrenError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.stat_folder_children_by_id(entry_id).await
}

pub async fn workspace_move_entry(
    workspace: Handle,
    src: FsPath,
    dst: FsPath,
    mode: MoveEntryMode,
) -> Result<(), WorkspaceMoveEntryError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.move_entry(src, dst, mode).await
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
    data: &[u8],
) -> Result<u64, WorkspaceFdWriteError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.fd_write(fd, offset, data).await
}

pub async fn fd_write_constrained_io(
    workspace: Handle,
    fd: FileDescriptor,
    offset: u64,
    data: &[u8],
) -> Result<u64, WorkspaceFdWriteError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.fd_write_constrained_io(fd, offset, data).await
}

pub async fn fd_write_start_eof(
    workspace: Handle,
    fd: FileDescriptor,
    data: &[u8],
) -> Result<u64, WorkspaceFdWriteError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.fd_write_start_eof(fd, data).await
}

pub async fn workspace_generate_path_addr(
    workspace: Handle,
    path: &FsPath,
) -> Result<ParsecWorkspacePathAddr, WorkspaceGeneratePathAddrError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.generate_path_addr(path).await
}

pub async fn workspace_decrypt_path_addr(
    workspace: Handle,
    link: &ParsecWorkspacePathAddr,
) -> Result<FsPath, WorkspaceDecryptPathAddrError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.decrypt_path_addr(link).await
}
