// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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
    DieselDatabaseError(
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
            diesel::result::Error::DatabaseError(kind, err) => Self::DieselDatabaseError(kind, err),
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

/// A result wrapper that return [DatabaseError] on error.
pub type DatabaseResult<T> = Result<T, DatabaseError>;
