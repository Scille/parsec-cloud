// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{ops::DerefMut, sync::Arc};

use libparsec_types::prelude::*;

use crate::workspace::{OpenedFile, WorkspaceOps, WriteMode};

use super::ReshapeAndFlushError;

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceFdWriteError {
    #[error("File descriptor not found")]
    BadFileDescriptor,
    #[error("File is not opened in write mode")]
    NotInWriteMode,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub enum FdWriteStrategy {
    Normal { offset: u64 },
    ConstrainedIO { offset: u64 },
    StartEOF,
}

pub async fn fd_write(
    ops: &WorkspaceOps,
    fd: FileDescriptor,
    data: &[u8],
    mode: FdWriteStrategy,
) -> Result<u64, WorkspaceFdWriteError> {
    // Retrieve the opened file & cursor from the file descriptor

    let opened_file = {
        let guard = ops.opened_files.lock().expect("Mutex is poisoned");

        let file_id = match guard.file_descriptors.get(&fd) {
            Some(file_id) => file_id,
            None => return Err(WorkspaceFdWriteError::BadFileDescriptor),
        };

        let opened_file = guard
            .opened_files
            .get(file_id)
            .expect("File descriptor always refers to an opened file");
        opened_file.clone()
    };

    let mut opened_file = opened_file.lock().await;
    let OpenedFile {
        cursors, manifest, ..
    } = opened_file.deref_mut();

    let cursor = cursors
        .iter()
        .find(|c| c.file_descriptor == fd)
        // The cursor might have been closed while we were waiting for opened_file's lock
        .ok_or(WorkspaceFdWriteError::BadFileDescriptor)?;

    if matches!(cursor.write_mode, WriteMode::Denied) {
        return Err(WorkspaceFdWriteError::NotInWriteMode);
    }

    // Truncate the data to the right length if the write is constrained
    let (offset, data) = match mode {
        FdWriteStrategy::ConstrainedIO { offset } => {
            if offset + data.len() as u64 > manifest.size {
                let end = manifest.size.saturating_sub(offset);
                (offset, &data[..end as usize])
            } else {
                (offset, data)
            }
        }
        FdWriteStrategy::Normal { offset } => (offset, data),
        FdWriteStrategy::StartEOF => (manifest.size, data),
    };

    // Writing an empty buffer is a no-op
    // (it does **not** extend the file if the offset is past the end of the file)
    if data.is_empty() {
        return Ok(0);
    }

    let manifest: &mut LocalFileManifest = Arc::make_mut(manifest);
    let (write_operations, removed_chunks) =
        super::prepare_write(manifest, data.len() as u64, offset, ops.device.now());

    for to_remove_id in removed_chunks {
        let found = opened_file
            .new_chunks
            .iter()
            .position(|(id, _)| *id == to_remove_id);
        match found {
            Some(to_remove_index) => {
                opened_file.new_chunks.swap_remove(to_remove_index);
            }
            None => {
                opened_file.removed_chunks.push(to_remove_id);
            }
        }
    }

    for write_operation in write_operations {
        let chunk_start = write_operation.chunk_view.start;
        let chunk_stop = write_operation.chunk_view.stop.get();
        // TODO: replace this by a check when building `Chunk`
        assert!(chunk_start <= chunk_stop);

        let chunk_data_start = write_operation.offset;
        let chunk_data_stop = {
            write_operation
                .offset
                .checked_add_unsigned(chunk_stop - chunk_start)
                .expect("overflow !")
        };

        let chunk_data = match (chunk_data_start, chunk_data_stop) {
            // 1) This chunk view is entirely composed of data from the buffer
            (chunk_data_start, _) if chunk_data_start >= 0 => {
                data[chunk_data_start as usize..chunk_data_stop as usize].to_vec()
            }
            // 2) This chunk view is entirely composed of zeroes filler data
            (_, chunk_data_stop) if chunk_data_stop <= 0 => {
                let chunk_data_size = chunk_data_start
                    .checked_sub(chunk_data_stop)
                    .expect("overflow !") as usize;
                vec![0; chunk_data_size]
            }
            // 3) This chunk view contains first some zeroes, then some data from the buffer
            // chunk_data_start < 0 and chunk_data_stop > 0
            (_, _) => {
                let zeroes_filler_size = (0 - chunk_data_start) as usize;
                let from_buffer_size = chunk_data_stop as usize;
                let chunk_data_size = zeroes_filler_size + from_buffer_size;
                let mut chunk_data = vec![0; chunk_data_size];
                chunk_data[zeroes_filler_size..].copy_from_slice(&data[0..from_buffer_size]);
                chunk_data
            }
        };

        opened_file
            .new_chunks
            .push((write_operation.chunk_view.id, chunk_data));
    }

    opened_file.bytes_written_since_last_flush += data.len() as u64;
    opened_file.flush_needed = true;

    super::maybe_early_reshape_and_flush(ops, &mut opened_file)
        .await
        .or_else(|err| match err {
            // Given flush is not mandatory here, just ignore if we cannot do it
            ReshapeAndFlushError::Stopped => Ok(()),
            ReshapeAndFlushError::Internal(err) => Err(WorkspaceFdWriteError::Internal(
                err.context("cannot flush file"),
            )),
        })?;

    Ok(data.len() as u64)
}
