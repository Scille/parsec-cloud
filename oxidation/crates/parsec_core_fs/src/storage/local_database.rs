// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use diesel::connection::SimpleConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;

use crate::error::{FSError, FSResult};

pub const SQLITE_MAX_VARIABLE_NUMBER: usize = 999;

pub struct SqlitePool(Pool<ConnectionManager<SqliteConnection>>);
pub type SqliteConn = PooledConnection<ConnectionManager<SqliteConnection>>;

impl SqlitePool {
    pub fn new<P: Into<String>>(path: P) -> FSResult<SqlitePool> {
        let manager = ConnectionManager::<SqliteConnection>::new(path);
        Pool::builder()
            .build(manager)
            .map_err(|_| FSError::Pool)
            .map(Self)
    }

    pub fn conn(&self) -> FSResult<SqliteConn> {
        let mut conn = self.0.get().map_err(|_| FSError::Connection)?;
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
