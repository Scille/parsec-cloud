// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::workspace::WorkspaceOps;

use super::ReshapeAndFlushError;

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceFdCloseError {
    #[error("Component has stopped")]
    Stopped,
    #[error("File descriptor not found")]
    BadFileDescriptor,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn fd_close(ops: &WorkspaceOps, fd: FileDescriptor) -> Result<(), WorkspaceFdCloseError> {
    // 1) Retrieve the opened file from the file descriptor

    let (opened_file, still_opened) = {
        let mut guard = ops.opened_files.lock().expect("Mutex is poisoned");

        let file_id = match guard.file_descriptors.remove(&fd) {
            Some(file_id) => file_id,
            None => return Err(WorkspaceFdCloseError::BadFileDescriptor),
        };

        // Check is the file is opened by another file descriptor.
        // - In this case we should only remove our cursor.
        // - Otherwise, we should entirely destroy the opened file and it underlying updater lock.
        //
        // Note it could be tempting to use `opened_file.cursors.len() != 0` to check if the file
        // has been opened multiple times. However, this would be incorrect if we do this check
        // with a concurrent open (given the open must release the `ops.opened_files` sync lock
        // before taking the async lock to update `opened_file` with the new cursor).

        let still_opened = guard
            .file_descriptors
            .iter()
            .any(|(_, candidate_file_id)| *candidate_file_id == file_id);

        let opened_file = match guard.opened_files.entry(file_id) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                if !still_opened {
                    entry.remove()
                } else {
                    entry.get().clone()
                }
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                unreachable!("File descriptor always refers to an opened file")
            }
        };

        (opened_file, still_opened)
    };

    // /!\ From now on, no error should be returned !!! /!\
    //
    // We are currently in a strange state where the file descriptor is no longer
    // accessible, but the opened_file object (and it underlying updater) is still alive.
    // What this means is if we abruptly return while the file is opened once, the un-flushed
    // data will simply be lost and the updater will never be gracefully closed (leading to
    // a crash on drop).
    //
    // In practice the only operation that could fail is the flush. So we capture its outcome,
    // finish the close no matter what, then finally return the outcome as our own result.

    // 2) Destroy the cursor corresponding to the file descriptor, and flush the changes if needed

    let flush_outcome = {
        let mut opened_file = opened_file.lock().await;

        // Remove our cursor

        let cursor_index = opened_file
            .cursors
            .iter()
            .position(|c| c.file_descriptor == fd)
            // The cursor might have been closed while we were waiting for opened_file's lock
            .ok_or(WorkspaceFdCloseError::BadFileDescriptor)?;
        opened_file.cursors.swap_remove(cursor_index);

        // Flush the changes
        super::force_reshape_and_flush(ops, &mut opened_file)
            .await
            .map_err(|err| match err {
                ReshapeAndFlushError::Stopped => WorkspaceFdCloseError::Stopped,
                ReshapeAndFlushError::Internal(err) => err.context("cannot flush file").into(),
            }) // No ? here: we want to first finish the close before returning the error !
    };

    // 3) Close the updater if the file is no longer opened

    if !still_opened {
        // Last step is to close the updater so that the file manifest is no longer locked
        // in the store.
        //
        // To do that we need to take ownership of the updater, which is not easy given it
        // is stored in an Arc !
        //
        // However there is a trick here: given `opened_file` is no longer referenced in
        // `ops.opened_files`, only the operations that are currently running can have a
        // reference on it.
        // On top of that, those operations should fail as soon as they realize `opened_file`
        // no longer contains their file descriptor's cursor.
        // So we can just wait in a loop until the Arc is only referenced by ourselves.
        let mut our_opened_file_ref_wrapper = Some(opened_file);
        loop {
            match Arc::try_unwrap(
                our_opened_file_ref_wrapper
                    .take()
                    .expect("contains our reference"),
            ) {
                // The Arc is single referenced
                Ok(opened_file_mutex) => {
                    // Unwrap the mutex to obtain ownership on the `OpenedFile` object...
                    let opened_file = opened_file_mutex.into_inner();
                    // ...and finally close the updater !
                    opened_file.updater.close(&ops.store);
                    break;
                }
                // The Arc is still referenced by others coroutines...
                Err(our_opened_file_ref) => {
                    our_opened_file_ref.lock().await;
                    // Put back our reference in the wrapper for the next try
                    our_opened_file_ref_wrapper = Some(our_opened_file_ref);
                }
            }
        }
    }

    flush_outcome
}
