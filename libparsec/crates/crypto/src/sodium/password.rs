// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};
use libsodium_rs::crypto_pwhash::{argon2id, pwhash, ALG_ARGON2ID13};

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
            let opslimit = *opslimit as u64;
            let memlimit = usize::try_from(*memlimit_kb).map_err(|_| CryptoError::DataSize)? * 1024;

            // Libsodium only support parallelism of 1
            if *parallelism != 1 {
                return Err(CryptoError::DataSize);
            }

            debug_assert_eq!(salt.len(), argon2id::SALTBYTES);

            if !(argon2id::OPSLIMIT_MIN..argon2id::OPSLIMIT_MAX).contains(&opslimit) {
                return Err(CryptoError::DataSize);
            }

            if !(argon2id::MEMLIMIT_MIN..argon2id::MEMLIMIT_MAX).contains(&memlimit) {
                return Err(CryptoError::DataSize);
            }

            // We could use `argon2id::pwhash` instead since it use v1.3 but at least here the
            // version is explicit with `ALG_ARGON2ID13`
            pwhash(
                Size::USIZE,
                password.as_bytes(),
                salt,
                opslimit,
                memlimit,
                ALG_ARGON2ID13,
            )
            .map_err(|_| CryptoError::DataSize)
            .and_then(|key| GenericArray::from_exact_iter(key).ok_or(CryptoError::DataSize))
        }
    }
}
