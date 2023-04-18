// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{cell::RefCell, path::Path};

use diesel::{Connection, SqliteConnection};

use libparsec_testbed::test_get_testbed;

use super::DBPathInfo;

const STORE_ENTRY_KEY: &str = "platform_local_db";

/// A database associated to a path can be ether closed or opened.
enum ClosableInMemoryDB {
    /// The database is open an currently in used.
    Opened,
    /// The database is closed and we store the executor to provide it on later call of [maybe_open_sqlite_in_memory].
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

#[derive(Debug)]
struct PseudoPersistentStorage {
    dbs: Vec<DBEntry>,
}

/// Retrieve (or create) our pseudo persistent storage, check it type and pass it to `cb`.
/// Return `None` if `discriminant_dir` doesn't correspond to a testbed env.
fn with_pseudo_persistent_storage<T>(
    discriminant_dir: &Path,
    cb: impl FnOnce(&mut PseudoPersistentStorage) -> T,
) -> Option<T> {
    test_get_testbed(discriminant_dir).map(|env| {
        let mut global_store = env.persistence_store.lock().expect("Mutex is poisoned");
        let store = global_store
            .entry(STORE_ENTRY_KEY)
            .or_insert_with(|| Box::new(PseudoPersistentStorage { dbs: vec![] }));
        let store = store
            .downcast_mut::<PseudoPersistentStorage>()
            .expect("Unexpected pseudo persistent storage type for platform_device_loader");
        cb(store)
    })
}

pub fn maybe_open_sqlite_in_memory(path_info: &DBPathInfo) -> Option<SqliteConnection> {
    with_pseudo_persistent_storage(&path_info.data_base_dir, |store| {
        for db in store.dbs.iter_mut() {
            // This database has already been used in the past...
            if &db.path_info == path_info {
                // For rusty move reasons we have to first mark the database as
                // opened, then check if it previous state was valid.
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
                        return conn;
                    }
                }
            }
        }
        // This database is brand new
        // Now create the database and register it path as opened
        let conn =
            SqliteConnection::establish(":memory:").expect("Cannot create in-memory database");
        store.dbs.push(DBEntry {
            path_info: path_info.clone(),
            state: ClosableInMemoryDB::Opened,
        });
        conn
    })
}

pub fn maybe_return_sqlite_in_memory_conn(
    path_info: &DBPathInfo,
    conn: SqliteConnection,
) -> Result<(), SqliteConnection> {
    let conn = RefCell::new(Some(conn));
    with_pseudo_persistent_storage(path_info.data_base_dir.as_path(), |store| {
        for db in store.dbs.iter_mut() {
            match db {
                DBEntry {
                    state: ClosableInMemoryDB::Opened,
                    ..
                } if &db.path_info == path_info => {
                    db.state = ClosableInMemoryDB::Closed(conn.take().expect("refcell used once"));
                    return;
                }
                _ => (),
            }
        }
        panic!(
            "Inconsistent state with SQLite in-memory mock:\nDB path info: {:?}\ndbs: {:?}",
            path_info, store.dbs
        )
    })
    .ok_or_else(|| conn.take().expect("refcell used once"))
}
