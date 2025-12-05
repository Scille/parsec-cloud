// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// What is this crate even for ? Can't we just use tokio anywhere and call it a day ?
//
// Currently, Tokio supports wasm32 but not web.
//
// What its means is Tokio compiles fine on `wasm32-*` platform, but runtime
// errors will occur if the wrong feature is used.
//
// Indeed, from WebAssembly's point of view the web platform is not standardized
// (i.e. WASI is not supported on web platform), hence the fact the compilation
// platform for web is `wasm32-unknown-unknown`.
// In practice this means things such as timer or network are not available from
// Tokio point of view (and also from the Rust std lib), so we have to rely on
// 3rd party crates to access them (mostly `web_sys`).
//
// On top of that we got additional fun with Send+Sync futures given Tokio runtime
// is designed around a multi-threaded work-stealing runtime, while on web the runtime
// is single threaded (and hence the `web_sys` APIs are !Send).

pub mod prelude {
    pub use super::*;
}

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(not(target_arch = "wasm32"))]
use native as platform;
#[cfg(target_arch = "wasm32")]
use web as platform;

// Try to rely as much as possible on platform-agnostic components
pub use async_broadcast as broadcast;
pub use async_lock as lock;
pub use event_listener as event;
pub use flume as channel;
pub use futures::future;
pub use futures::pin_mut;
pub use futures::stream;
pub use tokio::sync::oneshot;
pub use tokio::sync::watch;

// We don't expose `futures::select` macro & co for the moment: let's try to use only our
// custom `select2_biased` macro and `futures::future::select` instead.

/// Poor's man version of `tokio::select!`, be careful when using in loop (see fused future) !
///
/// The typical intended use for this is to add timeout/cancel signal support
/// to an asynchronous operation.
///
/// The reasons for this macro are:
/// - `tokio::select!` is not available on web.
/// - `futures::select!` requires the futures to be fused, which is cumbersome and doesn't
///   do much (see https://users.rust-lang.org/t/why-doesnt-tokio-select-require-fusedfuture/46975/7).
/// - Doing a select on two futures is the most common case.
///
/// This select is biased given it is implemented with [futures::future::select], so first
/// future is prefered on simultaneous completion.
/// For instance, a loop with a select on a queue first and then a cancel signal will
/// struggle to stop if the queue is always full.
#[macro_export]
macro_rules! select2_biased {
    (
        $(_)? $($r1:ident)? = $f1:expr => $e1:expr,
        $(_)? $($r2:ident)? = $f2:expr => $e2:expr $(,)?
    ) => {
        {
        let f1 = $f1;
        let f2 = $f2;
        $crate::pin_mut!(f1);
        $crate::pin_mut!(f2);

        match $crate::future::select(f1, f2).await {
            $crate::future::Either::Left((_r1, _f2)) => { $(let $r1 = _r1; )?  $e1 },
            $crate::future::Either::Right((_r2, _f1)) => { $(let $r2 = _r2; )?  $e2 },
        }
        }
    };
}

/// Poor's man version of `tokio::select!`
///
/// Just like for select2, it is implemented with [futures::future::select] so
/// it is definitely no fair (first future gets polled much more than the last one).
///
/// This should be okay as long as the futures don't wrap an actual coroutine but
/// instead correspond to sleep or event wait.
#[macro_export]
macro_rules! select3_biased {
    (
        $(_)? $($r1:ident)? = $f1:expr => $e1:expr,
        $(_)? $($r2:ident)? = $f2:expr => $e2:expr,
        $(_)? $($r3:ident)? = $f3:expr => $e3:expr $(,)?
    ) => {
        {
        let f1 = $f1;
        let f2 = $f2;
        let f3 = $f3;
        $crate::pin_mut!(f1);
        $crate::pin_mut!(f2);
        $crate::pin_mut!(f3);

        let f23 = async move {
            $crate::future::select(
                f2,
                f3,
            ).await
        };
        $crate::pin_mut!(f23);

        let outcome = $crate::future::select(f1, f23).await;

        match outcome {
            $crate::future::Either::Left((_r1, _f23)) => { $(let $r1 = _r1; )?  $e1 },
            $crate::future::Either::Right((r23, _f1)) =>  match r23 {
                $crate::future::Either::Left((_r2, _f3)) => { $(let $r2 = _r2; )?  $e2 },
                $crate::future::Either::Right((_r3, _f2)) => { $(let $r3 = _r3; )?  $e3 },
            }
        }
        }
    };
}

// Platform specific stuff

#[cfg(target_arch = "wasm32")]
pub use platform::WithTaskIDFuture;
pub use platform::{
    pretend_future_is_send_on_web, pretend_stream_is_send_on_web, sleep, spawn, try_task_id,
    yield_now, AbortHandle, Instant, JoinHandle, TaskID,
};

pub use std::time::Duration; // Re-exposed to simplify use of `sleep`

#[cfg(target_arch = "wasm32")]
pub type BoxStream<'a, R> = futures::stream::LocalBoxStream<'a, R>;
#[cfg(not(target_arch = "wasm32"))]
pub type BoxStream<'a, R> = futures::stream::BoxStream<'a, R>;

/// Helper used when having async methods in trait, e.g.:
/// ```ignore
/// trait MyTrait: std::fmt::Debug + Send + Sync {
///     fn do_something_async(
///         &self,
///     ) -> PinBoxFuture<u32>;
/// }
/// ```
///
/// Note we cannot use `async fn` in the traits since it is not compatible
/// with dyn object (and we need to store the object implementing the trait
/// as `Arc<dyn AccountVaultOperations>`).
pub type PinBoxFuture<O> = std::pin::Pin<Box<dyn std::future::Future<Output = O> + Send>>;
pub type PinBoxFutureResult<O, E> = PinBoxFuture<Result<O, E>>;

#[cfg(test)]
#[path = "../tests/unit/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
