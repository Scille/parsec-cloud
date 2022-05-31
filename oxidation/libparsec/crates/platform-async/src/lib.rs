// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native as platform;

#[cfg(target_arch = "wasm32")]
pub mod wasm32;
#[cfg(target_arch = "wasm32")]
pub use wasm32 as platform;

pub use flume as channel;
pub use platform::join_set::JoinSet;
pub use platform::sync::Notify;
pub use platform::task::{spawn, Task};
pub use platform::timer::Timer;
