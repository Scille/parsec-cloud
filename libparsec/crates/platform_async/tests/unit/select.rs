// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use crate::{Duration, future::pending, lock::Mutex, select2_biased, sleep};
use libparsec_tests_lite::prelude::*;

#[parsec_test]
pub async fn select2_biased() {
    let ret = select2_biased!(
        _ = sleep(Duration::from_secs(0)) => 1,
        _ = pending::<()>() => unreachable!(),
    );
    p_assert_eq!(ret, 1);

    let ret = select2_biased!(
        _ = pending::<()>() => unreachable!(),
        _ = sleep(Duration::from_secs(0)) => 2,
    );
    p_assert_eq!(ret, 2);
}

#[parsec_test]
pub async fn select2_biased_with_bind() {
    let ret = select2_biased!(
        a = async { sleep(Duration::from_secs(0)).await; 1 } => {
            a + 1
        },
        _ = pending::<()>() => unreachable!(),
    );
    p_assert_eq!(ret, 2);
}

#[parsec_test]
pub async fn select2_biased_actually_non_blocking() {
    let events = Arc::new(Mutex::new(vec![]));
    macro_rules! push_event {
        ($event:expr) => {{
            let mut guard = events.lock().await;
            guard.push($event);
        }};
    }

    let ret = select2_biased!(
        _ = async {
                push_event!("1 starts");
                sleep(Duration::from_millis(1)).await;
                push_event!("1 continue");
                sleep(Duration::from_millis(1)).await;
                push_event!("1 done");
            } => 1,
        _ = async {
            push_event!("2 starts");
            sleep(Duration::from_millis(1)).await;
            push_event!("2 continue");
            sleep(Duration::from_millis(2)).await;
            push_event!("2 done");
        } => unreachable!(),
    );
    p_assert_eq!(ret, 1);
    p_assert_eq!(
        *events.lock().await,
        vec!["1 starts", "2 starts", "1 continue", "2 continue", "1 done",]
    );
}

#[parsec_test]
pub async fn select3_biased() {
    let ret = select3_biased!(
        _ = sleep(Duration::from_secs(0)) => 1,
        _ = pending::<()>() => unreachable!(),
        _ = pending::<()>() => unreachable!(),
    );
    p_assert_eq!(ret, 1);
}

#[parsec_test]
pub async fn select3_biased_actually_non_blocking() {
    let events = Arc::new(Mutex::new(vec![]));
    macro_rules! push_event {
        ($event:expr) => {{
            let mut guard = events.lock().await;
            guard.push($event);
        }};
    }

    let ret = select3_biased!(
        _ = async {
                push_event!("1 starts");
                sleep(Duration::from_millis(1)).await;
                push_event!("1 continue");
                sleep(Duration::from_millis(1)).await;
                push_event!("1 done");
            } => 1,
        _ = async {
            push_event!("2 starts");
            sleep(Duration::from_millis(1)).await;
            push_event!("2 continue");
            sleep(Duration::from_millis(2)).await;
            push_event!("2 done");
        } => unreachable!(),
        _ = async {
            push_event!("3 starts");
            sleep(Duration::from_millis(1)).await;
            push_event!("3 continue");
            sleep(Duration::from_millis(2)).await;
            push_event!("3 done");
        } => unreachable!(),
    );
    p_assert_eq!(ret, 1);
    p_assert_eq!(
        *events.lock().await,
        vec![
            "1 starts",
            "2 starts",
            "3 starts",
            "1 continue",
            "2 continue",
            "3 continue",
            "1 done",
        ]
    );
}
