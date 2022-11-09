// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use crate::{FSError, FSResult};

use diesel::{connection::SimpleConnection, Connection, SqliteConnection};

use std::path::PathBuf;

/// Maximum Number Of Host Parameters In A Single SQL Statement
/// https://www.sqlite.org/limits.html#max_variable_number
pub const SQLITE_MAX_VARIABLE_NUMBER: usize = 999;

/// A Connection to Sqlite.
pub struct SqliteConn {
    pub connection: SqliteConnection,
    database_url: String,
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

        Ok(Self {
            connection,
            database_url: path,
        })
    }

    /// Reopen the sqlite connection using the previously given path.
    pub fn reopen(&self) -> FSResult<Self> {
        Self::new(&self.database_url)
    }
}
