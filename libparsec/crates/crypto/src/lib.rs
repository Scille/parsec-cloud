// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod common;

// Default to RustCrypto as it is much more convenient for dev ;-)
#[cfg(not(feature = "use-libsodium"))]
mod rustcrypto;
#[cfg(feature = "use-libsodium")]
mod sodium;

pub mod prelude {
    pub use crate::common::*;
    #[cfg(not(feature = "use-libsodium"))]
    pub use crate::rustcrypto::*;
    #[cfg(feature = "use-libsodium")]
    pub use crate::sodium::*;
}

pub use prelude::*;

#[cfg(test)]
#[path = "../tests/unit/mod.rs"]
// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#[allow(clippy::unwrap_used)]
mod tests;
