// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
//! Manage a local database connection.

#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_safety_doc)]
#![deny(clippy::future_not_send)]
#![deny(clippy::undocumented_unsafe_blocks)]

use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

use diesel::{connection::SimpleConnection, sqlite::SqliteConnection, Connection};
use executor::{ExecJob, SqliteExecutor};

mod error;
mod executor;
mod option;
#[cfg(feature = "test-utils")]
mod test_utils;

pub use error::{DatabaseError, DatabaseResult};
use libparsec_platform_async::futures::TryFutureExt;
pub use option::AutoVacuum;
use option::SqliteOptions;
#[cfg(feature = "test-utils")]
pub use test_utils::{test_clear_local_db_in_memory_mock, test_toggle_local_db_in_memory_mock};

/// Maximum number of parameters to be sent in a single SQL query.
/// In theory SQLite provide a `SQLITE_MAX_VARIABLE_NUMBER` that is:
///
/// - 999 for sqlite < 3.32.0
/// - 32_766 for sqlite >= 3.32.0
///
/// ref: <https://www.sqlite.org/limits.html#max_variable_number>
///
/// However this misleading given the query will most likely reach other limits
/// before this one when passing a huge number of params:
///
/// - `SELECT (?, ?, ...)` will reach `SQLITE_MAX_COLUMN`.
/// - `SELECT 1 WHERE ? AND ? AND ...` will reach `SQLITE_MAX_EXPR_DEPTH`.
///
/// So to reach `SQLITE_MAX_VARIABLE_NUMBER`, one should put multiple SQL expression
/// in a single execution: `SELECT (?, ?, ...); SELECT (?, ?, ...); ...`
pub const LOCAL_DATABASE_MAX_VARIABLE_NUMBER: usize = 999;

/// How vacuumming should be done for the database.
#[derive(Debug, Clone, Copy)]
pub enum VacuumMode {
    /// Use a threshold, when the disk usage is above the provided value calling [LocalDatabase::vacuum] will reduce the disk usage.
    /// We use this threshold to limit how often we run a full vacuum (meaning executing `VACUUM` and re-opening the database).
    WithThreshold(usize),
    /// Configure the sqlite driver to vacuum for us.
    Automatic(AutoVacuum),
}

impl VacuumMode {
    /// Return the auto vacuum value (if set).
    fn auto_vacuum(&self) -> Option<AutoVacuum> {
        if let VacuumMode::Automatic(value) = self {
            Some(*value)
        } else {
            None
        }
    }
}

impl Default for VacuumMode {
    fn default() -> Self {
        Self::Automatic(AutoVacuum::Full)
    }
}

/// Help manage and execute sql query to sqlite in an async manner.
pub struct LocalDatabase {
    /// The database path that was provided in [LocalDatabase::from_path].
    path: PathBuf,
    /// Flag that will be `true` when the database has been open in memory (RAM).
    #[cfg(feature = "test-utils")]
    is_in_memory: bool,
    /// The executor that will execute the sql query.
    executor: Mutex<Option<SqliteExecutor>>,
    /// How we vacuum the database.
    vacuum_mode: VacuumMode,
}

impl LocalDatabase {
    /// Create a new database connection with the provided flag.
    ///
    /// Since `test-utils` is enabled, it will try to open a database in memory if we configured it to allow it.
    ///
    /// # Errors
    ///
    /// We can have an error when we can't create a database connection in memory.
    #[cfg(feature = "test-utils")]
    pub async fn from_path(path: &str, vacuum_mode: VacuumMode) -> DatabaseResult<Self> {
        let real_path = AsRef::<Path>::as_ref(&path).to_path_buf();

        let (executor, is_in_memory) = match test_utils::maybe_open_sqlite_in_memory(&real_path) {
            Some(executor) => (executor, true),
            None => {
                let executor =
                    new_sqlite_connection_from_path(path, vacuum_mode.auto_vacuum()).await?;
                (executor, false)
            }
        };
        Ok(Self {
            path: real_path,
            is_in_memory,
            executor: Mutex::new(Some(executor)),
            vacuum_mode,
        })
    }

    /// Create a new database connection with the provided path.
    ///
    /// # Errors
    ///
    /// We can have an error if we can't open the connection to the database.
    #[cfg(not(feature = "test-utils"))]
    pub async fn from_path(path: &str, vacuum_mode: VacuumMode) -> DatabaseResult<Self> {
        let executor = new_sqlite_connection_from_path(path, vacuum_mode.auto_vacuum()).await?;
        Ok(Self {
            path: AsRef::<Path>::as_ref(&path).to_path_buf(),
            executor: Mutex::new(Some(executor)),
            vacuum_mode,
        })
    }
}

/// Create a new [SqliteExecutor] from the provided `path`.
async fn new_sqlite_connection_from_path(
    path: &str,
    auto_vacuum: Option<option::AutoVacuum>,
) -> DatabaseResult<SqliteExecutor> {
    if let Some(prefix) = PathBuf::from(path).parent() {
        tokio::fs::create_dir_all(prefix).await.map_err(|e| {
            diesel::result::ConnectionError::BadConnection(format!(
                "Can't create sub-directory `{}`: {e}",
                prefix
                    .to_str()
                    .expect("We generate the Path from a `&str` so UTF-8 is already checked")
            ))
        })?;
    }

    let connection = SqliteConnection::establish(path)?;
    let path = path.to_string();
    let executor = SqliteExecutor::spawn(connection, move |_conn| {
        SqliteConnection::establish(&path).map_err(DatabaseError::from)
    });
    let pragma_options = SqliteOptions::default()
        .journal_mode(option::JournalMode::Wal)
        .synchronous(option::Synchronous::Normal)
        .to_sql_batch_query();

    executor
        .exec(move |conn| conn.batch_execute(&pragma_options))
        .send()
        .and_then(|_| async {
            if let Some(auto_vacuum) = auto_vacuum {
                auto_vacuum.safely_set_value(&executor).await?;
            }
            Ok(())
        })
        .await
        .and(Ok(executor))
}

