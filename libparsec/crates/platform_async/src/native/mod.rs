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
    not_send_future: impl std::future::Future<Output = O> + Send + 'static,
) -> impl std::future::Future<Output = O> + Send + 'static {
    not_send_future
}
