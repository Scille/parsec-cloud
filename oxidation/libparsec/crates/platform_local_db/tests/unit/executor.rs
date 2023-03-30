// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{connection::SimpleConnection, Connection, SqliteConnection};
use std::sync::Arc;

use libparsec_tests_fixtures::parsec_test;

use super::SqliteExecutor;
use crate::DatabaseError;

#[parsec_test]
async fn stop_with_multiple_jobs() {
    let connection = SqliteConnection::establish(":memory:").unwrap();
    let executor = Arc::new(SqliteExecutor::start(connection, Ok));

    // Here is the expected time table for this test:
    // 1. barrier job is submitted
    // 2. barrier job start to be executed, it wait for barrier-release signal to finish
    // 3. will-be-cancelled job is submitted, this will block is long as
    //    barrier job is executed
    // 4. executor gets closed and barrier-release signal is emitted
    // 5. barrier job finished, will-be-cancelled job is not executed
    //
    // The tricky part is we cannot know when a job is actually submitted but not
    // executed yet: in this case the future is waiting on channel but this actually
    // occurs within the submit function which can take an arbitrary amount of time
    // and computation to get there.
    //
    // So the solution we take here is to consider the time between when submit is called
    // and the wait on the channel actually starts is small (let's say < 1ms).
    // Hence we put synchronization event right before calling submit and then add
    // a 10ms sleep just for good measure.

    let (barrier_ready_signal_tx, barrier_ready_signal_rx) =
        libparsec_platform_async::channel::bounded(0);
    let (barrier_release_signal_tx, barrier_release_signal_rx) =
        libparsec_platform_async::channel::bounded(0);

    // Steps 1 & 2: barrier job is submitted...
    let barrier_job_task = tokio::spawn({
        let executor = executor.clone();
        async move {
            executor
                .exec({
                    move |_conn| {
                        barrier_ready_signal_tx.send(()).unwrap();
                        barrier_release_signal_rx.recv().unwrap();
                        // Add a sleep to ensure `executor.stop` had time to set the internal
                        // terminated flag, and hence the executor will stop as soon as
                        // our job returns.
                        // On top of that we may have `executor.exec` calls in-flight
                        // that benefit from this wait to settle down.
                        // There is no guarantee 10ms is enough, but `executor.stop` &
                        // `executor.exec` only do simple synchronization operations so it1
                        // should be plenty.
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                })
                .await
        }
    });

    // ...and now barrier job is currently executed
    barrier_ready_signal_rx.recv_async().await.unwrap();

    // Step 3: will-be-cancelled job is submitted
    let (about_to_submit_will_be_cancelled_tx, about_to_submit_will_be_cancelled_rx) =
        libparsec_platform_async::channel::bounded(0);
    let submit_will_be_cancelled_job_task = tokio::spawn({
        let executor = executor.clone();
        async move {
            about_to_submit_will_be_cancelled_tx
                .send_async(())
                .await
                .unwrap();
            executor.exec(|_conn| unreachable!("never scheduled")).await
        }
    });
    about_to_submit_will_be_cancelled_rx
        .recv_async()
        .await
        .unwrap();

    // Step 4: executor gets closed and barrier-release signal is emitted
    barrier_release_signal_tx.send_async(()).await.unwrap();
    executor.stop().await;

    // Step 5: barrier job finished, will-be-cancelled job is not executed
    assert_eq!(barrier_job_task.await.unwrap(), Ok(()));

    // The other job should have been cancelled before execution
    let will_be_cancelled_job = submit_will_be_cancelled_job_task.await.unwrap();
    assert_eq!(will_be_cancelled_job.unwrap_err(), DatabaseError::Closed);

    // Finally any new submited job will fail
    let err = executor
        .exec(|conn| conn.batch_execute("SELECT 1"))
        .await
        .unwrap_err();
    assert_eq!(err, DatabaseError::Closed);

    // Same thing if we try to do a vacuum
    let err = executor.full_vacuum().await.unwrap_err();
    assert_eq!(err, DatabaseError::Closed);
}

#[parsec_test]
#[case::asap(false)]
#[case::after_a_job(true)]
async fn stop_while_idle(#[case] run_a_job: bool) {
    let connection = SqliteConnection::establish(":memory:").unwrap();
    let executor = Arc::new(SqliteExecutor::start(connection, Ok));

    // If we run a job first, the background executor is in it nominal mode.
    // If we don't run however, it's very likely (again no 100% guarantee because
    // concurrency is hard ><'') we will stop the executor before the background
    // executor thread has even started !
    if run_a_job {
        executor.exec(|_conn| ()).await.unwrap();
    }
    executor.stop().await;

    // Finally any new submited job will fail
    let err = executor
        .exec(|conn| conn.batch_execute("SELECT 1"))
        .await
        .unwrap_err();
    assert_eq!(err, DatabaseError::Closed);

    // Same thing if we try to do a vacuum
    let err = executor.full_vacuum().await.unwrap_err();
    assert_eq!(err, DatabaseError::Closed);
}

#[parsec_test]
#[should_panic]
async fn panicking_job() {
    let connection = SqliteConnection::establish(":memory:").unwrap();
    let executor = SqliteExecutor::start(connection, Ok);

    executor
        .exec::<_, DatabaseError>(|_conn| panic!("D'oh !"))
        .await
        .unwrap();
}

#[parsec_test]
async fn fail_reopen_database() {
    let connection = SqliteConnection::establish(":memory:").unwrap();
    let executor = SqliteExecutor::start(connection, |_conn| Err(crate::DatabaseError::Closed));

    let err = executor.full_vacuum().await.unwrap_err();
    assert_eq!(err, DatabaseError::Closed);
    // TODO: Check that we've outputted a log warning saying we failed to reopen the database connection.

    // Because `full_vacuum` failed, the executor should be closed.
    let err = executor
        .exec(|conn| conn.batch_execute("SELECT 1"))
        .await
        .unwrap_err();
    assert_eq!(err, DatabaseError::Closed);

    // Same thing if we try to do a new vacuum
    let err = executor.full_vacuum().await.unwrap_err();
    assert_eq!(err, DatabaseError::Closed);
}
