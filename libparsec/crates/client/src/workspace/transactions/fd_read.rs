// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    workspace::{store::ReadChunkOrBlockError, ReadMode, WorkspaceOps},
    InvalidBlockAccessError, InvalidCertificateError, InvalidKeysBundleError,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceFdReadError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("File descriptor not found")]
    BadFileDescriptor,
    #[error("File is not opened in read mode")]
    NotInReadMode,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error(transparent)]
    InvalidBlockAccess(#[from] Box<InvalidBlockAccessError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
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

    let (written_size, chunk_views) = super::prepare_read(&opened_file.manifest, size, offset);
    let mut buf_size = 0;
    for chunk_view in chunk_views {
        let found = opened_file
            .new_chunks
            .iter()
            .find_map(|(candidate_id, chunk_data)| {
                if *candidate_id == chunk_view.id {
                    Some(chunk_data)
                } else {
                    None
                }
            });
        let chunk_data_bytes: Bytes;
        let chunk_data = match found {
            Some(chunk_data) => chunk_data,
            None => {
                chunk_data_bytes = ops
                    .store
                    .get_chunk_or_block(&chunk_view, &opened_file.manifest.base)
                    .await
                    .map_err(|err| match err {
                        ReadChunkOrBlockError::Offline(e) => WorkspaceFdReadError::Offline(e),
                        ReadChunkOrBlockError::Stopped => WorkspaceFdReadError::Stopped,
                        ReadChunkOrBlockError::NoRealmAccess => WorkspaceFdReadError::NoRealmAccess,
                        ReadChunkOrBlockError::InvalidBlockAccess(err) => {
                            WorkspaceFdReadError::InvalidBlockAccess(err)
                        }
                        ReadChunkOrBlockError::InvalidCertificate(err) => {
                            WorkspaceFdReadError::InvalidCertificate(err)
                        }
                        ReadChunkOrBlockError::InvalidKeysBundle(err) => {
                            WorkspaceFdReadError::InvalidKeysBundle(err)
                        }
                        ReadChunkOrBlockError::ChunkNotFound => anyhow::anyhow!(
                            "Chunk ID {} referenced in local manifest not in local storage !",
                            chunk_view.id
                        )
                        .into(),
                        ReadChunkOrBlockError::Internal(err) => {
                            err.context("cannot read chunk").into()
                        }
                    })?;
                chunk_data_bytes.as_ref()
            }
        };
        chunk_view
            .copy_between_start_and_stop(chunk_data, offset, buf, &mut buf_size)
            .expect("prepare_read/buf/size are consistent");
    }
    if buf_size < written_size as usize {
        buf.write_all(&vec![0; written_size as usize - buf_size])
            .expect("write_all should not fail");
    }
    Ok(written_size)
}
