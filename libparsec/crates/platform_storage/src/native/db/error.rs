// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
//! Module that contain code about error manipulation of `local-db`.

use libparsec_types::prelude::*;

/// Possible errors when manipulating the database.
#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    /// The database has been closed.
    #[error("Database is closed")]
    Closed,
    #[error("Invalid data: {0}")]
    InvalidData(DataError),
    /// The database returned an error.
    #[error("{}", .1.message())]
    DieselDatabase(
        diesel::result::DatabaseErrorKind,
        Box<dyn diesel::result::DatabaseErrorInformation + Send + Sync>,
    ),
    /// Diesel generated an error
    #[error("{0}")]
    Diesel(diesel::result::Error),
    /// Diesel cannot connect to the database.
    #[error("{0}")]
    DieselConnectionError(diesel::result::ConnectionError),
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::DatabaseError(kind, err) => Self::DieselDatabase(kind, err),
            _ => Self::Diesel(e),
        }
    }
}

impl From<diesel::result::ConnectionError> for DatabaseError {
    fn from(e: diesel::result::ConnectionError) -> Self {
        Self::DieselConnectionError(e)
    }
}

impl From<DataError> for DatabaseError {
    fn from(e: DataError) -> Self {
        Self::InvalidData(e)
    }
}

impl From<CryptoError> for DatabaseError {
    fn from(_e: CryptoError) -> Self {
        Self::InvalidData(DataError::Decryption)
    }
}

/// A result wrapper that return [DatabaseError] on error.
pub type DatabaseResult<T> = Result<T, DatabaseError>;
