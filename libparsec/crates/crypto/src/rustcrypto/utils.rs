// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use blake2::{Blake2b, Digest};
use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};
use rand::{rngs::SysRng, TryRng};

pub(crate) fn generate_rand(out: &mut [u8]) {
    SysRng.try_fill_bytes(out).expect("Failed to fill bytes")
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
