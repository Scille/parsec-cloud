// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod common;

// Default to RustCrypto as it is much more convenient for dev ;-)
#[cfg(not(feature = "use-sodiumoxide"))]
mod rustcrypto;
#[cfg(feature = "use-sodiumoxide")]
mod sodiumoxide;

pub mod prelude {
    pub use crate::common::*;
    #[cfg(not(feature = "use-sodiumoxide"))]
    pub use crate::rustcrypto::*;
    #[cfg(feature = "use-sodiumoxide")]
    pub use crate::sodiumoxide::*;
}

pub use prelude::*;
