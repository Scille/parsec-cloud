#![cfg(not(target_arch = "wasm32"))]
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_platform_async::{
    channel::{bounded, RecvError, Sender},
    JoinSet, Notify, Timer,
};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

const TIMEOUT: u64 = 250;

#[derive(Debug)]
struct Load {
    delay: Duration,
    result: Mode,
}

impl Load {
    fn new(delay: Duration, result: Mode) -> Self {
        Self { delay, result }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Fail,
    Success,
}

#[tokio::test]
async fn test_eyeballs() -> Result<(), ()> {
    simple_logger::init_with_level(log::Level::Debug).expect("cannot initialize simple logger");

    let targets = vec![
        Load::new(Duration::from_millis(2300), Mode::Fail),
        Load::new(Duration::from_millis(300), Mode::Fail),
        Load::new(Duration::from_millis(300), Mode::Success),
        Load::new(Duration::from_millis(100), Mode::Success),
    ];

    log::debug!("starting with {} targets", targets.len());
    let start_time = Instant::now();
    assert_eq!(Ok(2), dbg!(eyeballs(targets).await));
    let elapsed = start_time.elapsed();
    log::info!("elapsed time {}ms", elapsed.as_millis());
    assert!(elapsed < Duration::from_millis(900));
    Ok(())
}

async fn eyeballs(targets: Vec<Load>) -> Result<usize, RecvError> {
    let failed_attempt = targets
        .iter()
        .map(|_| Arc::new(Notify::default()))
        .collect::<Vec<_>>();
    let targets = Arc::new(targets);
    let join_set = JoinSet::default();
    let join_set = Arc::new(Mutex::new(join_set));
    let (tx, rx) = bounded::<usize>(targets.len());

    spawn_attempt(0, targets, failed_attempt, join_set.clone(), tx);
    let result = dbg!(rx.recv_async().await);
    join_set.lock().unwrap().abort_all();
    result
}

fn spawn_attempt(
    which: usize,
    targets: Arc<Vec<Load>>,
    failed_attempt: Vec<Arc<Notify>>,
    join_set: Arc<Mutex<JoinSet<()>>>,
    sender: Sender<usize>,
) {
    log::debug!("spawning task #{which}");

    join_set.lock().unwrap().spawn(attempt(
        which,
        targets,
        failed_attempt,
        join_set.clone(),
        sender,
    ));
}

async fn attempt(
    which: usize,
    targets: Arc<Vec<Load>>,
    failed_attempt: Vec<Arc<Notify>>,
    join_set: Arc<Mutex<JoinSet<()>>>,
    sender: Sender<usize>,
) {
    log::info!("[#{which}] starting task");
    if which > 0 {
        let attempt_to_wait = which - 1;
        tokio::select! {
            _ = Timer::after(Duration::from_millis(TIMEOUT)) => {
                log::info!("[#{which}] previous task timed out");
            }
            _ = failed_attempt[attempt_to_wait].notified() => {
                log::info!("[#{which}] previous task finished");
            }
        }
    }

    if which + 1 < targets.len() {
        let next_attempt = which + 1;
        log::info!("[#{which}] start next task");
        spawn_attempt(
            next_attempt,
            targets.clone(),
            failed_attempt.clone(),
            join_set.clone(),
            sender.clone(),
        );
    }

    let load = &targets[which];
    log::info!("[#{which}] executing load {load:?}");
    match mock_load(load).await {
        Mode::Fail => {
            log::info!("[#{which}] finished with error");
            failed_attempt[which].notify_one();
        }
        Mode::Success => {
            log::info!("[#{which}] finished with success");
            sender.send_async(which).await.unwrap();
        }
    }
}

async fn mock_load(load: &Load) -> Mode {
    Timer::after(load.delay).await;
    load.result
}
