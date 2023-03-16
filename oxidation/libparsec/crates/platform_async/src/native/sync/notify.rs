// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// Notifies a single task to wake up.
///
/// `Notify` provides a basic mechanism to notify a single task of an event.
/// `Notify` itself does not carry any data.
/// Instead, it is to be used to signal another task to perform an operation.
pub struct Notify(tokio::sync::Notify);

impl Notify {
    /// Wait for a notification.
    pub fn notified(&self) -> Notified {
        Notified(Box::pin(self.0.notified()))
    }

    /// Notifies a waiting task.
    ///
    /// ```
    /// use libparsec_platform_async::{Notify, spawn};
    /// use std::sync::{Arc, atomic::{Ordering, AtomicBool}};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let notify = Arc::new(Notify::default());
    /// let notify2 = notify.clone();
    ///
    /// let handle = spawn(async move {
    ///     notify2.notified().await;
    ///     42
    /// });
    ///
    /// println!("sending notification");
    /// notify.notify_one();
    ///
    /// assert_eq!(handle.await, 42);
    /// # }
    /// ```
    pub fn notify_one(&self) {
        self.0.notify_one()
    }
}

impl Default for Notify {
    fn default() -> Self {
        Self(tokio::sync::Notify::new())
    }
}

/// Future returned by [Notify::notified()]
pub struct Notified<'a>(Pin<Box<tokio::sync::futures::Notified<'a>>>);

impl<'a> Future for Notified<'a> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

#[test_log::test(tokio::test)]
async fn notify_before() {
    let notify = Notify::default();

    notify.notify_one();
    notify.notified().await;
}

#[test_log::test(tokio::test)]
async fn notify_after() {
    use crate::spawn;
    use std::sync::Arc;
    use tokio::task::yield_now;

    let notify = Arc::new(Notify::default());
    let notify2 = notify.clone();

    let task = spawn(async move {
        notify2.notified().await;
        42
    });

    yield_now().await;
    notify.notify_one();
    let result = task.await;
    assert_eq!(result, 42);
}
