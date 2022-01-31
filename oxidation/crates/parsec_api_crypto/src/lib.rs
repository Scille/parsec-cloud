// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[cfg_attr(feature = "use-sodiumoxide", path = "sodiumoxide/mod.rs")]
#[cfg_attr(feature = "use-rustcrypto", path = "rustcrypto/mod.rs")]
mod implementation;

pub use implementation::*;
