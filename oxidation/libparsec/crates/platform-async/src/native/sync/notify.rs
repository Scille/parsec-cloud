// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

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
///
/// ```
/// use libparsec_platform_async::{Notify, spawn};
/// use std::sync::Arc;
///
/// # #[tokio::main]
/// # async fn main() {
/// let notify = Arc::new(Notify::default());
/// let notify2 = notify.clone();
///
/// let handle = spawn(async move {
///     notify2.notified().await;
///     println!("receive notification");
/// });
///
/// println!("sending notification");
/// notify.notify_one();
///
/// handle.await;
/// # }
/// ```
pub struct Notify(tokio::sync::Notify);

impl Notify {
    /// Wait for a notification.
    pub fn notified(&self) -> Notified<'_> {
        Notified(Box::pin(self.0.notified()))
    }

    /// Notifies a waiting task.
    pub fn notify_one(&self) {
        self.0.notify_one()
    }

    /// Notifies all waiting tasks.
    pub fn notify_waiters(&self) {
        self.0.notify_waiters()
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

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}
