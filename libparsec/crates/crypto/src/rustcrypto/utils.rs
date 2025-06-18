// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use blake2::Blake2bMac;
use digest::Mac;
use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};
use rand::{rngs::OsRng, RngCore};

pub(crate) fn generate_rand(out: &mut [u8]) {
    OsRng.fill_bytes(out);
}

pub(crate) fn blake2b_hash<'a, Size>(data: impl Iterator<Item = &'a [u8]>) -> GenericArray<u8, Size>
where
    Size: ArrayLength<u8> + IsLessOrEqual<U64>,
    LeEq<Size, U64>: NonZero,
{
    let mut hasher = Blake2bMac::new_with_salt_and_personal(b"", b"", b"").expect("valid params");
    for part in data {
        hasher.update(part);
    }
    let res = hasher.finalize();
    res.into_bytes()
}
