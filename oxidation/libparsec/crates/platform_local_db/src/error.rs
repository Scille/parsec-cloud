// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Database is closed")]
    Closed,
    #[error("{}", .1.message())]
    DieselDatabaseError(
        diesel::result::DatabaseErrorKind,
        Box<dyn diesel::result::DatabaseErrorInformation + Send + Sync>,
    ),
    #[error("{0}")]
    Diesel(diesel::result::Error),
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
        Self::Diesel(e)
    }
}

impl From<diesel::result::ConnectionError> for DatabaseError {
    fn from(e: diesel::result::ConnectionError) -> Self {
        Self::DieselConnectionError(e)
    }
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;
