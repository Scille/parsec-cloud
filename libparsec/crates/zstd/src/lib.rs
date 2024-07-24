// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Re-expose the bare minimum used in `libparsec/crates/types/src/serialization.rs`
// to simplify the alternative implementation.
#[cfg(not(use_pure_rust_but_dirty_zstd))]
pub mod stream {
    pub use zstd::stream::{copy_encode, decode_all};
}

#[cfg(any(test, use_pure_rust_but_dirty_zstd))]
mod dirty;

#[cfg(use_pure_rust_but_dirty_zstd)]
pub mod stream {
    pub use crate::dirty::{copy_encode, decode_all};
}
