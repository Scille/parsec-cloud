// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;
use std::time::Duration;

use crate::{oneshot, select2_biased, sleep};

#[parsec_test]
pub async fn oneshot() {
    let (sx, rx) = oneshot::channel::<u8>();

    #[allow(clippy::let_unit_value)]
    let ret = sx.send(42).unwrap();
    p_assert_eq!(ret, ());

    let ret = rx.await.unwrap();
    p_assert_eq!(ret, 42);
}

#[parsec_test]
pub async fn oneshot_try_recv() {
    let (sx, mut rx) = oneshot::channel::<u8>();

    let err = rx.try_recv().unwrap_err();
    p_assert_matches!(err, oneshot::error::TryRecvError::Empty);

    #[allow(clippy::let_unit_value)]
    let ret = sx.send(42).unwrap();
    p_assert_eq!(ret, ());

    let ret = rx.try_recv().unwrap();
    p_assert_eq!(ret, 42);

    let err = rx.try_recv().unwrap_err();
    p_assert_matches!(err, oneshot::error::TryRecvError::Closed);
}

#[parsec_test]
pub async fn oneshot_receiver_dropped_before_send() {
    let (sx, rx) = oneshot::channel::<u8>();
    drop(rx);

    let err = sx.send(42).unwrap_err();
    p_assert_eq!(err, 42);
}

#[parsec_test]
pub async fn oneshot_sender_dropped_before_send() {
    let (sx, rx) = oneshot::channel::<u8>();
    drop(sx);

    let err = rx.await.unwrap_err();
    p_assert_matches!(err, oneshot::error::RecvError { .. });
}

#[parsec_test]
pub async fn oneshot_sender_dropped_before_send_try_recv() {
    let (sx, mut rx) = oneshot::channel::<u8>();
    drop(sx);

    let err = rx.try_recv().unwrap_err();
    p_assert_matches!(err, oneshot::error::TryRecvError::Closed);
}

#[parsec_test]
pub async fn oneshot_recv_actually_non_blocking() {
    let (sx, rx) = oneshot::channel::<u8>();

    let ret = select2_biased!(
        // Since select2 is biased, rx is awaited first...
        r = rx => r.unwrap() + 1,
        _ = async {
            sleep(Duration::from_millis(1)).await;
            sx.send(41).unwrap();
            // ...and this sleep never returns since "rx got message" is checked first
            sleep(Duration::from_millis(0)).await;
        } => unreachable!(),
    );

    p_assert_eq!(ret, 42);
}
