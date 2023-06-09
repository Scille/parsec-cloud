// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use js_sys::Promise;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

pub struct Timer(Pin<Box<JsFuture>>);

impl Timer {
    /// Create a timer that expires after the given duration of time.
    ///
    /// # Panics
    ///
    /// The Timer may never resolve if we cannot configure the callback timeout.
    pub fn after(duration: Duration) -> Self {
        let promise = Promise::new(&mut |resolve, _reject| {
            let win = window().expect("cannot get the navigator window to set the callback");
            win.set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve,
                duration.as_millis() as i32,
            )
            .expect("cannot set callback for Timer");
        });
        let js_fut = JsFuture::from(promise);
        Self(Box::pin(js_fut))
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx).map(|_| ())
    }
}

pub async fn sleep(duration: Duration) {
    Timer::after(duration).await
}
