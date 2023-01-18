// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
use tokio::sync::RwLock;

use diesel::{Connection, SqliteConnection};

use crate::{executor::SqliteExecutor, DatabaseError, DatabaseResult, LocalDatabase};

static OPEN_CONNECTION_IN_MEMORY: AtomicBool = AtomicBool::new(false);

lazy_static::lazy_static! {
    static ref LOCAL_DB_IN_MEMORY: Arc<Mutex<HashMap<String, Arc<SqliteExecutor>>>> =
        Arc::new(Mutex::new(HashMap::default()));
}

pub(crate) fn open_local_db_in_memory(path: &str) -> Option<DatabaseResult<LocalDatabase>> {
    if !OPEN_CONNECTION_IN_MEMORY.load(Ordering::SeqCst) {
        return None;
    }
    let res = {
        match LOCAL_DB_IN_MEMORY
            .lock()
            .expect("Mutex is poisoned")
            .entry(path.to_string())
        {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => SqliteConnection::establish(":memory:")
                .map_err(DatabaseError::from)
                .map(|conn| {
                    let value = Arc::new(SqliteExecutor::spawn(conn));
                    entry.insert(value).clone()
                }),
        }
    };

    Some(res.map(|executor| LocalDatabase {
        executor: Arc::new(RwLock::new(Some(executor))),
    }))
}

/// Enable or not to have [LocalDatabase] in memory.
///
/// Note: Already openned connection would remain unchanged.
pub fn toggle_local_db_in_memory(enabled: bool) {
    OPEN_CONNECTION_IN_MEMORY.store(enabled, Ordering::SeqCst)
}

/// Will clear saved database opened in memory.
///
/// Note: If [LocalDatabase] are still running, those connection would remain openned until it's dropped.
pub fn clear_local_db_in_memory() {
    LOCAL_DB_IN_MEMORY
        .lock()
        .expect("Mutex is poisoned")
        .clear();
}
