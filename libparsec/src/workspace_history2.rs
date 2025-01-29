// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

pub use libparsec_client::{
    workspace_history::{
        WorkspaceHistoryEntryStat as WorkspaceHistory2EntryStat,
        WorkspaceHistoryFdCloseError as WorkspaceHistory2FdCloseError,
        WorkspaceHistoryFdReadError as WorkspaceHistory2FdReadError,
        WorkspaceHistoryFdStatError as WorkspaceHistory2FdStatError,
        WorkspaceHistoryFileStat as WorkspaceHistory2FileStat,
        WorkspaceHistoryOpenFileError as WorkspaceHistory2OpenFileError,
        WorkspaceHistorySetTimestampOfInterestError as WorkspaceHistory2SetTimestampOfInterestError,
        WorkspaceHistoryStatEntryError as WorkspaceHistory2StatEntryError,
        WorkspaceHistoryStatFolderChildrenError as WorkspaceHistory2StatFolderChildrenError,
    },
    ClientStartWorkspaceHistoryError as ClientStartWorkspaceHistory2Error,
};
use libparsec_types::prelude::*;

use crate::handle::{
    borrow_from_handle, register_handle_with_init, take_and_close_handle, Handle, HandleItem,
};

fn borrow_workspace_history(
    workspace_history: Handle,
) -> anyhow::Result<Arc<libparsec_client::WorkspaceHistoryOps>> {
    borrow_from_handle(workspace_history, |x| match x {
        HandleItem::WorkspaceHistory {
            workspace_history_ops,
            ..
        } => Some(workspace_history_ops.clone()),
        _ => None,
    })
}

/*
 * Start workspace history
 */

pub async fn client_start_workspace_history2(
    client: Handle,
    realm_id: VlobID,
) -> Result<Handle, ClientStartWorkspaceHistory2Error> {
    // 1. Register the start of the workspace history

    // Unlike for `WorkspaceOps`, it is totally possible to have multiple
    // `WorkspaceHistoryOps` concurrently for the same realm.
    // For this reason `register_handle_with_init` is used in a special way:
    // - We still need it to correctly handle client shutdown (i.e. a client stop
    //   must way for the completion of a concurrent workspace history start in order
    //   to close its handle right away).
    // - We consider each workspace history start totally isolated from each other,
    //   hance having an always succeeding precondition.
    let initializing = register_handle_with_init(
        HandleItem::StartingWorkspaceHistory {
            client,
            to_wake_on_done: vec![],
        },
        |_handle, _item| Result::<_, ()>::Ok(()),
    )
    .expect("Always succeeds");

    // 2. Actually start the workspace

    let client_handle = client;
    let client = borrow_from_handle(client_handle, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let workspace_history_ops = client.start_workspace_history(realm_id).await?;

    // 3. Finally register the workspace history ops to get a handle

    let workspace_history_handle = initializing.initialized(move |item: &mut HandleItem| {
        let starting = std::mem::replace(
            item,
            HandleItem::WorkspaceHistory {
                client: client_handle,
                workspace_history_ops,
            },
        );
        match starting {
            HandleItem::StartingWorkspaceHistory {
                to_wake_on_done, ..
            } => {
                for event in to_wake_on_done {
                    event.notify(usize::MAX);
                }
            }
            _ => unreachable!(),
        }
    });

    Ok(workspace_history_handle)
}

/*
 * Stop workspace history
 */

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistory2InternalOnlyError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub fn workspace_history2_stop(
    workspace_history: Handle,
) -> Result<(), WorkspaceHistory2InternalOnlyError> {
    take_and_close_handle(workspace_history, |x| match x {
        HandleItem::WorkspaceHistory { .. } => {
            // `WorkspaceHistoryOps` has no stop method, dropping it is enough
            Ok(())
        }
        invalid => Err(invalid),
    })
    .map_err(|err| err.into())
}

/*
 * Workspace history FS operations
 */

pub fn workspace_history2_get_timestamp_lower_bound(
    workspace_history: Handle,
) -> Result<DateTime, WorkspaceHistory2InternalOnlyError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    Ok(workspace_history.timestamp_lower_bound())
}

pub fn workspace_history2_get_timestamp_higher_bound(
    workspace_history: Handle,
) -> Result<DateTime, WorkspaceHistory2InternalOnlyError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    Ok(workspace_history.timestamp_higher_bound())
}

pub fn workspace_history2_get_timestamp_of_interest(
    workspace_history: Handle,
) -> Result<DateTime, WorkspaceHistory2InternalOnlyError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    Ok(workspace_history.timestamp_of_interest())
}

pub async fn workspace_history2_set_timestamp_of_interest(
    workspace_history: Handle,
    toi: DateTime,
) -> Result<(), WorkspaceHistory2SetTimestampOfInterestError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.set_timestamp_of_interest(toi).await
}

pub async fn workspace_history2_stat_entry(
    workspace_history: Handle,
    path: &FsPath,
) -> Result<WorkspaceHistory2EntryStat, WorkspaceHistory2StatEntryError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.stat_entry(path).await
}

pub async fn workspace_history2_stat_entry_by_id(
    workspace_history: Handle,
    entry_id: VlobID,
) -> Result<WorkspaceHistory2EntryStat, WorkspaceHistory2StatEntryError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.stat_entry_by_id(entry_id).await
}

pub async fn workspace_history2_stat_folder_children(
    workspace_history: Handle,
    path: &FsPath,
) -> Result<Vec<(EntryName, WorkspaceHistory2EntryStat)>, WorkspaceHistory2StatFolderChildrenError>
{
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.stat_folder_children(path).await
}

pub async fn workspace_history2_stat_folder_children_by_id(
    workspace_history: Handle,
    entry_id: VlobID,
) -> Result<Vec<(EntryName, WorkspaceHistory2EntryStat)>, WorkspaceHistory2StatFolderChildrenError>
{
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.stat_folder_children_by_id(entry_id).await
}

pub async fn workspace_history2_open_file(
    workspace_history: Handle,
    path: FsPath,
) -> Result<FileDescriptor, WorkspaceHistory2OpenFileError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.open_file(path).await
}

pub async fn workspace_history2_open_file_by_id(
    workspace_history: Handle,
    entry_id: VlobID,
) -> Result<FileDescriptor, WorkspaceHistory2OpenFileError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.open_file_by_id(entry_id).await
}

pub async fn workspace_history2_open_file_and_get_id(
    workspace_history: Handle,
    path: FsPath,
) -> Result<(FileDescriptor, VlobID), WorkspaceHistory2OpenFileError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.open_file_and_get_id(path).await
}

pub fn workspace_history2_fd_close(
    workspace_history: Handle,
    fd: FileDescriptor,
) -> Result<(), WorkspaceHistory2FdCloseError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.fd_close(fd)
}

pub async fn workspace_history2_fd_read(
    workspace_history: Handle,
    fd: FileDescriptor,
    offset: u64,
    size: u64,
) -> Result<Vec<u8>, WorkspaceHistory2FdReadError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    let mut buf = Vec::with_capacity(size as usize);
    workspace_history
        .fd_read(fd, offset, size, &mut buf)
        .await?;
    Ok(buf)
}

pub async fn workspace_history2_fd_stat(
    workspace_history: Handle,
    fd: FileDescriptor,
) -> Result<WorkspaceHistory2FileStat, WorkspaceHistory2FdStatError> {
    let workspace_history = borrow_workspace_history(workspace_history)?;

    workspace_history.fd_stat(fd).await
}
