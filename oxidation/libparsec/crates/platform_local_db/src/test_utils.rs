// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{Connection, SqliteConnection};
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Mutex,
};

use super::executor::SqliteExecutor;

enum ClosableInMemoryDB {
    Opened,
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

#[derive(Debug)]
enum TestDBStrategy {
    InMemory(HashMap<String, ClosableInMemoryDB>),
    OnDisk,
}

static TEST_DB_STRATEGY: Mutex<TestDBStrategy> = Mutex::new(TestDBStrategy::OnDisk);

pub(crate) fn maybe_open_sqlite_in_memory(path: &str) -> Option<SqliteExecutor> {
    let mut strategy = TEST_DB_STRATEGY.lock().expect("Mutex is poisoned");
    match &mut *strategy {
        TestDBStrategy::InMemory(dbs) => {
            match dbs.entry(path.to_string()) {
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
                            panic!(
                                "Trying to open already opened in memory database `{}`",
                                path
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
                    Some(SqliteExecutor::spawn(conn))
                }
            }
        }

        TestDBStrategy::OnDisk => None,
    }
}

pub(crate) fn return_sqlite_in_memory_db(path: &str, db: SqliteExecutor) {
    let mut strategy = TEST_DB_STRATEGY.lock().expect("Mutex is poisoned");
    if let TestDBStrategy::InMemory(dbs) = &mut *strategy {
        if let Entry::Occupied(mut entry) = dbs.entry(path.to_string()) {
            if let ClosableInMemoryDB::Opened = entry.get() {
                entry.insert(ClosableInMemoryDB::Closed(db));
                return;
            }
        }
    }
    panic!(
        "Inconsistent state with SQLite in-memory mock:\npath: {}\ndbs: {:?}",
        path, *strategy
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
pub fn test_clear_local_db_in_memory_mock() {
    let mut strategy = TEST_DB_STRATEGY.lock().expect("Mutex is poisoned");
    if let TestDBStrategy::InMemory(dbs) = &mut *strategy {
        for (path, closable) in dbs.iter() {
            if let ClosableInMemoryDB::Opened = closable {
                panic!(
                    "In-memory database `{}` is still opened, cannot clear the mock !",
                    path
                );
            }
        }
        dbs.clear();
    }
}
