// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[derive(Debug, thiserror::Error)]
pub enum ConfigurationError {
    #[error("Fail to create tables: {0}")]
    CreateTables(diesel::result::Error),
    #[error("Database error: {0}")]
    DatabaseError(super::db::DatabaseError),
}

impl From<super::db::DatabaseError> for ConfigurationError {
    fn from(value: super::db::DatabaseError) -> Self {
        Self::DatabaseError(value)
    }
}

impl From<ConfigurationError> for crate::StorageError {
    fn from(value: ConfigurationError) -> Self {
        Self::Internal(Box::new(value))
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Query error on table {table_name}: {error}")]
    Query {
        table_name: &'static str,
        error: diesel::result::Error,
    },
    #[error("Database error: {0}")]
    DatabaseError(super::db::DatabaseError),
}

impl From<super::db::DatabaseError> for Error {
    fn from(value: super::db::DatabaseError) -> Self {
        Self::DatabaseError(value)
    }
}

impl From<Error> for crate::StorageError {
    fn from(value: Error) -> Self {
        Self::Internal(Box::new(value))
    }
}
