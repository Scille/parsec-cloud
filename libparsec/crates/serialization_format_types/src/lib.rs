// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub mod ext_types;
pub mod rmp_serialize;
mod format;
mod id;
mod maybe;
mod protocol;

pub use format::*;
pub use id::*;
pub use maybe::*;
pub use protocol::*;

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
