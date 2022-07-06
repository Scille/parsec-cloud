// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#![cfg(target_arch = "wasm32")]

use platform_async::wasm32::{spawn, Notify};
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
