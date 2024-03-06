// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: UNIX implementation is linux-only for the moment
#[cfg(target_os = "linux")]
mod unix;
// Windows is mocked, so we can use it for macOS too for the moment...
#[cfg(any(target_os = "windows", target_os = "macos"))]
mod windows;

#[cfg(target_os = "linux")]
pub(crate) use unix as platform;
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub(crate) use windows as platform;

// TODO: implement for Windows & macOS !
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[cfg(test)]
pub(crate) use platform::LOOKUP_HOOK;

pub use platform::Mountpoint;

// TODO: implement for Windows & macOS !
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[cfg(test)]
#[path = "../tests/unit/operations/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
