// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::parsec_test;

#[parsec_test]
pub async fn sleep() {
    use crate::{sleep, Duration};

    let ret = sleep(Duration::from_secs(0)).await;
    assert!(matches!(ret, ()));
}

#[parsec_test]
pub async fn concurrency() {
    use crate::{future::select, pin_mut, sleep, Duration};

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
    assert_eq!(res1, 1);

    let res2 = f2.await;
    assert_eq!(res2, 2);
}

#[parsec_test]
pub async fn select2_biased() {
    use crate::{future::pending, select2_biased, sleep, Duration};

    let ret = select2_biased!(
        _ = sleep(Duration::from_secs(0)) => 1,
        _ = pending::<()>() => unreachable!(),
    );
    assert_eq!(ret, 1);

    let ret = select2_biased!(
        _ = pending::<()>() => unreachable!(),
        _ = sleep(Duration::from_secs(0)) => 2,
    );
    assert_eq!(ret, 2);
}

#[parsec_test]
pub async fn select2_biased_with_bind() {
    use crate::{future::pending, select2_biased, sleep, Duration};

    let ret = select2_biased!(
        a = async { sleep(Duration::from_secs(0)).await; 1 } => {
            a + 1
        },
        _ = pending::<()>() => unreachable!(),
    );
    assert_eq!(ret, 2);
}

#[parsec_test]
pub async fn select3_biased() {
    use crate::{future::pending, select3_biased, sleep, Duration};

    let ret = select3_biased!(
        _ = sleep(Duration::from_secs(0)) => 1,
        _ = pending::<()>() => unreachable!(),
        _ = pending::<()>() => unreachable!(),
    );
    assert_eq!(ret, 1);
}

// TODO: spawn is not implemented for web
#[cfg(not(target_arch = "wasm32"))]
#[parsec_test]
pub async fn spawn() {
    use crate::{sleep, spawn, Duration};

    let fut = spawn(async {
        sleep(Duration::from_secs(0)).await;
        1
    });
    let ret = fut.await.unwrap();
    assert_eq!(ret, 1);
}
