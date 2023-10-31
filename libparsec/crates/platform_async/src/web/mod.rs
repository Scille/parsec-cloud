// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub mod oneshot;
pub mod watch;

use pin_project_lite::pin_project;

#[inline(always)]
pub async fn sleep(duration: std::time::Duration) {
    let not_send_future = gloo_timers::future::sleep(duration);
    pretend_future_is_send_on_web(not_send_future).await
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

pin_project! {
    struct FutureForceSendWrapper<F: std::future::Future>{
        #[pin]
        not_send_future: F
    }
}
unsafe impl<F: std::future::Future> Send for FutureForceSendWrapper<F> {}
impl<F: std::future::Future> std::future::Future for FutureForceSendWrapper<F> {
    type Output = F::Output;

    #[inline(always)]
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        this.not_send_future.poll(cx)
    }
}

/// This function is a noop on non-web platform.
///
/// If you are unhappy with the weather, break the thermometer !
///
/// Web API are not thread safe, hence they are expose in Rust as `!Send`.
/// On native platform however, we use a multi-threaded tokio runtime.
///
/// The outcome of this is that any async function pointer must be `Send` on native
/// by `!Send` on web...
/// This is "slightly annoying" given it means we would have to basically duplicate
/// any code involve async function pointer with `#[cfg(target_arch = "wasm32")]`
/// to mark `Future` vs `Future + Send + 'static`.
///
/// To add insult to injury, compiler is complaining about our stuff not being `Send`
/// web, where is impossible to share data accros multiple threads... which makes
/// `Send` useless in practice (yes WebAssembly is planning to support thread in the
/// future, but for now this is just "pain but no gain").
///
/// So we are taking the "too old for this shit" approach and just mark the future
/// in web as `Send`. Which is completely safe at least until multithread support
/// is added to WebAssembly (at this point, we will have to be careful not to
/// use this feature).
#[inline(always)]
pub fn pretend_future_is_send_on_web<O>(
    not_send_future: impl std::future::Future<Output = O> + 'static,
) -> impl std::future::Future<Output = O> + Send + 'static {
    FutureForceSendWrapper { not_send_future }
}
