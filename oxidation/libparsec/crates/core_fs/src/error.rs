// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client_connection::CommandError;
use libparsec_core::RemoteDevicesManagerError;
use thiserror::Error;
use uuid::Uuid;

use libparsec_crypto::CryptoError;
use libparsec_platform_local_db::DatabaseError;
use libparsec_types::{EntryID, FileDescriptor};

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

    #[error("LocalMissError: {0}")]
    LocalMiss(Uuid),

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

impl From<diesel::result::Error> for FSError {
    fn from(e: diesel::result::Error) -> Self {
        use diesel::result::{DatabaseErrorKind, Error};

        match e {
            Error::DatabaseError(DatabaseErrorKind::ClosedConnection, e) => {
                Self::DatabaseClosed(e.message().to_string())
            }
            Error::DatabaseError(_kind, msg) => {
                Self::DatabaseOperationalError(msg.message().to_string())
            }
            _ => Self::DatabaseQueryError(e.to_string()),
        }
    }
}

impl From<diesel::result::ConnectionError> for FSError {
    fn from(e: diesel::result::ConnectionError) -> Self {
        match e {
            diesel::ConnectionError::InvalidCString(e) => {
                Self::Configuration(format!("Invalid c string: {e}"))
            }
            diesel::ConnectionError::BadConnection(e) => {
                Self::Configuration(format!("Bad connection: {e}"))
            }
            diesel::ConnectionError::InvalidConnectionUrl(e) => {
                Self::Configuration(format!("Invalid connection url: {e}"))
            }
            diesel::ConnectionError::CouldntSetupConfiguration(e) => {
                Self::Configuration(format!("Couldnt setup configuration: {e}"))
            }
            _ => Self::Configuration("Unknown error".to_string()),
        }
    }
}

impl From<DatabaseError> for FSError {
    fn from(e: DatabaseError) -> Self {
        match e {
            DatabaseError::Closed => Self::DatabaseClosed("Database is closed".to_string()),
            DatabaseError::DieselDatabaseError(kind, err) => {
                Self::from(diesel::result::Error::DatabaseError(kind, err))
            }
            DatabaseError::Diesel(e) => Self::from(e),
            DatabaseError::DieselConnectionError(e) => Self::from(e),
        }
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
