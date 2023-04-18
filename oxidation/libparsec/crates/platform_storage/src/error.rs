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

#[cfg(any(test, feature = "test-utils"))]
impl PartialEq for StorageError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InvalidRegexPattern(l0, l1), Self::InvalidRegexPattern(r0, r1)) => {
                l0 == r0 && l1 == r1
            }
            (Self::Internal(l0), Self::Internal(r0)) | (Self::Vacuum(l0), Self::Vacuum(r0)) => {
                l0.to_string() == r0.to_string()
            }
            (
                Self::InvalidEntryID {
                    used_as: l_used_as,
                    error: l_error,
                },
                Self::InvalidEntryID {
                    used_as: r_used_as,
                    error: r_error,
                },
            ) => l_used_as == r_used_as && l_error == r_error,
            (Self::LocalChunkIDMiss(l0), Self::LocalChunkIDMiss(r0)) => l0 == r0,
            (Self::LocalBlockIDMiss(l0), Self::LocalBlockIDMiss(r0)) => l0 == r0,
            (Self::LocalEntryIDMiss(l0), Self::LocalEntryIDMiss(r0)) => l0 == r0,
            (Self::Crypto(l0), Self::Crypto(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl From<CryptoError> for StorageError {
    fn from(value: CryptoError) -> Self {
        Self::Crypto(value)
    }
}

pub type DynError = Box<dyn std::error::Error + Send + Sync>;
