#![cfg(target_arch = "wasm32")]
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_platform_async::wasm32::{spawn, Notify};
use std::sync::Arc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn notify_before() {
    let notify = Notify::default();

    notify.notify_one();
    notify.notified().await;
}

#[wasm_bindgen_test]
async fn notify_after() {
    let notify = Arc::new(Notify::default());
    let notify2 = notify.clone();

    let task = spawn(async move {
        notify2.notified().await;
        42
    });

    notify.notify_one();
    let result = task.await;
    assert_eq!(result, 42);
}
