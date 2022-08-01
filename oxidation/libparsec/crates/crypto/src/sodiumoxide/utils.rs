// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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
