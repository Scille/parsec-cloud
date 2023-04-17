// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[macro_use]
extern crate lazy_static;

#[cfg(any(test, feature = "test-fixtures"))]
/// We define test fixture in that module.
/// We defined those fixtures here and not in a different crate to prevent cyclic dependencies.
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
pub use time::*;
pub use user::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientType {
    Authenticated,
    Invited,
    Anonymous,
}
