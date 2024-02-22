// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::workspace::{store::ReadChunkError, ReadMode, WorkspaceOps};

#[derive(Debug, thiserror::Error)]
pub enum FdReadToEndError {
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

impl From<ConnectionError> for FdReadToEndError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn fd_read_to_end(
    ops: &WorkspaceOps,
    fd: FileDescriptor,
    buf: &mut Vec<u8>,
) -> Result<usize, FdReadToEndError> {
    // Retrieve the opened file & cursor from the file descriptor

    let opened_file = {
        let guard = ops.opened_files.lock().expect("Mutex is poisoned");

        let file_id = match guard.file_descriptors.get(&fd) {
            Some(file_id) => file_id,
            None => return Err(FdReadToEndError::BadFileDescriptor),
        };

        let opened_file = guard
            .opened_files
            .get(file_id)
            .expect("File descriptor always refers to an opened file");
        opened_file.clone()
    };

    let mut opened_file = opened_file.lock().await;

    let cursor = opened_file
        .cursors
        .iter()
        .find(|c| c.file_descriptor == fd)
        // The cursor might have been closed while we were waiting for opened_file's lock
        .ok_or(FdReadToEndError::BadFileDescriptor)?;

    if matches!(cursor.read_mode, ReadMode::Denied) {
        return Err(FdReadToEndError::NotInReadMode);
    }

    let chunks = super::prepare_read(&opened_file.manifest, u64::MAX, cursor.position);
    buf.clear();
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
                let start = chunk.raw_offset - chunk.start;
                let stop = chunk.raw_offset - chunk.stop.get();
                buf.extend_from_slice(&chunk_data[start as usize..stop as usize]);
            }
            None => {
                ops.store
                    .read_chunk(&chunk, buf)
                    .await
                    .map_err(|err| match err {
                        ReadChunkError::Offline => FdReadToEndError::Offline,
                        ReadChunkError::Stopped => FdReadToEndError::Stopped,
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
            }
        }
    }

    // Finally, update the cursor position

    let read_bytes = buf.len();
    let cursor = opened_file
        .cursors
        .iter_mut()
        .find(|c| c.file_descriptor == fd)
        .expect("Already checked fd refers to this opened file");
    cursor.position += read_bytes as u64;

    Ok(read_bytes)
}

// pub async fn fd_read_exact(ops: &WorkspaceOps, fd: FileDescriptor, buf: &mut [u8]) -> Result<(), WorkspaceFdReadToEndError> {
//     transactions::fd_read_exact(self, fd, buf).await
// }

// pub async fn fd_write(ops: &WorkspaceOps, fd: FileDescriptor, buf: &[u8]) -> Result<(), WorkspaceFdReadToEndError> {
//     transactions::fd_write(self, fd, buf).await
// }

// pub async fn fd_flush(ops: &WorkspaceOps, fd: FileDescriptor) -> Result<(), WorkspaceFdReadToEndError> {
//     transactions::fd_flush(self, fd).await
// }

// pub async fn fd_seek(ops: &WorkspaceOps, fd: FileDescriptor, pos: SeekFrom) -> Result<u64, WorkspaceFdReadToEndError> {
//     transactions::fd_seek(self, fd, pos).await
// }
