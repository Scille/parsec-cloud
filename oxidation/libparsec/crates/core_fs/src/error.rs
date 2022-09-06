// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use thiserror::Error;
use uuid::Uuid;

use libparsec_crypto::CryptoError;
use libparsec_types::{EntryID, FileDescriptor};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum FSError {
    #[error("ConfigurationError")]
    Configuration,

    #[error("ConnectionError: {0}")]
    Connection(String),

    #[error("CreateTableError: {0}")]
    CreateTable(String),

    #[error("CreateDirError")]
    CreateDir,

    #[error("CryptoError: {0}")]
    Crypto(CryptoError),

    #[error("DeleteTableError: {0}")]
    DeleteTable(String),

    #[error("DropTableError: {0}")]
    DropTable(String),

    #[error("InsertTableError: {0}")]
    InsertTable(String),

    #[error("Invalid FileDescriptor {0:?}")]
    InvalidFileDescriptor(FileDescriptor),

    #[error("Invalid indexes, trying to access {begin}..{end} in data of length {len}")]
    InvalidIndexes {
        begin: usize,
        end: usize,
        len: usize,
    },

    #[error("LocalMissError: {0}")]
    LocalMiss(Uuid),

    #[error("QueryTableError: {0}")]
    QueryTable(String),

    #[error("PermissionError")]
    Permission,

    #[error("PoolError")]
    Pool,

    #[error("Entry `{0}` modified without beeing locked")]
    Runtime(EntryID),

    #[error("UpdateTableError: {0}")]
    UpdateTable(String),

    #[error("UserManifest is missing")]
    UserManifestMissing,

    #[error("VacuumError: {0}")]
    Vacuum(String),

    /// Error returned by [crate::storage::WorkspaceStorageTimestamped]
    /// when requiring more features than it's able to provide.
    #[error("Not implemented: WorkspaceStorage is timestamped")]
    WorkspaceStorageTimestamped,
}

pub type FSResult<T> = Result<T, FSError>;

impl From<CryptoError> for FSError {
    fn from(e: CryptoError) -> Self {
        Self::Crypto(e)
    }
}
