// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[tokio::test]
pub async fn sleep() {
    let ret =
        libparsec_platform_async::sleep(libparsec_platform_async::Duration::from_secs(0)).await;
    assert!(matches!(ret, ()));
}

#[tokio::test]
pub async fn concurrency() {
    use libparsec_platform_async::{future::*, sleep, Duration};

    let f1 = async {
        sleep(Duration::from_secs(0)).await;
        1
    };

    let f2 = async {
        sleep(Duration::from_secs(0)).await;
        2
    };

    let outcome = f1.or(f2).await;
    // Given both futures resolve after a single yield, `or` has preference on
    // the first one
    assert_eq!(outcome, 1);
}

#[tokio::test]
pub async fn select2() {
    use libparsec_platform_async::{select2, sleep, Duration};
    let ret = select2!(
        _ = sleep(Duration::from_secs(0)) => 1,
        _ = sleep(Duration::from_secs(0)) => 2,
    );
    // Given both futures resolve after a single yield, `or` has preference on
    // the first one
    assert_eq!(ret, 1);
}

#[tokio::test]
pub async fn select2_with_bind() {
    use libparsec_platform_async::{select2, sleep, Duration};
    let ret = select2!(
        a = async { sleep(Duration::from_secs(0)).await; 1 } => {
            a + 1
        },
        _ = sleep(Duration::from_secs(1000)) => unreachable!(),
    );
    assert_eq!(ret, 2);
}

#[tokio::test]
pub async fn future_or_on_tasks_then_await_unfinished_one() {
    use libparsec_platform_async::{future::FutureExt, oneshot, sleep, spawn, Duration};
    let (f2_sx, f2_rx) = oneshot::channel::<i32>();
    let f1 = spawn(async {
        sleep(Duration::from_secs(0)).await;
        1
    });
    let mut f2 = spawn(async { f2_rx.await.unwrap() });

    let outcome = f1.or(&mut f2).await;
    assert_eq!(outcome.unwrap(), 1);

    f2_sx.send(2).unwrap();
    let outcome2 = f2.await;
    assert_eq!(outcome2.unwrap(), 2);
}

// #[tokio::test]
// pub async fn spawn() {
//     use libparsec_platform_async::{spawn, sleep, Duration};
//     let fut = spawn(async { sleep(Duration::from_secs(0)).await; 1 });
//     let ret = fut.await;
//     assert_eq!(ret, 1);

//     let ret = select2!(
//         a = async { sleep(Duration::from_secs(0)).await; 1 } => {
//             a + 1
//         },
//         _ = sleep(Duration::from_secs(1000)) => unreachable!(),
//     );
//     assert_eq!(ret, 2);
// }
