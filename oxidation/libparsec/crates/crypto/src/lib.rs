// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod error;
#[cfg_attr(feature = "use-sodiumoxide", path = "sodiumoxide/mod.rs")]
#[cfg_attr(feature = "use-rustcrypto", path = "rustcrypto/mod.rs")]
mod implementation;
mod macros;

pub use error::*;
pub use implementation::*;
