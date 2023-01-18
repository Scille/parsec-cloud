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
pub use test_utils::{test_clear_local_db_in_memory_mock, test_toggle_local_db_in_memory_mock};

/// Maximum Number Of Host Parameters In A Single SQL Statement
/// https://www.sqlite.org/limits.html#max_variable_number
pub const LOCAL_DATABASE_MAX_VARIABLE_NUMBER: usize = 999;

#[derive(Clone)]
pub struct LocalDatabase {
    #[cfg(feature = "test-utils")]
    path: String,
    #[cfg(feature = "test-utils")]
    is_in_memory: bool,
    executor: Arc<RwLock<Option<SqliteExecutor>>>,
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
            executor: Arc::new(RwLock::new(Some(executor))),
        })
    }

    #[cfg(not(feature = "test-utils"))]
    pub async fn from_path(path: &str) -> DatabaseResult<Self> {
        let conn = new_sqlite_connection_from_path(path).await?;
        let executor = SqliteExecutor::spawn(conn);
        Ok(Self {
            executor: Arc::new(RwLock::new(Some(executor))),
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
    pub async fn close(&self) {
        let _executor = self.executor.write().await.take();
        #[cfg(feature = "test-utils")]
        if let Some(executor) = _executor {
            if self.is_in_memory {
                test_utils::return_sqlite_in_memory_db(&self.path, executor);
            }
        }
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
