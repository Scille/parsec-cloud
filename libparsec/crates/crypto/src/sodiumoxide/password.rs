// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};
use sodiumoxide::crypto::pwhash::argon2id13::{derive_key, MemLimit, OpsLimit, Salt};

use crate::{CryptoError, Password, PasswordAlgorithm};

pub(crate) fn compute_from_password<Size>(
    algo: &PasswordAlgorithm,
    password: &Password,
) -> Result<GenericArray<u8, Size>, CryptoError>
where
    Size: ArrayLength<u8> + IsLessOrEqual<U64>,
    LeEq<Size, U64>: NonZero,
{
    match algo {
        PasswordAlgorithm::Argon2id {
            salt,
            opslimit,
            memlimit_kb,
            parallelism,
        } => {
            let mut key = GenericArray::<u8, Size>::default();

            // Libsodium only support parallelism of 1
            if *parallelism != 1 {
                return Err(CryptoError::DataSize);
            }

            let salt = Salt::from_slice(salt).ok_or(CryptoError::DataSize)?;

            let opslimit = OpsLimit((*opslimit).try_into().map_err(|_| CryptoError::DataSize)?);
            let memlimit_kb: usize = (*memlimit_kb)
                .try_into()
                .map_err(|_| CryptoError::DataSize)?;
            let memlimit = MemLimit(memlimit_kb * 1024);

            derive_key(&mut key, password.as_bytes(), &salt, opslimit, memlimit)
                .map_err(|_| CryptoError::DataSize)?;

            Ok(key)
        }
    }
}
