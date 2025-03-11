// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod instant;
mod pretend_future;
mod task;

pub use instant::Instant;
pub use pretend_future::{pretend_future_is_send_on_web, pretend_stream_is_send_on_web};
pub use task::*;

/// Maximum duration allowed by [`gloo_timers::future::sleep`].
/// Internally it convert the duration to u32 milliseconds.
const MAX_SLEEP_DURATION: std::time::Duration = std::time::Duration::from_millis(u32::MAX as u64);

/// Utility function that actually performs the sleep.
/// The duration must be <= [`MAX_SLEEP_DURATION`]
/// To prevent panic when duration is > u32::MAX milliseconds
/// In [`gloo_timers::future::sleep`]
#[inline(always)]
async fn internal_sleep(duration: std::time::Duration) {
    // Cannot use `tokio::time::sleep` since it depends on a timer API that is
    // not provided by `wasm32-unknown-unknown`.
    // Instead we use `gloo_timers` that is a convenient wrapper around js_sys.

    debug_assert!(duration <= MAX_SLEEP_DURATION);
    let not_send_future = gloo_timers::future::sleep(duration);
    pretend_future_is_send_on_web(not_send_future).await
}

#[inline(always)]
pub async fn sleep(mut duration: std::time::Duration) {
    if duration == std::time::Duration::ZERO {
        internal_sleep(duration).await
    } else {
        while duration > MAX_SLEEP_DURATION {
            duration -= MAX_SLEEP_DURATION;
            internal_sleep(MAX_SLEEP_DURATION).await;
        }

        if duration > std::time::Duration::ZERO {
            internal_sleep(duration).await
        }
    }
}

#[inline(always)]
pub async fn yield_now() {
    internal_sleep(std::time::Duration::ZERO).await
}
