// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
//! Manage a local database connection.

use std::path::{Path, PathBuf};

use diesel::{connection::SimpleConnection, sqlite::SqliteConnection, Connection};
use executor::SqliteExecutor;

mod error;
mod executor;
mod option;

// We have two ways of mocking database:
// - `test-in-memory-mock`: the legacy global switch (should only be used with the Python tests)
// - `test-with-testbed`: the faster and parallel system (which uses configuration dir as discriminant)
#[cfg(feature = "test-in-memory-mock")]
mod in_memory_mock;
#[cfg(feature = "test-with-testbed")]
mod testbed;

pub use error::{DatabaseError, DatabaseResult};
#[cfg(feature = "test-in-memory-mock")]
pub use in_memory_mock::{test_clear_local_db_in_memory_mock, test_toggle_local_db_in_memory_mock};
pub use option::AutoVacuum;
use option::SqliteOptions;

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

#[cfg(any(feature = "test-in-memory-mock", feature = "test-with-testbed"))]
#[derive(Clone, PartialEq, Debug)]
pub struct DBPathInfo {
    db_relative_path: PathBuf,
    data_base_dir: PathBuf,
}

/// Help manage and execute sql query to sqlite in an async manner.
pub struct LocalDatabase {
    #[cfg(any(feature = "test-in-memory-mock", feature = "test-with-testbed"))]
    path_info: DBPathInfo,
    #[cfg(any(feature = "test-in-memory-mock", feature = "test-with-testbed"))]
    is_in_memory: bool,
    path: PathBuf,
    executor: SqliteExecutor,
    vacuum_mode: VacuumMode,
}

impl LocalDatabase {
    /// Create a new database connection with the provided path.
    ///
    /// # Errors
    ///
    /// We can have an error if we can't open the connection to the database.
    pub async fn from_path(
        data_base_dir: &Path,
        db_relative_path: &Path,
        vacuum_mode: VacuumMode,
    ) -> DatabaseResult<Self> {
        let path = data_base_dir.join(db_relative_path);
        #[cfg(any(feature = "test-in-memory-mock", feature = "test-with-testbed"))]
        let path_info = DBPathInfo {
            data_base_dir: data_base_dir.to_owned(),
            db_relative_path: db_relative_path.to_owned(),
        };
        let conn: Option<SqliteConnection> = None;

        #[cfg(feature = "test-in-memory-mock")]
        let conn = conn.or_else(|| in_memory_mock::maybe_open_sqlite_in_memory(&path_info));
        #[cfg(feature = "test-with-testbed")]
        let conn = conn.or_else(|| testbed::maybe_open_sqlite_in_memory(&path_info));

        #[cfg(any(feature = "test-in-memory-mock", feature = "test-with-testbed"))]
        let is_in_memory = conn.is_some();

        let executor = match conn {
            // In-memory database for testing
            Some(conn) => SqliteExecutor::start(conn, Ok),
            // Actual production code: open the connection on disk
            None => new_sqlite_executor_from_path(&path, vacuum_mode.auto_vacuum()).await?,
        };
        Ok(Self {
            #[cfg(any(feature = "test-in-memory-mock", feature = "test-with-testbed"))]
            path_info,
            #[cfg(any(feature = "test-in-memory-mock", feature = "test-with-testbed"))]
            is_in_memory,
            path,
            executor,
            vacuum_mode,
        })
    }
}

/// Create a new [SqliteExecutor] from the provided `path`.
async fn new_sqlite_executor_from_path(
    path: &Path,
    auto_vacuum: Option<option::AutoVacuum>,
) -> DatabaseResult<SqliteExecutor> {
    // SQlite only support utf8 path (and so does Diesel interface)
    let path_as_string = {
        match path.to_str() {
            Some(path_as_str) => Ok(path_as_str.to_owned()),
            None => Err(DatabaseError::DieselConnectionError(
                diesel::result::ConnectionError::InvalidConnectionUrl(
                    "Non-Utf-8 character found in db path".to_owned(),
                ),
            )),
        }
    }?;
    if let Some(prefix) = path.parent() {
        tokio::fs::create_dir_all(prefix).await.map_err(|e| {
            diesel::result::ConnectionError::BadConnection(format!(
                "Can't create sub-directory `{}`: {e}",
                prefix
                    .to_str()
                    .expect("We generate the Path from a `&str` so UTF-8 is already checked")
            ))
        })?;
    }

    let connection = SqliteConnection::establish(&path_as_string)?;
    let executor = SqliteExecutor::start(connection, move |_conn| {
        SqliteConnection::establish(&path_as_string).map_err(DatabaseError::from)
    });
    let pragma_options = SqliteOptions::default()
        .journal_mode(option::JournalMode::Wal)
        .synchronous(option::Synchronous::Normal)
        .to_sql_batch_query();

    executor
        .exec(move |conn| conn.batch_execute(&pragma_options))
        .await??;

    if let Some(auto_vacuum) = auto_vacuum {
        auto_vacuum.safely_set_value(&executor).await?;
    }

    Ok(executor)
}

