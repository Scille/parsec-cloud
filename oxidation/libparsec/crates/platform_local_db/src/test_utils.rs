// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
//! Module that provide utilities functions for `local-db`, for example to open the database in memory and keep it for latter use when the [crate::LocalDatabase] is closed.

use diesel::{Connection, SqliteConnection};
use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
    sync::Mutex,
};

use super::executor::SqliteExecutor;

/// A database associated to a path can be ether closed or opened.
enum ClosableInMemoryDB {
    /// The database is open an currently in used.
    Opened,
    /// The database is closed and we store the executor to provide it on later call of [maybe_open_sqlite_in_memory].
    Closed(SqliteExecutor),
}

impl std::fmt::Debug for ClosableInMemoryDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ClosableInMemoryDB({})",
            match self {
                ClosableInMemoryDB::Opened => "Opened",
                ClosableInMemoryDB::Closed(_) => "Closed",
            }
        ))
    }
}

/// The strategy when opening database connection.
#[derive(Debug)]
enum TestDBStrategy {
    /// Opening a database will be done in RAM.
    /// We store the path alongside it to be able to provide the same connection without losing data.
    InMemory(HashMap<PathBuf, ClosableInMemoryDB>),
    /// When opening a database connection will use the underling filesystem of the OS.
    OnDisk,
}

/// The value that will keep the state in memory for the lifetime of a program.
static TEST_DB_STRATEGY: Mutex<TestDBStrategy> = Mutex::new(TestDBStrategy::OnDisk);

/// This function maybe open a sqlite connection.
///
/// This will depend on the value of [TEST_DB_STRATEGY] that can be changed with [test_toggle_local_db_in_memory_mock].
///
/// # Panics
///
/// This function will panic if we try to open a connection to an already opened file.
pub(crate) fn maybe_open_sqlite_in_memory(path: &Path) -> Option<SqliteExecutor> {
    let mut strategy = TEST_DB_STRATEGY.lock().expect("Mutex is poisoned");
    match &mut *strategy {
        TestDBStrategy::InMemory(dbs) => {
            match dbs.entry(path.to_path_buf()) {
                // This database has already been used in the past...
                Entry::Occupied(mut entry) => {
                    // For rusty move reasons we have to first mark the database as
                    // opened, then check if it previous state was valid.
                    match entry.insert(ClosableInMemoryDB::Opened) {
                        ClosableInMemoryDB::Opened => {
                            // The database is *currently* used, this is something we don't
                            // do in Parsec (we have a single process running that have no
                            // benefit at connecting multiple times to a database).
                            // This is most likely in a bug (typically trying to start two
                            // client with the same device).
                            drop(strategy);
                            panic!(
                                "Trying to open already opened in memory database `{}`",
                                path.display()
                            )
                            // In theory here we should reset the entry state (given we set
                            // it before doing the check), however in the current case both
                            // state have the same `Opened` value so all is already good ;-)
                        }
                        ClosableInMemoryDB::Closed(db) => {
                            // The database has been closed by the one that created it.
                            // Of course in-memory db loose all data on close so it has
                            // been kept on the side instead, and now is time to re-use it !
                            Some(db)
                        }
                    }
                }

                // This database is brand new
                Entry::Vacant(entry) => {
                    // Now create the database and register it path as opened
                    let conn = SqliteConnection::establish(":memory:")
                        .expect("Cannot create in-memory database");
                    entry.insert(ClosableInMemoryDB::Opened);
                    Some(SqliteExecutor::spawn(conn, Ok))
                }
            }
        }

        TestDBStrategy::OnDisk => None,
    }
}

/// When closing a database connection the user must call this function to reset the state of an entry identified by `path`.
///
/// # Panics
///
/// Will panic if:
///
/// - We haven't allow the connection to be openned in memory, see [test_toggle_local_db_in_memory_mock].
/// - The entry specified by `path` is not found.
/// - The entry specified by `path` is already closed.
pub(crate) fn return_sqlite_in_memory_db(path: &Path, db: SqliteExecutor) {
    let mut strategy_guard = TEST_DB_STRATEGY.lock().expect("Mutex is poisoned");
    if let TestDBStrategy::InMemory(dbs) = &mut *strategy_guard {
        if let Entry::Occupied(mut entry) = dbs.entry(path.to_path_buf()) {
            if let ClosableInMemoryDB::Opened = entry.get() {
                entry.insert(ClosableInMemoryDB::Closed(db));
                return;
            }
        }
    }
    // Note: because we panic the mutex on `TEST_DB_STRATEGY` will be poisoned.
    panic!(
        "Inconsistent state with SQLite in-memory mock:\npath: {}\ndbs: {:?}",
        path.display(),
        *strategy_guard
    )
}

/// Enable or not to open databases in memory.
///
/// Note: Already opened connections remain unchanged.
pub fn test_toggle_local_db_in_memory_mock(enabled: bool) {
    let mut strategy = TEST_DB_STRATEGY.lock().expect("Mutex is poisoned");
    match (enabled, &mut *strategy) {
        (false, TestDBStrategy::InMemory(_)) => {
            *strategy = TestDBStrategy::OnDisk;
        }
        (true, TestDBStrategy::OnDisk) => *strategy = TestDBStrategy::InMemory(HashMap::default()),
        _ => (),
    };
}

/// Will clear saved databases opened in memory.
///
/// # Panics
///
/// This function will panic an entry isn't closed (it's state isn't [ClosableInMemoryDB::Closed]).
pub fn test_clear_local_db_in_memory_mock() {
    let mut strategy_guard = TEST_DB_STRATEGY.lock().expect("Mutex is poisoned");
    if let TestDBStrategy::InMemory(ref mut dbs) = *strategy_guard {
        let still_opened_path = dbs.iter().find_map(|(path, closable)| {
            if let ClosableInMemoryDB::Opened = closable {
                Some(path.clone())
            } else {
                None
            }
        });
        if let Some(path) = still_opened_path {
            drop(strategy_guard);
            panic!(
                "In-memory database `{}` is still opened, cannot clear the mock !",
                path.display()
            );
        }

        dbs.clear();
    }
}
