// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub mod oneshot;
pub mod watch;

use std::pin::Pin;

pub struct Delay {
    sleep: Pin<Box<gloo_timers::future::TimeoutFuture>>,
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
        sleep: Box::pin(gloo_timers::future::sleep(duration)),
    }
}

#[derive(Debug)]
pub struct JoinHandle<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<T> JoinHandle<T> {
    #[inline(always)]
    pub fn abort(&self) {
        todo!()
    }
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct JoinError(());

impl JoinError {
    pub fn is_cancelled(&self) -> bool {
        todo!()
    }
    pub fn is_panic(&self) -> bool {
        todo!()
    }
    pub fn into_panic(self) -> Box<dyn std::any::Any> {
        todo!()
    }
    pub fn try_into_panic(self) -> Result<Box<dyn std::any::Any>, JoinError> {
        todo!()
    }
}

impl<T> crate::future::Future for JoinHandle<T> {
    type Output = std::result::Result<T, JoinError>;

    #[inline(always)]
    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

#[inline(always)]
pub fn spawn<T>(_future: T) -> JoinHandle<T::Output>
where
    T: crate::future::Future,
{
    todo!()
}
