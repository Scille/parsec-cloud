// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
//! Module that wrap an [diesel::SqliteConnection] behind a executor to allow to have an async manner to executor sql queries.

use diesel::{connection::SimpleConnection, SqliteConnection};
use libparsec_platform_async::channel;
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

use crate::{DatabaseError, DatabaseResult};

enum JobOperation {
    /// Operate a full vacuum, meaning executing `VACUUM` on the sqlite connection
    /// and re-opening the connection to force cleanup of the driver.
    FullVacuum(JobResultSender<DatabaseResult<()>>),
    /// Execute an arbitrary operation with the SQLite connection.
    Fn(JobFunc),
    /// Dummy operation to wake up the background executor for stop
    CheckTerminatedFlag,
}

#[derive(Debug)]
#[must_use]
struct JobHandle<R>(channel::Receiver<R>)
where
    R: Send + 'static;

impl<R> JobHandle<R>
where
    R: Send + 'static,
{
    pub async fn join(self) -> DatabaseResult<R> {
        self.0.recv_async().await.map_err(|_| DatabaseError::Closed)
    }
}

struct JobResultSender<R>(channel::Sender<R>)
where
    R: Send + 'static;

impl<R> JobResultSender<R>
where
    R: Send + 'static,
{
    pub fn new() -> (Self, JobHandle<R>) {
        // We use a bounded queue with a size of 1 to receive the result from
        // the background executor.
        // We don't use a rendez-vous point (i.e. size of 0) because the background
        // executor doesn't care about us and just want to move on processing the next
        // job. On top of that there is no risk of deadlock if the receiver gets dropped
        // because the queue contains opaque data from the background executor point of
        // view (i.e. it cannot have an await depending on data it doesn't understand !).
        let (tx, rx) = channel::bounded::<R>(1);

        (Self(tx), JobHandle(rx))
    }

    pub fn send(&self, res: R) {
        // If send fails it means the curresponding `JobHandle` has been drooped (most
        // likely because caller's future has been dropped due to a panic or abort).
        // In theory there is nothing wrong about it, however we log it anyway given
        // the caller's unexpected drop may also be the sign of a bug...
        if self.0.send(res).is_err() {
            log::warn!("Caller has left");
        }
    }
}

/// A type alias for function that will be sent to the background executor.
type JobFunc = Box<dyn FnOnce(&mut SqliteConnection) + Send>;

/// The executor that manage and send job to the background executor.
pub(crate) struct SqliteExecutor {
    /// The channel that will be used to send job to the background executor.
    job_sender: channel::Sender<JobOperation>,
    /// Handle to the background executor's thread
    handle: Mutex<Option<tokio::task::JoinHandle<()>>>,
    /// Set to true once the executor is no longer running due to panic
    terminated_flag: Arc<AtomicBool>,
    /// This channel is used to retreive the Sqlite connection when the background
    /// executor has stopped, this is only useful in the tests
    on_terminate_conn_receiver: channel::Receiver<SqliteConnection>,
}

impl SqliteExecutor {
    /// Start the executor by spawning a thread for background operations
    pub fn start<F>(connection: SqliteConnection, reopen_connection: F) -> Self
    where
        F: Send + (Fn(SqliteConnection) -> DatabaseResult<SqliteConnection>) + 'static,
    {
        // Flume's channel doesn't drop the queue's content when the receiver is
        // dropped. This has unexpected consequences if the queue contains a sender
        // we are waiting on somewhere else (and yes, this is precisely what we have
        // here !)
        // So the solution is to have a zero-size-bonded queue, this way the queue is
        // just a rendez-vous point and never contains anything.
        // (see https://github.com/zesterer/flume/issues/89)
        let (job_sender, job_receiver) = channel::bounded(0);

        let (on_terminate_conn_sender, on_terminate_conn_receiver) = channel::bounded(1);

        let terminated_flag = Arc::new(AtomicBool::new(false));

        // TODO: instead of starting a thread, we could also use `tokio::task::block_in_place`
        // to avoid sending job. The good thing about this is it should be faster and remove
        // the Send requirements on the submitted function.
        // On the other hand, this is incompatible with monothreaded runtime. By default
        // the runtime for test is monothreaded so we will have to specify
        // `#[tokio::test(flavor = "multi_thread", worker_threads = 1)]` for each test...
        // On top of that unexpected deadlock are easier to spot when the runtime is
        // monothreaded, so do we really want to abandon this ?

        // TODO: currently if the thread panic the error is printed to stderr,
        // we should instead have a proper panic handler that log an error
        let handle = tokio::task::spawn_blocking({
            let terminated_flag = terminated_flag.clone();
            move || {
                background_executor(
                    job_receiver,
                    terminated_flag,
                    on_terminate_conn_sender,
                    connection,
                    reopen_connection,
                )
            }
        });

        Self {
            job_sender,
            handle: Mutex::new(Some(handle)),
            terminated_flag,
            on_terminate_conn_receiver,
        }
    }

