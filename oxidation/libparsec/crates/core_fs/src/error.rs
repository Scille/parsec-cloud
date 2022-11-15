// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use thiserror::Error;
use uuid::Uuid;

use libparsec_crypto::CryptoError;
use libparsec_types::{EntryID, FileDescriptor};

lazy_static::lazy_static! {
    pub static ref SQLITE_DISK_FULL_MSG: String = {
        use std::ffi::CStr;

        // SAFETY: `sqlite3_errstr` convert an sqlite3 error code into an error message.
        // Here we want to get the error message for the `disk full` error type.
        // for that we have the correct error type `SQLITE_FULL`.
        let disk_full_error_msg_raw = unsafe {
            CStr::from_ptr(libsqlite3_sys::sqlite3_errstr(libsqlite3_sys::SQLITE_FULL))
        };
        disk_full_error_msg_raw.to_string_lossy().to_string()
    };
}

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

    #[error("Database query error: {0}")]
    DatabaseQueryError(String),

    #[error("Database is closed: {0}")]
    DatabaseClosed(String),

    #[error("No space left on device")]
    NoSpaceLeftOnDevice,

    #[error("Database operational error: {0}")]
    DatabaseOperationalError(String),

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

    #[error("Entry `{0}` modified without being locked")]
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

impl From<diesel::result::Error> for FSError {
    fn from(e: diesel::result::Error) -> Self {
        use diesel::result::{DatabaseErrorKind, Error};

        match e {
            Error::DatabaseError(DatabaseErrorKind::ClosedConnection, e) => {
                Self::DatabaseClosed(e.message().to_string())
            }
            Error::DatabaseError(_, err_msg) if err_msg.message() == *SQLITE_DISK_FULL_MSG => {
                Self::NoSpaceLeftOnDevice
            }
            Error::DatabaseError(_kind, msg) => {
                Self::DatabaseOperationalError(msg.message().to_string())
            }
            _ => Self::DatabaseQueryError(e.to_string()),
        }
    }
}
