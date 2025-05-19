// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

pub use libparsec_client::workspace::{
    WorkspaceHistoryEntryStat, WorkspaceHistoryFdCloseError, WorkspaceHistoryFdReadError,
    WorkspaceHistoryFdStatError, WorkspaceHistoryFileStat,
    WorkspaceHistoryGetWorkspaceManifestV1TimestampError, WorkspaceHistoryOpenFileError,
    WorkspaceHistoryStatEntryError, WorkspaceHistoryStatFolderChildrenError,
};
use libparsec_types::prelude::*;

use crate::handle::{Handle, HandleItem, borrow_from_handle};

fn borrow_workspace(workspace: Handle) -> anyhow::Result<Arc<libparsec_client::WorkspaceOps>> {
    borrow_from_handle(workspace, |x| match x {
        HandleItem::Workspace { workspace_ops, .. } => Some(workspace_ops.clone()),
        _ => None,
    })
}

/*
 * Workspace history FS operations
 */

pub async fn workspace_history_get_workspace_manifest_v1_timestamp(
    workspace: Handle,
) -> Result<Option<DateTime>, WorkspaceHistoryGetWorkspaceManifestV1TimestampError> {
    let workspace = borrow_workspace(workspace)?;

    workspace
        .history
        .get_workspace_manifest_v1_timestamp()
        .await
}

pub async fn workspace_history_stat_entry(
    workspace: Handle,
    at: DateTime,
    path: &FsPath,
) -> Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.history.stat_entry(at, path).await
}

pub async fn workspace_history_stat_entry_by_id(
    workspace: Handle,
    at: DateTime,
    entry_id: VlobID,
) -> Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.history.stat_entry_by_id(at, entry_id).await
}

pub async fn workspace_history_stat_folder_children(
    workspace: Handle,
    at: DateTime,
    path: &FsPath,
) -> Result<Vec<(EntryName, WorkspaceHistoryEntryStat)>, WorkspaceHistoryStatFolderChildrenError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.history.stat_folder_children(at, path).await
}

pub async fn workspace_history_stat_folder_children_by_id(
    workspace: Handle,
    at: DateTime,
    entry_id: VlobID,
) -> Result<Vec<(EntryName, WorkspaceHistoryEntryStat)>, WorkspaceHistoryStatFolderChildrenError> {
    let workspace = borrow_workspace(workspace)?;

    workspace
        .history
        .stat_folder_children_by_id(at, entry_id)
        .await
}

pub async fn workspace_history_open_file(
    workspace: Handle,
    at: DateTime,
    path: FsPath,
) -> Result<FileDescriptor, WorkspaceHistoryOpenFileError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.history.open_file(at, path).await
}

pub async fn workspace_history_open_file_by_id(
    workspace: Handle,
    at: DateTime,
    entry_id: VlobID,
) -> Result<FileDescriptor, WorkspaceHistoryOpenFileError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.history.open_file_by_id(at, entry_id).await
}

pub fn workspace_history_fd_close(
    workspace: Handle,
    fd: FileDescriptor,
) -> Result<(), WorkspaceHistoryFdCloseError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.history.fd_close(fd)
}

pub async fn workspace_history_fd_read(
    workspace: Handle,
    fd: FileDescriptor,
    offset: u64,
    size: u64,
) -> Result<Vec<u8>, WorkspaceHistoryFdReadError> {
    let workspace = borrow_workspace(workspace)?;

    let mut buf = Vec::with_capacity(size as usize);
    workspace
        .history
        .fd_read(fd, offset, size, &mut buf)
        .await?;
    Ok(buf)
}

pub async fn workspace_history_fd_stat(
    workspace: Handle,
    fd: FileDescriptor,
) -> Result<WorkspaceHistoryFileStat, WorkspaceHistoryFdStatError> {
    let workspace = borrow_workspace(workspace)?;

    workspace.history.fd_stat(fd).await
}
