// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::connection::SimpleConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;
use std::path::PathBuf;

use crate::error::{FSError, FSResult};

// https://www.sqlite.org/limits.html
// #9 Maximum Number Of Host Parameters In A Single SQL Statement
pub const SQLITE_MAX_VARIABLE_NUMBER: usize = 999;
pub struct SqlitePool(Pool<ConnectionManager<SqliteConnection>>);
pub type SqliteConn = PooledConnection<ConnectionManager<SqliteConnection>>;

impl SqlitePool {
    pub fn new<P: Into<String>>(path: P) -> FSResult<SqlitePool> {
        let path = path.into();
        if let Some(prefix) = PathBuf::from(&path).parent() {
            std::fs::create_dir_all(prefix).map_err(|_| FSError::CreateDir)?;
        }
        let manager = ConnectionManager::<SqliteConnection>::new(path);
        Pool::builder()
            .build(manager)
            .map_err(|_| FSError::Pool)
            .map(Self)
    }

    pub fn conn(&self) -> FSResult<SqliteConn> {
        let mut conn = self
            .0
            .get()
            .map_err(|e| FSError::Connection(e.to_string()))?;
        // The combination of WAL journal mode and NORMAL synchronous mode
        // is a great combination: it allows for fast commits (~10 us compare
        // to 15 ms the default mode) but still protects the database against
        // corruption in the case of OS crash or power failure.
        conn.batch_execute(
            "
            PRAGMA journal_mode = WAL; -- better write-concurrency
            PRAGMA synchronous = NORMAL; -- fsync only in critical moments
        ",
        )
        .map_err(|_| FSError::Configuration)?;

        Ok(conn)
    }
}
