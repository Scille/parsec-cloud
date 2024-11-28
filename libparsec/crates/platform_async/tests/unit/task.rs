// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::{event::Event, future::select, pin_mut, sleep, spawn, try_task_id, Duration, Instant};

#[parsec_test]
pub async fn sleep_simple() {
    let now = Instant::now();

    let ret = sleep(Duration::from_millis(3)).await;
    p_assert_eq!(ret, ());

    assert!(now.elapsed().as_millis() >= 3);
}

#[parsec_test]
pub async fn concurrency() {
    let f1 = async {
        sleep(Duration::from_secs(0)).await;
        1
    };

    let f2 = async {
        sleep(Duration::from_secs(0)).await;
        2
    };

    pin_mut!(f1);
    pin_mut!(f2);

    let (res1, f2) = select(f1, f2).await.factor_first();

    // Given both futures resolve after a single yield, `or` has preference on
    // the first one
    p_assert_eq!(res1, 1);

    let res2 = f2.await;
    p_assert_eq!(res2, 2);
}

#[parsec_test]
pub async fn simple() {
    let handle = spawn(async {
        sleep(Duration::from_secs(0)).await;
        1
    });
    let ret = handle.await.unwrap();
    p_assert_eq!(ret, 1);
}

#[parsec_test]
pub async fn task_id() {
    // Not running from a task returns `None`
    p_assert_matches!(try_task_id(), None);

    let handle = spawn(async {
        let task_id = try_task_id();
        let nested_task_id = spawn(async { try_task_id() }).await.unwrap();

        (task_id, nested_task_id)
    });
    let (task_id, nested_task_id) = handle.await.unwrap();
    p_assert_matches!(task_id, Some(_));
    p_assert_matches!(nested_task_id, Some(_));
    p_assert_ne!(task_id, nested_task_id);
}

#[parsec_test]
pub async fn ensure_non_blocking() {
    let event = Event::new();
    let event_listen = event.listen();

    let handle1 = spawn(async {
        let nested = spawn(async move {
            event.notify(1);
            1
        });

        nested.await.unwrap()
    });

    let handle2 = spawn(async {
        event_listen.await;
        2
    });

    let ret = handle2.await.unwrap();
    p_assert_eq!(ret, 2);

    let ret = handle1.await.unwrap();
    p_assert_eq!(ret, 1);
}

#[parsec_test]
#[cfg_attr(
    target_arch = "wasm32",
    ignore = "TODO: wasm-pack test doesn't seem compatible with panic unwind..."
)]
pub async fn panicking() {
    let handle = spawn(async {
        sleep(Duration::from_secs(0)).await;
        panic!("D'oh!")
    });

    let err = handle.await.unwrap_err();
    assert!(err.is_panic());
    let err_as_str = format!("{}", err);
    assert!(
        err_as_str.ends_with("panicked with message \"D'oh!\""),
        "{}",
        err_as_str
    );
}

#[parsec_test]
pub async fn aborted_by_handle() {
    let handle = spawn(async {
        sleep(Duration::MAX).await;
    });

    pin_mut!(handle);

    p_assert_eq!(handle.is_finished(), false);
    handle.abort();

    let err = handle.as_mut().await.unwrap_err();
    assert!(err.is_cancelled());

    let err_as_str = format!("{}", err);
    assert!(err_as_str.ends_with("cancelled"), "{}", err_as_str);

    p_assert_eq!(handle.is_finished(), true);
}

#[parsec_test]
pub async fn aborted_by_aborter() {
    let handle = spawn(async {
        sleep(Duration::MAX).await;
    });

    p_assert_eq!(handle.is_finished(), false);

    let aborter = handle.abort_handle();
    p_assert_eq!(aborter.is_finished(), false);

    aborter.abort();

    let err = handle.await.unwrap_err();
    assert!(err.is_cancelled());

    p_assert_eq!(aborter.is_finished(), true);
}
