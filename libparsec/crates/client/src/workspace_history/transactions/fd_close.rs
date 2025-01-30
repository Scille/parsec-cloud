// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::workspace_history::WorkspaceHistoryOps;

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryFdCloseError {
    #[error("File descriptor not found")]
    BadFileDescriptor,
    // Internal used in main `libparsec` crate to handle bad workspace handle.
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(crate) fn fd_close(
    ops: &WorkspaceHistoryOps,
    fd: FileDescriptor,
) -> Result<(), WorkspaceHistoryFdCloseError> {
    let mut cache = ops.rw.lock().expect("Mutex is poisoned");

    match cache.opened_files.remove(&fd) {
        Some(_) => Ok(()),
        None => Err(WorkspaceHistoryFdCloseError::BadFileDescriptor),
    }
}
