// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::PathBuf;
use thiserror::Error;

use libparsec_types::{BlockID, DataError, EntryID};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ExportError {
    #[error("Export error: connection failed {path:?}")]
    ConnectionFailed { path: PathBuf },

    #[error("Export error: create file failed {path:?}")]
    CreateFileFailed { path: PathBuf },

    #[error("Export error: create directory failed {path:?}")]
    CreateDirFailed { path: PathBuf },

    #[error("Export error: invalid certificate (author: {author})")]
    InvalidCertificate { author: i64 },

    #[error("Export error: invalid data")]
    InvalidData,

    #[error("Export error: invalid encryption")]
    InvalidEncryption,

    #[error("Export error: invalid EntryID ({0:?})")]
    InvalidEntryID(Vec<u8>),

    #[error("Export error: invalid key {path:?}")]
    InvalidKey { path: PathBuf },

    #[error("Export error: invalid signature")]
    InvalidSignature,

    #[error("Export error: missing block (block_id: {block_id})")]
    MissingSpecificBlock { block_id: BlockID },

    #[error("Export error: missing device")]
    MissingDevice,

    #[error("Export error: missing key {path:?}")]
    MissingKey { path: PathBuf },

    #[error("Export error: missing manifest (EntryID: {entry_id})")]
    MissingManifest { entry_id: EntryID },

    #[error("Export error: missing specific device (author: {author})")]
    MissingSpecificDevice { author: i64 },

    #[error("Export error: missing WorkspaceID")]
    MissingWorkspaceID,

    #[error("Export error: write failed")]
    WriteFailed,
}

impl From<DataError> for ExportError {
    fn from(e: DataError) -> Self {
        match e {
            DataError::Crypto { .. } => Self::InvalidEncryption,
            DataError::Serialization | DataError::Compression => Self::InvalidData,
            DataError::UnexpectedAuthor { .. } | DataError::UnexpectedTimestamp { .. } => {
                Self::InvalidSignature
            }
        }
    }
}
