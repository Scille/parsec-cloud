// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
//! Module that provide utilities functions for `local-db`, for example to open the database in memory and keep it for latter use when the [crate::LocalDatabase] is closed.

use std::{ops::DerefMut, sync::Mutex};

use diesel::{Connection, SqliteConnection};

use super::DBPathInfo;

/// A database associated to a path can be ether closed or opened.
enum ClosableInMemoryDB {
    /// The database is open an currently in used.
    Opened,
    /// The database is theoretically closed, so we keep here since in-memory db lose data on drop.
    Closed(SqliteConnection),
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

#[derive(Debug)]
struct DBEntry {
    pub path_info: DBPathInfo,
    pub state: ClosableInMemoryDB,
}

/// The strategy when opening database connection.
#[derive(Debug)]
enum TestDBStrategy {
    /// Opening a database will be done in RAM.
    /// We store the path alongside it to be able to provide the same connection without losing data.
    InMemory(Vec<DBEntry>), // Expected to be small, so Vec is fast
    /// When opening a database connection will use the underling filesystem of the OS.
    OnDisk,
}

/// The value that will keep the state in memory for the lifetime of a program.
static TEST_DB_STRATEGY: Mutex<TestDBStrategy> = Mutex::new(TestDBStrategy::OnDisk);

/// This function maybe open a sqlite connection.
///
/// This will depend on the value of [TEST_DB_STRATEGY] that can be changed with [test_toggle_local_db_in_memory_mock].
///
/// Return an error if the in memory mock is not not enabled.
///
/// # Panics
///
/// This function will panic if we try to open a connection to an already opened file.
pub(super) fn maybe_open_sqlite_in_memory(
    path_info: &super::DBPathInfo,
) -> Option<SqliteConnection> {
    let mut strategy = TEST_DB_STRATEGY.lock().expect("Mutex is poisoned");
    match strategy.deref_mut() {
        TestDBStrategy::InMemory(dbs) => {
            for db in dbs.iter_mut() {
                // This database has already been used in the past...
                if &db.path_info == path_info {
                    // For rusty move reasons we have to first mark the database as
                    // opened, then check if its previous state was valid.
                    match std::mem::replace(&mut db.state, ClosableInMemoryDB::Opened) {
                        ClosableInMemoryDB::Opened => {
                            // The database is *currently* used, this is something we don't
                            // do in Parsec (we have a single process running that have no
                            // benefit at connecting multiple times to a database).
                            // This is most likely in a bug (typically trying to start two
                            // client with the same device).
                            panic!(
                                "Trying to open already opened in memory database {:?}",
                                path_info
                            )
                            // In theory here we should reset the entry state (given we set
                            // it before doing the check), however in the current case both
                            // state have the same `Opened` value so all is already good ;-)
                        }
                        ClosableInMemoryDB::Closed(conn) => {
                            // The database has been closed by the one that created it.
                            // Of course in-memory database loose all data on close so it has
                            // been kept on the side instead, and now is time to re-use it !
                            return Some(conn);
                        }
                    }
                }
            }
            // This database is brand new
            // Now create the database and register it path as opened
            let conn =
                SqliteConnection::establish(":memory:").expect("Cannot create in-memory database");
            dbs.push(DBEntry {
                path_info: path_info.clone(),
                state: ClosableInMemoryDB::Opened,
            });
            Some(conn)
        }

        TestDBStrategy::OnDisk => None,
    }
}

/// When closing a database connection the user must call this function to reset the state of an entry identified by `path`.
/// Return an error if the in memory mock is not not enabled.
///
/// # Panics
///
/// Will panic if:
///
/// - We haven't allow the connection to be openned in memory, see [test_toggle_local_db_in_memory_mock].
/// - The entry specified by `path` is not found.
/// - The entry specified by `path` is already closed.
pub(super) fn maybe_return_sqlite_in_memory_conn(
    path_info: &DBPathInfo,
    conn: SqliteConnection,
) -> Result<(), SqliteConnection> {
    let mut strategy_guard = TEST_DB_STRATEGY.lock().expect("Mutex is poisoned");
    match strategy_guard.deref_mut() {
        TestDBStrategy::InMemory(dbs) => {
            for db in dbs.iter_mut() {
                match db {
                    DBEntry {
                        state: ClosableInMemoryDB::Opened,
                        ..
                    } if &db.path_info == path_info => {
                        db.state = ClosableInMemoryDB::Closed(conn);
                        return Ok(());
                    }
                    _ => (),
                }
            }
            // Note: because we panic the mutex on `TEST_DB_STRATEGY` will be poisoned.
            panic!(
                "Inconsistent state with SQLite in-memory mock:\nDB path info: {:?}\ndbs: {:?}",
                path_info, *dbs
            )
        }
        TestDBStrategy::OnDisk => Err(conn),
    }
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
        (true, TestDBStrategy::OnDisk) => *strategy = TestDBStrategy::InMemory(vec![]),
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
        let still_opened = dbs.iter().find_map(|entry| {
            if let ClosableInMemoryDB::Opened = entry.state {
                Some(&entry.path_info)
            } else {
                None
            }
        });
        if let Some(path_info) = still_opened {
            panic!(
                "In-memory database {:?} is still opened, cannot clear the mock !",
                path_info
            );
        }

        dbs.clear();
    }
}
