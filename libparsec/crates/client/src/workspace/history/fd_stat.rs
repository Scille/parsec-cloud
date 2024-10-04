// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::workspace::{FileStat, WorkspaceHistoryOps};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryFdStatError {
    #[error("File descriptor not found")]
    BadFileDescriptor,
}

pub async fn fd_stat(
    ops: &WorkspaceHistoryOps,
    fd: FileDescriptor,
) -> Result<FileStat, WorkspaceHistoryFdStatError> {
    let cache = ops.cache.lock().expect("Mutex is poisoned");
    let manifest = cache
        .opened_files
        .get(&fd)
        .ok_or(WorkspaceHistoryFdStatError::BadFileDescriptor)?;

    Ok(FileStat {
        id: manifest.base.id,
        created: manifest.base.created,
        updated: manifest.updated,
        base_version: manifest.base.version,
        is_placeholder: manifest.base.version == 0,
        need_sync: manifest.need_sync,
        size: manifest.size,
    })
}
