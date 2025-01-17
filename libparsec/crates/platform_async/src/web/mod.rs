// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod instant;
mod pretend_future;
mod task;

pub use instant::Instant;
pub use pretend_future::{pretend_future_is_send_on_web, pretend_stream_is_send_on_web};
pub use task::*;

#[inline(always)]
pub async fn sleep(duration: std::time::Duration) {
    // Cannot use `tokio::time::sleep` since it depends on a timer API that is
    // not provided by `wasm32-unknown-unknown`.
    // Instead we use `gloo_timers` that is a convenient wrapper around js_sys.

    // Hotfix: sleep forever if Duration is MAX
    // In practice, gloo_timers::future::sleep panics if the duration cannot be casted into a u32 in milliseconds.
    // TODO: divide the duration in u32::MAX chunks and sleep for each chunk
    if duration == std::time::Duration::MAX {
        loop {
            let not_send_future =
                gloo_timers::future::sleep(std::time::Duration::from_secs_f64(1.0));
            pretend_future_is_send_on_web(not_send_future).await
        }
        return;
    }
    let not_send_future = gloo_timers::future::sleep(duration);
    pretend_future_is_send_on_web(not_send_future).await
}

#[inline(always)]
pub async fn yield_now() {
    sleep(std::time::Duration::from_secs(0)).await
}
