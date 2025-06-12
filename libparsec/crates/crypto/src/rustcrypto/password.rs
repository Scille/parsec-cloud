// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use argon2::{Algorithm, Argon2, Params, Version};
use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};

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

            let argon = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(
                    *memlimit_kb,
                    *opslimit,
                    *parallelism,
                    Some(Size::to_usize()),
                )
                .map_err(|_| CryptoError::DataSize)?,
            );

            argon
                .hash_password_into(password.as_bytes(), salt, &mut key)
                .map_err(|_| CryptoError::DataSize)?;

            Ok(key)
        }
    }
}