impl LocalDatabase {
    /// Close the actual connection to the database.
    pub async fn close(&self) {
        let conn = self.executor.stop().await;
        // Close is idempotent, so it can be called multiple times (typically we close
        // ourself everytime `LocalDatabase::exec` fails with an unexpected error from
        // SQLite).
        // Hence it's very possible we've already been there and the connection has
        // already been returned.
        // A special case however is if the executor crashed hard (e.g. couldn't re-open
        // the connection during a full vacuum): the connection has been dropped and
        // we won't be able to return it.
        // However even that is not a big deal: testbed/in-memory-mock makes sure to panic
        // if an already opened (i.e. a not returned) connection got re-opened. This
        // should be enough to debug the code !
        #[cfg(feature = "test-in-memory-mock")]
        let conn = conn.and_then(|conn| {
            in_memory_mock::maybe_return_sqlite_in_memory_conn(&self.path_info, conn).err()
        });
        #[cfg(feature = "test-with-testbed")]
        let conn = conn.and_then(|conn| {
            testbed::maybe_return_sqlite_in_memory_conn(&self.path_info, conn).err()
        });
        drop(conn);
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

                self.executor.full_vacuum().await
            }
        }
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
        let res = self.executor.exec(job).await?.map_err(DatabaseError::from);
        let mut close_needed = false;
        // TODO: move this directly in the background executor thread for more reliable close
        if let Err(DatabaseError::DieselDatabaseError(kind, _err)) = res.as_ref() {
            match kind {
                // Expected errors that can be trigger by a regular SQL query
                diesel::result::DatabaseErrorKind::UniqueViolation
                | diesel::result::DatabaseErrorKind::ForeignKeyViolation
                | diesel::result::DatabaseErrorKind::UnableToSendCommand
                | diesel::result::DatabaseErrorKind::SerializationFailure
                | diesel::result::DatabaseErrorKind::ReadOnlyTransaction
                | diesel::result::DatabaseErrorKind::NotNullViolation
                | diesel::result::DatabaseErrorKind::CheckViolation => (),
                // The Sqlite database is closed, this is either due to (by order of likeliness):
                // - the executor has been stopped, typically because we are shutting down
                // - an unexpected panic in the executor (or it background thread)
                // - an internal error in SQLite
                // In any case, we want to make sure no more job can be send now (to avoid weird
                // situation where SQLite had a hiccup and rejected a single query).
                diesel::result::DatabaseErrorKind::ClosedConnection => {
                    log::warn!("Unexpected closed database");
                    close_needed = true;
                }
                // And on unknown error, we could be more picky and only close the connection on specific unknown error (for example only close the connection on `disk full`)
                // But checking for those is hard and implementation specific (we need to check against a `&str` which formatting could change).
                _ => {
                    log::warn!("Diesel unknown error: {kind:?}");
                    close_needed = true;
                }
            }
        }
        // Cannot do the close().await in the `if let` as at this point `res` is
        // borrowed, which would make our future not `Send`
        // future returned by `exec` is not `Send`
        if close_needed {
            self.close().await;
        }
        res
    }
}

impl std::fmt::Debug for LocalDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LocalDatabase");
        fmt.field("path", &self.path)
            .field("vacuum_mode", &self.vacuum_mode);

        #[cfg(any(feature = "test-in-memory-mock", feature = "test-with-testbed"))]
        {
            fmt.field("is_in_memory", &self.is_in_memory);
        }

        fmt.finish_non_exhaustive()
    }
}
