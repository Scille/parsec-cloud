// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[macro_use]
extern crate lazy_static;

// Re-expose crypto so that `use libparsec_types::prelude::*` is the single-no-brainer-one-liner™
pub use libparsec_crypto::*;

#[cfg(any(test, feature = "test-fixtures"))]
/// We define test fixtures here and not in a different crate to prevent cyclic dependencies.
/// Given this is just an implementation detail, only `libparsec_test_fixtures` should access
/// this module (and, in turn, 3rd party crates should use `libparsec_test_fixtures`)
pub mod fixtures;

mod addr;
mod certif;
pub mod data_macros;
mod error;
mod ext_types;
mod id;
mod invite;
mod local_device;
mod local_device_file;
mod local_manifest;
mod manifest;
mod message;
mod organization;
mod pki;
mod protocol;
mod regex;
mod time;
mod user;

pub use crate::regex::*;
pub use addr::*;
pub use certif::*;
pub use error::*;
pub use ext_types::*;
pub use id::*;
pub use invite::*;
pub use local_device::*;
pub use local_device_file::*;
pub use local_manifest::*;
pub use manifest::*;
pub use message::*;
pub use organization::*;
pub use pki::*;
pub use protocol::*;
pub use time::*;
pub use user::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientType {
    Authenticated,
    Invited,
    Anonymous,
}

pub mod prelude {
    pub use super::*;
}
