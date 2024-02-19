// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{any::Any, path::PathBuf, sync::Arc};

use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Connection, SqliteConnection};

use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_testbed::{test_get_testbed_component_store, TestbedEnv};

const STORE_ENTRY_KEY: &str = "platform_local_db_sqlx";

/// A database associated to a path can be either closed or opened.
enum ClosableInMemoryDB {
    /// The database is open and currently in used.
    Opened,
    /// The database is closed and we store the executor to provide it on later call of [maybe_open_sqlite_in_memory].
    Closed(SqliteConnection),
}

impl std::fmt::Debug for ClosableInMemoryDB {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ClosableInMemoryDB({})",
            match self {
                ClosableInMemoryDB::Opened => "Opened",
                ClosableInMemoryDB::Closed(_) => "Closed",
            }
        ))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(super) struct DBPathInfo {
    pub db_relative_path: PathBuf,
    pub data_base_dir: PathBuf,
}

#[derive(Debug)]
struct DBEntry {
    pub path_info: DBPathInfo,
    pub state: ClosableInMemoryDB,
}

#[derive(Debug)]
struct ComponentStore {
    dbs: AsyncMutex<Vec<DBEntry>>,
}

fn store_factory(_env: &TestbedEnv) -> Arc<dyn Any + Send + Sync> {
    Arc::new(ComponentStore {
        dbs: AsyncMutex::new(vec![]),
    })
}

pub(super) async fn maybe_open_sqlite_in_memory(
    path_info: &DBPathInfo,
) -> Option<SqliteConnection> {
    let store = match test_get_testbed_component_store::<ComponentStore>(
        &path_info.data_base_dir,
        STORE_ENTRY_KEY,
        store_factory,
    ) {
        Some(store) => store,
        None => {
            return None;
        }
    };

    let mut dbs = store.dbs.lock().await;
    for db in dbs.iter_mut() {
        // This database has already been used in the past...
        if &db.path_info == path_info {
            // For rusty move reasons we have to first mark the database as
            // opened, then check if its previous state was valid.
            match std::mem::replace(&mut db.state, ClosableInMemoryDB::Opened) {
                ClosableInMemoryDB::Opened => {
                    // The database is *currently* used, this is something we don't
                    // do in Parsec (we have a single process running that has no
                    // benefit at connecting multiple times to a database).
                    // This is most likely in a bug (typically trying to start two
                    // clients with the same device).
                    panic!(
                        "Trying to open already opened in memory database {:?}",
                        path_info
                    )
                    // In theory here we should reset the entry state (given we set
                    // it before doing the check), however in the current case both
                    // states have the same `Opened` value so all is already good ;-)
                }
                ClosableInMemoryDB::Closed(conn) => {
                    // The database has been closed by the one that created it.
                    // Of course in-memory database lose all data on close so it has
                    // been kept on the side instead, and now is time to re-use it !
                    return Some(conn);
                }
            }
        }
    }

    // This database is brand new
    // Now create the database and register its path as opened
    let force_db_on_disk = std::env::var("TESTBED_FORCE_SQLITE_ON_DISK").is_ok();
    let conn = if !force_db_on_disk {
        SqliteConnection::connect("sqlite::memory:")
            .await
            .expect("Cannot create in-memory database")
    } else {
        // Our function is basically called "open db in memory", wtf is this hack ???
        // The idea is to provide an easy way to inspect the content of the sqlite db
        // by just setting `TESTBED_FORCE_SQLITE_ON_DISK` environ variable.
        // Note this is not concurrent-proof (so only a single test should be run !).
        static TEMP_DIR: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
        let temp_dir = TEMP_DIR.get_or_init(|| std::env::temp_dir().join("parsec-test-dbs"));
        // e.g. `/parsec/testbed/0/OfflineOrg` -> `parsec-testbed-0-OfflineOrg`
        let base = path_info
            .data_base_dir
            .iter()
            .skip(1)
            .map(|x| x.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join("-");
        let db_path = temp_dir.join(base).join(&path_info.db_relative_path);
        println!("Saving DB {:?} at {}", path_info, db_path.display());
        std::fs::create_dir_all(db_path.parent().expect("path is not root"))
            .expect("Cannot create on-disk database path");
        _ = std::fs::remove_file(&db_path); // Remove db from a previous run if any

        SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
            .connect()
            .await
            .expect("Cannot create on-disk database")
    };
    dbs.push(DBEntry {
        path_info: path_info.clone(),
        state: ClosableInMemoryDB::Opened,
    });

    Some(conn)
}

pub(super) async fn maybe_return_sqlite_in_memory_conn(
    path_info: &DBPathInfo,
    conn: SqliteConnection,
) -> Result<(), SqliteConnection> {
    let maybe_store = test_get_testbed_component_store::<ComponentStore>(
        &path_info.data_base_dir,
        STORE_ENTRY_KEY,
        store_factory,
    );

    match maybe_store {
        None => Err(conn),
        Some(store) => {
            let mut dbs = store.dbs.lock().await;
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
            panic!(
                "Inconsistent state with SQLite in-memory mock:\nDB path info: {:?}\ndbs: {:?}",
                path_info, store.dbs
            )
        }
    }
}
