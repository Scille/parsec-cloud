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
pub use rmpv;
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
pub const BASE32_ALPHABET: &[u8; 32] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

// Serialization format support modules
pub mod ext_types;
mod format;
mod maybe;
pub mod rmp_serialize;

pub use format::FormatError;
pub use maybe::*;

// Numeric types aliases for the data in the schemas.
pub type Integer = i64;
pub type Float = f64;
pub type VersionInt = u32;
pub type SizeInt = u64;
pub type IndexInt = u64;

// Core types
mod addr;
pub mod error;
mod fs_path;
mod id;
mod invite;
mod openbao;
mod organization;
mod pki;
mod prevent_sync_pattern;
mod protocol;
mod round_robin_cache;
mod sas_code;
mod time;
mod token;

pub use addr::*;
pub use error::*;
pub use fs_path::*;
pub use id::*;
pub use invite::*;
pub use openbao::*;
pub use organization::*;
pub use pki::*;
pub use prevent_sync_pattern::*;
pub use protocol::*;
pub use round_robin_cache::*;
pub use sas_code::*;
pub use time::*;
pub use token::*;

#[cfg(any(test, feature = "test-fixtures"))]
pub mod fixtures;

pub mod prelude {
    pub use super::*;
    pub use anyhow::Context;
}
