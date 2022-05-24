// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use thiserror::Error;
use uuid::Uuid;

use parsec_api_crypto::CryptoError;
use parsec_api_types::FileDescriptor;

#[derive(Error, Debug)]
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

    #[error("LocalMissError: {0}")]
    LocalMiss(Uuid),

    #[error("QueryTableError: {0}")]
    QueryTable(String),

    #[error("PermissionError")]
    Permission,

    #[error("PoolError")]
    Pool,

    #[error("UpdateTableError {0}")]
    UpdateTable(String),

    #[error("UserManifest is missing")]
    UserManifestMissing,

    #[error("VacuumError: {0}")]
    Vacuum(String),
}

pub type FSResult<T> = Result<T, FSError>;

impl From<CryptoError> for FSError {
    fn from(e: CryptoError) -> Self {
        Self::Crypto(e)
    }
}
