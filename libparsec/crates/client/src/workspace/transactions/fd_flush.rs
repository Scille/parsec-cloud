// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::workspace::{
    store::{ReadChunkLocalOnlyError, UpdateFileManifestAndContinueError},
    OpenedFile, WorkspaceOps, WriteMode,
};

use super::prepare_reshape;

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceFdFlushError {
    #[error("Component has stopped")]
    Stopped,
    #[error("File descriptor not found")]
    BadFileDescriptor,
    #[error("File is not opened in write mode")]
    NotInWriteMode,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn fd_flush(ops: &WorkspaceOps, fd: FileDescriptor) -> Result<(), WorkspaceFdFlushError> {
    // Retrieve the opened file & cursor from the file descriptor

    let opened_file = {
        let guard = ops.opened_files.lock().expect("Mutex is poisoned");

        let file_id = match guard.file_descriptors.get(&fd) {
            Some(file_id) => file_id,
            None => return Err(WorkspaceFdFlushError::BadFileDescriptor),
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
        .ok_or(WorkspaceFdFlushError::BadFileDescriptor)?;

    if matches!(cursor.write_mode, WriteMode::Denied) {
        return Err(WorkspaceFdFlushError::NotInWriteMode);
    }

    force_reshape_and_flush(ops, &mut opened_file)
        .await
        .map_err(|err| match err {
            ReshapeAndFlushError::Stopped => WorkspaceFdFlushError::Stopped,
            ReshapeAndFlushError::Internal(err) => err.into(),
        })?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ReshapeAndFlushError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn maybe_early_reshape_and_flush(
    ops: &WorkspaceOps,
    opened_file: &mut OpenedFile,
) -> Result<(), ReshapeAndFlushError> {
    if opened_file.bytes_written_since_last_flush >= opened_file.manifest.blocksize.into() {
        force_reshape_and_flush(ops, opened_file).await
    } else {
        Ok(())
    }
}

pub(super) async fn force_reshape_and_flush(
    ops: &WorkspaceOps,
    opened_file: &mut OpenedFile,
) -> Result<(), ReshapeAndFlushError> {
    if !opened_file.flush_needed {
        return Ok(());
    }

    reshape(ops, opened_file).await?;

    opened_file
        .updater
        .update_file_manifest_and_continue(
            &ops.store,
            opened_file.manifest.clone(),
            &opened_file.new_chunks,
            &opened_file.removed_chunks,
        )
        .await
        .map_err(|err| match err {
            UpdateFileManifestAndContinueError::Stopped => ReshapeAndFlushError::Stopped,
            UpdateFileManifestAndContinueError::Internal(err) => {
                err.context("cannot flush data").into()
            }
        })?;

    opened_file.new_chunks.clear();
    opened_file.removed_chunks.clear();
    opened_file.bytes_written_since_last_flush = 0;
    opened_file.flush_needed = false;

    Ok(())
}

async fn reshape(
    ops: &WorkspaceOps,
    opened_file: &mut OpenedFile,
) -> Result<(), ReshapeAndFlushError> {
    let manifest: &mut LocalFileManifest = Arc::make_mut(&mut opened_file.manifest);
    for reshape in prepare_reshape(manifest) {
        // Build the chunk of data resulting of the reshape...
        let mut buf = Vec::with_capacity(reshape.destination().size() as usize);
        let mut reshape_ok = true;
        for chunk in reshape.source().iter() {
            let outcome = ops.store.get_chunk_local_only(chunk).await;
            match outcome {
                Ok(chunk_data) => {
                    chunk
                        .copy_between_start_and_stop(&chunk_data, &mut buf)
                        .expect("write on vec cannot fail");
                }
                Err(ReadChunkLocalOnlyError::ChunkNotFound) => {
                    // ...if some data are missing in local, this reshape operation is not possible
                    // so we simply ignore it by rollback its corresponding changes in the manifest.
                    reshape_ok = false;
                    break;
                }
                Err(ReadChunkLocalOnlyError::Stopped) => return Err(ReshapeAndFlushError::Stopped),
                Err(ReadChunkLocalOnlyError::Internal(err)) => {
                    return Err(err.context("cannot read chunks").into())
                }
            }
        }
        if reshape_ok {
            let new_chunk_id = reshape.destination().id;
            // Remove old chunks
            for to_remove_chunk_id in reshape.cleanup_ids() {
                let found = opened_file
                    .new_chunks
                    .iter()
                    .position(|(id, _)| *id == to_remove_chunk_id);
                match found {
                    Some(index) => {
                        opened_file.new_chunks.remove(index);
                    }
                    None => {
                        opened_file.removed_chunks.push(to_remove_chunk_id);
                    }
                }
            }
            // Add new chunk
            let buf_ref = if reshape.write_back() {
                opened_file.new_chunks.push((new_chunk_id, buf));
                &opened_file
                    .new_chunks
                    .last()
                    .expect("An item has just been pushed")
                    .1
            } else {
                &buf
            };
            // Commit the changes
            reshape.commit(buf_ref);
        }
    }

    Ok(())
}
