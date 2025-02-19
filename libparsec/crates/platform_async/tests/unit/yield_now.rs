// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{event::Event, spawn, yield_now};
use libparsec_tests_lite::prelude::*;

#[parsec_test]
pub async fn aborted_by_handle() {
    let should_stop_busy_task = Event::new();
    let on_should_stop_busy_task = should_stop_busy_task.listen();
    let handle = spawn(async move {
        // Busy loop that will never finish if yield_now is not working
        // (i.e. under a monothreaded async runtime such as on web or on native
        // for tests, the other coroutine won't get a chance to run to abort this
        // mad coroutine).
        let mut count = 0usize;
        loop {
            yield_now().await;
            count += 1;
            if count == 100 {
                should_stop_busy_task.notify(u32::MAX);
            }
        }
    });

    on_should_stop_busy_task.await;
    handle.abort();
    handle.await.unwrap_err();
}
