// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::workspace::WorkspaceOps;

#[derive(Debug, Clone)]
pub struct FileStat {
    pub id: VlobID,
    pub created: DateTime,
    pub updated: DateTime,
    pub base_version: VersionInt,
    pub is_placeholder: bool,
    pub need_sync: bool,
    pub size: SizeInt,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceFdStatError {
    #[error("File descriptor not found")]
    BadFileDescriptor,
}

pub async fn fd_stat(
    ops: &WorkspaceOps,
    fd: FileDescriptor,
) -> Result<FileStat, WorkspaceFdStatError> {
    let opened_file = {
        let guard = ops.opened_files.lock().expect("Mutex is poisoned");

        let file_id = match guard.file_descriptors.get(&fd) {
            Some(file_id) => file_id,
            None => return Err(WorkspaceFdStatError::BadFileDescriptor),
        };

        let opened_file = guard
            .opened_files
            .get(file_id)
            .expect("File descriptor always refers to an opened file");
        opened_file.clone()
    };

    let opened_file = opened_file.lock().await;

    Ok(FileStat {
        id: opened_file.manifest.base.id,
        created: opened_file.manifest.base.created,
        updated: opened_file.manifest.updated,
        base_version: opened_file.manifest.base.version,
        is_placeholder: opened_file.manifest.base.version == 0,
        need_sync: opened_file.manifest.need_sync,
        size: opened_file.manifest.size,
    })
}
