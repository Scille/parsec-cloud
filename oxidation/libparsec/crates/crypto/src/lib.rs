// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(all(not(feature = "use-sodiumoxide"), not(feature = "use-rustcrypto")))]
compile_error!("feature `use-sodiumoxide` or `use-rustcrypto` is mandatory");
#[cfg(all(feature = "use-sodiumoxide", feature = "use-rustcrypto"))]
compile_error!("features `use-sodiumoxide` and `use-rustcrypto` are mutually exclusive");

mod common;
mod error;
mod macros;
#[cfg(all(feature = "use-rustcrypto", not(feature = "use-sodiumoxide")))]
mod rustcrypto;
#[cfg(all(feature = "use-sodiumoxide", not(feature = "use-rustcrypto")))]
mod sodiumoxide;

#[cfg(all(feature = "use-sodiumoxide", not(feature = "use-rustcrypto")))]
pub use crate::sodiumoxide::*;
pub use error::*;
#[cfg(all(feature = "use-rustcrypto", not(feature = "use-sodiumoxide")))]
pub use rustcrypto::*;

pub mod prelude {
    pub use super::common::*;
}

/// 34 symbols to 32 values due to 0/O and 1/I
const RECOVERY_PASSPHRASE_SYMBOLS: [char; 34] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7',
];
