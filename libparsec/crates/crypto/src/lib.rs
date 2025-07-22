// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod common;

// Default to RustCrypto as it is much more convenient for dev ;-)
#[cfg(not(feature = "use-libsodium"))]
mod rustcrypto;
#[cfg(feature = "use-libsodium")]
mod sodiumoxide;

pub mod prelude {
    pub use crate::common::*;
    #[cfg(not(feature = "use-libsodium"))]
    pub use crate::rustcrypto::*;
    #[cfg(feature = "use-libsodium")]
    pub use crate::sodiumoxide::*;
}

pub use prelude::*;
