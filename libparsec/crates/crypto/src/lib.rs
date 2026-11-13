// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

mod common;

// Default to RustCrypto as it is much more convenient for dev ;-)
#[cfg(not(crypto_backend = "libsodium"))]
mod rustcrypto;
#[cfg(crypto_backend = "libsodium")]
mod sodium;

pub mod prelude {
    pub use crate::common::*;
    cfg_select! {
        crypto_backend = "libsodium" => {
            pub use crate::sodium::*;
        }
        _ => {
            pub use crate::rustcrypto::*;
        }
    }
}

pub use prelude::*;

#[cfg(test)]
#[path = "../tests/unit/mod.rs"]
// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#[allow(clippy::unwrap_used)]
mod tests;
