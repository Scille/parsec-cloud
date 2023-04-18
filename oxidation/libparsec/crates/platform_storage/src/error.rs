// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_types::prelude::*;

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Invalid regex pattern `{0}`: {1}")]
    InvalidRegexPattern(String, RegexError),
    #[error("{0}")]
    Internal(DynError),
    #[error("Invalid entry id used for {used_as}: {error}")]
    InvalidEntryID {
        used_as: &'static str,
        error: &'static str,
    },
    #[error("Chunk {0} not found in storage")]
    LocalChunkIDMiss(ChunkID),
    #[error("Block {0} not found in storage")]
    LocalBlockIDMiss(BlockID),
    #[error("Entry {0} not found in storage")]
    LocalEntryIDMiss(EntryID),
    #[error("{0}")]
    Crypto(CryptoError),
    #[error("Fail to vacuum: {0}")]
    Vacuum(DynError),
}

impl From<CryptoError> for StorageError {
    fn from(value: CryptoError) -> Self {
        Self::Crypto(value)
    }
}

pub type DynError = Box<dyn std::error::Error + Send + Sync>;
