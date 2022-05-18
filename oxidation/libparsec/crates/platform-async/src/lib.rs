// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native as platform;

pub use flume as channel;
pub use platform::Notify;
pub use platform::Timer;
pub use platform::{spawn, JoinSet, Task};
