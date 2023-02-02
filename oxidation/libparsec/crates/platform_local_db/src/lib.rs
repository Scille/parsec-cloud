// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{path::PathBuf, sync::RwLock};

use diesel::{connection::SimpleConnection, sqlite::SqliteConnection, Connection};
use executor::SqliteExecutor;

mod error;
mod executor;
#[cfg(feature = "test-utils")]
mod test_utils;

pub use error::{DatabaseError, DatabaseResult};
#[cfg(feature = "test-utils")]
pub use test_utils::{test_clear_local_db_in_memory_mock, test_toggle_local_db_in_memory_mock};

/// Maximum Number Of Host Parameters In A Single SQL Statement
/// https://www.sqlite.org/limits.html#max_variable_number
pub const LOCAL_DATABASE_MAX_VARIABLE_NUMBER: usize = 999;

pub struct LocalDatabase {
    #[cfg(feature = "test-utils")]
    path: String,
    #[cfg(feature = "test-utils")]
    is_in_memory: bool,
    executor: RwLock<Option<SqliteExecutor>>,
}

impl LocalDatabase {
    #[cfg(feature = "test-utils")]
    pub async fn from_path(path: &str) -> DatabaseResult<Self> {
        let (executor, is_in_memory) = match test_utils::maybe_open_sqlite_in_memory(path) {
            Some(executor) => (executor, true),
            None => {
                let conn = new_sqlite_connection_from_path(path).await?;
                (SqliteExecutor::spawn(conn), false)
            }
        };
        Ok(Self {
            path: path.to_string(),
            is_in_memory,
            executor: RwLock::new(Some(executor)),
        })
    }

    #[cfg(not(feature = "test-utils"))]
    pub async fn from_path(path: &str) -> DatabaseResult<Self> {
        let conn = new_sqlite_connection_from_path(path).await?;
        let executor = SqliteExecutor::spawn(conn);
        Ok(Self {
            executor: RwLock::new(Some(executor)),
        })
    }
}

async fn new_sqlite_connection_from_path(path: &str) -> Result<SqliteConnection, DatabaseError> {
    if let Some(prefix) = PathBuf::from(path).parent() {
        // TODO: Create path in async mode with tokio
        // TODO: Is the default permission ok ?
        std::fs::create_dir_all(prefix).map_err(|e| {
            diesel::result::ConnectionError::BadConnection(format!(
                "Can't create sub-directory `{}`: {e}",
                prefix
                    .to_str()
                    .expect("We generate the Path from a `&str` so UTF-8 is already checked")
            ))
        })?;
    }
    let mut connection = SqliteConnection::establish(path)?;
    connection.batch_execute(
        "
        PRAGMA journal_mode = WAL; -- better write-concurrency
        PRAGMA synchronous = NORMAL; -- fsync only in critical moments
    ",
    )?;
    Ok(connection)
}

impl LocalDatabase {
    /// Close the actual connection to the database.
    pub fn close(&self) {
        let _executor = self.executor.write().expect("RwLock is poisoned").take();
        #[cfg(feature = "test-utils")]
        if let Some(executor) = _executor {
            if self.is_in_memory {
                test_utils::return_sqlite_in_memory_db(&self.path, executor);
            }
        }
    }
}

impl Drop for LocalDatabase {
    fn drop(&mut self) {
        self.close()
    }
}

impl LocalDatabase {
    pub async fn exec<F, R>(&self, job: F) -> DatabaseResult<R>
    where
        F: (FnOnce(&mut SqliteConnection) -> diesel::result::QueryResult<R>) + Send + 'static,
        R: Send + 'static,
    {
        let job = {
            let guard = self.executor.read().expect("RwLock is poisoned");
            guard
                .as_ref()
                .map(|exec| exec.exec(job))
                .ok_or(DatabaseError::Closed)
        }?;
        let res = job.send().await?;
        match res {
            Err(diesel::result::Error::DatabaseError(kind, err)) => {
                match kind {
                    diesel::result::DatabaseErrorKind::UniqueViolation
                    | diesel::result::DatabaseErrorKind::ForeignKeyViolation
                    | diesel::result::DatabaseErrorKind::UnableToSendCommand
                    | diesel::result::DatabaseErrorKind::SerializationFailure
                    | diesel::result::DatabaseErrorKind::ReadOnlyTransaction
                    | diesel::result::DatabaseErrorKind::NotNullViolation
                    | diesel::result::DatabaseErrorKind::CheckViolation => (),
                    // We want to remove the ability to send job when the error is `CloseConnection`
                    // (the database is close so no way we could execute those jobs).
                    diesel::result::DatabaseErrorKind::ClosedConnection => {
                        self.close();
                    }
                    // And on unknown error, we could be more picky and only close the connection on specific unknown error (for example only close the connection on `disk full`)
                    // But checking for those is hard and implementation specific (we need to check against a `&str` which formating could change).
                    _ => {
                        eprintln!("Diesel unknown error: {kind:?}");
                        self.close();
                    }
                }
                Err(DatabaseError::DieselDatabaseError(kind, err))
            }
            Err(e) => Err(DatabaseError::Diesel(e)),
            Ok(value) => Ok(value),
        }
    }

    pub async fn exec_with_error_handler<F, HANDLER, R, E>(
        &self,
        executor: F,
        error_handler: HANDLER,
    ) -> Result<R, E>
    where
        F: (FnOnce(&mut SqliteConnection) -> diesel::result::QueryResult<R>) + Send + 'static,
        HANDLER: FnOnce(diesel::result::Error) -> E + Send,
        R: Send + 'static,
        E: From<DatabaseError>,
    {
        self.exec(executor).await.map_err(|e| match e {
            DatabaseError::Diesel(diesel_error) => error_handler(diesel_error),
            _ => E::from(e),
        })
    }
}
