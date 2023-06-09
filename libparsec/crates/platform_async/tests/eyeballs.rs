// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use instant::Instant;
use libparsec_platform_async::{
    channel::{bounded, RecvError, Sender},
    JoinSet, Notify,
};
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
    time::Duration,
};

const TIMEOUT: u64 = 250;
const TIMEOUT_DURATION: Duration = Duration::from_millis(TIMEOUT);

#[derive(Debug)]
struct Load {
    id: u32,
    delay: Duration,
    result: Mode,
    timer: Mutex<Option<Instant>>,
    duration: Mutex<Option<Duration>>,
}

impl Load {
    fn new(id: u32, delay: Duration, result: Mode) -> Self {
        Self {
            id,
            delay,
            result,
            timer: Mutex::new(None),
            duration: Mutex::new(None),
        }
    }

    fn expected_duration(&self) -> Duration {
        if self.delay < TIMEOUT_DURATION || self.result == Mode::Success {
            self.delay
        } else {
            TIMEOUT_DURATION
        }
    }

    async fn start(&self) -> Mode {
        self.timer.lock().unwrap().replace(Instant::now());
        log::debug!("[{self}] start");
        libparsec_platform_async::sleep(self.delay).await;
        self.stop();
        self.result
    }

    fn stop(&self) {
        let elapsed = self.timer.lock().unwrap().unwrap().elapsed();
        let mut duration = self.duration.lock().unwrap();
        if duration.is_none() || self.result == Mode::Success {
            log::debug!("[{self}] stop elapsed={:?}", elapsed);
            duration.replace(elapsed);
        } else {
            log::debug!(
                "[{self}] stop elapsed={:?} (ignoring because of failure)",
                elapsed
            );
        }
    }
}

impl Display for Load {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "load#{}", self.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Fail,
    Success,
}

async fn test_eyeballs() -> Result<(), ()> {
    let targets = vec![
        Load::new(0, Duration::from_millis(2300), Mode::Fail),
        Load::new(1, Duration::from_millis(300), Mode::Fail),
        Load::new(2, Duration::from_millis(300), Mode::Success),
        Load::new(3, Duration::from_millis(100), Mode::Success),
    ];
    let targets = Arc::new(targets);
    /// The expected time is calculated like like so:
    /// - 2 Task will take too much time and fail (the first 2 one).
    /// - The first task to success will take 300ms to finish.
    const EXPECTED_DURATION: Duration = Duration::from_millis(TIMEOUT * 2 + 300);

    log::debug!("starting with {} targets", targets.len());
    let start_time = Instant::now();
    assert_eq!(Ok(2), dbg!(eyeballs(targets.clone()).await));
    let elapsed = start_time.elapsed();

    log::info!("elapsed time {:?}", elapsed);
    assert!(
        elapsed > EXPECTED_DURATION,
        "We finished earlier than expected"
    );
    // We add some value to take account the possible overhead to switch between async task (here half the expected duration).
    assert!(
        elapsed < EXPECTED_DURATION + EXPECTED_DURATION / 2,
        "We take too much time"
    );

    // We check the time taken by the tasks that actually finished.
    for (load, duration) in targets
        .iter()
        .filter_map(|load| (*load.duration.lock().unwrap()).map(|duration| (load, duration)))
    {
        let expected_duration = load.expected_duration();
        log::debug!(
            "{load} take {:?}, we expect {:?}",
            duration,
            expected_duration
        );
        assert!(duration > expected_duration);
        assert!(duration < expected_duration + expected_duration / 2);
    }
    Ok(())
}

async fn eyeballs(targets: Arc<Vec<Load>>) -> Result<usize, RecvError> {
    let failed_attempt = targets
        .iter()
        .map(|_| Arc::new(Notify::default()))
        .collect::<Vec<_>>();
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
        let timer = Instant::now();
        futures_lite::future::or(
            async {
                libparsec_platform_async::sleep(Duration::from_millis(TIMEOUT)).await;
                targets[attempt_to_wait].stop();
                let elapsed = timer.elapsed();
                log::info!("[#{which}] previous task timed out after {:?} ", elapsed);
            },
            async {
                failed_attempt[attempt_to_wait].notified().await;
                targets[attempt_to_wait].stop();
                let elapsed = timer.elapsed();
                log::info!("[#{which}] previous task finished after {:?}", elapsed);
            },
        )
        .await;
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
    match load.start().await {
        Mode::Fail => {
            log::info!("[{load}] finished with error");
            failed_attempt[which].notify_one();
        }
        Mode::Success => {
            log::info!("[{load}] finished with success");
            sender.send_async(which).await.unwrap();
        }
    }
}

mod web {
    #![cfg(target_arch = "wasm32")]

    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_eyeballs() {
        console_log::init_with_level(log::Level::Debug).expect("cannot initialize console logger");
        super::test_eyeballs().await.unwrap()
    }
}

mod native {
    #![cfg(not(target_arch = "wasm32"))]

    #[libparsec_tests_fixtures::parsec_test]
    async fn test_eyeballs() -> Result<(), ()> {
        super::test_eyeballs().await
    }
}
