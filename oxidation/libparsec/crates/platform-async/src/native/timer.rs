use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures::FutureExt;
use tokio::time::{sleep, Sleep};

pub struct Timer {
    sleep: Pin<Box<Sleep>>,
}

impl Timer {
    /// Create a timer that expires after the given duration of time.
    ///
    /// ```
    /// use std::time::{Duration, Instant};
    /// use libparsec_platform_async::Timer;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let timer = Instant::now();
    /// Timer::after(Duration::from_millis(100)).await;
    /// assert!(timer.elapsed() < Duration::from_millis(110));
    /// # }
    /// ```
    pub fn after(duration: Duration) -> Timer {
        Self {
            sleep: Box::pin(sleep(duration)),
        }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.sleep.poll_unpin(cx)
    }
}
