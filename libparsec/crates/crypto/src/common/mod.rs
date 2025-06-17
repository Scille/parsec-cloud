// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod error;
mod key_derivation;
mod password;
mod sas;
mod secret;
mod sequester;

pub use error::*;
pub(crate) use key_derivation::*;
pub use password::*;
pub use sas::*;
pub(crate) use secret::*;
pub use sequester::*;

#[macro_export]
macro_rules! impl_key_debug {
    ($name: ident) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                // Hide secrets from debug output.
                write!(formatter, concat!(stringify!($name), "(****)"))
            }
        }
    };
}

pub use impl_key_debug;

#[derive(Clone)]
pub struct SecretKeyPassphrase(zeroize::Zeroizing<String>);

impl std::ops::Deref for SecretKeyPassphrase {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SecretKeyPassphrase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for SecretKeyPassphrase {
    fn from(password: String) -> Self {
        Self(password.into())
    }
}

impl From<&str> for SecretKeyPassphrase {
    fn from(password: &str) -> Self {
        Self(password.to_string().into())
    }
}

impl_key_debug!(SecretKeyPassphrase);

#[derive(Clone, PartialEq, Eq)]
pub struct Password(zeroize::Zeroizing<String>);

impl std::ops::Deref for Password {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Password {
    fn from(password: String) -> Self {
        Self(password.into())
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl_key_debug!(Password);
