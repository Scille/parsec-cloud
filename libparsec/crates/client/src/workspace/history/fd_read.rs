// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::{WorkspaceHistoryGetBlockError, WorkspaceHistoryOps};
use crate::{InvalidBlockAccessError, InvalidCertificateError, InvalidKeysBundleError};

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
    #[error("Block doesn't exist on the server")]
    BlockNotFound,
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
        let cache = ops.cache.lock().expect("Mutex is poisoned");
        let manifest = cache
            .opened_files
            .get(&fd)
            .ok_or(WorkspaceHistoryFdReadError::BadFileDescriptor)?;
        manifest.clone()
    };

    let (written_size, chunk_views) =
        crate::workspace::transactions::prepare_read(&manifest, size, offset);
    let mut buf_size = 0;
    for chunk_view in chunk_views {
        let access = chunk_view
            .access
            .as_ref()
            .expect("history only works on blocks");
        let chunk_data = ops
            .get_block(access, &manifest.base)
            .await
            .map_err(|err| match err {
                WorkspaceHistoryGetBlockError::Offline(e) => {
                    WorkspaceHistoryFdReadError::Offline(e)
                }
                WorkspaceHistoryGetBlockError::Stopped
                | WorkspaceHistoryGetBlockError::StoreUnavailable => {
                    WorkspaceHistoryFdReadError::Stopped
                }
                WorkspaceHistoryGetBlockError::NoRealmAccess => {
                    WorkspaceHistoryFdReadError::NoRealmAccess
                }
                WorkspaceHistoryGetBlockError::BlockNotFound => {
                    WorkspaceHistoryFdReadError::BlockNotFound
                }
                WorkspaceHistoryGetBlockError::RealmNotFound => {
                    // The realm doesn't exist on server side, hence we are its creator and
                    // the block has never been uploaded to the server yet.
                    WorkspaceHistoryFdReadError::BlockNotFound
                }
                WorkspaceHistoryGetBlockError::InvalidBlockAccess(err) => {
                    WorkspaceHistoryFdReadError::InvalidBlockAccess(err)
                }
                WorkspaceHistoryGetBlockError::InvalidCertificate(err) => {
                    WorkspaceHistoryFdReadError::InvalidCertificate(err)
                }
                WorkspaceHistoryGetBlockError::InvalidKeysBundle(err) => {
                    WorkspaceHistoryFdReadError::InvalidKeysBundle(err)
                }
                WorkspaceHistoryGetBlockError::Internal(err) => {
                    err.context("cannot read chunk").into()
                }
            })?;
        chunk_view
            .copy_between_start_and_stop(&chunk_data, offset, buf, &mut buf_size)
            .expect("prepare_read/buf/size are consistent");
    }
    if buf_size < written_size as usize {
        buf.write_all(&vec![0; written_size as usize - buf_size])
            .expect("write_all should not fail");
    }
    Ok(written_size)
}
