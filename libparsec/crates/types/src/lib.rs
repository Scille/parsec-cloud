// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Currently only used when generating the mocked version of the time provider
#[cfg(feature = "test-mock-time")]
#[macro_use]
extern crate lazy_static;

// Convenient dependencies we are going to need everywhere
/// ThisError vs Anyhow ?
/// `anyhow::Error` is your friend: the idea is when calling a function we can have 3 outcomes:
/// - success, wrap it with `Result::Ok` and you're good ^^
/// - error we care about, your function should have a dedicated error for each one of those,
///   so that the caller can do pattern matching on the result
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
pub use bytes;
pub use bytes::Bytes;
pub use uuid;

// Re-expose crypto so that `use libparsec_types::prelude::*` is the single-no-brainer-one-liner™
pub use libparsec_crypto::*;
// The realm keys bundle contains a key derivation secret that cannot be used as-is,
// the idea is instead to use the ID of the vlob/block to derivate the key to encrypt
// them with.
// However we also need to derivate keys for other purposes where no ID is available,
// hence those constants.
pub const CANARY_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("00000000-0000-0000-0000-000000000001");
pub const REALM_RENAME_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("00000000-0000-0000-0000-000000000002");
pub const PATH_URL_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("00000000-0000-0000-0000-000000000003");

/// Base32 alphabet used for human facing codes (see `SASCode` & `ValidationCode`)
/// (Note I/1 and 0/O are skipped to avoid visual confusion)
const BASE32_ALPHABET: &[u8; 32] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

#[cfg(any(test, feature = "test-fixtures"))]
/// We define test fixtures here and not in a different crate to prevent cyclic dependencies.
///
/// Given this is just an implementation detail, only `libparsec_test_fixtures` should access
/// this module (and, in turn, 3rd party crates should use `libparsec_test_fixtures`)
pub mod fixtures;

mod account;
mod addr;
mod async_enrollment;
mod certif;
pub mod data_macros;
mod error;
mod ext_types;
mod fs_path;
mod id;
mod invite;
mod local_device;
mod local_device_file;
mod local_manifest;
mod manifest;
mod openbao;
mod organization;
mod pki;
mod prevent_sync_pattern;
mod protocol;
mod realm;
mod round_robin_cache;
mod sas_code;
mod serialization;
mod shamir;
mod time;
mod token;
mod utils;

pub use account::*;
pub use addr::*;
pub use async_enrollment::*;
pub use certif::*;
pub use error::*;
pub use ext_types::*;
pub use fs_path::*;
pub use id::*;
pub use invite::*;
pub use local_device::*;
pub use local_device_file::*;
pub use local_manifest::*;
pub use manifest::*;
pub use openbao::*;
pub use organization::*;
pub use pki::*;
pub use prevent_sync_pattern::*;
pub use protocol::*;
pub use realm::*;
pub use round_robin_cache::*;
pub use sas_code::*;
pub use shamir::*;
pub use time::*;
pub use token::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientType {
    Authenticated,
    Invited,
    Anonymous,
}

// Re-export types from `libparsec_serialization_format_types` so that downstream
// crates can keep using `libparsec_types::Integer` etc.
pub use libparsec_serialization_format_types::{
    rmp_serialize, Float, IndexInt, Integer, Maybe, SizeInt, VersionInt,
};

pub mod prelude {
    pub use super::*;
    pub use anyhow::Context;
}
