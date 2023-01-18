// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::thread::JoinHandle;

use diesel::SqliteConnection;
use platform_async::channel;

use crate::{DatabaseError, DatabaseResult};

pub struct SqliteExecutor {
    job_sender: channel::Sender<ExecOrStop>,
    handle: Option<JoinHandle<()>>,
}

type JobFunc = Box<dyn FnOnce(&mut SqliteConnection) + Send>;

enum ExecOrStop {
    Exec(JobFunc),
    Stop,
}

impl SqliteExecutor {
    /// Spawn the executor in a thread.
    pub fn spawn(connection: SqliteConnection) -> Self {
        let (job_sender, job_receiver) = channel::bounded(32);
        let background_executor = BackgroundSqliteExecutor {
            job_receiver,
            connection,
        };
        // TODO: currently if the thread panic the error is printed to stderr,
        // we should instead have a proper panic handler that log an error
        let handle = std::thread::Builder::new()
            .name("SqliteExecutor".to_string())
            .spawn(move || background_executor.serve())
            .expect("failed to spawn thread");

        Self {
            job_sender,
            handle: Some(handle),
        }
    }

    pub async fn exec<'a, F, R>(&self, job: F) -> DatabaseResult<R>
    where
        F: (FnOnce(&mut SqliteConnection) -> R) + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = channel::bounded::<R>(1);
        let wrapped_job = move |conn: &mut SqliteConnection| {
            let res = job(conn);
            // If send fails it means the caller's future has been dropped
            // (hence dropping `rx`). In theory there is nothing wrong about
            // it, however we log it anyway given the caller's unexpected drop
            // may also be the sign of a bug...
            if tx.send(res).is_err() {
                // TODO: replace this by a proper warning log !
                eprintln!("Caller has left");
            }
        };
        let wrapped_job = Box::new(wrapped_job);
        self.raw_exec(wrapped_job).await?;
        rx.recv_async().await.map_err(|_| DatabaseError::Closed)
    }

    pub async fn raw_exec(&self, job: JobFunc) -> DatabaseResult<()> {
        self.job_sender
            .send_async(ExecOrStop::Exec(job))
            .await
            .map_err(|_| DatabaseError::Closed)
    }
}

impl Drop for SqliteExecutor {
    fn drop(&mut self) {
        if self.job_sender.send(ExecOrStop::Stop).is_err() {
            // If the send fails, this is most likely because the executor
            // thread is already stopped. This might be because:
            // - the thread has panicked
            // - a bug in the code has cause another `ExecOrStop::Stop` to be
            //   send to the thead before
            // In both case something fishy is going on here :/
            // TODO: replace this by a proper warning log !
            eprintln!("Cannot send stop message to thread");
        }
        if let Some(handle) = self.handle.take() {
            // An error is returned in case the joined thread has panicked
            // We can ignore the error given it should have already been
            // logged as part of the panic handling system.
            let _ = handle.join();
        }
    }
}

struct BackgroundSqliteExecutor {
    job_receiver: channel::Receiver<ExecOrStop>,
    connection: SqliteConnection,
}

impl BackgroundSqliteExecutor {
    fn serve(mut self) {
        for msg in self.job_receiver.into_iter() {
            match msg {
                ExecOrStop::Exec(job) => job(&mut self.connection),
                ExecOrStop::Stop => return,
            }
        }
    }
}
