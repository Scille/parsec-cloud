// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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

#[inline(always)]
pub fn sleep(duration: std::time::Duration) -> impl crate::future::Future<Output = ()> {
    tokio::time::sleep(duration)
}

#[derive(Debug)]
pub struct JoinHandle<F>(tokio::task::JoinHandle<F>);

impl<F> JoinHandle<F> {
    #[inline(always)]
    pub fn abort(&self) {
        self.0.abort()
    }
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        self.0.is_finished()
    }
}

impl<F> crate::future::Future for JoinHandle<F> {
    type Output = std::result::Result<F, tokio::task::JoinError>;

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
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: crate::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    JoinHandle(tokio::task::spawn(future))
}

pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output>
where
    F: crate::future::Future + 'static,
    F::Output: 'static,
{
    JoinHandle(tokio::task::spawn_local(future))
}
