// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::workspace::{store::ReadChunkError, ReadMode, WorkspaceOps};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceFdReadError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("File descriptor not found")]
    BadFileDescriptor,
    #[error("File is not opened in read mode")]
    NotInReadMode,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for WorkspaceFdReadError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn fd_read(
    ops: &WorkspaceOps,
    fd: FileDescriptor,
    offset: u64,
    size: u64,
    buf: &mut impl std::io::Write,
) -> Result<u64, WorkspaceFdReadError> {
    // Retrieve the opened file & cursor from the file descriptor

    let opened_file = {
        let guard = ops.opened_files.lock().expect("Mutex is poisoned");

        let file_id = match guard.file_descriptors.get(&fd) {
            Some(file_id) => file_id,
            None => return Err(WorkspaceFdReadError::BadFileDescriptor),
        };

        let opened_file = guard
            .opened_files
            .get(file_id)
            .expect("File descriptor always refers to an opened file");
        opened_file.clone()
    };

    let opened_file = opened_file.lock().await;

    let cursor = opened_file
        .cursors
        .iter()
        .find(|c| c.file_descriptor == fd)
        // The cursor might have been closed while we were waiting for opened_file's lock
        .ok_or(WorkspaceFdReadError::BadFileDescriptor)?;

    if matches!(cursor.read_mode, ReadMode::Denied) {
        return Err(WorkspaceFdReadError::NotInReadMode);
    }

    let (written_size, chunks) = super::prepare_read(&opened_file.manifest, size, offset);
    for chunk in chunks {
        let found = opened_file
            .new_chunks
            .iter()
            .find_map(|(candidate_id, chunk_data)| {
                if *candidate_id == chunk.id {
                    Some(chunk_data)
                } else {
                    None
                }
            });
        match found {
            Some(chunk_data) => {
                chunk
                    .copy_between_start_and_stop(chunk_data, buf)
                    .expect("prepare_read/buf/size are consistent");
            }
            None => {
                let chunk_data = ops.store.get_chunk(&chunk).await.map_err(|err| match err {
                    ReadChunkError::Offline => WorkspaceFdReadError::Offline,
                    ReadChunkError::Stopped => WorkspaceFdReadError::Stopped,
                    ReadChunkError::BadDecryption => todo!(),
                    ReadChunkError::NoRealmAccess => todo!(),
                    ReadChunkError::StoreUnavailable => todo!(),
                    ReadChunkError::ChunkNotFound => anyhow::anyhow!(
                        "Chunk ID {} referenced in local manifest not in local storage !",
                        chunk.id
                    )
                    .into(),
                    ReadChunkError::Internal(err) => err.context("cannot read chunk").into(),
                })?;

                chunk
                    .copy_between_start_and_stop(&chunk_data, buf)
                    .expect("prepare_read/buf/size are consistent");
            }
        }
    }

    Ok(written_size)
}
