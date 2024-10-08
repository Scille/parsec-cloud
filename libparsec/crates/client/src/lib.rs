// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![doc = include_str!("../README.md")]
mod certif;
mod client;
mod config;
mod device;
mod event_bus;
mod invite;
mod monitors;
mod user;

// Workspaces can be started & accessed independently of each other, so we expose it directly
pub mod workspace;

// For clarity, user & certificate stuff are re-exposed through client
pub use certif::*;
pub use client::*;
pub use config::*;
pub use device::remove_device;
pub use event_bus::*;
pub use invite::*;
