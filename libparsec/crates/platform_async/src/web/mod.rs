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

    pretend_future_is_send_on_web(async {
        // There is two issues here:
        // - `gloo_timers` requires a u32 as duration in milliseconds which is smaller (~49 days)
        //   than what `std::time::Duration` can represent (~584,942,417,355 years).
        // - The Javascript timeout API internally uses a i32 to store the duration in
        //   milliseconds (~24 days), leading to an instantaneous wake up if `gloo_timers`'s
        //   u32 represents a value too large.
        //
        // Realistically, all sleep are below 24 days so those issues are not a problem...
        // unless when sleeping forever, since we use `std::time::Duration::MAX` in this case!
        //
        // So the solution is to consider everything above 24 days as an infinite sleep.
        //
        // see:
        // - https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout#maximum_delay_value0
        // - https://github.com/rustwasm/gloo/issues/121
        match i32::try_from(duration.as_millis()) {
            Ok(millis) => {
                gloo_timers::future::TimeoutFuture::new(millis as u32).await;
            }
            // Sleep is bigger than 24 days, consider it is a sleep forever then !
            Err(_) => loop {
                gloo_timers::future::TimeoutFuture::new(i32::MAX as u32).await;
            },
        };
    })
    .await
}
