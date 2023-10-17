// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::pin::Pin;

pub mod oneshot {
    pub use tokio::sync::oneshot::{
        channel,
        error::{RecvError, TryRecvError},
        Receiver, Sender,
    };
}

pub mod watch {
    pub use tokio::sync::watch::{
        channel,
        error::{RecvError, SendError},
        Receiver, Ref, Sender,
    };
}

pub struct Delay {
    sleep: Pin<Box<tokio::time::Sleep>>,
}

impl std::future::Future for Delay {
    type Output = ();

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.sleep.as_mut().poll(cx)
    }
}

#[inline(always)]
pub fn sleep(duration: std::time::Duration) -> Delay {
    Delay {
        sleep: Box::pin(tokio::time::sleep(duration)),
    }
}

#[derive(Debug)]
pub struct JoinHandle<T>(tokio::task::JoinHandle<T>);

impl<T> JoinHandle<T> {
    #[inline(always)]
    pub fn abort(&self) {
        self.0.abort()
    }
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        self.0.is_finished()
    }
}

impl<T> crate::future::Future for JoinHandle<T> {
    type Output = std::result::Result<T, tokio::task::JoinError>;

    #[inline(always)]
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        use crate::future::FutureExt;
        self.0.poll(cx)
    }
}

#[inline(always)]
pub fn spawn<T>(future: T) -> JoinHandle<T::Output>
where
    T: crate::future::Future + Send + 'static,
    T::Output: Send + 'static,
{
    JoinHandle(tokio::spawn(future))
}
