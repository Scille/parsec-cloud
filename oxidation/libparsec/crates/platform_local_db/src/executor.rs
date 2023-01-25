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
        let handle = std::thread::spawn(move || background_executor.serve());

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
            tx.send(res).unwrap();
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
        if let Err(e) = self.job_sender.send(ExecOrStop::Stop) {
            eprintln!("Cannot send message to the background executor (Could just mean the executor is closed): {e}")
        }
        if let Some(Err(e)) = self.handle.take().map(|handle| handle.join()) {
            eprintln!("Cannot wait for the thread to finish: {e:?}");
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
