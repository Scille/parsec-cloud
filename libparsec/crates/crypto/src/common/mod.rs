// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod error;
mod key_derivation;
mod secret;
mod sequester;

pub use error::*;
pub(crate) use key_derivation::*;
pub(crate) use secret::*;
pub use sequester::*;

#[macro_export]
macro_rules! impl_key_debug {
    ($name: ident) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                // Hide secrets from debug output.
                write!(formatter, "{}(****)", stringify!($name))
            }
        }
    };
}

pub use impl_key_debug;

pub type SecretKeyPassphrase = zeroize::Zeroizing<String>;
pub type Password = zeroize::Zeroizing<String>;
