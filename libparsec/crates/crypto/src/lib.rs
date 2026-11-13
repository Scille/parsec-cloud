// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

mod common;

// Default to RustCrypto as it is much more convenient for dev ;-)
#[cfg(feature = "use-rustcrypto")]
mod rustcrypto;
#[cfg(feature = "use-libsodium")]
mod sodium;
#[cfg(all(feature = "use-rustcrypto", feature = "use-libsodium"))]
compile_error!("`use-rustcrypto` and `use-libsodium` cannot be enabled at the same time");

pub mod prelude {
    pub use crate::common::*;
    cfg_select! {
        feature = "use-rustcrypto" =>  {
            pub use crate::rustcrypto::*;
        }
        feature = "use-libsodium" => {
            pub use crate::sodium::*;
        }
        _ => {
            compile_error!(
                "No crypto backend enable, you need to enable either `use-rustcrypto` or `use-libsodium`"
            );
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
