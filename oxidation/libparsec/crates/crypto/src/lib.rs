// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(all(not(feature = "use-sodiumoxide"), not(feature = "use-rustcrypto")))]
compile_error!("feature `use-sodiumoxide` or `use-rustcrypto` is mandatory");
#[cfg(all(feature = "use-sodiumoxide", feature = "use-rustcrypto"))]
compile_error!("features `use-sodiumoxide` and `use-rustcrypto` are mutually exclusive");

#[cfg(any(
    all(feature = "use-rustcrypto", not(feature = "use-sodiumoxide")),
    all(feature = "use-sodiumoxide", not(feature = "use-rustcrypto")),
))]
mod common;
#[cfg(all(feature = "use-rustcrypto", not(feature = "use-sodiumoxide")))]
mod rustcrypto;
#[cfg(all(feature = "use-sodiumoxide", not(feature = "use-rustcrypto")))]
mod sodiumoxide;

#[cfg(any(
    all(feature = "use-rustcrypto", not(feature = "use-sodiumoxide")),
    all(feature = "use-sodiumoxide", not(feature = "use-rustcrypto")),
))]
pub mod prelude {
    pub use crate::common::*;
    #[cfg(all(feature = "use-sodiumoxide", not(feature = "use-rustcrypto")))]
    pub use crate::sodiumoxide::*;
    #[cfg(all(feature = "use-rustcrypto", not(feature = "use-sodiumoxide")))]
    pub use crate::rustcrypto::*;
}

#[cfg(any(
    all(feature = "use-rustcrypto", not(feature = "use-sodiumoxide")),
    all(feature = "use-sodiumoxide", not(feature = "use-rustcrypto")),
))]
pub use prelude::*;