impl LocalDatabase {
    /// Close the actual connection to the database.
    pub fn close(&self) {
        let _executor = self.executor.lock().expect("Mutex is poisoned").take();
        #[cfg(feature = "test-utils")]
        if let Some(executor) = _executor {
            if self.is_in_memory {
                test_utils::return_sqlite_in_memory_db(&self.path, executor);
            }
        }
    }

    /// Return an approximate amount of bytes sqlite is using on the filesystem.
    pub async fn get_disk_usage(&self) -> usize {
        use std::ffi::OsStr;

        /// Get the size of a file.
        async fn get_file_size(path: &Path) -> usize {
            tokio::fs::metadata(path)
                .await
                .map(|meta| meta.len() as usize)
                .ok()
                .unwrap_or_default()
        }

        /// Add a suffix to a file extension.
        /// I.e.: with the suffix `-wal`, `.sqlite3` become `.sqlite3-wal`.
        fn add_suffix_to_extension(path: &Path, suffix: &str) -> PathBuf {
            let mut extension = path.extension().unwrap_or_default().to_os_string();

            extension.extend([AsRef::<OsStr>::as_ref(suffix)]);

            path.with_extension(extension)
        }

        let wal_filename = add_suffix_to_extension(&self.path, "-wal");
        let shared_filename = add_suffix_to_extension(&self.path, "-shm");

        get_file_size(&self.path).await
            + get_file_size(&wal_filename).await
            + get_file_size(&shared_filename).await
    }

    /// Vacuum the database shrink it's size.
    /// The method will do nothing if we have configured the auto vacuum to [AutoVacuum::Full].
    ///
    /// # Errors
    ///
    /// Can fail if we can't execute the vacuum query.
    pub async fn vacuum(&self) -> DatabaseResult<()> {
        match self.vacuum_mode {
            VacuumMode::Automatic(_) => Ok(()),
            VacuumMode::WithThreshold(threshold) => {
                let disk_usage = self.get_disk_usage().await;

                if disk_usage < threshold {
                    return Ok(());
                }

                let vacuum_job = {
                    let guard = self.executor.lock().expect("Mutex is poisoned");
                    guard
                        .as_ref()
                        .map(|exec| exec.full_vacuum())
                        .ok_or(DatabaseError::Closed)
                }?;
                let res = self.exec_job(vacuum_job).await;

                if res.is_err() {
                    self.close();
                }
                res
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
    /// Like [LocalDatabase::exec] but allow to provide a closure that transform the resulting error (if any) into another error type.
    ///
    /// # Errors
    ///
    /// Will return a error on the same condition in [LocalDatabase::exec] so refer to it.
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

    /// Execute the provided closure.
    ///
    /// # Errors
    ///
    /// Will return an error if the database is close or if the job execution result in error, see [LocalDatabase::exec_job] for more information.
    pub async fn exec<F, R>(&self, job: F) -> DatabaseResult<R>
    where
        F: (FnOnce(&mut SqliteConnection) -> diesel::result::QueryResult<R>) + Send + 'static,
        R: Send + 'static,
    {
        let job = {
            let guard = self.executor.lock().expect("Mutex is poisoned");
            guard
                .as_ref()
                .map(|exec| exec.exec(job))
                .ok_or(DatabaseError::Closed)
        }?;
        self.exec_job(job).await
    }

    /// Execute a `ExecJob` and parse its result.
    ///
    /// # Errors
    ///
    /// Will return a errors if the execution of the job failed for any reason.
    ///
    /// Note: If the error is considered as critical, it will consider that the driver can't be used and will close the connection like [LocalDatabase::close].
    async fn exec_job<R, E>(&self, job: ExecJob<Result<R, E>>) -> DatabaseResult<R>
    where
        R: Send + 'static,
        DatabaseError: From<E>,
        E: Send,
    {
        let res = job.send().await?.map_err(DatabaseError::from);
        if let Err(DatabaseError::DieselDatabaseError(kind, _err)) = res.as_ref() {
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
                    // TODO: improve logging with tracing or log see: #3930
                    eprintln!("The sqlite connection shouldn't be close at that step");
                    self.close();
                }
                // And on unknown error, we could be more picky and only close the connection on specific unknown error (for example only close the connection on `disk full`)
                // But checking for those is hard and implementation specific (we need to check against a `&str` which formatting could change).
                _ => {
                    // TODO: improve logging with tracing or log see: #3930
                    eprintln!("Diesel unknown error: {kind:?}");
                    self.close();
                }
            }
        }
        res
    }
}

impl std::fmt::Debug for LocalDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LocalDatabase");
        fmt.field("path", &self.path)
            .field("vacuum_mode", &self.vacuum_mode);

        #[cfg(feature = "test-utils")]
        {
            fmt.field("is_in_memory", &self.is_in_memory);
        }

        fmt.finish_non_exhaustive()
    }
}
