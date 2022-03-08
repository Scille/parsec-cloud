// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

macro_rules! impl_try_from {
    ($name: ident, $sub_type: expr) => {
        impl TryFrom<&[u8]> for $name {
            type Error = &'static str;

            fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
                // if you wonder, `try_into` will also fail if v is too small
                let arr: [u8; Self::SIZE] = v.try_into().map_err(|_| "Invalid data size")?;
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
