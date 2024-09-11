// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![doc = include_str!("../README.md")]
mod client;
mod invite;
mod monitors;
mod user;
// Workspaces can be started & accessed independently of each other, so we expose it directly
pub use libparsec_client_workspace as workspace;

// For clarity, user & certificate stuff are re-exposed through client
pub use client::*;
pub use invite::*;
use libparsec_client_certif as certif;
pub use libparsec_client_certif::*;
pub use libparsec_client_common::config::*;
use libparsec_client_common::event_bus;
pub use libparsec_client_common::event_bus::*;
