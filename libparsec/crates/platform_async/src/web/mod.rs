// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub mod oneshot;
pub mod watch;

#[inline(always)]
pub async fn sleep(duration: std::time::Duration) {
    gloo_timers::future::sleep(duration).await;
}

#[derive(Debug)]
pub struct JoinHandle<F> {
    phantom: std::marker::PhantomData<F>,
}

impl<F> JoinHandle<F> {
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

impl<F> crate::future::Future for JoinHandle<F> {
    type Output = std::result::Result<F, JoinError>;

    #[inline(always)]
    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

#[inline(always)]
pub fn spawn<F>(_future: F) -> JoinHandle<F::Output>
where
    F: crate::future::Future,
{
    todo!()
}

pub fn spawn_local<F>(_future: F) -> JoinHandle<F::Output>
where
    F: crate::future::Future,
{
    todo!()
}
