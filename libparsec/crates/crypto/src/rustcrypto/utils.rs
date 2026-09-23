// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use blake2::{Blake2b, Digest};
use generic_array::{
    ArrayLength, GenericArray,
    typenum::{IsLessOrEqual, LeEq, NonZero, consts::U64},
};
use rand::{RngCore, rngs::OsRng};

pub(crate) fn generate_rand(out: &mut [u8]) {
    OsRng.fill_bytes(out);
}

pub(crate) fn blake2b_hash<'a, Size>(data: impl Iterator<Item = &'a [u8]>) -> GenericArray<u8, Size>
where
    Size: ArrayLength<u8> + IsLessOrEqual<U64>,
    LeEq<Size, U64>: NonZero,
{
    let mut hasher = Blake2b::<Size>::new();
    for part in data {
        hasher.update(part);
    }
    hasher.finalize()
}
