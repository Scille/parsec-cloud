// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;
use std::time::Duration;

use crate::{select2_biased, sleep, watch};

#[parsec_test]
pub async fn watch() {
    let (sx, mut rx) = watch::channel("state1");

    let rx2 = rx.clone();
    let sx2 = sx.clone();

    p_assert_eq!(rx.has_changed().unwrap(), false);
    p_assert_eq!(*rx.borrow(), "state1");

    p_assert_eq!(*rx.borrow_and_update(), "state1");
    p_assert_eq!(rx.has_changed().unwrap(), false);

    sx.send("state2").unwrap();

    p_assert_eq!(rx.has_changed().unwrap(), true);
    p_assert_eq!(rx2.has_changed().unwrap(), true);
    p_assert_eq!(*rx.borrow_and_update(), "state2");
    p_assert_eq!(rx.has_changed().unwrap(), false);
    p_assert_eq!(rx2.has_changed().unwrap(), true);

    sx2.send("state3").unwrap();
    p_assert_eq!(rx.has_changed().unwrap(), true);
    p_assert_eq!(*rx.borrow(), "state3");
}

#[parsec_test]
pub async fn watch_recv_actually_non_blocking() {
    let (sx, mut rx) = watch::channel(0);

    let ret = select2_biased!(
        // Since select2 is biased, rx is awaited first...
        r = rx.wait_for(|x| *x == 41) => *r.unwrap() + 1,
        _ = async {
            sleep(Duration::from_millis(1)).await;
            sx.send(1).unwrap(); // Ignored event
            sleep(Duration::from_millis(1)).await;
            sx.send(41).unwrap(); // Expected event
            // ...and this sleep never returns since "rx got message" is checked first
            sleep(Duration::from_millis(0)).await;
        } => unreachable!(),
    );

    p_assert_eq!(ret, 42);
}
