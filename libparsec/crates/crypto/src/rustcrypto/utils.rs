// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use argon2::{Algorithm, Argon2, Params, Version};
use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};
use rand::{rngs::OsRng, RngCore};

use crate::{CryptoError, Password};

pub fn generate_nonce() -> Vec<u8> {
    let mut nonce = vec![0; 64];
    OsRng.fill_bytes(&mut nonce);

    nonce
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

    let argon = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(memlimit_kb, opslimit, parallelism, Some(Size::to_usize()))
            .map_err(|_| CryptoError::DataSize)?,
    );

    argon
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|_| CryptoError::DataSize)?;

    Ok(key)
}
