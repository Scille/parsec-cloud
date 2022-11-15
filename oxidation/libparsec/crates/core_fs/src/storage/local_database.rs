// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use crate::{FSError, FSResult, SQLITE_DISK_FULL_MSG};

use diesel::{connection::SimpleConnection, Connection, SqliteConnection};

use std::{
    ops::DerefMut,
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};

/// Maximum Number Of Host Parameters In A Single SQL Statement
/// https://www.sqlite.org/limits.html#max_variable_number
pub const SQLITE_MAX_VARIABLE_NUMBER: usize = 999;

/// A Connection to Sqlite.
#[derive(Clone)]
pub struct SqliteConn {
    connection: Arc<Mutex<Option<SqliteConnection>>>,
}

impl SqliteConn {
    /// Create a new Sqlite connection with some initialization steps.
    pub fn new<P: Into<String>>(path: P) -> FSResult<Self> {
        let path = path.into();
        if let Some(prefix) = PathBuf::from(&path).parent() {
            std::fs::create_dir_all(prefix).map_err(|_| FSError::CreateDir)?;
        }

        let mut connection =
            SqliteConnection::establish(&path).map_err(|e| FSError::Connection(e.to_string()))?;

        // The combination of WAL journal mode and NORMAL synchronous mode
        // is a great combination: it allows for fast commits (~10 us compare
        // to 15 ms the default mode) but still protects the database against
        // corruption in the case of OS crash or power failure.
        connection
            .batch_execute(
                "
                    PRAGMA journal_mode = WAL; -- better write-concurrency
                    PRAGMA synchronous = NORMAL; -- fsync only in critical moments
                ",
            )
            .map_err(|_| FSError::Configuration)?;

        Ok(connection.into())
    }

    /// Get exclusive access to the Sqlite connection among cloned [SqliteConn]s.
    pub fn lock_connection(&self) -> LockedSqliteConnection {
        self.connection.lock().expect("Mutex is poisoned").into()
    }

    /// Close the connection to the local database.
    pub fn close_connection(&self) {
        *self.lock_connection().conn = None;
    }

    pub fn exec<T, F>(&self, f: F) -> Result<T, ExecError>
    where
        F: FnOnce(&mut SqliteConnection) -> Result<T, diesel::result::Error>,
    {
        self.lock_connection().exec(f)
    }

    pub fn exec_with_handler<T, F, G>(&self, f: F, error_handler: G) -> FSResult<T>
    where
        F: FnOnce(&mut SqliteConnection) -> Result<T, diesel::result::Error>,
        G: FnOnce(diesel::result::Error) -> FSError,
    {
        self.lock_connection().exec_with_handler(f, error_handler)
    }
}

impl From<SqliteConnection> for SqliteConn {
    fn from(conn: SqliteConnection) -> Self {
        Self {
            connection: Arc::new(Mutex::new(Some(conn))),
        }
    }
}

pub struct LockedSqliteConnection<'a> {
    conn: MutexGuard<'a, Option<SqliteConnection>>,
}

impl<'a> From<MutexGuard<'a, Option<SqliteConnection>>> for LockedSqliteConnection<'a> {
    fn from(guard: MutexGuard<'a, Option<SqliteConnection>>) -> Self {
        Self { conn: guard }
    }
}

impl<'a> LockedSqliteConnection<'a> {
    pub fn exec<T, F>(&mut self, f: F) -> Result<T, ExecError>
    where
        F: FnOnce(&mut SqliteConnection) -> Result<T, diesel::result::Error>,
    {
        let conn = self.deref_mut().map_err(ExecError::Internal)?;

        let res = f(conn);

        match res {
            // We've got a disk full error, we need to stop sending request to the database.
            Err(diesel::result::Error::DatabaseError(_, err_msg))
                if err_msg.message() == *SQLITE_DISK_FULL_MSG =>
            {
                *self.conn = None;
                Err(ExecError::Internal(FSError::NoSpaceLeftOnDevice))
            }
            Err(e) => Err(ExecError::DieselError(e)),
            Ok(ok) => Ok(ok),
        }
    }

    pub fn exec_with_handler<T, F, G>(&mut self, f: F, error_handler: G) -> FSResult<T>
    where
        F: FnOnce(&mut SqliteConnection) -> Result<T, diesel::result::Error>,
        G: FnOnce(diesel::result::Error) -> FSError,
    {
        self.exec(f).map_err(|e| match e {
            ExecError::Internal(e) => e,
            ExecError::DieselError(e) => error_handler(e),
        })
    }

    fn deref_mut(&mut self) -> FSResult<&mut SqliteConnection> {
        self.conn
            .deref_mut()
            .as_mut()
            .ok_or_else(|| FSError::DatabaseClosed("database is closed".to_string()))
    }
}

#[derive(Debug)]
pub enum ExecError {
    DieselError(diesel::result::Error),
    Internal(FSError),
}

impl From<ExecError> for FSError {
    fn from(err: ExecError) -> FSError {
        match err {
            ExecError::Internal(err) => err,
            ExecError::DieselError(err) => FSError::from(err),
        }
    }
}
