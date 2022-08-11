// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

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

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.sleep.as_mut().poll(cx)
    }
}
