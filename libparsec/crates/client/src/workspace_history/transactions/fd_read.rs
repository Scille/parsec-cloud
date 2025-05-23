// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::cmp::{min, Ordering};

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    workspace_history::{WorkspaceHistoryOps, WorkspaceHistoryStoreGetBlockError},
    InvalidBlockAccessError, InvalidCertificateError, InvalidKeysBundleError,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryFdReadError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("File descriptor not found")]
    BadFileDescriptor,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Block access is temporary unavailable on the server")]
    ServerBlockstoreUnavailable,
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
    ops: &WorkspaceHistoryOps,
    fd: FileDescriptor,
    offset: u64,
    size: u64,
    buf: &mut impl std::io::Write,
) -> Result<u64, WorkspaceHistoryFdReadError> {
    let manifest = {
        let cache = ops.rw.lock().expect("Mutex is poisoned");
        let manifest = cache
            .opened_files
            .get(&fd)
            .ok_or(WorkspaceHistoryFdReadError::BadFileDescriptor)?;
        manifest.clone()
    };

    // To better understand what is happening here, let's consider the following example:
    //
    // - We have a 300 bytes long file splitted in 5 blocks.
    // - We do `ops.fd_read(fd, 100, 100, buf)`
    //
    // file start   ...  `offset`<----- `size` bytes ------->`end`  ...   file end
    //     |0                 |100            |200           |300
    //     [ block 1 ][ block 2 ][ block 3 ][ block 4 ][ block 5 ]
    //     0         64         128        192        256         300
    //
    // - `offset` = 100
    // - `size` = 100
    // - `end` = 200
    // - `involved_blocks` = [block 2, block 3, block 4]
    // - When reading block 2: `to_write_start` = 36, `to_write_size` = 28 (i.e. skip the beginning of the block)
    // - When reading block 3: `to_write_start` = 0, `to_write_size` = 64 (i.e. write the whole block)
    // - When reading block 4: `to_write_start` = 0, `to_write_size` = 8 (i.e. skip the end of the block)

    // Sanitize size and offset to fit the manifest
    let offset = min(offset, manifest.size);
    let size = min(
        size,
        manifest
            .size
            .checked_sub(offset)
            .expect("The offset computed above cannot be greater than the manifest size"),
    );
    let end = offset + size;

    // File manifest blocks comes with two important guarantees:
    // - The blocks are sorted by offset, with no overlap.
    // - Each block has a `blocksize` size, except for the last one that can be smaller.

    // Note since zero-filled blocks are not stored, we cannot just use
    // `manifest.blocksize` to determine the range of indexes of involved blocks.
    let involved_blocks = manifest
        .blocks
        .iter()
        // TODO: use binary search to avoid O(n) complexity
        .skip_while(|block| block.offset + block.size.get() <= offset)
        .take_while(|block| block.offset < end);

    let mut current_position = offset;
    for block_access in involved_blocks {
        let to_write_start = match block_access.offset.cmp(&current_position) {
            Ordering::Equal => 0,
            // There is a hole between the last block and the current one.
            Ordering::Greater => {
                let hole_size = block_access.offset - current_position;
                buf.write_all(&vec![0; hole_size as usize])
                    .expect("write_all should not fail");
                current_position += hole_size;

                0
            }
            // Block starts before the current position, this is only possible for
            // the very first involved block.
            Ordering::Less => {
                assert_eq!(current_position, offset);

                current_position - block_access.offset
            }
        };

        // Sanity check to ensure this block contains something to be written,
        // otherwise it should have been skipped when computing `involved_blocks`.
        assert!(to_write_start < block_access.size.get());

        let to_write_size = min(
            block_access.size.get() - to_write_start,
            end - current_position,
        ) as usize;

        // Get back the block

        let block_data: Bytes =
            ops.store
                .get_block(&manifest, block_access)
                .await
                .map_err(|err| match err {
                    WorkspaceHistoryStoreGetBlockError::Offline(e) => {
                        WorkspaceHistoryFdReadError::Offline(e)
                    }
                    WorkspaceHistoryStoreGetBlockError::Stopped
                    | WorkspaceHistoryStoreGetBlockError::ServerBlockstoreUnavailable => {
                        WorkspaceHistoryFdReadError::Stopped
                    }
                    WorkspaceHistoryStoreGetBlockError::NoRealmAccess => {
                        WorkspaceHistoryFdReadError::NoRealmAccess
                    }
                    WorkspaceHistoryStoreGetBlockError::BlockNotFound => {
                        WorkspaceHistoryFdReadError::ServerBlockstoreUnavailable
                    }
                    WorkspaceHistoryStoreGetBlockError::InvalidBlockAccess(err) => {
                        WorkspaceHistoryFdReadError::InvalidBlockAccess(err)
                    }
                    WorkspaceHistoryStoreGetBlockError::InvalidCertificate(err) => {
                        WorkspaceHistoryFdReadError::InvalidCertificate(err)
                    }
                    WorkspaceHistoryStoreGetBlockError::InvalidKeysBundle(err) => {
                        WorkspaceHistoryFdReadError::InvalidKeysBundle(err)
                    }
                    WorkspaceHistoryStoreGetBlockError::Internal(err) => {
                        err.context("cannot read chunk").into()
                    }
                })?;

        // Note `block_data` has been validated against `block_access`, and hence
        // the data are guaranteed to have the size specified in the access.
        let block_data =
            &block_data[to_write_start as usize..to_write_start as usize + to_write_size];
        buf.write_all(block_data)
            .expect("write_all should not fail");

        current_position += to_write_size as u64;
        assert!(current_position <= end);
    }

    // Last hole

    let missing = end - current_position;
    if missing > 0 {
        buf.write_all(&vec![0; missing as usize])
            .expect("write_all should not fail");
    }

    Ok(size)
}
