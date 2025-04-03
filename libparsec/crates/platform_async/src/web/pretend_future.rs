// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pin_project::pin_project;

#[pin_project]
struct FutureForceSendWrapper<F: std::future::Future> {
    #[pin]
    not_send_future: F,
}

unsafe impl<F: std::future::Future> Send for FutureForceSendWrapper<F> {}
impl<F: std::future::Future> std::future::Future for FutureForceSendWrapper<F> {
    type Output = F::Output;

    #[inline(always)]
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
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
/// but `!Send` on web...
/// This is "slightly annoying" given it means we would have to basically duplicate
/// any code involving async function pointer with `#[cfg(target_arch = "wasm32")]`
/// to mark `Future` vs `Future + Send + 'static`.
///
/// To add insult to injury, compiler is complaining about our stuff not being `Send`
/// no web, where it is impossible to share data across multiple threads... which makes
/// `Send` useless in practice (yes WebAssembly is planning to support thread in the
/// future, but for now this is just "pain but no gain").
///
/// So we are taking the "too old for this shit" approach and just mark the future
/// in web as `Send`. Which is completely safe at least until multithread support
/// is added to WebAssembly (at this point, we will have to be careful not to
/// use this feature).
#[inline(always)]
pub fn pretend_future_is_send_on_web<'a, O>(
    not_send_future: impl std::future::Future<Output = O> + 'a,
) -> impl std::future::Future<Output = O> + Send + 'a {
    FutureForceSendWrapper { not_send_future }
}

#[pin_project]
struct StreamForceSendWrapper<S: futures::stream::Stream> {
    #[pin]
    not_send_stream: S,
}

unsafe impl<S: futures::stream::Stream> Send for StreamForceSendWrapper<S> {}
impl<S: futures::stream::Stream> futures::stream::Stream for StreamForceSendWrapper<S> {
    type Item = S::Item;

    #[inline(always)]
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();
        this.not_send_stream.poll_next(cx)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.not_send_stream.size_hint()
    }
}

/// This function is a noop on non-web platform.
///
/// See `pretend_future_is_send_on_web` for the full explanation.
#[inline(always)]
pub fn pretend_stream_is_send_on_web<'a, I>(
    not_send_stream: impl futures::stream::Stream<Item = I> + 'a,
) -> impl futures::stream::Stream<Item = I> + Send + 'a {
    StreamForceSendWrapper { not_send_stream }
}
