// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use thiserror::Error;
use uuid::Uuid;

use parsec_api_crypto::CryptoError;
use parsec_api_types::FileDescriptor;

#[derive(Error, Debug)]
pub enum FSError {
    #[error("ConfigurationError")]
    Configuration,

    #[error("ConnectionError")]
    Connection,

    #[error("CreateTableError {0}")]
    CreateTable(&'static str),

    #[error("CreateDirError")]
    CreateDir,

    #[error("CryptoError: {0}")]
    Crypto(CryptoError),

    #[error("DeleteTableError {0}")]
    DeleteTable(&'static str),

    #[error("DropTableError {0}")]
    DropTable(&'static str),

    #[error("Insert table {0}")]
    InsertTable(&'static str),

    #[error("Invalid FileDescriptor {0:?}")]
    InvalidFileDescriptor(FileDescriptor),

    #[error("LocalMissError: {0}")]
    LocalMiss(Uuid),

    #[error("Query table {0}")]
    QueryTable(&'static str),

    #[error("PoolError")]
    Pool,

    #[error("Update table {0}")]
    UpdateTable(&'static str),

    #[error("UserManifest is missing")]
    UserManifestMissing,

    #[error("VacuumError")]
    Vacuum,
}

pub type FSResult<T> = Result<T, FSError>;

impl From<CryptoError> for FSError {
    fn from(e: CryptoError) -> Self {
        Self::Crypto(e)
    }
}
