// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{DataAccessFetchBlockError, WorkspaceHistoryStore};

pub(crate) type WorkspaceHistoryStoreGetBlockError = DataAccessFetchBlockError;

// #[derive(Debug, thiserror::Error)]
// pub enum WorkspaceHistoryStoreGetBlockError {
//     #[error("Component has stopped")]
//     Stopped,
//     #[error("Cannot reach the server")]
//     Offline,
//     #[error("The block doesn't exist on the server")]
//     BlockNotFound,
//     #[error("Not allowed to access this realm")]
//     NoRealmAccess,
//     #[error("Block access is temporary unavailable on the server")]
//     StoreUnavailable,
//     #[error(transparent)]
//     InvalidBlockAccess(#[from] Box<InvalidBlockAccessError>),
//     #[error(transparent)]
//     InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
//     #[error(transparent)]
//     InvalidCertificate(#[from] Box<InvalidCertificateError>),
//     #[error(transparent)]
//     Internal(#[from] anyhow::Error),
// }

// impl From<ConnectionError> for WorkspaceHistoryStoreGetBlockError {
//     fn from(value: ConnectionError) -> Self {
//         match value {
//             ConnectionError::NoResponse(_) => Self::Offline,
//             err => Self::Internal(err.into()),
//         }
//     }
// }

pub(super) async fn get_block(
    ops: &WorkspaceHistoryStore,
    manifest: &FileManifest,
    access: &BlockAccess,
) -> Result<Bytes, WorkspaceHistoryStoreGetBlockError> {
    // Cache lookup

    {
        let cache = ops.cache.lock().expect("Mutex is poisoned");
        if let Some(block) = cache.get_block(access.id) {
            return Ok(block.to_owned());
        }
    }

    // Cache miss, must fetch from server

    let block = ops.access.fetch_block(manifest, access).await?;

    let mut cache = ops.cache.lock().expect("Mutex is poisoned");
    cache.populate_block(access.id, block.clone());

    Ok(block)
}
