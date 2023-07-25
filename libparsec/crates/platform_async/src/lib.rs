// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native as platform;

#[cfg(target_arch = "wasm32")]
pub mod wasm32;
#[cfg(target_arch = "wasm32")]
pub use wasm32 as platform;

pub use flume as channel;
#[cfg(target_arch = "wasm32")]
pub use futures::lock::{Mutex, MutexGuard};
pub use futures::{self, prelude::*, select};
pub use platform::{
    join_set::JoinSet,
    sleep,
    sync::{watch, Notify},
    task::{spawn, Task},
};
#[cfg(not(target_arch = "wasm32"))]
pub use tokio::{
    fs::{create_dir_all as tokio_create_dir_all, metadata as tokio_metadata},
    runtime::Runtime,
    sync::{
        mpsc::unbounded_channel,
        watch::{channel, Receiver},
        Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError,
    },
    task::{spawn as tokio_spawn, spawn_blocking as tokio_spawn_blocking, JoinHandle},
};
