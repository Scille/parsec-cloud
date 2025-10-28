// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![doc = include_str!("../README.md")]

mod certif;
mod client;
mod config;
mod device;
mod event_bus;
mod invite;
mod monitors;
mod server_fetch;
mod user;

// Workspaces can be started & accessed independently of each other, so we expose it directly
pub mod workspace;
pub mod workspace_history;

// For clarity, user & certificate stuff are re-exposed through client
pub use certif::*;
pub use client::*;
pub use config::*;
pub use device::remove_device;
pub use event_bus::*;
pub use invite::*;
pub use workspace_history::*;

// Re-expose device save/access objects since they are part of this crate interface
pub use libparsec_platform_device_loader::{
    AvailableDevice, AvailableDeviceType, DeviceAccessStrategy, DeviceSaveStrategy,
};

// Testing on web requires this macro configuration to be present anywhere in
// the crate.
// In most cases, we put it in `tests/unit/mod.rs` and call it a day. However
// this crate doesn't have such file, so we put it here instead...
#[cfg(all(test, target_arch = "wasm32"))]
libparsec_tests_lite::platform::wasm_bindgen_test_configure!(run_in_browser);
