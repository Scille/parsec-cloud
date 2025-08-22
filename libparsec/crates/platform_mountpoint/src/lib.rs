// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![cfg(not(target_arch = "wasm32"))]

#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_family = "unix")]
pub(crate) use unix as platform;
#[cfg(target_os = "windows")]
pub(crate) use windows as platform;

// TODO: implement for Windows !
#[cfg(not(target_os = "windows"))]
#[cfg(test)]
pub(crate) use platform::LOOKUP_HOOK;

pub use platform::clean_base_mountpoint_dir;
pub use platform::Mountpoint;

#[cfg(test)]
#[path = "../tests/unit/operations/mod.rs"]
#[allow(clippy::unwrap_used)]
mod operations;
