// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::workspace_history::WorkspaceHistoryOps;

#[derive(Debug, Clone)]
pub struct WorkspaceHistoryFileStat {
    pub id: VlobID,
    pub created: DateTime,
    pub updated: DateTime,
    pub version: VersionInt,
    pub size: SizeInt,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryFdStatError {
    #[error("File descriptor not found")]
    BadFileDescriptor,
    // Internal used in main `libparsec` crate to handle bad workspace handle.
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn fd_stat(
    ops: &WorkspaceHistoryOps,
    fd: FileDescriptor,
) -> Result<WorkspaceHistoryFileStat, WorkspaceHistoryFdStatError> {
    let cache = ops.rw.lock().expect("Mutex is poisoned");
    let manifest = cache
        .opened_files
        .get(&fd)
        .ok_or(WorkspaceHistoryFdStatError::BadFileDescriptor)?;

    Ok(WorkspaceHistoryFileStat {
        id: manifest.id,
        created: manifest.created,
        updated: manifest.updated,
        version: manifest.version,
        size: manifest.size,
    })
}