    /// Graciously stop the background executor, all subsequent jobs submit will fail
    pub async fn stop(&self) -> Option<SqliteConnection> {
        if let Some(handle) = self.blind_stop() {
            // An error is returned in case the joined thread has panicked
            // We can ignore the error given it should have already been
            // logged as part of the panic handling system.
            let _ = handle.await;
            match self.on_terminate_conn_receiver.recv() {
                Ok(conn) => Some(conn),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    /// Stopping the executor is divided into two operations:
    /// 1. Notifying the background executor thread it should stop
    /// 2. Make sure it is actually stopped
    ///
    /// This blind stop only does step 1, hence once done we can be pretty confident the
    /// background executor will stop once it has finished with it current (if any) job.
    /// On top of that, this operation is synchronous so it's obviously what we use when
    /// dropping the executor.
    /// On the drawback, we cannot get back the SQLite connection object and we lack a
    /// strong guarantee the thread actually terminated, hence the gracious stop (aka
    /// `stop`) that should be used whenever possible instead.
    fn blind_stop(&self) -> Option<tokio::task::JoinHandle<()>> {
        let maybe_handle = {
            let mut guard = self.handle.lock().expect("Mutex is poisoned");
            guard.take()
        };
        maybe_handle.map(|handle| {
            // `terminated_flag` checked periodically by the background executor...
            self.terminated_flag.store(true, Ordering::Relaxed);

            // ...however the check is only done before starting to wait for a new job,
            // hence we must trigger a fake job in case it is waiting.
            // If the background executor is already executing a job, the rendez-vous
            // point is not ready and we will have an error that we can safely ignore.
            let _ = self.job_sender.try_send(JobOperation::CheckTerminatedFlag);

            handle
        })
    }

    pub async fn exec<F, R>(&self, job: F) -> DatabaseResult<R>
    where
        F: (FnOnce(&mut SqliteConnection) -> R) + Send + 'static,
        R: Send + 'static,
    {
        // Just a convenient handler over submit & join
        let job = self.submit(job).await?;
        job.join().await
    }

    pub async fn full_vacuum(&self) -> DatabaseResult<()> {
        // Just a convenient handler over submit & join
        let job = self.submit_full_vacuum().await?;
        job.join().await?
    }

    /// Blocks until the background executor has actually started processing our job, then
    /// [JobHandle::join] should be used to wait for conclusion and get back the result
    async fn submit<F, R>(&self, job: F) -> DatabaseResult<JobHandle<R>>
    where
        F: (FnOnce(&mut SqliteConnection) -> R) + Send + 'static,
        R: Send + 'static,
    {
        let (result_sender, result_receiver) = JobResultSender::new();
        let job_operation = JobOperation::Fn(Box::new(move |conn: &mut SqliteConnection| {
            let res = job(conn);
            result_sender.send(res);
        }));

        // This is going to block until the background executor is ready to process
        // our job (i.e. all the jobs before us has been processed)
        self.job_sender
            .send_async(job_operation)
            .await
            .map_err(|_| DatabaseError::Closed)?;

        Ok(result_receiver)
    }

    /// Blocks until the background executor has actually started our vacuum job
    async fn submit_full_vacuum(&self) -> DatabaseResult<JobHandle<DatabaseResult<()>>> {
        let (result_sender, result_receiver) = JobResultSender::new();
        let job_operation = JobOperation::FullVacuum(result_sender);

        // This is going to block until the background executor is ready to process
        // our job (i.e. all the jobs before us has been processed)
        self.job_sender
            .send_async(job_operation)
            .await
            .map_err(|_| DatabaseError::Closed)?;

        Ok(result_receiver)
    }
}

impl Drop for SqliteExecutor {
    fn drop(&mut self) {
        self.blind_stop();
    }
}

/// Start the background executor to listen for incoming jobs.
/// This method will stop when all sender channel are closed and no more job are present on the channel queue.
fn background_executor<F>(
    job_receiver: channel::Receiver<JobOperation>,
    terminated_flag: Arc<AtomicBool>,
    on_terminate_conn_sender: channel::Sender<SqliteConnection>,
    mut connection: SqliteConnection,
    reopen_connection: F,
) where
    F: Fn(SqliteConnection) -> DatabaseResult<SqliteConnection>,
{
    loop {
        // Gracious stop hook
        if terminated_flag.load(Ordering::Relaxed) {
            break;
        }
        match job_receiver.recv() {
            Ok(operation) => match operation {
                JobOperation::FullVacuum(result_sender) => {
                    // Run a full vacuum, meaning that it will execute a standar vacuum
                    // and re-open the database connection to force cleanup.
                    let res = connection
                        .batch_execute("VACUUM")
                        .map_err(DatabaseError::from)
                        .and_then(|_| reopen_connection(connection));

                    match res {
                        Ok(conn) => {
                            connection = conn;
                            result_sender.send(Ok(()));
                        }
                        // Oh no, we have an error, the background executor will stop just after notifying the caller.
                        Err(err) => {
                            log::warn!(
                                "Stopping background executor due to failure during full vacuum: {:?}",
                                &err
                            );
                            result_sender.send(Err(err));
                            // At this point we don't have a valid SQLite connection object to
                            // use, so we have no choice but to abruptly stop.
                            return;
                        }
                    }
                }
                JobOperation::Fn(func) => func(&mut connection),
                JobOperation::CheckTerminatedFlag => (),
            },
            // Sender is closed
            Err(_) => break,
        }
    }
    log::info!("Gracious stop of background executor");
    // Return the SQLite connection instead of dropping it, this is useful during
    // tests where we use in-memory connections
    let _ = on_terminate_conn_sender.send(connection);
}

#[cfg(test)]
#[path = "../tests/unit/executor.rs"]
mod test;
