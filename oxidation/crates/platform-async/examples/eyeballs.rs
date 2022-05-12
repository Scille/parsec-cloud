#![cfg(not(target_arch = "wasm32"))]

use platform_async::{
    platform::{spawn, Task},
    task::Taskable,
};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, Notify};

const TIMEOUT: u64 = 250;

#[derive(Clone, Copy, Debug)]
enum Mode {
    Fail(Duration),
    Success(Duration),
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    simple_logger::init_with_level(log::Level::Debug).expect("cannot initialise simple logger");

    let targets = vec![
        Mode::Fail(Duration::from_millis(2300)),
        Mode::Fail(Duration::from_millis(300)),
        Mode::Success(Duration::from_millis(300)),
        Mode::Success(Duration::from_millis(100)),
    ];

    log::debug!("starting with {} targets", targets.len());
    let start_time = Instant::now();
    assert_eq!(Ok(2), dbg!(eyeballs(targets).await));
    let elapsed = start_time.elapsed();
    log::info!("elapsed time {}ms", elapsed.as_millis());
    assert!(elapsed < Duration::from_millis(900));
    Ok(())
}

async fn eyeballs(targets: Vec<Mode>) -> Result<usize, ()> {
    let failed_attempt = targets
        .iter()
        .map(|_| Arc::new(Notify::new()))
        .collect::<Vec<_>>();
    let targets = Arc::new(targets);
    let join_set = Vec::with_capacity(targets.len());
    let join_set = Arc::new(Mutex::new(join_set));
    let (tx, mut rx) = mpsc::channel::<usize>(targets.len());

    spawn_attempt(0, targets, failed_attempt, join_set.clone(), tx);
    let result = dbg!(rx.recv().await);
    for task in join_set.lock().unwrap().iter() {
        task.cancel();
    }
    result.ok_or(())
}

fn spawn_attempt(
    which: usize,
    targets: Arc<Vec<Mode>>,
    failed_attempt: Vec<Arc<Notify>>,
    join_set: Arc<Mutex<Vec<Task<()>>>>,
    sender: mpsc::Sender<usize>,
) {
    log::debug!("spawning task #{which}");

    let task = spawn(attempt(
        which,
        targets,
        failed_attempt,
        join_set.clone(),
        sender,
    ));

    join_set.lock().unwrap().push(task);
}

async fn attempt(
    which: usize,
    targets: Arc<Vec<Mode>>,
    failed_attempt: Vec<Arc<Notify>>,
    join_set: Arc<Mutex<Vec<Task<()>>>>,
    sender: mpsc::Sender<usize>,
) {
    log::info!("[#{which}] starting task");
    if which > 0 {
        let attempt_to_wait = which - 1;
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_millis(TIMEOUT)) => {
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

    let load = targets[which];
    log::info!("[#{which}] executing load {load:?}");
    match mock_load(load).await {
        Err(_) => {
            log::info!("[#{which}] finished with error");
            failed_attempt[which].notify_waiters();
        }
        Ok(_) => {
            log::info!("[#{which}] finished with success");
            sender.send(which).await.unwrap();
        }
    }
}

async fn mock_load(mode: Mode) -> Result<(), ()> {
    match mode {
        Mode::Fail(delay) => {
            tokio::time::sleep(delay).await;
            Err(())
        }
        Mode::Success(delay) => {
            tokio::time::sleep(delay).await;
            Ok(())
        }
    }
}
