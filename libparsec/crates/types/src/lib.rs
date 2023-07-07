// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[macro_use]
extern crate lazy_static;

// Convenient dependencies we are going to need everywhere
/// ThisError vs Anyhow ?
/// `anyhow::Error` is your friend: the idea is when call a function we can have 3 outcomes:
/// - success, wrap it with `Result::Ok` and you're good ^^
/// - error we care about, your function should have a dedicated error listing those
///   errors so that caller can do pattern matching on the result
/// - error we don't care about, that's where we want to use anyhow !
///
/// Typically you should end-up with something like:
/// ```
/// #[derive(Debug, thiserror::Error)]
/// pub enum AddMoreSpamError {
///     #[error("Not enough spam")]
///     NotEnoughSpam,
///     #[error(transparent)]
///     Internal(#[from] anyhow::Error),
/// }
/// ```
pub use anyhow;
pub use bytes::Bytes;
pub use thiserror;

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

// Numeric types aliases for the data in the schemas.
// We use them instead of standard Rust types for readability and to ease changing
// their size.
// Changing the size of those types (i.e. switching from `u32` to `u64`) won't
// break compatibility (given msgpack try to use as few bytes as possible to
// encode a value, for instance `1u64` and `1u32` both get encoded as `0x01`),
// the only risk is to have a type too small for the value to decode (hence the
// need to change it in the future if we misjudged the needed size...)
pub type Integer = i64;
pub type Float = f64;
pub type VersionInt = u32;
pub type SizeInt = u64;
pub type IndexInt = u64;
// // Index starts at 1, hence offset on index starts at 0
// pub type IndexInt = std::num::NonZeroU64;
// pub type IndexOffsetInt = u64;

pub mod prelude {
    pub use super::*;
}
