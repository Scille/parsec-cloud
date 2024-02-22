// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{ops::DerefMut, sync::Arc};

use libparsec_types::prelude::*;

use crate::workspace::{OpenedFile, WorkspaceOps, WriteMode};

use super::ReshapeAndFlushError;

#[derive(Debug, thiserror::Error)]
pub enum FdWriteError {
    #[error("File descriptor not found")]
    BadFileDescriptor,
    #[error("File is not opened in write mode")]
    NotInWriteMode,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn fd_write(
    ops: &WorkspaceOps,
    fd: FileDescriptor,
    buf: &[u8],
    constrained_io: bool,
) -> Result<u64, FdWriteError> {
    // Retrieve the opened file & cursor from the file descriptor

    let opened_file = {
        let guard = ops.opened_files.lock().expect("Mutex is poisoned");

        let file_id = match guard.file_descriptors.get(&fd) {
            Some(file_id) => file_id,
            None => return Err(FdWriteError::BadFileDescriptor),
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
        .ok_or(FdWriteError::BadFileDescriptor)?;

    if matches!(cursor.write_mode, WriteMode::Denied) {
        return Err(FdWriteError::NotInWriteMode);
    }

    let buf = if constrained_io && cursor.position + buf.len() as u64 > manifest.size {
        let end = match manifest.size.checked_sub(cursor.position) {
            Some(end) => end,
            // Cursor is past the end of the file, we should not write anything
            None => return Ok(0),
        };
        &buf[..end as usize]
    } else {
        // No early exit if buf is empty: if cursor is past the end of the file a
        // zero-length write means we extend the file !
        buf
    };

    let manifest: &mut LocalFileManifest = Arc::make_mut(manifest);
    let (write_operations, removed_chunks) = super::prepare_write(
        manifest,
        buf.len() as u64,
        cursor.position,
        ops.device.now(),
    );

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
        let chunk_start = write_operation.chunk.start;
        let chunk_stop = write_operation.chunk.stop.get();
        // TODO: replace this by a check when building `Chunk`
        assert!(chunk_start <= chunk_stop);

        let chunk_data_start = write_operation.offset;
        let chunk_data_stop = {
            write_operation
                .offset
                .checked_sub_unsigned(chunk_stop - chunk_start)
                .expect("overflow !")
        };

        let chunk_data = match (chunk_data_start, chunk_data_stop) {
            // 1) This chunk is entirely composed of data from the buffer
            (chunk_data_start, _) if chunk_data_start >= 0 => {
                buf[chunk_data_start as usize..chunk_data_stop as usize].to_vec()
            }
            // 2) This chunk is entirely composed of zeroes filler data
            (_, chunk_data_stop) if chunk_data_stop <= 0 => {
                let chunk_data_size = chunk_data_start
                    .checked_sub(chunk_data_stop)
                    .expect("overflow !") as usize;
                vec![0; chunk_data_size]
            }
            // 3) This chunk contains first some zeroes, then some data from the buffer
            // chunk_data_start < 0 and chunk_data_stop > 0
            (_, _) => {
                let zeroes_filler_size = 0 - chunk_data_start as usize;
                let from_buffer_size = chunk_data_stop as usize;
                let chunk_data_size = zeroes_filler_size + from_buffer_size;
                let mut chunk_data = vec![0; chunk_data_size];
                chunk_data[zeroes_filler_size..].copy_from_slice(&buf[0..from_buffer_size]);
                chunk_data
            }
        };

        opened_file
            .new_chunks
            .push((write_operation.chunk.id, chunk_data));
    }

    opened_file.bytes_written_since_last_flush += buf.len() as u64;
    opened_file.flush_needed = true;

    super::maybe_early_reshape_and_flush(ops, &mut opened_file)
        .await
        .or_else(|err| match err {
            // Given flush is not mandatory here, just ignore if we cannot do it
            ReshapeAndFlushError::Stopped => Ok(()),
            ReshapeAndFlushError::Internal(err) => {
                Err(FdWriteError::Internal(err.context("cannot flush file")))
            }
        })?;

    Ok(buf.len() as u64)
}
