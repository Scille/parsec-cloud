// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub use std::io::SeekFrom;
use std::ops::DerefMut;

use libparsec_types::prelude::*;

use crate::workspace::{OpenedFile, WorkspaceOps};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceFdSeekError {
    #[error("File descriptor not found")]
    BadFileDescriptor,
    #[error("Seek to a negative offset")]
    NegativeOffset,
}

pub async fn fd_seek(
    ops: &WorkspaceOps,
    fd: FileDescriptor,
    pos: SeekFrom,
) -> Result<u64, WorkspaceFdSeekError> {
    // Retrieve the opened file & cursor from the file descriptor

    let opened_file = {
        let guard = ops.opened_files.lock().expect("Mutex is poisoned");

        let file_id = match guard.file_descriptors.get(&fd) {
            Some(file_id) => file_id,
            None => return Err(WorkspaceFdSeekError::BadFileDescriptor),
        };

        let opened_file = guard
            .opened_files
            .get(file_id)
            .expect("File descriptor always refers to an opened file");
        opened_file.clone()
    };

    let mut opened_file = opened_file.lock().await;

    let OpenedFile {
        manifest, cursors, ..
    } = opened_file.deref_mut();

    let cursor = cursors
        .iter_mut()
        .find(|c| c.file_descriptor == fd)
        // The cursor might have been closed while we were waiting for opened_file's lock
        .ok_or(WorkspaceFdSeekError::BadFileDescriptor)?;

    match pos {
        SeekFrom::Start(pos) => {
            cursor.position = pos;
        }
        SeekFrom::End(pos) => match manifest.size.checked_add_signed(pos) {
            Some(new_position) => cursor.position = new_position,
            None => return Err(WorkspaceFdSeekError::NegativeOffset),
        },
        SeekFrom::Current(pos) => match cursor.position.checked_add_signed(pos) {
            Some(new_position) => cursor.position = new_position,
            None => return Err(WorkspaceFdSeekError::NegativeOffset),
        },
    }

    Ok(cursor.position)
}
