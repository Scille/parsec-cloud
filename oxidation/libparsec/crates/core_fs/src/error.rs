// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client_connection::CommandError;
use libparsec_core::RemoteDevicesManagerError;
use thiserror::Error;

use libparsec_crypto::CryptoError;
use libparsec_types::{BlockID, ChunkID, EntryID, FileDescriptor};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum FSError {
    #[error("{0}")]
    BackendOffline(String),

    #[error("ConfigurationError: {0}")]
    Configuration(String),

    #[error("ConnectionError: {0}")]
    Connection(String),

    #[error("CreateTableError: {0}")]
    CreateTable(String),

    #[error("CreateDirError")]
    CreateDir,

    #[error("CryptoError: {0}")]
    Crypto(CryptoError),

    #[error("{0}")]
    Custom(String),

    #[error("Database query error: {0}")]
    DatabaseQueryError(String),

    #[error("Database is closed: {0}")]
    DatabaseClosed(String),

    #[error("Database operational error: {0}")]
    DatabaseOperationalError(String),

    #[error("{0}")]
    DeviceNotFound(RemoteDevicesManagerError),

    #[error("Invalid FileDescriptor {0:?}")]
    InvalidFileDescriptor(FileDescriptor),

    #[error("Invalid indexes, trying to access {begin}..{end} in data of length {len}")]
    InvalidIndexes {
        begin: usize,
        end: usize,
        len: usize,
    },

    #[error("Invalid realm role certificates: {0}")]
    InvalidRealmRoleCertificates(String),

    #[error("{0}")]
    InvalidTrustchain(RemoteDevicesManagerError),

    #[error("{0}")]
    LocalChunkIDMiss(ChunkID),

    #[error("{0}")]
    LocalBlockIDMiss(BlockID),

    #[error("{0}")]
    LocalEntryIDMiss(EntryID),

    #[error("QueryTableError: {0}")]
    QueryTable(String),

    #[error("PermissionError")]
    Permission,

    #[error("PoolError")]
    Pool,

    #[error("Remote manifest not found: {0}")]
    RemoteManifestNotFound(EntryID),

    #[error("{0}")]
    RemoteOperation(String),

    #[error("Entry `{0}` modified without being locked")]
    Runtime(EntryID),

    #[error("UpdateTableError: {0}")]
    UpdateTable(String),

    #[error("UserManifest is missing")]
    UserManifestMissing,

    #[error("{0}")]
    UserNotFound(RemoteDevicesManagerError),

    #[error("VacuumError: {0}")]
    Vacuum(String),

    #[error("Cannot download vlob while the workspace is in maintenance")]
    WorkspaceInMaintenance,

    #[error("Cannot get workspace roles: no read access")]
    WorkspaceNoReadAccess,

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

impl From<CommandError> for FSError {
    fn from(e: CommandError) -> Self {
        match e {
            CommandError::NoResponse(..) => Self::BackendOffline(e.to_string()),
            _ => Self::RemoteOperation(e.to_string()),
        }
    }
}

impl From<RemoteDevicesManagerError> for FSError {
    fn from(e: RemoteDevicesManagerError) -> Self {
        match e {
            RemoteDevicesManagerError::BackendOffline { .. } => Self::BackendOffline(e.to_string()),
            RemoteDevicesManagerError::InvalidTrustchain { .. } => Self::InvalidTrustchain(e),
            RemoteDevicesManagerError::DeviceNotFound { .. } => Self::DeviceNotFound(e),
            RemoteDevicesManagerError::UserNotFound { .. } => Self::UserNotFound(e),
            _ => Self::RemoteOperation(e.to_string()),
        }
    }
}

impl From<libparsec_platform_storage::StorageError> for FSError {
    fn from(value: libparsec_platform_storage::StorageError) -> Self {
        match value {
            libparsec_platform_storage::StorageError::Internal(_)
            | libparsec_platform_storage::StorageError::InvalidEntryID { .. }
            | libparsec_platform_storage::StorageError::InvalidRegexPattern(_, _) => {
                Self::Custom(value.to_string())
            }
            libparsec_platform_storage::StorageError::LocalChunkIDMiss(id) => {
                Self::LocalChunkIDMiss(id)
            }
            libparsec_platform_storage::StorageError::LocalBlockIDMiss(id) => {
                Self::LocalBlockIDMiss(id)
            }
            libparsec_platform_storage::StorageError::LocalEntryIDMiss(id) => {
                Self::LocalEntryIDMiss(id)
            }
            libparsec_platform_storage::StorageError::Crypto(e) => Self::Crypto(e),
            libparsec_platform_storage::StorageError::Vacuum(e) => Self::Vacuum(e.to_string()),
        }
    }
}
