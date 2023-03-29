// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
//! Module that wrap an [diesel::SqliteConnection] behind a executor to allow to have an async manner to executor sql queries.

#[cfg(test)]
mod tests;

use diesel::{connection::SimpleConnection, SqliteConnection};
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::sync::{atomic::AtomicBool, Arc};

use super::{DatabaseError, DatabaseResult};

pub(super) struct SqliteExecutor {
    internal: Arc<Mutex<SqliteExecutorInternal>>,
    // Shared with [SqliteExecutorInternal::force_stop]
    force_stop: Arc<AtomicBool>,
}

struct SqliteExecutorInternal {
    connection: Option<SqliteConnection>,
    reopen_connection: Box<dyn Fn(SqliteConnection) -> DatabaseResult<SqliteConnection> + Send>,
    // Shared with [SqliteExecutor::force_stop]
    force_stop: Arc<AtomicBool>,
}

impl SqliteExecutor {
    pub fn start<F>(connection: SqliteConnection, reopen_connection: F) -> Self
    where
        F: (Fn(SqliteConnection) -> DatabaseResult<SqliteConnection>) + 'static + Send + Sync,
    {
        let force_stop = Arc::new(AtomicBool::new(false));
        Self {
            internal: Arc::new(Mutex::new(SqliteExecutorInternal {
                connection: Some(connection),
                reopen_connection: Box::new(reopen_connection),
                force_stop: force_stop.clone(),
            })),
            force_stop,
        }
    }

    /// Graciously stop, all subsequent jobs exec will fail
    pub async fn stop(&self) -> Option<SqliteConnection> {
        // If there is other jobs waiting for `internal` mutex, they will be
        // served before us. However from now on no more job should be scheduled.
        // So the solution is this `force_stop` flag that we can set right away.
        self.force_stop.store(true, Ordering::Relaxed);

        let internal = self.internal.clone();
        tokio::task::spawn_blocking(move || {
            let mut guard = internal.lock().expect("Mutex is poisoned");
            guard.connection.take()
        })
        .await
        .expect("blocking task panicked")
    }

    pub async fn exec<F, R>(&self, job: F) -> DatabaseResult<R>
    where
        F: (FnOnce(&mut SqliteConnection) -> R) + Send + 'static,
        R: Send + 'static,
    {
        let internal = self.internal.clone();
        tokio::task::spawn_blocking(move || {
            let mut guard = internal.lock().expect("Mutex is poisoned");

            // Ensure the executor hasn't been stopped while we were waiting for the lock
            if guard.force_stop.load(Ordering::Relaxed) {
                return Err(DatabaseError::Closed);
            }

            match &mut guard.connection {
                Some(conn) => Ok(job(conn)),
                None => Err(DatabaseError::Closed),
            }
        })
        .await
        .expect("blocking job task panicked")
    }

    pub async fn full_vacuum(&self) -> DatabaseResult<()> {
        let internal = self.internal.clone();
        tokio::task::spawn_blocking(move || {
            let mut guard = internal.lock().expect("Mutex is poisoned");

            // Ensure the executor hasn't been stopped while we where waiting for the lock
            if guard.force_stop.load(Ordering::Relaxed) {
                return Err(DatabaseError::Closed);
            }

            match guard.connection.take() {
                Some(mut connection) => {
                    // Run a full vacuum, meaning that it will execute a standar vacuum
                    // and re-open the database connection to force cleanup.
                    let res = connection
                        .batch_execute("VACUUM")
                        .map_err(DatabaseError::from)
                        .and_then(|_| (guard.reopen_connection)(connection));

                    match res {
                        Ok(new_connection) => {
                            guard.connection = Some(new_connection);
                            Ok(())
                        }
                        // Oh no, we have an error, the background executor will stop just after notifying the caller.
                        Err(err) => {
                            log::warn!(
                                "Stopping background executor due to failure during full vacuum: {:?}",
                                &err
                            );
                            Err(err)
                        }
                    }
                },
                None => Err(DatabaseError::Closed),
            }
        }).await.expect("blocking job task panicked")
    }
}
