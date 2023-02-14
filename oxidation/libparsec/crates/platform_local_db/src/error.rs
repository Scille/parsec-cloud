// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
//! Module that contain code about error manipulation of `local-db`.

/// Possible errors when manipulating the database.
#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    /// The database has been closed.
    #[error("Database is closed")]
    Closed,
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

impl PartialEq for DatabaseError {
    fn eq(&self, other: &Self) -> bool {
        use DatabaseError::*;

        match (self, other) {
            (Closed, Closed) => true,
            (DieselConnectionError(left), DieselConnectionError(right)) => left.eq(right),
            (Diesel(left), Diesel(right)) => left.eq(right),
            (DieselDatabaseError(_, left), DieselDatabaseError(_, right)) => {
                left.message() == right.message()
            }
            _ => false,
        }
    }
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

/// A result wrapper that return [DatabaseError] on error.
pub type DatabaseResult<T> = Result<T, DatabaseError>;
