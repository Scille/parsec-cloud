// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Re-export everything from types_lite
pub use libparsec_types_lite::*;
// Explicit re-export for `$crate::error` paths used in macros
pub use libparsec_types_lite::error;

#[cfg(any(test, feature = "test-fixtures"))]
/// We define test fixtures here and not in a different crate to prevent cyclic dependencies.
///
/// Given this is just an implementation detail, only `libparsec_test_fixtures` should access
/// this module (and, in turn, 3rd party crates should use `libparsec_test_fixtures`)
pub mod fixtures;

mod account;
mod async_enrollment;
mod certif;
pub mod data_macros;
mod invite;
mod local_device;
mod local_device_file;
mod local_manifest;
mod manifest;
mod realm;
pub(crate) mod serialization;
mod shamir;
mod utils;

pub use account::*;
pub use async_enrollment::*;
pub use certif::*;
pub use invite::*;
pub use local_device::*;
pub use local_device_file::*;
pub use local_manifest::*;
pub use manifest::*;
pub use realm::*;
pub use shamir::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientType {
    Authenticated,
    Invited,
    Anonymous,
}

pub mod prelude {
    pub use super::*;
    pub use anyhow::Context;
}
