// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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

pub(super) use impl_key_debug;
