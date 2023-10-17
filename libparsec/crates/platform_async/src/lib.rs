// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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
pub use futures_lite::future;
pub use futures_lite::stream;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// Poor's man version of `tokio::select!`
/// Doing a select on two futures is the most common case.
/// Note it is implemented with (with a [futures_lite::future::Or] so first future
/// is prefered on simultaneous completion).
/// For more complicated use case, implement by hand a similar code at call site ;-)
#[macro_export]
macro_rules! select2 {
    (
        $(_)? $($r1:ident)? = $f1:expr => $e1:expr,
        $(_)? $($r2:ident)? = $f2:expr => $e2:expr $(,)?
    ) => {
        {
        use $crate::future::FutureExt;
        let f1 = async {
            $crate::Either::Left($f1.await)
        };
        let f2 = async {
            $crate::Either::Right($f2.await)
        };
        match f1.or(f2).await {
            $crate::Either::Left(r1) => { $(let $r1 = r1; )?  $e1 }
            $crate::Either::Right(r2) => { $(let $r2 = r2; )? $e2 }
        }
        }
    };
}

// Platform specific stuff

pub use platform::{oneshot, sleep, spawn, watch, Delay, JoinHandle};
pub use std::time::Duration; // Re-exposed to simplify use of `sleep`

#[cfg(target_arch = "wasm32")]
pub type BoxStream<'a, R> = futures_core::stream::LocalBoxStream<'a, R>;
#[cfg(not(target_arch = "wasm32"))]
pub type BoxStream<'a, R> = futures_core::stream::BoxStream<'a, R>;
