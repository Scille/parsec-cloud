// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};
use sodiumoxide::{
    crypto::pwhash::argon2id13::{derive_key, MemLimit, OpsLimit, Salt},
    randombytes::randombytes,
};

use crate::{CryptoError, Password};

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

pub fn generate_nonce() -> Vec<u8> {
    randombytes(64)
}

pub fn from_argon2id_password<Size>(
    password: &Password,
    salt: &[u8],
    opslimit: u32,
    memlimit_kb: u32,
    parallelism: u32,
) -> Result<GenericArray<u8, Size>, CryptoError>
where
    Size: ArrayLength<u8> + IsLessOrEqual<U64>,
    LeEq<Size, U64>: NonZero,
{
    let mut key = GenericArray::<u8, Size>::default();

    // Libsodium only support parallelism of 1
    if parallelism != 1 {
        return Err(CryptoError::DataSize);
    }

    let salt = Salt::from_slice(salt).ok_or(CryptoError::DataSize)?;

    let opslimit = OpsLimit(opslimit.try_into().map_err(|_| CryptoError::DataSize)?);
    let memlimit_kb: usize = memlimit_kb.try_into().map_err(|_| CryptoError::DataSize)?;
    let memlimit = MemLimit(memlimit_kb * 1024);

    derive_key(&mut key, password.as_bytes(), &salt, opslimit, memlimit)
        .map_err(|_| CryptoError::DataSize)?;

    Ok(key)
}
