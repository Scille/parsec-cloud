// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::workspace::{WorkspaceOps, WriteMode};

use super::{ReshapeAndFlushError, WriteOperation};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceFdResizeError {
    #[error("File descriptor not found")]
    BadFileDescriptor,
    #[error("File is not opened in write mode")]
    NotInWriteMode,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn fd_resize(
    ops: &WorkspaceOps,
    fd: FileDescriptor,
    length: u64,
    truncate_only: bool,
) -> Result<(), WorkspaceFdResizeError> {
    // Retrieve the opened file & cursor from the file descriptor

    let opened_file = {
        let guard = ops.opened_files.lock().expect("Mutex is poisoned");

        let file_id = match guard.file_descriptors.get(&fd) {
            Some(file_id) => file_id,
            None => return Err(WorkspaceFdResizeError::BadFileDescriptor),
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
        .ok_or(WorkspaceFdResizeError::BadFileDescriptor)?;

    if matches!(cursor.write_mode, WriteMode::Denied) {
        return Err(WorkspaceFdResizeError::NotInWriteMode);
    }

    // No-op

    if opened_file.manifest.size == length || (truncate_only && opened_file.manifest.size <= length)
    {
        return Ok(());
    }

    // Actual resize is needed

    let manifest: &mut LocalFileManifest = Arc::make_mut(&mut opened_file.manifest);
    let (write_operations, removed_chunks) =
        super::file_operations::prepare_resize(manifest, length, ops.device.now());

    for removed_chunk in removed_chunks {
        let found = opened_file
            .new_chunks
            .iter()
            .position(|(id, _)| *id == removed_chunk);
        match found {
            Some(index) => {
                opened_file.new_chunks.remove(index);
            }
            None => {
                opened_file.removed_chunks.push(removed_chunk);
            }
        }
    }

    for WriteOperation { chunk, .. } in write_operations {
        let chunk_size = (chunk.stop.get() - chunk.start) as usize;
        let chunk_data = vec![0; chunk_size];
        opened_file.new_chunks.push((chunk.id, chunk_data));
    }

    opened_file.flush_needed = true;

    super::maybe_early_reshape_and_flush(ops, &mut opened_file)
        .await
        .or_else(|err| match err {
            // Given flush is not mandatory here, just ignore if we cannot do it
            ReshapeAndFlushError::Stopped => Ok(()),
            ReshapeAndFlushError::Internal(err) => Err(WorkspaceFdResizeError::Internal(
                err.context("cannot flush file"),
            )),
        })?;

    Ok(())
}
