// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use sodiumoxide::randombytes::randombytes_into;

macro_rules! impl_try_from {
    ($name: ident, $sub_type: expr) => {
        impl TryFrom<&[u8]> for $name {
            type Error = CryptoError;

            fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
                let arr: [u8; Self::SIZE] = v.try_into().map_err(|_| CryptoError::DataSize)?;
                Ok(Self($sub_type(arr)))
            }
        }

        impl From<[u8; Self::SIZE]> for $name {
            fn from(key: [u8; Self::SIZE]) -> Self {
                Self($sub_type(key))
            }
        }
    };
}

pub(super) use impl_try_from;

pub(crate) fn generate_rand(out: &mut [u8]) {
    randombytes_into(out)
}
