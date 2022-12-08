// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
use native as platform;

#[cfg(target_arch = "wasm32")]
mod wasm32;
#[cfg(target_arch = "wasm32")]
use wasm32 as platform;

pub use flume as channel;
pub use futures::prelude::*;
pub use futures::select;
pub use platform::join_set::JoinSet;
pub use platform::sleep;
pub use platform::sync::watch;
pub use platform::sync::Notify;
pub use platform::task::{spawn, Task};
