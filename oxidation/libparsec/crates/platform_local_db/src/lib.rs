// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use diesel::{connection::SimpleConnection, sqlite::SqliteConnection, Connection};
use executor::SqliteExecutor;
use tokio::sync::RwLock;

mod error;
mod executor;
#[cfg(feature = "test-utils")]
mod test_utils;

pub use error::{DatabaseError, DatabaseResult};
#[cfg(feature = "test-utils")]
pub use test_utils::{clear_local_db_in_memory, toggle_local_db_in_memory};

/// Maximum Number Of Host Parameters In A Single SQL Statement
/// https://www.sqlite.org/limits.html#max_variable_number
pub const LOCAL_DATABASE_MAX_VARIABLE_NUMBER: usize = 999;

#[derive(Clone)]
pub struct LocalDatabase {
    #[cfg(feature = "test-utils")]
    executor: Arc<RwLock<Option<Arc<SqliteExecutor>>>>,
    #[cfg(not(feature = "test-utils"))]
    executor: Arc<RwLock<Option<SqliteExecutor>>>,
}

impl LocalDatabase {
    pub async fn from_path(path: &str) -> DatabaseResult<Self> {
        #[cfg(feature = "test-utils")]
        if let Some(local_db) = test_utils::open_local_db_in_memory(path) {
            return local_db;
        }

        let connection = new_sqlite_connection_from_path(path).await?;
        Self::from_sqlite_connection(connection).await
    }

    pub async fn new_in_memory() -> DatabaseResult<Self> {
        let connection = SqliteConnection::establish(":memory:")?;

        Self::from_sqlite_connection(connection).await
    }

    pub async fn from_sqlite_connection(connection: SqliteConnection) -> DatabaseResult<Self> {
        let executor = SqliteExecutor::spawn(connection);
        #[cfg(feature = "test-utils")]
        let executor = Arc::new(executor);
        let executor = Arc::new(RwLock::new(Some(executor)));

        Ok(Self { executor })
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
    pub async fn close(&self) {
        self.executor.write().await.take();
    }
}

impl LocalDatabase {
    pub async fn exec<F, R>(&self, executor: F) -> DatabaseResult<R>
    where
        F: (FnOnce(&mut SqliteConnection) -> diesel::result::QueryResult<R>) + Send + 'static,
        R: Send + 'static,
    {
        let res = self
            .executor
            .read()
            .await
            .as_ref()
            .ok_or(DatabaseError::Closed)?
            .exec(executor)
            .await?;
        match res {
            Err(diesel::result::Error::DatabaseError(kind, err)) => {
                self.executor.write().await.take();
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
        HANDLER: FnOnce(diesel::result::Error) -> E,
        R: Send + 'static,
        E: From<DatabaseError>,
    {
        self.exec(executor).await.map_err(|e| match e {
            DatabaseError::Diesel(diesel_error) => error_handler(diesel_error),
            _ => E::from(e),
        })
    }
}
